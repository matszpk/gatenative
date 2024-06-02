use crate::utils::{gen_var_usage, CircuitTrait, VarAllocator};
use gatesim::*;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::*;

use crate::vbinopcircuit::*;
use crate::vcircuit::*;
use crate::vlop3circuit::*;

// TODO: Later: separate circuit traversal logic (number ways, ways, first way...)
// from gen_var_allocs, gen_func_code into some traits and use that traits.
// If can be better if adding new type of circuits.
// Hint: Join circuit and swap_args together as circuit object.

fn single_var_alloc<T>(var_alloc: &mut VarAllocator<T>, alloc_vars: &mut [Option<T>], var: T)
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let var_u = usize::try_from(var).unwrap();
    if alloc_vars[var_u].is_none() {
        alloc_vars[var_u] = Some(var_alloc.alloc());
    }
    // println!("  Alloc: {:?} {:?}", usize::try_from(var).unwrap(),
    //          alloc_vars[var_u].map(|x| usize::try_from(x).unwrap()));
}

// single variable use - just mark that variable has been used by decreasing its usage.
fn single_var_use<T>(
    var_alloc: &mut VarAllocator<T>,
    alloc_vars: &[Option<T>],
    var_usage: &mut [T],
    var: T,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let var_u = usize::try_from(var).unwrap();
    let mut vu = usize::try_from(var_usage[var_u]).unwrap();
    vu -= 1;
    // println!("  VarUsage: {:?} {:?}", usize::try_from(var).unwrap(), vu);
    var_usage[var_u] = T::try_from(vu).unwrap();
    if vu == 0 {
        // if no further usage
        var_alloc.free(alloc_vars[var_u].unwrap());
    }
}

#[inline]
fn get_bit_place(
    placement: Option<(&[usize], usize)>,
    output_map: Option<&HashMap<usize, usize>>,
    bit: usize,
) -> Option<usize> {
    if let Some(output_map) = output_map {
        if let Some(real_bit) = output_map.get(&bit) {
            Some(placement.map(|(p, _)| p[*real_bit]).unwrap_or(*real_bit))
        } else {
            None
        }
    } else {
        Some(placement.map(|(p, _)| p[bit]).unwrap_or(bit))
    }
}

fn get_input_orig_index_map(
    input_len: usize,
    input_placement: Option<(&[usize], usize)>,
    single_buffer: bool,
    input_map: Option<&HashMap<usize, usize>>,
) -> HashMap<usize, usize> {
    if single_buffer {
        if let Some((input_p, _)) = input_placement {
            if let Some(input_map) = input_map {
                HashMap::from_iter((0..input_len).filter_map(|i| {
                    if let Some(index) = input_map.get(&i) {
                        Some((input_p[*index], i))
                    } else {
                        None
                    }
                }))
            } else {
                HashMap::from_iter(input_p.iter().enumerate().map(|(i, x)| (*x, i)))
            }
        } else {
            HashMap::from_iter((0..input_len).filter_map(|i| {
                if let Some(input_map) = input_map {
                    if let Some(index) = input_map.get(&i) {
                        Some((*index, i))
                    } else {
                        // if not found then ignore, becuase is arg_inputs or elem_inputs
                        None
                    }
                } else {
                    Some((i, i))
                }
            }))
        }
    } else {
        HashMap::new()
    }
}

fn load_input_later(
    input_map: Option<&HashMap<usize, usize>>,
    pop_inputs: Option<&[usize]>,
    gi0: usize,
) -> bool {
    let (pop_input, pop_input_list) = if let Some(pop_inputs) = pop_inputs {
        if !pop_inputs.is_empty() {
            (true, Some(pop_inputs))
        } else {
            (true, None)
        }
    } else {
        (false, None)
    };
    if let Some(pop_input_list) = pop_input_list {
        pop_input_list.binary_search(&gi0).is_err()
    } else {
        !pop_input || !input_map.map(|im| im.contains_key(&gi0)).unwrap_or(true)
    }
}

// output negations generating:
// based on difference between first occurrence of output and nexts occurrences.
// due to differences of final output negations between original Circuit and
// VCircuit and VBinOpCircuit.

// input_map - input map after filtering arg inputs, elem inputs and other.
// keep_output_vars - keep output variables to later usage.
//          if supplied empty array then use all circuits outputs as aggr outputs.
// pop_input - include rest of inputs (without arg inputs, elem inputs) to be loaded
//    from populating code. If supplied empty array then use all circuit inputs as pop inputs.
// returns: variable id after allocation: index - wire index (circuit input and gate outputs)
//          value - id of allocated variable
//          size of allocation (number of allocated variables)
//          optional: map of output variables: key - circuit output number
//                   value - (id of allocated variable for this circuit output,
//                            optional: original variable for first sign occurred in
//                            circuit's output list)
fn gen_var_allocs<T, CT>(
    circuit: &CT,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    var_usage: &mut [T],
    single_buffer: bool,
    input_map: Option<&HashMap<usize, usize>>,
    keep_output_vars: Option<&[usize]>,
    pop_inputs: Option<&[usize]>,
    output_map: Option<&HashMap<usize, usize>>,
    inner_loop: bool,
) -> (
    Vec<T>,
    usize,
    Option<BTreeMap<usize, (usize, Option<usize>)>>,
)
where
    CT: CircuitTrait<T>,
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    #[derive(Clone, Copy)]
    struct StackEntry {
        node: usize,
        way: usize,
    }
    let single_buffer = single_buffer
        && !(inner_loop
            || (keep_output_vars.map(|x| x.is_empty()).unwrap_or(false)
                && pop_inputs.map(|x| x.is_empty()).unwrap_or(false)));
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();
    let output_len = circuit.outputs().len();
    let gate_num = circuit.len();
    let mut alloc_vars: Vec<Option<T>> = vec![None; input_len + gate_num];
    let mut var_alloc = VarAllocator::<T>::new();
    let mut output_vars = if let Some(vars) = keep_output_vars {
        if vars.is_empty() {
            Some(BTreeMap::from_iter((0..output_len).map(|i| (i, (0, None)))))
        } else {
            Some(BTreeMap::from_iter(
                vars.into_iter().map(|i| (*i, (0, None))),
            ))
        }
    } else if inner_loop {
        Some(BTreeMap::from_iter((0..output_len).map(|i| (i, (0, None)))))
    } else {
        None
    };

    let mut visited = vec![false; gate_num];

    // out_map is list of circuit nodes with list of connected circuit outputs.
    // key - node index (wire index), value - list of connected circuit outputs.
    let out_map = {
        let mut out_map = HashMap::<T, Vec<usize>>::new();
        for (i, (o, _)) in circuit.outputs().iter().enumerate() {
            if let Some(outlist) = out_map.get_mut(o) {
                outlist.push(i);
            } else {
                out_map.insert(*o, vec![i]);
            }
        }
        out_map
    };
    // conversion from input placement to original input bit
    let input_orig_index_map =
        get_input_orig_index_map(input_len, input_placement, single_buffer, input_map);

    let mut input_already_read = vec![false; input_len];

    // list of outputs awaits allocation for second value
    let mut outputs_awaits_alloc = BTreeMap::new();

    let is_in_input_map = |i| input_map.map(|im| im.contains_key(&i)).unwrap_or(true);
    // if populated input then allocate variables as first to avoid next allocations
    // if inner_loop allocate all variables as first to allow initialization
    if inner_loop {
        let pop_inputs = pop_inputs.unwrap_or(&[]);
        for i in 0..input_len {
            if is_in_input_map(i) || pop_inputs.iter().any(|x| *x == i) {
                single_var_alloc(&mut var_alloc, &mut alloc_vars, T::try_from(i).unwrap());
                input_already_read[i] = true;
            }
        }
    } else if let Some(pop_inputs) = pop_inputs {
        if !pop_inputs.is_empty() {
            for i in pop_inputs {
                single_var_alloc(&mut var_alloc, &mut alloc_vars, T::try_from(*i).unwrap());
                input_already_read[*i] = true;
            }
        } else {
            for i in 0..input_len {
                if is_in_input_map(i) {
                    single_var_alloc(&mut var_alloc, &mut alloc_vars, T::try_from(i).unwrap());
                    input_already_read[i] = true;
                }
            }
        }
    }

    for (o, _) in circuit.outputs().iter() {
        if *o < input_len_t {
            continue;
        }
        let oidx = usize::try_from(*o).unwrap() - input_len;
        let mut stack = Vec::new();
        stack.push(StackEntry { node: oidx, way: 0 });

        while !stack.is_empty() {
            let top = stack.last_mut().unwrap();
            let node_index = top.node;
            let way = top.way;

            let gi_num = circuit.gate_input_num(node_index);
            if way == 0 {
                if !visited[node_index] {
                    visited[node_index] = true;
                } else {
                    stack.pop();
                    continue;
                }

                top.way += 1;
                let gi0 = circuit.gate_input(node_index, 0);
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way < gi_num {
                top.way += 1;
                let gi1 = circuit.gate_input(node_index, way);
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                // allocate and use
                // allocate circuit inputs now if not allocated
                for ii in 0..gi_num {
                    let gi0t = circuit.gate_op_input(node_index, ii);
                    if gi0t < input_len_t {
                        let gi0 = usize::try_from(gi0t).unwrap();
                        if load_input_later(input_map, pop_inputs, gi0) && !input_already_read[gi0]
                        {
                            single_var_alloc(&mut var_alloc, &mut alloc_vars, gi0t);
                            input_already_read[gi0] = true;
                        }
                    }
                }
                for ii in 0..gi_num {
                    single_var_use(
                        &mut var_alloc,
                        &alloc_vars,
                        var_usage,
                        circuit.gate_op_input(node_index, ii),
                    );
                }
                let tnode = T::try_from(node_index + input_len).unwrap();
                single_var_alloc(&mut var_alloc, &mut alloc_vars, tnode);
                if let Some(outlist) = out_map.get(&tnode) {
                    if single_buffer {
                        for oi in outlist {
                            // check output mapping to not already inputs
                            if let Some(out_p) = get_bit_place(output_placement, output_map, *oi) {
                                if let Some(input_bit) = input_orig_index_map.get(&out_p) {
                                    // if input_bit under output placement is not read
                                    if !input_already_read[*input_bit] {
                                        // println!(
                                        //     "Not already alloc: {}, {} {}: {}",
                                        //     node_index + input_len,
                                        //     *oi,
                                        //     input_bit,
                                        //     input_already_read[*input_bit]
                                        // );
                                        // just input bit must be read now
                                        single_var_alloc(
                                            &mut var_alloc,
                                            &mut alloc_vars,
                                            T::try_from(*input_bit).unwrap(),
                                        );
                                        input_already_read[*input_bit] = true;
                                    }
                                }
                            }
                        }
                    }
                    // use output at this point
                    if let Some(output_vars) = output_vars.as_mut() {
                        let mut use_normal = false;
                        let mut use_neg = false;
                        let circ_outputs = circuit.outputs();
                        let out_var =
                            usize::try_from(alloc_vars[usize::try_from(tnode).unwrap()].unwrap())
                                .unwrap();
                        // check whether both normal and neg
                        for oi in outlist {
                            if output_vars.contains_key(oi) {
                                if circ_outputs[*oi].1 {
                                    use_neg = true;
                                } else {
                                    use_normal = true;
                                }
                            }
                        }
                        // allocate neg_var (for negated out_var)
                        // It is using first occurred sign in output list ordered by
                        // circuit outputs index. Main goal is keeping sign of output
                        // if it can be changed by circuit conversion.
                        let mut first_neg = None;
                        let mut output_used_later = false;
                        for oi in outlist {
                            if let Some(out_var_entry) = output_vars.get_mut(oi) {
                                if first_neg.is_none() {
                                    first_neg = Some(circ_outputs[*oi].1);
                                }
                                if use_neg && use_normal {
                                    // it will be allocated later after releasing other variables
                                    outputs_awaits_alloc
                                        .insert(*oi, (out_var, !first_neg.unwrap()));
                                    if circ_outputs[*oi].1 == first_neg.unwrap() {
                                        // if first sign occurence
                                        *out_var_entry = (out_var, None);
                                    }
                                } else {
                                    *out_var_entry = (out_var, None);
                                }
                                output_used_later = true;
                            }
                        }
                        if !output_used_later {
                            // if this output not used later then use in this
                            single_var_use(&mut var_alloc, &alloc_vars, var_usage, tnode);
                        }
                    } else {
                        single_var_use(&mut var_alloc, &alloc_vars, var_usage, tnode);
                    }
                }
                stack.pop();
            }
        }
    }

    // for outputs just
    for (o, _) in circuit.outputs().iter() {
        if *o < input_len_t {
            single_var_alloc(&mut var_alloc, &mut alloc_vars, *o);
            let outlist = out_map.get(o).unwrap();
            if single_buffer {
                for oi in outlist {
                    // check output mapping to not already inputs
                    if let Some(out_p) = get_bit_place(output_placement, output_map, *oi) {
                        if let Some(input_bit) = input_orig_index_map.get(&out_p) {
                            // if input_bit under output placement is not read
                            if !input_already_read[*input_bit] {
                                // println!(
                                //     "Not already alloc: {}, {} {}: {}",
                                //     usize::try_from(*o).unwrap(),
                                //     *oi,
                                //     input_bit,
                                //     input_already_read[*input_bit]
                                // );
                                // just input bit must be read now
                                single_var_alloc(
                                    &mut var_alloc,
                                    &mut alloc_vars,
                                    T::try_from(*input_bit).unwrap(),
                                );
                                input_already_read[*input_bit] = true;
                            }
                        }
                    }
                }
            }

            if let Some(output_vars) = output_vars.as_mut() {
                let mut use_normal = false;
                let mut use_neg = false;
                let circ_outputs = circuit.outputs();
                let out_var =
                    usize::try_from(alloc_vars[usize::try_from(*o).unwrap()].unwrap()).unwrap();
                // check whether both normal and neg
                for oi in outlist {
                    if output_vars.contains_key(oi) {
                        if circ_outputs[*oi].1 {
                            use_neg = true;
                        } else {
                            use_normal = true;
                        }
                    }
                }
                // allocate neg_var (for negated out_var)
                // It is using first occurred sign in output list ordered by
                // circuit outputs index. Main goal is keeping sign of output
                // if it can be changed by circuit conversion.
                let mut first_neg = None;
                let mut output_used_later = false;
                for oi in outlist {
                    if let Some(out_var_entry) = output_vars.get_mut(oi) {
                        if first_neg.is_none() {
                            first_neg = Some(circ_outputs[*oi].1);
                        }
                        if use_neg && use_normal {
                            // it will be allocated later after releasing other variables
                            outputs_awaits_alloc.insert(*oi, (out_var, !first_neg.unwrap()));
                            if circ_outputs[*oi].1 == first_neg.unwrap() {
                                // if first sign occurence
                                *out_var_entry = (out_var, None);
                            }
                        } else {
                            *out_var_entry = (out_var, None);
                        }
                        output_used_later = true;
                    }
                }
                if !output_used_later {
                    // if this output not used later then use in this
                    single_var_use(&mut var_alloc, &alloc_vars, var_usage, *o);
                }
            } else {
                single_var_use(&mut var_alloc, &alloc_vars, var_usage, *o);
            }
        }
    }

    if !outputs_awaits_alloc.is_empty() {
        // allocate now pending outputs
        let mut second_var_for_outputs = BTreeMap::new();
        for (_, (out_var, _)) in &outputs_awaits_alloc {
            second_var_for_outputs.insert(out_var, None);
        }
        for (_, v) in &mut second_var_for_outputs {
            *v = Some(usize::try_from(var_alloc.alloc()).unwrap());
        }
        let circ_outputs = circuit.outputs();
        if let Some(output_vars) = output_vars.as_mut() {
            for (oi, (out_var, second_neg)) in outputs_awaits_alloc.iter() {
                if circ_outputs[*oi].1 == *second_neg {
                    if let Some(out_var_entry) = output_vars.get_mut(oi) {
                        *out_var_entry = (
                            second_var_for_outputs.get(out_var).unwrap().unwrap(),
                            Some(*out_var),
                        );
                    }
                }
            }
        }
    }

    // DEBUG
    for i in 0..circuit.len() {
        println!(
            "G: {}: {:?} {:?} {} {} {}",
            input_len + i,
            circuit.gate_op(i).0.int_value(),
            circuit.gate_op(i).1,
            usize::try_from(circuit.gate_op_input(i, 0)).unwrap(),
            usize::try_from(circuit.gate_op_input(i, 1)).unwrap(),
            usize::try_from(circuit.gate_op_input(i, 2)).unwrap(),
        );
    }
    for (i, (o, n)) in circuit.outputs().iter().enumerate() {
        println!("O: {}: {} {}", i, usize::try_from(*o).unwrap(), *n);
    }
    // DEBUG
    (
        alloc_vars
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>(),
        var_alloc.len(),
        output_vars,
    )
}

fn gen_copy_to_input<FW: FuncWriter, T>(
    writer: &mut FW,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    var_allocs: &[T],
    extra_swap_var: usize,
    input_map: Option<&HashMap<usize, usize>>,
    output_map: Option<&HashMap<usize, usize>>,
    output_vars: &BTreeMap<usize, (usize, Option<usize>)>,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // println!("CopyToInput Start");
    let input_place_map: HashMap<usize, usize> = if let Some((input_p, _)) = input_placement {
        if let Some(input_map) = input_map {
            HashMap::from_iter((0..input_len).filter_map(|i| {
                if let Some(index) = input_map.get(&i) {
                    Some((input_p[*index], i))
                } else {
                    None
                }
            }))
        } else {
            HashMap::from_iter(input_p.iter().enumerate().map(|(i, x)| (*x, i)))
        }
    } else {
        HashMap::from_iter((0..input_len).filter_map(|i| {
            if let Some(input_map) = input_map {
                if let Some(index) = input_map.get(&i) {
                    Some((*index, i))
                } else {
                    // if not found then ignore, because is arg_inputs or elem_inputs
                    None
                }
            } else {
                Some((i, i))
            }
        }))
    };
    // output to input map
    // key - original output index, value - original input index
    let output_input_map = if let Some(output_map) = output_map {
        output_map
            .iter()
            .map(|(bit, real_bit)| {
                let place = if let Some((output_p, _)) = output_placement {
                    output_p[*real_bit]
                } else {
                    *real_bit
                };
                (*bit, input_place_map[&place])
            })
            .collect::<HashMap<_, _>>()
    } else {
        (0..output_len)
            .map(|bit| {
                let place = if let Some((output_p, _)) = output_placement {
                    output_p[bit]
                } else {
                    bit
                };
                (bit, input_place_map[&place])
            })
            .collect::<HashMap<_, _>>()
    };
    // to map: output_variable to input variable
    // key - circuit output index, value - (circuit output var, circuit input var)
    let out_outvar_invar_map: HashMap<usize, (usize, usize)> =
        HashMap::from_iter(output_input_map.iter().map(|(oi, ii)| {
            (
                *oi,
                (output_vars[oi].0, usize::try_from(var_allocs[*ii]).unwrap()),
            )
        }));
    let mut var_output_map = BTreeMap::<usize, Vec<usize>>::new();
    for (oi, (var, _)) in output_vars.iter() {
        if let Some(oilist) = var_output_map.get_mut(&var) {
            oilist.push(*oi);
        } else {
            var_output_map.insert(*var, vec![*oi]);
        }
    }
    // DEBUG
    // println!("VarOutputMap: {:?}", var_output_map);
    // println!("OutOutVarInVarMap: {:?}", out_outvar_invar_map);
    // DEBUG
    let var_output_map_initial = var_output_map.clone();
    // process outputs
    for (var, root_output_list) in var_output_map_initial {
        //let last_to_process = None;
        // instead stack construct tree:
        //                    /---> outvar3
        //       /---> outvar0----> outvar4
        //       |            /---> outvar5
        // outvar----> outvar1----> outvar6
        //       |            \---> outvar7
        //       \---> outvar2----> outvar8
        //                    \---> outvar9
        // outvar9 = outvar2
        // outvar8 = outvar2
        // outvar7 = outvar1
        // outvar6 = outvar1
        // outvar5 = outvar1
        // outvar4 = outvar0
        // outvar3 = outvar0
        // outvar2 = outvar
        // outvar1 = outvar
        // outvar0 = outvar
        #[derive(Clone, Debug)]
        struct Entry {
            outvar: usize,
            invar: usize,
            entries: Vec<Entry>,
        }
        #[derive(Clone)]
        struct ConstructStackEntry {
            outvar: Option<usize>,
            invar: usize,
            entry: Option<Entry>,
            way: usize,
            cycle: bool,
        }
        // 1. collect tree of dependencies between output var and input var
        let mut dep_tree: Vec<Entry> = vec![];
        let mut stack: Vec<ConstructStackEntry> = vec![ConstructStackEntry {
            outvar: None, //  no given parent
            invar: var,   // in root node is connection to child nodes
            entry: None,
            way: 0,
            cycle: false,
        }];
        let mut cycle_path = None;
        while !stack.is_empty() {
            let top = stack.last_mut().unwrap();
            let mut do_pop = false;
            if top.cycle {
                // skip cycle
                // cycle or if this same variable in next step
                do_pop = true;
            } else if var_output_map.contains_key(&top.outvar.unwrap_or(var)) {
                // it is not already processed
                let output_list = if top.outvar.is_some() {
                    if let Some(output_list) = var_output_map.get(&top.invar) {
                        // conflict with next output
                        Some(&output_list[..])
                    } else {
                        None
                    }
                } else {
                    Some(&root_output_list[..])
                };
                if let Some(output_list) = output_list {
                    // to process children becuase conflict or root
                    let top_way = top.way;
                    if top_way < output_list.len() {
                        top.way += 1;
                        let oi = output_list[top_way];
                        let top_invar = top.invar;
                        if let Some((_, new_invar)) = out_outvar_invar_map.get(&oi) {
                            if *new_invar == var && top_invar != *new_invar {
                                // detected only real cycle, not case if same variable
                                // in next step.
                                cycle_path =
                                    Some(stack.iter().map(|e| e.way - 1).collect::<Vec<_>>());
                            }
                            stack.push(ConstructStackEntry {
                                outvar: Some(top_invar),
                                invar: *new_invar,
                                entry: Some(Entry {
                                    outvar: 0,
                                    invar: 0,
                                    entries: vec![],
                                }),
                                way: 0,
                                // cycle or if this same variable in next step
                                cycle: *new_invar == var,
                            });
                        } else {
                            do_pop = true;
                        }
                    } else {
                        do_pop = true;
                    }
                } else {
                    do_pop = true;
                }
            } else {
                // already processed, skip
                stack.pop();
            }
            if do_pop {
                if let Some(child_top) = stack.pop() {
                    if let Some(top) = stack.last_mut() {
                        let children = if let Some(entry) = top.entry.as_mut() {
                            &mut entry.entries
                        } else {
                            &mut dep_tree
                        };
                        children.push(Entry {
                            outvar: child_top.outvar.unwrap(),
                            invar: child_top.invar,
                            entries: child_top.entry.unwrap().entries.clone(),
                        });
                    }
                }
            }
        }
        // DEBUG
        // println!("Tree: {:?}", dep_tree);
        // DEBUG
        // // 2. move to end path where is cycle
        let have_cycle = cycle_path.is_some();
        if let Some(cycle_path) = cycle_path {
            // println!("Cycle path: {:?}", cycle_path);
            let way0 = *cycle_path.first().unwrap();
            // swap last element and choosen cycle element
            let t = dep_tree.swap_remove(way0);
            dep_tree.push(t);
            let mut entry = dep_tree.last_mut().unwrap();
            for way in cycle_path.into_iter().skip(1) {
                // swap last element and choosen cycle element
                let t = entry.entries.swap_remove(way);
                entry.entries.push(t);
                entry = entry.entries.last_mut().unwrap();
            }
        }
        // DEBUG
        // println!("NewTree: {:?}", dep_tree);
        // DEBUG
        // // 3. make store operation in order of dep tree.
        #[derive(Clone, Debug)]
        struct StackEntry<'a> {
            entry: Option<&'a Entry>,
            way: usize,
        }
        let mut stack = vec![StackEntry {
            entry: None,
            way: 0,
        }];
        while !stack.is_empty() {
            let top = stack.last_mut().unwrap();
            let children = if let Some(entry) = top.entry.as_ref() {
                &entry.entries
            } else {
                &dep_tree
            };
            if top.way < children.len() {
                let top_way = top.way;
                top.way += 1;
                stack.push(StackEntry {
                    entry: Some(&children[top_way]),
                    way: 0,
                });
            } else if let Some(top) = stack.pop() {
                if let Some(entry) = top.entry {
                    if entry.outvar != entry.invar {
                        var_output_map.remove(&entry.outvar);
                        if entry.invar == var {
                            // if cycle detected and last store
                            // then store input var to extra swap
                            writer.gen_set(extra_swap_var, entry.invar);
                        }
                        // if output var and input are not same
                        if stack.len() != 1 || !have_cycle || stack[0].way != dep_tree.len() {
                            // normal store operation
                            writer.gen_set(entry.invar, entry.outvar);
                        } else {
                            // if last operation to store and cycle detected
                            writer.gen_set(entry.invar, extra_swap_var);
                        }
                    }
                }
            }
        }
    }
}

fn gen_func_code_for_circuit<FW: FuncWriter, T, CT>(
    writer: &mut FW,
    circuit: &CT,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    var_allocs: &[T],
    var_num: usize,
    single_buffer: bool,
    input_map: Option<&HashMap<usize, usize>>,
    output_vars: Option<&BTreeMap<usize, (usize, Option<usize>)>>,
    pop_inputs: Option<&[usize]>,
    store_output_vars_always: bool,
    output_map: Option<&HashMap<usize, usize>>,
    inner_loop: bool,
    have_aggr_code: bool,
) where
    CT: CircuitTrait<T>,
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    #[derive(Clone, Copy)]
    struct StackEntry {
        node: usize,
        way: usize,
    }
    // store_output_vars_always - aggr_to_buffer
    let single_buffer = single_buffer
        && !(inner_loop
            || (output_vars.is_some()
                && !store_output_vars_always
                && pop_inputs.map(|x| x.is_empty()).unwrap_or(false)));
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.len();

    // out_map is list of circuit nodes with list of connected circuit outputs.
    // key - node index (wire index), value - list of connected circuit outputs.
    let out_map = {
        let mut out_map = HashMap::<T, Vec<(usize, bool)>>::new();
        for (i, (o, n)) in circuit.outputs().iter().enumerate() {
            if let Some(outlist) = out_map.get_mut(o) {
                outlist.push((i, *n));
            } else {
                out_map.insert(*o, vec![(i, *n)]);
            }
        }
        out_map
    };

    // conversion from input placement to original input bit
    let input_orig_index_map =
        get_input_orig_index_map(input_len, input_placement, single_buffer, input_map);
    let mut used_inputs = vec![false; input_len];
    let is_in_input_map = |i| input_map.map(|im| im.contains_key(&i)).unwrap_or(true);
    let is_in_output_map = |i| output_map.map(|om| om.contains_key(&i)).unwrap_or(true);
    // if populated input then allocate variables as first to avoid next allocations
    // if inner_loop allocate all variables as first to allow initialization
    if inner_loop {
        let pop_input_empty = pop_inputs.map(|x| x.is_empty()).unwrap_or(false);
        if !pop_input_empty {
            // only if pop_input map is not empty (if not all pop_input from input)
            writer.gen_if_loop_start();
            for i in 0..input_len {
                if is_in_input_map(i) {
                    writer.gen_load(usize::try_from(var_allocs[i]).unwrap(), i);
                    used_inputs[i] = true;
                }
            }
            writer.gen_end_if();
        }
    } else if let Some(pop_inputs) = pop_inputs {
        if !pop_inputs.is_empty() {
            for i in pop_inputs {
                used_inputs[*i] = true;
            }
        } else {
            for i in 0..input_len {
                if is_in_input_map(i) {
                    used_inputs[i] = true;
                }
            }
        }
    }

    let mut visited = vec![false; gate_num];
    for (o, _) in circuit.outputs().iter() {
        if *o < input_len_t {
            continue;
        }
        let oidx = usize::try_from(*o).unwrap() - input_len;
        let mut stack = Vec::new();
        stack.push(StackEntry { node: oidx, way: 0 });

        while !stack.is_empty() {
            let top = stack.last_mut().unwrap();
            let node_index = top.node;
            let way = top.way;
            let gi_num = circuit.gate_input_num(node_index);

            if way == 0 {
                if !visited[node_index] {
                    visited[node_index] = true;
                } else {
                    stack.pop();
                    continue;
                }

                top.way += 1;
                let gi0 = circuit.gate_input(node_index, 0);
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way < gi_num {
                top.way += 1;
                let gi1 = circuit.gate_input(node_index, way);
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                for ii in 0..gi_num {
                    let gi0 = usize::try_from(circuit.gate_op_input(node_index, ii)).unwrap();
                    if gi0 < input_len && !used_inputs[gi0] {
                        if load_input_later(input_map, pop_inputs, gi0) {
                            writer.gen_load(usize::try_from(var_allocs[gi0]).unwrap(), gi0);
                            used_inputs[gi0] = true;
                        }
                    }
                }
                let (instr_op, vnegs) = circuit.gate_op(node_index);
                // dst and arg
                let dst = usize::try_from(var_allocs[input_len + node_index]).unwrap();
                let arg0 = usize::try_from(
                    var_allocs[usize::try_from(circuit.gate_op_input(node_index, 0)).unwrap()],
                )
                .unwrap();
                let arg1 = usize::try_from(
                    var_allocs[usize::try_from(circuit.gate_op_input(node_index, 1)).unwrap()],
                )
                .unwrap();
                if instr_op.arg_num() == 2 {
                    writer.gen_op(instr_op, vnegs, dst, arg0, arg1);
                } else if instr_op.arg_num() == 3 {
                    writer.gen_op3(
                        instr_op,
                        dst,
                        arg0,
                        arg1,
                        usize::try_from(
                            var_allocs
                                [usize::try_from(circuit.gate_op_input(node_index, 2)).unwrap()],
                        )
                        .unwrap(),
                    );
                } else {
                    panic!("Unsupported!");
                }
                let tnode = T::try_from(input_len + node_index).unwrap();
                if let Some(outlist) = out_map.get(&tnode) {
                    for (oi, on) in outlist {
                        if single_buffer {
                            // check output mapping to not already inputs
                            if let Some(out_p) = get_bit_place(output_placement, output_map, *oi) {
                                if let Some(input_bit) = input_orig_index_map.get(&out_p) {
                                    // if input_bit under output placement is not read
                                    if !used_inputs[*input_bit] {
                                        writer.gen_load(
                                            usize::try_from(var_allocs[*input_bit]).unwrap(),
                                            *input_bit,
                                        );
                                        used_inputs[*input_bit] = true;
                                    }
                                }
                            }
                        }
                        // if output then store
                        if is_in_output_map(*oi)
                            && (store_output_vars_always || output_vars.is_none())
                        {
                            writer.gen_store(
                                *on,
                                *oi,
                                usize::try_from(var_allocs[input_len + node_index]).unwrap(),
                            );
                        }
                    }
                }
                stack.pop();
            }
        }
    }

    let mut out_negs = HashSet::new();
    let mut out_negs_2 = HashSet::new();
    for (oi, (o, on)) in circuit.outputs().iter().enumerate() {
        if *o < input_len_t {
            if single_buffer {
                // check output mapping to not already inputs
                if let Some(out_p) = get_bit_place(output_placement, output_map, oi) {
                    if let Some(input_bit) = input_orig_index_map.get(&out_p) {
                        // if input_bit under output placement is not read
                        if !used_inputs[*input_bit] {
                            writer.gen_load(
                                usize::try_from(var_allocs[*input_bit]).unwrap(),
                                *input_bit,
                            );
                            used_inputs[*input_bit] = true;
                        }
                    }
                }
            }
            let ou = usize::try_from(*o).unwrap();
            if !used_inputs[ou] {
                writer.gen_load(usize::try_from(var_allocs[ou]).unwrap(), ou);
                used_inputs[ou] = true;
            }
            if is_in_output_map(oi) && (store_output_vars_always || output_vars.is_none()) {
                writer.gen_store(
                    *on,
                    oi,
                    usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap(),
                );
            }
        }
    }
    for (oi, (o, on)) in circuit.outputs().iter().enumerate() {
        if let Some(output_vars) = output_vars {
            if let Some(out_var_entry) = output_vars.get(&oi) {
                if !out_negs.contains(&o) && *on && out_var_entry.1.is_none() {
                    // only if negation neeeded and is first occurred sign.
                    // negate output
                    let v = usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap();
                    writer.gen_not(v, v);
                    out_negs.insert(o);
                }
                if !out_negs_2.contains(&o) {
                    if let Some(orig_var) = out_var_entry.1 {
                        writer.gen_not(out_var_entry.0, orig_var);
                        out_negs_2.insert(o);
                    }
                }
            }
        }
    }
    if inner_loop {
        writer.gen_aggr_output_code();
        writer.gen_if_loop_end();
        if !have_aggr_code || store_output_vars_always {
            for oi in 0..circuit.outputs().len() {
                // if inner loop and if (not aggr_output_code without list)
                //   and if not excluded
                if let Some(output_vars) = output_vars {
                    if let Some(out_var_entry) = output_vars.get(&oi) {
                        if is_in_output_map(oi) {
                            writer.gen_store(false, oi, out_var_entry.0);
                        }
                    }
                }
            }
        }
        writer.gen_else();
        gen_copy_to_input(
            writer,
            input_len,
            circuit.outputs().len(),
            input_placement,
            output_placement,
            var_allocs,
            var_num,
            input_map,
            output_map,
            output_vars.unwrap(),
        );
        writer.gen_end_if();
    }
}

pub fn generate_code_with_config<'a, FW: FuncWriter, CW: CodeWriter<'a, FW>, T>(
    writer: &'a mut CW,
    name: &'a str,
    circuit: Circuit<T>,
    optimize_negs: bool,
    code_config: CodeConfig<'a>,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let supported_ops = writer.supported_ops();
    let basic_ops: u64 = (1u64 << InstrOp::And.int_value())
        | (1u64 << InstrOp::Or.int_value())
        | (1u64 << InstrOp::Xor.int_value());
    assert_eq!(basic_ops, (supported_ops & basic_ops));
    let impl_op = (supported_ops & (1u64 << InstrOp::Impl.int_value())) != 0;
    let nimpl_op = (supported_ops & (1u64 << InstrOp::Nimpl.int_value())) != 0;
    let have_lop3 = supported_ops & (1u64 << InstrOp::Lop3(0).int_value()) != 0;
    // DEBUG
    if have_lop3 {
        println!("HaveLOP3");
    }
    // DEBUG

    // generate input_map
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let input_map = {
        let arg_input_map = if let Some(arg_inputs) = code_config.arg_inputs {
            HashMap::from_iter(arg_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let elem_input_map = if let Some(elem_inputs) = code_config.elem_inputs {
            HashMap::from_iter(elem_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let pop_input_map = if code_config.pop_input_code.is_some() {
            if let Some(pop_inputs) = code_config.pop_from_buffer {
                HashMap::from_iter(pop_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };
        let mut input_map = HashMap::new();
        if !arg_input_map.is_empty() || !elem_input_map.is_empty() || !pop_input_map.is_empty() {
            let mut count = 0;
            for i in 0..input_len {
                if !arg_input_map.contains_key(&i)
                    && !elem_input_map.contains_key(&i)
                    && !pop_input_map.contains_key(&i)
                {
                    input_map.insert(i, count);
                    count += 1;
                }
            }
            Some(input_map)
        } else {
            // no input map - because no arg_inputs or elem_inputs
            None
        }
    };

    let pop_inputs = code_config.pop_input_code.map(|_| {
        if let Some(inputs) = code_config.pop_from_buffer {
            inputs
        } else {
            &[]
        }
    });
    // output_map - circuit output index map: key - original output index,
    // value - output index after excluding circuit's outputs.
    // destination is same order as key.
    let output_map = if let Some(excls) = code_config.exclude_outputs {
        let mut excls = excls.to_vec();
        excls.sort();
        let mut output_map = HashMap::new();
        let mut count = 0;
        let output_len = circuit.outputs().len();
        for o in 0..output_len {
            if excls.binary_search(&o).is_err() {
                output_map.insert(o, count);
                count += 1;
            }
        }
        Some(output_map)
    } else {
        None
    };

    if have_lop3 {
        let vlop3circuit = VLOP3Circuit::from(circuit.clone());
        let (var_allocs, var_num, output_vars) = gen_var_allocs(
            &vlop3circuit,
            code_config.input_placement,
            code_config.output_placement,
            &mut gen_var_usage(&vlop3circuit),
            code_config.single_buffer,
            input_map.as_ref(),
            if code_config.inner_loop.is_some() {
                // enable all outputs as used after gate calculation
                Some(&[])
            } else if code_config.aggr_output_code.is_some() {
                if code_config.aggr_to_buffer.is_some() {
                    code_config.aggr_to_buffer
                } else {
                    Some(&[])
                }
            } else {
                None
            },
            pop_inputs,
            output_map.as_ref(),
            code_config.inner_loop.is_some(),
        );

        let input_len = usize::try_from(circuit.input_len()).unwrap();
        let mut func_writer = writer.func_writer_with_config(
            name,
            input_len,
            circuit.outputs().len(),
            code_config.clone(),
            output_vars
                .as_ref()
                .map(|ov| ov.iter().map(|(i, (x, _))| (*i, *x)).collect()),
        );
        func_writer.func_start();
        func_writer.alloc_vars(var_num + usize::from(code_config.inner_loop.is_some()));
        gen_func_code_for_circuit(
            &mut func_writer,
            &vlop3circuit,
            code_config.input_placement,
            code_config.output_placement,
            &var_allocs,
            var_num,
            code_config.single_buffer,
            input_map.as_ref(),
            output_vars.as_ref(),
            pop_inputs,
            code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_some(),
            output_map.as_ref(),
            code_config.inner_loop.is_some(),
            code_config.aggr_output_code.is_some(),
        );

        func_writer.func_end();
    } else {
        let (var_allocs, var_num, output_vars) = gen_var_allocs(
            &circuit,
            code_config.input_placement,
            code_config.output_placement,
            &mut gen_var_usage(&circuit),
            code_config.single_buffer,
            input_map.as_ref(),
            if code_config.inner_loop.is_some() {
                // enable all outputs as used after gate calculation
                Some(&[])
            } else if code_config.aggr_output_code.is_some() {
                if code_config.aggr_to_buffer.is_some() {
                    code_config.aggr_to_buffer
                } else {
                    Some(&[])
                }
            } else {
                None
            },
            pop_inputs,
            output_map.as_ref(),
            code_config.inner_loop.is_some(),
        );

        let input_len = usize::try_from(circuit.input_len()).unwrap();
        let mut func_writer = writer.func_writer_with_config(
            name,
            input_len,
            circuit.outputs().len(),
            code_config.clone(),
            output_vars
                .as_ref()
                .map(|ov| ov.iter().map(|(i, (x, _))| (*i, *x)).collect()),
        );
        func_writer.func_start();
        func_writer.alloc_vars(var_num + usize::from(code_config.inner_loop.is_some()));

        if impl_op || nimpl_op {
            let vcircuit = VCircuit::to_op_and_ximpl_circuit(circuit.clone(), nimpl_op);
            // generate swap_args - it used to preserve order original traverse from original circuit.
            // if true then argument should be swapped while choosen way.
            let swap_args = circuit
                .gates()
                .iter()
                .enumerate()
                .map(|(i, g)| g.i0 != vcircuit.gates[i].i0)
                .collect::<Vec<_>>();
            gen_func_code_for_circuit(
                &mut func_writer,
                &(vcircuit, swap_args),
                code_config.input_placement,
                code_config.output_placement,
                &var_allocs,
                var_num,
                code_config.single_buffer,
                input_map.as_ref(),
                output_vars.as_ref(),
                pop_inputs,
                code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_some(),
                output_map.as_ref(),
                code_config.inner_loop.is_some(),
                code_config.aggr_output_code.is_some(),
            );
        } else {
            let mut vcircuit = VBinOpCircuit::from(circuit.clone());
            if optimize_negs {
                vcircuit.optimize_negs();
            }
            // generate swap_args - it used to preserve order original traverse from original circuit.
            // if true then argument should be swapped while choosen way.
            let swap_args = circuit
                .gates()
                .iter()
                .enumerate()
                .map(|(i, g)| g.i0 != vcircuit.gates[i].0.i0)
                .collect::<Vec<_>>();
            gen_func_code_for_circuit(
                &mut func_writer,
                &(vcircuit, swap_args),
                code_config.input_placement,
                code_config.output_placement,
                &var_allocs,
                var_num,
                code_config.single_buffer,
                input_map.as_ref(),
                output_vars.as_ref(),
                pop_inputs,
                code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_some(),
                output_map.as_ref(),
                code_config.inner_loop.is_some(),
                code_config.aggr_output_code.is_some(),
            );
        }

        func_writer.func_end();
    }
}

pub fn generate_code<'a, FW: FuncWriter, CW: CodeWriter<'a, FW>, T>(
    writer: &'a mut CW,
    name: &'a str,
    circuit: Circuit<T>,
    optimize_negs: bool,
    input_placement: Option<(&'a [usize], usize)>,
    output_placement: Option<(&'a [usize], usize)>,
    arg_inputs: Option<&'a [usize]>,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    generate_code_with_config(
        writer,
        name,
        circuit,
        optimize_negs,
        CodeConfig::new()
            .input_placement(input_placement)
            .output_placement(output_placement)
            .arg_inputs(arg_inputs),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_var_allocs_old<T>(
        circuit: &Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
        var_usage: &mut [T],
        single_buffer: bool,
        input_map: Option<&HashMap<usize, usize>>,
        keep_output_vars: Option<&[usize]>,
        pop_inputs: Option<&[usize]>,
    ) -> (Vec<T>, usize, Option<Vec<(usize, Option<usize>)>>)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        let (v, l, m) = gen_var_allocs(
            circuit,
            input_placement,
            output_placement,
            var_usage,
            single_buffer,
            input_map,
            keep_output_vars,
            pop_inputs,
            None,
            false,
        );
        (v, l, m.map(|x| x.values().copied().collect::<Vec<_>>()))
    }

    #[test]
    fn test_gen_var_usage_and_var_allocs() {
        let circuit = Circuit::new(
            3,
            [
                Gate::new_xor(0, 1),
                Gate::new_xor(2, 3),
                Gate::new_and(2, 3),
                Gate::new_and(0, 1),
                Gate::new_nor(5, 6),
            ],
            [(4, false), (7, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 2, 4, 2, 0, 0], 5, None),
            gen_var_allocs_old(&circuit, None, None, &mut var_usage, true, None, None, None)
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 2, 4, 2, 0, 0], 5, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                None
            )
        );
        // keep outputs
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(vec![(4, None), (0, None)])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None
            )
        );

        // with pop_input
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 2, 3, 4, 2, 0, 0], 5, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                Some(&[])
            )
        );

        // keep outputs with double outputs (with both normal and negated)
        let circuit = Circuit::new(
            3,
            [
                Gate::new_xor(0, 1),
                Gate::new_xor(2, 3),
                Gate::new_and(2, 3),
                Gate::new_and(0, 1),
                Gate::new_nor(5, 6),
            ],
            [(4, false), (7, true), (4, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 2, 1, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(vec![(4, None), (0, None), (1, Some(4))])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None
            )
        );
        // with various keep_output_vars
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 2, 1, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([(0, (4, None)), (2, (0, Some(4)))]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[0, 2]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 2, 1, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([(1, (0, None)), (2, (4, None))]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[1, 2]),
                None,
                None,
                false,
            )
        );
        // keep outputs with double outputs (with both normal and negated)
        let circuit = Circuit::new(
            3,
            [
                Gate::new_xor(0, 1),
                Gate::new_xor(2, 3),
                Gate::new_and(2, 3),
                Gate::new_and(0, 1),
                Gate::new_nor(5, 6),
            ],
            [(4, false), (7, true), (4, true), (7, false)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 2, 1, 1, 2], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(vec![(4, None), (0, None), (2, Some(4)), (1, Some(0))])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None
            )
        );

        // keep outputs with double outputs (with both normal and negated)
        let circuit = Circuit::new(
            3,
            [
                Gate::new_xor(0, 1),
                Gate::new_xor(2, 3),
                Gate::new_and(2, 3),
                Gate::new_and(0, 1),
                Gate::new_nor(5, 6),
            ],
            [
                (4, false),
                (7, true),
                (4, true),
                (7, false),
                (7, true),
                (4, false),
            ],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 3, 1, 1, 3], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(vec![
                    (4, None),
                    (0, None),
                    (2, Some(4)),
                    (1, Some(0)),
                    (0, None),
                    (4, None)
                ])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None
            )
        );
        // with various keep_output_vars
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 3, 1, 1, 3], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([
                    (0, (4, None)),
                    (2, (1, Some(4))),
                    (3, (0, None)),
                    (5, (4, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[0, 2, 3, 5]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 3, 1, 1, 3], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([
                    (2, (4, None)),
                    (3, (0, None)),
                    (4, (1, Some(0))),
                    (5, (2, Some(4)))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[2, 3, 4, 5]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 3, 1, 1, 3], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([
                    (0, (4, None)),
                    (2, (1, Some(4))),
                    (5, (4, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[0, 2, 5]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 3, 1, 1, 3], var_usage);
        assert_eq!(
            (
                vec![0, 1, 3, 2, 4, 2, 0, 0],
                5,
                Some(BTreeMap::from_iter([
                    (1, (0, None)),
                    (3, (1, Some(0))),
                    (4, (0, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[1, 3, 4]),
                None,
                None,
                false,
            )
        );

        let circuit = Circuit::new(
            4,
            [
                Gate::new_and(0, 2),
                Gate::new_and(1, 2),
                Gate::new_and(0, 3),
                Gate::new_and(1, 3),
                // add a1*b0 + a0*b1
                Gate::new_xor(5, 6),
                Gate::new_and(5, 6),
                // add c(a1*b0 + a0*b1) + a1*b1
                Gate::new_xor(7, 9),
                Gate::new_and(7, 9),
            ],
            [(4, false), (8, true), (10, false), (11, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 2, 1, 3, 2, 1, 0, 2, 4, 0, 1, 0], 5, None),
            gen_var_allocs_old(&circuit, None, None, &mut var_usage, true, None, None, None)
        );
        // single buffer with pop_input and aggr_output_code
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 2, 3, 4, 2, 0, 1, 5, 0, 2, 0],
                6,
                Some(vec![(4, None), (5, None), (2, None), (0, None)])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                true,
                None,
                Some(&[]),
                Some(&[])
            )
        );
        //
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 2, 1, 3, 2, 1, 0, 2, 4, 0, 1, 0], 5, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                None
            )
        );
        // keep outputs
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 3, 1, 4, 2, 1, 0, 3, 5, 0, 1, 0],
                6,
                Some(vec![(2, None), (5, None), (1, None), (0, None)])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None
            )
        );
        // keep outputs with pop_input
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 2, 3, 4, 2, 0, 1, 5, 0, 2, 0],
                6,
                Some(vec![(4, None), (5, None), (2, None), (0, None)])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                Some(&[])
            )
        );

        // read/write conflict
        let circuit = Circuit::new(
            4,
            [
                Gate::new_and(0, 1),
                Gate::new_xor(0, 1),
                Gate::new_nor(0, 1),
                Gate::new_nor(2, 3),
            ],
            [(4, false), (5, true), (6, false), (7, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 0, 1, 2, 2, 0, 0], 3, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                None
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 1, 0, 2, 2, 0, 0], 3, None),
            gen_var_allocs_old(&circuit, None, None, &mut var_usage, true, None, None, None)
        );
        // with single_buffer, pop_input and keep_output_vars
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (
                vec![0, 1, 2, 3, 4, 5, 0, 1],
                6,
                Some(vec![(4, None), (5, None), (0, None), (1, None)])
            ),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                true,
                None,
                Some(&[]),
                Some(&[0, 1, 2, 3])
            )
        );
        // with placement
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 1, 2, 2, 0, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 1, 3, 2, 2, 0, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                Some((&[1, 2, 0, 3], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 4, 3, 2, 2, 0, 0], 5, None),
            gen_var_allocs_old(
                &circuit,
                None,
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 0, 2, 2, 0, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                Some((&[1, 2, 0, 3], 4)),
                None,
                &mut var_usage,
                true,
                None,
                None,
                None
            )
        );

        let circuit = Circuit::new(
            4,
            [
                Gate::new_and(2, 3),
                Gate::new_xor(2, 3),
                Gate::new_nor(2, 3),
                Gate::new_nor(0, 1),
            ],
            [(4, false), (5, true), (6, false), (7, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 3, 3, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 0, 1, 2, 2, 0, 0], 3, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                None
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 3, 3, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 4, 0, 1, 2, 2, 0, 0], 5, None),
            gen_var_allocs_old(&circuit, None, None, &mut var_usage, true, None, None, None)
        );

        let circuit = Circuit::new(
            4,
            [
                Gate::new_and(2, 3),
                Gate::new_xor(2, 3),
                Gate::new_nor(0, 3),
                Gate::new_and(4, 5),
                Gate::new_nimpl(4, 6),
                Gate::new_xor(5, 6),
                Gate::new_xor(8, 9),
                Gate::new_nimpl(9, 1),
            ],
            [(7, false), (8, true), (10, false), (11, true)],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 1, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                None
            )
        );
        // with pop_input
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 2, 3, 4, 2, 0, 5, 3, 0, 2, 0], 6, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                Some(&[])
            )
        );
        // with pop input 2
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 2, 4, 3, 0, 5, 2, 0, 2, 0], 6, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                Some(&[0, 1, 3])
            )
        );
        // with pop input 3
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![4, 0, 2, 1, 3, 2, 1, 4, 3, 1, 2, 0], 5, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                None,
                Some(&[1, 3])
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![4, 3, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 5, None),
            gen_var_allocs_old(&circuit, None, None, &mut var_usage, true, None, None, None)
        );
        // testcase with placements
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 3, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None,
            )
        );
        // with output_map
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 1, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4, None),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[3, 1], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None,
                Some(&HashMap::from_iter([(0, 0), (3, 1)])),
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![4, 1, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 5, None),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[1, 3], 4)),
                &mut var_usage,
                true,
                None,
                None,
                None,
                Some(&HashMap::from_iter([(0, 0), (3, 1)])),
                false,
            )
        );
        // testcase with placements and with input_map (some input are used as arg_input)
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 2, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                Some((&[1, 0], 4)),
                Some((&[3, 2, 1, 0], 4)),
                &mut var_usage,
                true,
                Some(&HashMap::from_iter([(1, 0), (3, 1)])),
                None,
                None,
            )
        );
        let circuit = Circuit::new(
            6,
            [
                Gate::new_and(2, 3),
                Gate::new_xor(2, 3),
                Gate::new_nor(0, 3),
                Gate::new_and(6, 7),
                Gate::new_nimpl(6, 8),
                Gate::new_xor(7, 9),
                Gate::new_xor(10, 11),
                Gate::new_nimpl(11, 1),
            ],
            [(4, false), (5, true), (12, false), (13, true)],
        )
        .unwrap();
        // testcase with input_map (some input are used as arg_input)
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 2, 0, 1, 0, 1, 2, 0, 3, 1, 3, 0, 1, 0], 4, None),
            gen_var_allocs_old(
                &circuit,
                None,
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                Some(&HashMap::from_iter([(1, 0), (3, 1), (4, 2), (5, 3)])),
                None,
                None,
            )
        );
        // with pop_input and input_map
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![6, 0, 4, 1, 2, 3, 5, 1, 6, 4, 6, 1, 4, 0], 7, None),
            gen_var_allocs_old(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                Some(&HashMap::from_iter([(1, 0), (3, 1), (4, 2), (5, 3)])),
                None,
                Some(&[])
            )
        );

        let circuit = Circuit::new(
            4,
            [],
            [
                (0, false),
                (1, true),
                (2, false),
                (3, true),
                (1, false),
                (2, true),
                (3, true),
                (2, false),
                (1, true),
            ],
        )
        .unwrap();
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 3, 3, 2], var_usage);
        assert_eq!(
            (
                vec![0, 1, 2, 3],
                6,
                Some(BTreeMap::from_iter([
                    (0, (0, None)),
                    (1, (1, None)),
                    (2, (2, None)),
                    (3, (3, None)),
                    (4, (4, Some(1))),
                    (5, (5, Some(2))),
                    (6, (3, None)),
                    (7, (2, None)),
                    (8, (1, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 3, 3, 2], var_usage);
        assert_eq!(
            (
                vec![0, 0, 1, 2],
                3,
                Some(BTreeMap::from_iter([
                    (1, (0, None)),
                    (2, (1, None)),
                    (4, (2, Some(0))),
                    (7, (1, None)),
                    (8, (0, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[1, 2, 4, 7, 8]),
                None,
                None,
                false,
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 3, 3, 2], var_usage);
        assert_eq!(
            (
                vec![0, 0, 1, 2],
                4,
                Some(BTreeMap::from_iter([
                    (1, (0, None)),
                    (4, (3, Some(0))),
                    (5, (1, None)),
                    (6, (2, None)),
                    (8, (0, None))
                ]))
            ),
            gen_var_allocs(
                &circuit,
                None,
                None,
                &mut var_usage,
                false,
                None,
                Some(&[1, 4, 5, 6, 8]),
                None,
                None,
                false,
            )
        );
    }
}

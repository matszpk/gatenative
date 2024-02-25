use crate::utils::{gen_var_usage, VarAllocator};
use gatesim::*;

use int_enum::IntEnum;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::*;

use crate::vbinopcircuit::*;
use crate::vcircuit::*;

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
    //println!("  Alloc: {:?} {:?}", var, alloc_vars[var_u]);
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
    //println!("  VarUsage: {:?} {:?}", var, vu);
    var_usage[var_u] = T::try_from(vu).unwrap();
    if vu == 0 {
        // if no further usage
        var_alloc.free(alloc_vars[var_u].unwrap());
    }
}

#[inline]
fn get_bit_place(placement: Option<(&[usize], usize)>, bit: usize) -> usize {
    placement.map(|(p, _)| p[bit]).unwrap_or(bit)
}

fn get_input_orig_index_map(
    input_len: usize,
    input_placement: Option<(&[usize], usize)>,
    single_buffer: bool,
    input_map: &HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    if single_buffer {
        if let Some((input_p, _)) = input_placement {
            HashMap::from_iter(input_p.iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::from_iter((0..input_len).map(|i| {
                (
                    if !input_map.is_empty() {
                        *input_map.get(&i).unwrap_or(&input_len)
                    } else {
                        i
                    },
                    i,
                )
            }))
        }
    } else {
        HashMap::new()
    }
}

fn gen_var_allocs<T>(
    circuit: &Circuit<T>,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    var_usage: &mut [T],
    single_buffer: bool,
    input_map: &HashMap<usize, usize>,
) -> (Vec<T>, usize)
where
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
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.len();
    let gates = circuit.gates();
    let mut alloc_vars: Vec<Option<T>> = vec![None; input_len + gate_num];
    let mut var_alloc = VarAllocator::<T>::new();

    let mut visited = vec![false; gate_num];

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

            if way == 0 {
                if !visited[node_index] {
                    visited[node_index] = true;
                } else {
                    stack.pop();
                    continue;
                }

                top.way += 1;
                let gi0 = gates[node_index].i0;
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way == 1 {
                top.way += 1;
                let gi1 = gates[node_index].i1;
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                // allocate and use
                //println!("Stack: {:?} {:?}", input_len + node_index, gates[node_index]);
                // allocate circuit inputs now if not allocated
                if gates[node_index].i0 < input_len_t {
                    single_var_alloc(&mut var_alloc, &mut alloc_vars, gates[node_index].i0);
                    input_already_read[usize::try_from(gates[node_index].i0).unwrap()] = true;
                }
                if gates[node_index].i1 < input_len_t {
                    single_var_alloc(&mut var_alloc, &mut alloc_vars, gates[node_index].i1);
                    input_already_read[usize::try_from(gates[node_index].i1).unwrap()] = true;
                }
                single_var_use(&mut var_alloc, &alloc_vars, var_usage, gates[node_index].i0);
                single_var_use(&mut var_alloc, &alloc_vars, var_usage, gates[node_index].i1);
                let tnode = T::try_from(node_index + input_len).unwrap();
                single_var_alloc(&mut var_alloc, &mut alloc_vars, tnode);
                // if !single_buffer && out_map.contains(&tnode) {
                //     // use output at this point
                //     single_var_use(&mut var_alloc, &alloc_vars, var_usage, tnode);
                // }
                if let Some(outlist) = out_map.get(&tnode) {
                    if single_buffer {
                        for oi in outlist {
                            // check output mapping to not already inputs
                            let out_p = get_bit_place(output_placement, *oi);
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
                    // use output at this point
                    single_var_use(&mut var_alloc, &alloc_vars, var_usage, tnode);
                }
                stack.pop();
            }
        }
    }

    // for outputs just
    for (o, _) in circuit.outputs().iter() {
        if *o < input_len_t {
            single_var_alloc(&mut var_alloc, &mut alloc_vars, *o);
            single_var_use(&mut var_alloc, &alloc_vars, var_usage, *o);
        }
    }

    (
        alloc_vars
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>(),
        var_alloc.len(),
    )
}

fn gen_func_code_for_ximpl<FW: FuncWriter, T>(
    writer: &mut FW,
    circuit: &VCircuit<T>,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    swap_args: &[bool],
    var_allocs: &[T],
    single_buffer: bool,
    input_map: &HashMap<usize, usize>,
) where
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
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.gates.len();
    let gates = &circuit.gates;

    let out_map = {
        let mut out_map = HashMap::<T, Vec<(usize, bool)>>::new();
        for (i, (o, n)) in circuit.outputs.iter().enumerate() {
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

    let mut visited = vec![false; gate_num];
    for (o, _) in circuit.outputs.iter() {
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

            if way == 0 {
                if !visited[node_index] {
                    visited[node_index] = true;
                } else {
                    stack.pop();
                    continue;
                }

                top.way += 1;
                let gi0 = if swap_args[node_index] {
                    gates[node_index].i1
                } else {
                    gates[node_index].i0
                };
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way == 1 {
                top.way += 1;
                let gi1 = if swap_args[node_index] {
                    gates[node_index].i0
                } else {
                    gates[node_index].i1
                };
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                let gi0 = usize::try_from(gates[node_index].i0).unwrap();
                let gi1 = usize::try_from(gates[node_index].i1).unwrap();
                if gi0 < input_len && !used_inputs[gi0] {
                    writer.gen_load(usize::try_from(var_allocs[gi0]).unwrap(), gi0);
                    used_inputs[gi0] = true;
                }
                if gi1 < input_len && !used_inputs[gi1] {
                    writer.gen_load(usize::try_from(var_allocs[gi1]).unwrap(), gi1);
                    used_inputs[gi1] = true;
                }
                writer.gen_op(
                    match gates[node_index].func {
                        VGateFunc::And => InstrOp::And,
                        VGateFunc::Or => InstrOp::Or,
                        VGateFunc::Impl => InstrOp::Impl,
                        VGateFunc::Nimpl => InstrOp::Nimpl,
                        VGateFunc::Xor => InstrOp::Xor,
                        _ => {
                            panic!("Unsupported InstrOp")
                        }
                    },
                    VNegs::NoNegs,
                    usize::try_from(var_allocs[input_len + node_index]).unwrap(),
                    usize::try_from(var_allocs[usize::try_from(gates[node_index].i0).unwrap()])
                        .unwrap(),
                    usize::try_from(var_allocs[usize::try_from(gates[node_index].i1).unwrap()])
                        .unwrap(),
                );
                let tnode = T::try_from(input_len + node_index).unwrap();
                if let Some(outlist) = out_map.get(&tnode) {
                    for (oi, on) in outlist {
                        if single_buffer {
                            // check output mapping to not already inputs
                            let out_p = get_bit_place(output_placement, *oi);
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
                        // if output then store
                        writer.gen_store(
                            *on,
                            *oi,
                            usize::try_from(var_allocs[input_len + node_index]).unwrap(),
                        );
                    }
                }
                stack.pop();
            }
        }
    }

    for (oi, (o, on)) in circuit.outputs.iter().enumerate() {
        if *o < input_len_t {
            let ou = usize::try_from(*o).unwrap();
            if !used_inputs[ou] {
                writer.gen_load(usize::try_from(var_allocs[ou]).unwrap(), ou);
                used_inputs[ou] = true;
            }
            writer.gen_store(
                *on,
                oi,
                usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap(),
            );
        }
    }
}

fn gen_func_code_for_binop<FW: FuncWriter, T>(
    writer: &mut FW,
    circuit: &VBinOpCircuit<T>,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
    swap_args: &[bool],
    var_allocs: &[T],
    single_buffer: bool,
    input_map: &HashMap<usize, usize>,
) where
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
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.gates.len();
    let gates = &circuit.gates;

    let out_map = {
        let mut out_map = HashMap::<T, Vec<(usize, bool)>>::new();
        for (i, (o, n)) in circuit.outputs.iter().enumerate() {
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

    let mut visited = vec![false; gate_num];
    for (o, _) in circuit.outputs.iter() {
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

            if way == 0 {
                if !visited[node_index] {
                    visited[node_index] = true;
                } else {
                    stack.pop();
                    continue;
                }

                top.way += 1;
                let gi0 = if swap_args[node_index] {
                    gates[node_index].0.i1
                } else {
                    gates[node_index].0.i0
                };
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way == 1 {
                top.way += 1;
                let gi1 = if swap_args[node_index] {
                    gates[node_index].0.i0
                } else {
                    gates[node_index].0.i1
                };
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                let gi0 = usize::try_from(gates[node_index].0.i0).unwrap();
                let gi1 = usize::try_from(gates[node_index].0.i1).unwrap();
                if gi0 < input_len && !used_inputs[gi0] {
                    writer.gen_load(usize::try_from(var_allocs[gi0]).unwrap(), gi0);
                    used_inputs[gi0] = true;
                }
                if gi1 < input_len && !used_inputs[gi1] {
                    writer.gen_load(usize::try_from(var_allocs[gi1]).unwrap(), gi1);
                    used_inputs[gi1] = true;
                }
                writer.gen_op(
                    match gates[node_index].0.func {
                        VGateFunc::And => InstrOp::And,
                        VGateFunc::Or => InstrOp::Or,
                        VGateFunc::Xor => InstrOp::Xor,
                        _ => {
                            panic!("Unsupported InstrOp")
                        }
                    },
                    gates[node_index].1,
                    usize::try_from(var_allocs[input_len + node_index]).unwrap(),
                    usize::try_from(var_allocs[usize::try_from(gates[node_index].0.i0).unwrap()])
                        .unwrap(),
                    usize::try_from(var_allocs[usize::try_from(gates[node_index].0.i1).unwrap()])
                        .unwrap(),
                );
                let tnode = T::try_from(input_len + node_index).unwrap();
                if let Some(outlist) = out_map.get(&tnode) {
                    for (oi, on) in outlist {
                        if single_buffer {
                            // check output mapping to not already inputs
                            let out_p = get_bit_place(output_placement, *oi);
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
                        // if output then store
                        writer.gen_store(
                            *on,
                            *oi,
                            usize::try_from(var_allocs[input_len + node_index]).unwrap(),
                        );
                    }
                }
                stack.pop();
            }
        }
    }

    for (oi, (o, on)) in circuit.outputs.iter().enumerate() {
        if *o < input_len_t {
            let ou = usize::try_from(*o).unwrap();
            if !used_inputs[ou] {
                writer.gen_load(usize::try_from(var_allocs[ou]).unwrap(), ou);
                used_inputs[ou] = true;
            }
            writer.gen_store(
                *on,
                oi,
                usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap(),
            );
        }
    }
}

pub fn generate_code_ext<'a, FW: FuncWriter, CW: CodeWriter<'a, FW>, T>(
    writer: &'a mut CW,
    name: &'a str,
    circuit: Circuit<T>,
    optimize_negs: bool,
    input_placement: Option<(&'a [usize], usize)>,
    output_placement: Option<(&'a [usize], usize)>,
    arg_inputs: Option<&'a [usize]>,
    elem_inputs: Option<&'a [usize]>,
    single_buffer: bool,
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

    // generate input_map
    let input_map = {
        let input_len = usize::try_from(circuit.input_len()).unwrap();
        let arg_input_map = if let Some(arg_inputs) = arg_inputs {
            HashMap::from_iter(arg_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let elem_input_map = if let Some(elem_inputs) = elem_inputs {
            HashMap::from_iter(elem_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let mut input_map = HashMap::new();
        if !arg_input_map.is_empty() || !elem_input_map.is_empty() {
            let mut count = 0;
            for i in 0..input_len {
                if !arg_input_map.contains_key(&i) && !elem_input_map.contains_key(&i) {
                    input_map.insert(i, count);
                    count += 1;
                }
            }
        }
        input_map
    };

    let (var_allocs, var_num) = gen_var_allocs(
        &circuit,
        input_placement,
        output_placement,
        &mut gen_var_usage(&circuit),
        single_buffer,
        &input_map,
    );

    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let mut func_writer = writer.func_writer_ext(
        name,
        input_len,
        circuit.outputs().len(),
        input_placement,
        output_placement,
        arg_inputs,
        elem_inputs,
        single_buffer,
    );
    func_writer.func_start();
    func_writer.alloc_vars(var_num);

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
        gen_func_code_for_ximpl(
            &mut func_writer,
            &vcircuit,
            input_placement,
            output_placement,
            &swap_args,
            &var_allocs,
            single_buffer,
            &input_map,
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
        gen_func_code_for_binop(
            &mut func_writer,
            &vcircuit,
            input_placement,
            output_placement,
            &swap_args,
            &var_allocs,
            single_buffer,
            &input_map,
        );
    }

    func_writer.func_end();
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
    generate_code_ext(
        writer,
        name,
        circuit,
        optimize_negs,
        input_placement,
        output_placement,
        arg_inputs,
        None,
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (vec![0, 1, 3, 2, 4, 2, 0, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, true, &HashMap::new())
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 2, 4, 2, 0, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, false, &HashMap::new())
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
            (vec![0, 2, 1, 3, 2, 1, 0, 2, 4, 0, 1, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, true, &HashMap::new())
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 2, 1, 3, 2, 1, 0, 2, 4, 0, 1, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, false, &HashMap::new())
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
            (vec![0, 1, 0, 1, 2, 2, 0, 0], 3),
            gen_var_allocs(&circuit, None, None, &mut var_usage, false, &HashMap::new())
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 1, 0, 2, 2, 0, 0], 3),
            gen_var_allocs(&circuit, None, None, &mut var_usage, true, &HashMap::new())
        );
        // with placement
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 1, 2, 2, 0, 0], 4),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                &HashMap::new()
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 1, 3, 2, 2, 0, 0], 4),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 0, 3], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                &HashMap::new()
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 4, 3, 2, 2, 0, 0], 5),
            gen_var_allocs(
                &circuit,
                None,
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                &HashMap::new()
            )
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![3, 3, 1, 1, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![0, 1, 3, 0, 2, 2, 0, 0], 4),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 0, 3], 4)),
                None,
                &mut var_usage,
                true,
                &HashMap::new()
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
            (vec![0, 1, 0, 1, 2, 2, 0, 0], 3),
            gen_var_allocs(&circuit, None, None, &mut var_usage, false, &HashMap::new())
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 3, 3, 1, 1, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 4, 0, 1, 2, 2, 0, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, true, &HashMap::new())
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
            (vec![3, 1, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4),
            gen_var_allocs(&circuit, None, None, &mut var_usage, false, &HashMap::new())
        );
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![4, 3, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 5),
            gen_var_allocs(&circuit, None, None, &mut var_usage, true, &HashMap::new())
        );
        // testcase with placements
        let mut var_usage = gen_var_usage(&circuit);
        assert_eq!(vec![1, 1, 2, 3, 2, 2, 2, 1, 2, 2, 1, 1], var_usage);
        assert_eq!(
            (vec![3, 3, 0, 1, 2, 0, 1, 3, 2, 0, 1, 0], 4),
            gen_var_allocs(
                &circuit,
                Some((&[1, 2, 3, 0], 4)),
                Some((&[3, 2, 0, 1], 4)),
                &mut var_usage,
                true,
                &HashMap::new(),
            )
        );
    }
}

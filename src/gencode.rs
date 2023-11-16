use gatesim::*;

use int_enum::IntEnum;

use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::*;

use crate::vcircuit::*;
use crate::vbinopcircuit::*;

pub struct VarAllocator<T> {
    free_list: BinaryHeap<std::cmp::Reverse<T>>,
    alloc_map: Vec<bool>,
}

impl<T> VarAllocator<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn new() -> Self {
        Self {
            free_list: BinaryHeap::new(),
            alloc_map: vec![],
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.alloc_map.len()
    }

    fn alloc(&mut self) -> T {
        if let Some(std::cmp::Reverse(index)) = self.free_list.pop() {
            let index_u = usize::try_from(index).unwrap();
            self.alloc_map[index_u] = true;
            index
        } else {
            let index = self.alloc_map.len();
            self.alloc_map.push(true);
            T::try_from(index).unwrap()
        }
    }

    fn free(&mut self, index: T) -> bool {
        let index_u = usize::try_from(index).unwrap();
        assert!(index_u < self.len());
        if self.alloc_map[index_u] {
            self.free_list.push(std::cmp::Reverse(index));
            self.alloc_map[index_u] = false;
            true
        } else {
            false
        }
    }
}

// var usage - just counter of var usage.

fn gen_var_usage<T>(circuit: &Circuit<T>) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let mut var_usage = vec![T::default(); input_len + circuit.len()];
    for g in circuit.gates() {
        let gi0 = usize::try_from(g.i0).unwrap();
        let gi1 = usize::try_from(g.i1).unwrap();
        let var_usage_0 = usize::try_from(var_usage[gi0]).unwrap() + 1;
        var_usage[gi0] = T::try_from(var_usage_0).unwrap();
        let var_usage_1 = usize::try_from(var_usage[gi1]).unwrap() + 1;
        var_usage[gi1] = T::try_from(var_usage_1).unwrap();
    }
    for (o, _) in circuit.outputs() {
        let o = usize::try_from(*o).unwrap();
        let var_usage_0 = usize::try_from(var_usage[o]).unwrap() + 1;
        var_usage[o] = T::try_from(var_usage_0).unwrap();
    }
    var_usage
}

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

fn gen_var_allocs<T>(circuit: &Circuit<T>, var_usage: &mut [T]) -> (Vec<T>, usize)
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
    for i in 0..input_len {
        single_var_alloc(&mut var_alloc, &mut alloc_vars, T::try_from(i).unwrap());
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
                single_var_use(&mut var_alloc, &alloc_vars, var_usage, gates[node_index].i0);
                single_var_use(&mut var_alloc, &alloc_vars, var_usage, gates[node_index].i1);
                single_var_alloc(
                    &mut var_alloc,
                    &mut alloc_vars,
                    T::try_from(node_index + input_len).unwrap(),
                );
                stack.pop();
            }
        }

        single_var_use(&mut var_alloc, &alloc_vars, var_usage, *o);
    }
    (
        alloc_vars
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>(),
        var_alloc.len(),
    )
}

fn gen_func_code_for_ximpl<CW: CodeWriter, T>(
    writer: &CW,
    out: &mut Vec<u8>,
    circuit: &VCircuit<T>,
    swap_args: &[bool],
    var_allocs: &[T],
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

    let mut visited = vec![false; gate_num];
    for (oi, (o, on)) in circuit.outputs.iter().enumerate() {
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
                writer.gen_op(
                    out,
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
                stack.pop();
            }
        }
        writer.gen_store(
            out,
            *on,
            oi,
            usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap(),
        );
    }
}

fn gen_func_code_for_binop<CW: CodeWriter, T>(
    writer: &CW,
    out: &mut Vec<u8>,
    circuit: &VBinOpCircuit<T>,
    swap_args: &[bool],
    var_allocs: &[T],
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

    let mut visited = vec![false; gate_num];
    for (oi, (o, on)) in circuit.outputs.iter().enumerate() {
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
                writer.gen_op(
                    out,
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
                stack.pop();
            }
        }
        writer.gen_store(
            out,
            *on,
            oi,
            usize::try_from(var_allocs[usize::try_from(*o).unwrap()]).unwrap(),
        );
    }
}

pub fn generate_code<CW: CodeWriter, T>(
    writer: &CW,
    out: &mut Vec<u8>,
    name: &str,
    circuit: Circuit<T>,
    optimize_negs: bool,
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
    let (var_allocs, var_num) = gen_var_allocs(&circuit, &mut gen_var_usage(&circuit));

    let input_len = usize::try_from(circuit.input_len()).unwrap();
    writer.func_start(out, name, input_len, circuit.outputs().len());
    writer.alloc_vars(out, var_num);

    for i in 0..input_len {
        writer.gen_load(out, usize::try_from(var_allocs[i]).unwrap(), i);
    }

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
        gen_func_code_for_ximpl(writer, out, &vcircuit, &swap_args, &var_allocs);
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
        gen_func_code_for_binop(writer, out, &vcircuit, &swap_args, &var_allocs);
    }

    writer.func_end(out, name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_allocator() {
        let mut vacc = VarAllocator::new();
        assert_eq!(0, vacc.alloc());
        assert_eq!(1, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(3, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert!(vacc.free(2));
        assert!(!vacc.free(2));
        assert!(vacc.free(1));
        assert!(!vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(0));
        assert!(!vacc.free(0));
        assert_eq!(0, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(5, vacc.alloc());
        assert!(vacc.free(4));
        assert!(vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(3));
        assert_eq!(3, vacc.alloc());
        assert!(vacc.free(2));
        assert_eq!(2, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert_eq!(6, vacc.alloc());
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
            (vec![0, 1, 2, 3, 4, 2, 0, 0], 5),
            gen_var_allocs(&circuit, &mut var_usage)
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
            (vec![0, 1, 2, 3, 4, 2, 0, 1, 4, 0, 2, 0], 5),
            gen_var_allocs(&circuit, &mut var_usage)
        );
    }
}

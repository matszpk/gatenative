use crate::utils::{gen_var_usage, VarAllocator};
use gatesim::*;

use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Placement {
    placements: Vec<usize>,
    real_len: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DivCircuitEntry<T: Clone + Copy> {
    circuit: Circuit<T>,
    input_ps: Option<Placement>,  // input placement
    output_ps: Option<Placement>, // output placement
}

// IDEA: from circuit input to last circuit outputs:
// move some circuit outputs that will be used later to last outputs by copying.

// separate circuit while traversing through circuit
fn divide_circuit_traverse<T>(circuit: Circuit<T>, max_gates: usize) -> Vec<DivCircuitEntry<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    if circuit.len() <= max_gates {
        return vec![DivCircuitEntry {
            circuit,
            input_ps: None,
            output_ps: None,
        }];
    }

    #[derive(Clone, Copy)]
    struct StackEntry {
        node: usize,
        way: usize,
    }

    println!("DivideStart");
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.len();
    let gates = circuit.gates();
    let mut visited = vec![false; gate_num];
    let mut subcircuits = Vec::<DivCircuitEntry<T>>::new();
    let mut var_usage = gen_var_usage(&circuit);
    // index - original gate index, value - option: var index
    let mut global_vars = vec![None; input_len + gate_num];
    let mut cur_subc_gates = BTreeSet::new();
    let mut var_alloc = VarAllocator::<usize>::new();

    for i in 0..input_len {
        global_vars[i] = Some(var_alloc.alloc());
    }

    let mut push_subcircuit = |cur_subc_gates: &mut BTreeSet<usize>,
                               var_usage: &mut [T],
                               global_vars: &mut [Option<usize>],
                               var_alloc: &mut VarAllocator<usize>,
                               last: bool| {
        println!("Subcircuit {}", subcircuits.len());
        println!("  gates: {:?}", cur_subc_gates);
        let mut subc_inputs = BTreeSet::<usize>::new();
        let mut used_inputs = vec![false; input_len];
        if subcircuits.is_empty() {
            // first subcircuit
            subc_inputs.extend(0..input_len);
            for gidx in cur_subc_gates.iter() {
                let g: Gate<T> = gates[*gidx - input_len];
                let gi0 = usize::try_from(g.i0).unwrap();
                let gi1 = usize::try_from(g.i1).unwrap();
                if gi0 < input_len {
                    used_inputs[gi0] = true;
                }
                if gi1 < input_len {
                    used_inputs[gi1] = true;
                }
            }
        } else {
            // collect used gate outputs later to inputs
            for gidx in cur_subc_gates.iter() {
                let g: Gate<T> = gates[*gidx - input_len];
                let gi0 = usize::try_from(g.i0).unwrap();
                let gi1 = usize::try_from(g.i1).unwrap();
                if global_vars[gi0].is_some() {
                    subc_inputs.insert(gi0);
                }
                if global_vars[gi1].is_some() {
                    subc_inputs.insert(gi1);
                }
            }
        }
        if last {
            // if last circuit then add to subcircuit's inputs circuit outputs
            for (o, _) in circuit.outputs() {
                let o = usize::try_from(*o).unwrap();
                if !cur_subc_gates.contains(&o) {
                    subc_inputs.insert(o);
                }
            }
        }

        let subc_input_map = subc_inputs
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<HashMap<_, _>>();
        // input placement indices
        let input_ps = subc_inputs
            .into_iter()
            .map(|x| global_vars[x].unwrap())
            .collect::<Vec<_>>();

        // create new subcircuit
        let subc_input_len = subc_input_map.len();
        let subc_map = HashMap::<usize, usize>::from_iter(
            cur_subc_gates
                .iter()
                .enumerate()
                .map(|(i, x)| (*x, i + subc_input_len)),
        );
        println!("Sub_input_cmap: {:?}", subc_input_map);
        println!("Subcmap: {:?}", subc_map);
        // generate subcircuit gates
        let subc_gates = cur_subc_gates
            .iter()
            .map(|gidx| {
                let g: Gate<T> = gates[*gidx - input_len];
                println!("G: {} {}", gidx, g);
                Gate {
                    func: g.func,
                    i0: T::try_from({
                        let gi0 = usize::try_from(g.i0).unwrap();
                        if let Some(t) = subc_input_map.get(&gi0) {
                            *t
                        } else {
                            subc_map[&gi0]
                        }
                    })
                    .unwrap(),
                    i1: T::try_from({
                        let gi1 = usize::try_from(g.i1).unwrap();
                        if let Some(t) = subc_input_map.get(&gi1) {
                            *t
                        } else {
                            subc_map[&gi1]
                        }
                    })
                    .unwrap(),
                }
            })
            .collect::<Vec<_>>();
        // process current variables for usage
        // T::default is zero in T.
        for gidx in cur_subc_gates.iter() {
            if global_vars[*gidx].is_some() {
                if var_usage[*gidx] == T::default() {
                    // free global variable
                    global_vars[*gidx] = None;
                }
            } else if var_usage[*gidx] != T::default() {
                global_vars[*gidx] = Some(var_alloc.alloc());
            }
        }
        // generate outputs
        let subc_outputs = if !last {
            // if not last subcircuit
            let mut subc_outputs = cur_subc_gates
                .iter()
                .filter_map(|gidx| {
                    if global_vars[*gidx].is_some() {
                        if let Some(v) = subc_map.get(gidx) {
                            Some((T::try_from(*v).unwrap(), false))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if subcircuits.is_empty() {
                // if some input is not used in first subcircuit then will be added
                // as direct output of this subcircuit
                subc_outputs.extend(used_inputs.iter().enumerate().filter_map(|(i, used)| {
                    if !used {
                        let i = T::try_from(i).unwrap();
                        Some((i, false))
                    } else {
                        None
                    }
                }));
            }
            subc_outputs
        } else {
            // if last subcircuit - then get from circuit outputs
            circuit
                .outputs()
                .iter()
                .map(|(o, n)| {
                    let o = usize::try_from(*o).unwrap();
                    if let Some(v) = subc_map.get(&o) {
                        (T::try_from(*v).unwrap(), *n)
                    } else {
                        (T::try_from(subc_input_map[&o]).unwrap(), *n)
                    }
                })
                .collect::<Vec<_>>()
        };
        // generate placements indices
        let output_ps = if !last {
            // if not last subcircuit
            let mut output_ps = cur_subc_gates
                .iter()
                .filter_map(|gidx| {
                    if global_vars[*gidx].is_some() {
                        if subc_map.contains_key(gidx) {
                            Some(global_vars[*gidx].unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if subcircuits.is_empty() {
                // if some input is not used in first subcircuit then will be added
                // as direct output of this subcircuit
                output_ps.extend(used_inputs.into_iter().enumerate().filter_map(|(i, used)| {
                    if !used {
                        Some(i)
                    } else {
                        None
                    }
                }));
            }
            output_ps
        } else {
            // if last subcircuit - then get from circuit outputs
            vec![]
        };
        println!("Subcinputlen: {}", subc_input_len);
        println!("Subcgates: {:?}", subc_gates);
        println!("Subcoutputs: {:?}", subc_outputs);
        let first = subcircuits.is_empty();
        subcircuits.push(DivCircuitEntry {
            circuit: Circuit::new(
                T::try_from(subc_input_len).unwrap(),
                subc_gates,
                subc_outputs,
            )
            .unwrap(),
            input_ps: if !first {
                Some(Placement {
                    placements: input_ps,
                    real_len: 0,
                })
            } else {
                None
            },
            output_ps: if !last {
                Some(Placement {
                    placements: output_ps,
                    real_len: 0,
                })
            } else {
                None
            },
        });
        println!("GlobalVars: {:?}", global_vars);
        println!("Subcircuit last: {:?}", subcircuits.last().unwrap());
        println!("VarUsage: {:?}", var_usage);
        // clear current subcircuit gates
        cur_subc_gates.clear();
    };

    for (o, _) in circuit.outputs().iter() {
        let oidx = usize::try_from(*o).unwrap() - input_len;
        let mut stack = Vec::new();
        if *o >= input_len_t {
            stack.push(StackEntry { node: oidx, way: 0 });
        }

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
                if cur_subc_gates.len() >= max_gates {
                    push_subcircuit(
                        &mut cur_subc_gates,
                        &mut var_usage,
                        &mut global_vars,
                        &mut var_alloc,
                        false,
                    );
                }
                // use variable from gate.i0 and gate.i1
                let g = gates[node_index];
                let gi0 = usize::try_from(g.i0).unwrap();
                let vusage = usize::try_from(var_usage[gi0]).unwrap();
                var_usage[gi0] = T::try_from(vusage - 1).unwrap();
                if var_usage[gi0] == T::default() {
                    if let Some(v) = global_vars[gi0] {
                        // free in var allocator. do not free in global vars - because
                        // it can be used later
                        println!("FreeAlloc: {:?} {:?}", gi0, v);
                        var_alloc.free(v);
                    }
                }
                let gi1 = usize::try_from(g.i1).unwrap();
                let vusage = usize::try_from(var_usage[gi1]).unwrap();
                var_usage[gi1] = T::try_from(vusage - 1).unwrap();
                if var_usage[gi1] == T::default() {
                    if let Some(v) = global_vars[gi1] {
                        // free in var allocator. do not free in global vars - because
                        // it can be used later
                        println!("FreeAlloc: {:?} {:?}", gi1, v);
                        var_alloc.free(v);
                    }
                }
                // add new gate to current subcircuit
                cur_subc_gates.insert(input_len + node_index);
                stack.pop();
            }
        }
    }

    // generate last subcircuit
    push_subcircuit(
        &mut cur_subc_gates,
        &mut var_usage,
        &mut global_vars,
        &mut var_alloc,
        true,
    );

    let placement_len = var_alloc.len();
    for sc in &mut subcircuits {
        if let Some(ps) = &mut sc.input_ps {
            ps.real_len = placement_len;
        }
        if let Some(ps) = &mut sc.output_ps {
            ps.real_len = placement_len;
        }
    }
    subcircuits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_circuit_traverse() {
        let circuit = Circuit::new(
            6,
            [
                Gate::new_xor(1, 0),    // 6
                Gate::new_xor(1, 2),    // 7
                Gate::new_xor(3, 4),    // 8
                Gate::new_xor(4, 5),    // 9
                Gate::new_xor(6, 7),    // 10
                Gate::new_xor(8, 9),    // 11
                Gate::new_and(10, 11),  // 12
                Gate::new_and(1, 12),   // 13
                Gate::new_and(1, 13),   // 14
                Gate::new_and(14, 0),   // 15
                Gate::new_nor(1, 12),   // 16
                Gate::new_nor(16, 0),   // 17
                Gate::new_nor(1, 17),   // 18
                Gate::new_nor(1, 18),   // 19
                Gate::new_nor(19, 0),   // 20
                Gate::new_nor(1, 12),   // 21
                Gate::new_nor(1, 21),   // 22
                Gate::new_nor(1, 22),   // 23
                Gate::new_xor(12, 0),   // 24
                Gate::new_nimpl(12, 0), // 25
                Gate::new_nimpl(1, 25), // 26
            ],
            [(15, false), (20, true), (26, true), (23, false), (24, true)],
        )
        .unwrap();
        assert_eq!(
            vec![
                DivCircuitEntry {
                    circuit: Circuit::new(
                        6,
                        [
                            Gate::new_xor(1, 0),   // 6
                            Gate::new_xor(1, 2),   // 7
                            Gate::new_xor(3, 4),   // 8
                            Gate::new_xor(4, 5),   // 9
                            Gate::new_xor(6, 7),   // 10
                            Gate::new_xor(8, 9),   // 11
                            Gate::new_and(10, 11), // 12
                        ],
                        [(12, false)]
                    )
                    .unwrap(),
                    input_ps: None,
                    output_ps: Some(Placement {
                        placements: vec![2],
                        real_len: 6
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        3,
                        [
                            Gate::new_and(1, 2), // 13
                            Gate::new_and(1, 3), // 14
                            Gate::new_and(4, 0), // 15
                            Gate::new_nor(1, 2), // 16
                            Gate::new_nor(6, 0), // 17
                            Gate::new_nor(1, 7), // 18
                            Gate::new_nor(1, 8), // 19
                        ],
                        [(5, false), (9, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2],
                        real_len: 6
                    }),
                    output_ps: Some(Placement {
                        placements: vec![3, 4],
                        real_len: 6
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        5,
                        [
                            Gate::new_nor(4, 0),    // 20
                            Gate::new_nor(1, 2),    // 21
                            Gate::new_nor(1, 6),    // 22
                            Gate::new_nor(1, 7),    // 23
                            Gate::new_xor(2, 0),    // 24
                            Gate::new_nimpl(2, 0),  // 25
                            Gate::new_nimpl(1, 10), // 26
                        ],
                        [(3, false), (5, true), (11, true), (8, false), (9, true)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3, 4],
                        real_len: 6
                    }),
                    output_ps: None,
                },
            ],
            divide_circuit_traverse(circuit.clone(), 7)
        );
        assert_eq!(
            vec![
                DivCircuitEntry {
                    circuit: Circuit::new(
                        6,
                        [
                            Gate::new_xor(1, 0),   // 6
                            Gate::new_xor(1, 2),   // 7
                            Gate::new_xor(3, 4),   // 8
                            Gate::new_xor(4, 5),   // 9
                            Gate::new_xor(6, 7),   // 10
                            Gate::new_xor(8, 9),   // 11
                            Gate::new_and(10, 11), // 12
                            Gate::new_and(1, 12),  // 13
                            Gate::new_and(1, 13),  // 14
                        ],
                        [(12, false), (14, false)]
                    )
                    .unwrap(),
                    input_ps: None,
                    output_ps: Some(Placement {
                        placements: vec![2, 3],
                        real_len: 7
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        4,
                        [
                            Gate::new_and(3, 0),    // 15
                            Gate::new_nor(1, 2),    // 16
                            Gate::new_nor(5, 0),    // 17
                            Gate::new_nor(1, 6),    // 18
                            Gate::new_nor(1, 7),    // 19
                            Gate::new_nor(8, 0),    // 20
                            Gate::new_nor(1, 2),    // 21
                            Gate::new_nimpl(2, 0),  // 25
                            Gate::new_nimpl(1, 11), // 26
                        ],
                        [(4, false), (9, false), (10, false), (12, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3],
                        real_len: 7
                    }),
                    output_ps: Some(Placement {
                        placements: vec![3, 4, 5, 6],
                        real_len: 7
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        7,
                        [
                            Gate::new_nor(1, 5), // 22
                            Gate::new_nor(1, 7), // 23
                            Gate::new_xor(2, 0), // 24
                        ],
                        [(3, false), (4, true), (6, true), (8, false), (9, true)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3, 4, 5, 6],
                        real_len: 7
                    }),
                    output_ps: None,
                },
            ],
            divide_circuit_traverse(circuit.clone(), 9)
        );
        assert_eq!(
            vec![
                DivCircuitEntry {
                    circuit: Circuit::new(
                        6,
                        [
                            Gate::new_xor(1, 0), // 6
                            Gate::new_xor(1, 2), // 7
                            Gate::new_xor(3, 4), // 8
                            Gate::new_xor(6, 7), // 10
                        ],
                        [(8, false), (9, false), (5, false)]
                    )
                    .unwrap(),
                    input_ps: None,
                    output_ps: Some(Placement {
                        placements: vec![2, 3, 5],
                        real_len: 6
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        5,
                        [
                            Gate::new_xor(1, 2), // 9
                            Gate::new_xor(3, 5), // 11
                            Gate::new_and(4, 6), // 12
                            Gate::new_and(0, 7), // 13
                        ],
                        [(7, false), (8, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![1, 4, 5, 2, 3],
                        real_len: 6
                    }),
                    output_ps: Some(Placement {
                        placements: vec![2, 3],
                        real_len: 6
                    })
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        4,
                        [
                            Gate::new_and(1, 3), // 14
                            Gate::new_and(4, 0), // 15
                            Gate::new_nor(1, 2), // 16
                            Gate::new_nor(6, 0), // 17
                        ],
                        [(5, false), (7, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3],
                        real_len: 6
                    }),
                    output_ps: Some(Placement {
                        placements: vec![3, 4],
                        real_len: 6
                    })
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        4,
                        [
                            Gate::new_nor(1, 3),   // 18
                            Gate::new_nor(1, 4),   // 19
                            Gate::new_nor(5, 0),   // 20
                            Gate::new_nimpl(2, 0), // 25
                        ],
                        [(6, false), (7, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 4],
                        real_len: 6
                    }),
                    output_ps: Some(Placement {
                        placements: vec![4, 5],
                        real_len: 6
                    })
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        3,
                        [
                            Gate::new_nor(0, 1),   // 21
                            Gate::new_nor(0, 3),   // 22
                            Gate::new_nor(0, 4),   // 23
                            Gate::new_nimpl(0, 2), // 26
                        ],
                        [(5, false), (6, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![1, 2, 5],
                        real_len: 6
                    }),
                    output_ps: Some(Placement {
                        placements: vec![1, 5],
                        real_len: 6
                    })
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        6,
                        [
                            Gate::new_xor(1, 0),   // 24
                        ],
                        [(2, false), (3, true), (5, true), (4, false), (6, true)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 2, 3, 4, 1, 5],
                        real_len: 6
                    }),
                    output_ps: None
                },
            ],
            divide_circuit_traverse(circuit.clone(), 4)
        );
        // if no division
        assert_eq!(
            vec![DivCircuitEntry {
                circuit: circuit.clone(),
                input_ps: None,
                output_ps: None
            }],
            divide_circuit_traverse(circuit.clone(), 21)
        );
        // if no division
        assert_eq!(
            vec![DivCircuitEntry {
                circuit: circuit.clone(),
                input_ps: None,
                output_ps: None
            }],
            divide_circuit_traverse(circuit.clone(), 30)
        );

        // multiply 2x2 bit circuit
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
            [(4, false), (8, false), (10, false), (11, false)],
        )
        .unwrap();
        assert_eq!(
            vec![
                DivCircuitEntry {
                    circuit: Circuit::new(
                        4,
                        [
                            Gate::new_and(0, 2),
                            Gate::new_and(1, 2),
                            Gate::new_and(0, 3),
                            Gate::new_and(1, 3),
                            // add a1*b0 + a0*b1
                            Gate::new_xor(5, 6),
                        ],
                        [(4, false), (5, false), (6, false), (7, false), (8, false)]
                    )
                    .unwrap(),
                    input_ps: None,
                    output_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3, 4],
                        real_len: 5
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        5,
                        [
                            Gate::new_and(1, 2),
                            // add c(a1*b0 + a0*b1) + a1*b1
                            Gate::new_xor(3, 5),
                            Gate::new_and(3, 5),
                        ],
                        [(0, false), (4, false), (6, false), (7, false)]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 1, 2, 3, 4],
                        real_len: 5
                    }),
                    output_ps: None,
                },
            ],
            divide_circuit_traverse(circuit.clone(), 5)
        );
        // multiply 2x2 bit circuit with extra inputs and outputs
        let circuit = Circuit::new(
            6,
            [
                Gate::new_and(1, 3),
                Gate::new_and(2, 3),
                Gate::new_and(1, 4),
                Gate::new_and(2, 4),
                // add a1*b0 + a0*b1
                Gate::new_xor(7, 8),
                Gate::new_and(7, 8),
                // add c(a1*b0 + a0*b1) + a1*b1
                Gate::new_xor(9, 11),
                Gate::new_and(9, 11),
            ],
            [
                (6, false),
                (5, true),
                (10, false),
                (12, false),
                (13, false),
                (0, true),
            ],
        )
        .unwrap();
        assert_eq!(
            vec![
                DivCircuitEntry {
                    circuit: Circuit::new(
                        6,
                        [
                            Gate::new_and(1, 3),
                            Gate::new_and(2, 3),
                            Gate::new_and(1, 4),
                            Gate::new_and(2, 4),
                            // add a1*b0 + a0*b1
                            Gate::new_xor(7, 8),
                        ],
                        [
                            (6, false),
                            (7, false),
                            (8, false),
                            (9, false),
                            (10, false),
                            (0, false),
                            (5, false)
                        ]
                    )
                    .unwrap(),
                    input_ps: None,
                    output_ps: Some(Placement {
                        placements: vec![1, 2, 3, 4, 6, 0, 5],
                        real_len: 7
                    }),
                },
                DivCircuitEntry {
                    circuit: Circuit::new(
                        7,
                        [
                            Gate::new_and(3, 4),
                            // add c(a1*b0 + a0*b1) + a1*b1
                            Gate::new_xor(5, 7),
                            Gate::new_and(5, 7),
                        ],
                        [
                            (2, false),
                            (1, true),
                            (6, false),
                            (8, false),
                            (9, false),
                            (0, true)
                        ]
                    )
                    .unwrap(),
                    input_ps: Some(Placement {
                        placements: vec![0, 5, 1, 2, 3, 4, 6],
                        real_len: 7
                    }),
                    output_ps: None,
                },
            ],
            divide_circuit_traverse(circuit.clone(), 5)
        );
    }
}

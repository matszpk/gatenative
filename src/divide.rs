use gatesim::*;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;

pub(crate) struct Placement {
    id: usize,
    placements: Vec<usize>,
    real_len: usize,
}

pub(crate) struct DivCircuitEntry<T: Clone + Copy> {
    circuit: Circuit<T>,
    input_placements: Placement,
    output_placements: Placement,
}

pub(crate) struct DivCircuit<T: Clone + Copy>(Vec<DivCircuitEntry<T>>);

// separate circuit sequentially
fn separate_circuit_seq<T>(
    circuit: Circuit<T>,
    roots: &[usize],
    max_gates: usize,
    min_depth: usize,
) -> (Circuit<T>, Vec<usize>)
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let gates = circuit.gates();
    let mut cur_nodes = HashSet::<usize>::from_iter(roots.into_iter().copied());
    let mut next_nodes = HashSet::<usize>::new();
    let mut visited = BTreeMap::<usize, usize>::new();
    let mut new_inputs = BTreeSet::<usize>::new();

    // use BFS - breadth first search.
    let mut depth = 0;
    loop {
        for node in &cur_nodes {
            if *node < input_len {
                new_inputs.insert(*node);
            } else if !visited.contains_key(&node) {
                let g = gates[node - input_len];
                next_nodes.insert(usize::try_from(g.i0).unwrap());
                next_nodes.insert(usize::try_from(g.i1).unwrap());
            }
        }
        if depth >= min_depth && visited.len() + cur_nodes.len() >= max_gates {
            break;
        }
        // insert to visited map
        for node in &cur_nodes {
            if *node >= input_len && !visited.contains_key(&node) {
                visited.insert(*node, visited.len());
            }
        }
        std::mem::swap(&mut cur_nodes, &mut next_nodes);
        next_nodes.clear();
        depth += 1;
    }
    // collect new inputs
    new_inputs.extend(cur_nodes);
    let new_inputs = BTreeMap::from_iter(new_inputs.into_iter().enumerate().map(|(i, x)| (x, i)));
    let new_input_len = new_inputs.len();
    let new_gate_num = visited.len();

    (
        Circuit::<T>::new(
            T::try_from(new_input_len).unwrap(),
            visited
                .iter()
                .rev()
                .map(|(orig, new)| {
                    let g = gates[*orig];
                    let gi0 = usize::try_from(g.i0).unwrap();
                    let gi1 = usize::try_from(g.i0).unwrap();
                    Gate {
                        func: g.func,
                        i0: T::try_from(if gi0 >= input_len {
                            if let Some(idx) = visited.get(&gi0) {
                                (new_gate_num - idx - 1) + new_input_len
                            } else {
                                new_inputs[&gi0]
                            }
                        } else {
                            new_inputs[&gi0]
                        })
                        .unwrap(),
                        i1: T::try_from(if gi1 >= input_len {
                            if let Some(idx) = visited.get(&gi1) {
                                (new_gate_num - idx - 1) + new_input_len
                            } else {
                                new_inputs[&gi1]
                            }
                        } else {
                            new_inputs[&gi1]
                        })
                        .unwrap(),
                    }
                })
                .collect::<Vec<_>>(),
            roots.iter().map(|x| {
                (
                    T::try_from(if *x >= input_len {
                        if let Some(idx) = visited.get(x) {
                            (new_gate_num - idx - 1) + new_input_len
                        } else {
                            new_inputs[x]
                        }
                    } else {
                        new_inputs[x]
                    })
                    .unwrap(),
                    false,
                )
            }),
        )
        .unwrap(),
        Vec::<usize>::from_iter(new_inputs.keys().copied()),
    )
}

impl<T: Clone + Copy> DivCircuit<T> {
    pub(crate) fn new(circuit: Circuit<T>, max_gates: usize) -> Self {
        Self(vec![])
    }
}

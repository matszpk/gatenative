use gatesim::*;

use std::collections::{BTreeSet, HashSet};
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

fn decouple_circuit<T>(
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
    let mut visited = BTreeSet::<usize>::new();
    let mut new_inputs = BTreeSet::<usize>::new();

    // use BFS - breadth first search.
    let mut depth = 0;
    loop {
        for node in &cur_nodes {
            if *node < input_len {
                new_inputs.insert(*node);
            } else if !visited.contains(&node) {
                let g = gates[node - input_len];
                next_nodes.insert(usize::try_from(g.i0).unwrap());
                next_nodes.insert(usize::try_from(g.i1).unwrap());
            }
        }
        if depth >= min_depth && visited.len() + cur_nodes.len() >= max_gates {
            break;
        }
        visited.extend(cur_nodes.clone());
        std::mem::swap(&mut cur_nodes, &mut next_nodes);
        next_nodes.clear();
        depth += 1;
    }
    // collect new inputs
    new_inputs.extend(cur_nodes);

    (
        Circuit::<T>::new(T::try_from(new_inputs.len()).unwrap(), [], []).unwrap(),
        Vec::<usize>::from_iter(new_inputs),
    )
}

impl<T: Clone + Copy> DivCircuit<T> {
    pub(crate) fn new(circuit: Circuit<T>, max_gates: usize) -> Self {
        Self(vec![])
    }
}

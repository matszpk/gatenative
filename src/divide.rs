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
    circuit: &Circuit<T>,
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
    let mut cur_nodes = BTreeSet::<usize>::from_iter(roots.into_iter().copied());
    let mut next_nodes = BTreeSet::<usize>::new();
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
        // insert to visited map
        for node in &cur_nodes {
            if *node >= input_len && !visited.contains(&node) {
                visited.insert(*node);
            }
        }
        std::mem::swap(&mut cur_nodes, &mut next_nodes);
        next_nodes.clear();
        depth += 1;
    }
    // collect new inputs
    new_inputs.extend(cur_nodes);
    let new_inputs = BTreeMap::from_iter(new_inputs.into_iter().enumerate().map(|(i, x)| (x, i)));
    let visited = BTreeMap::from_iter(visited.into_iter().enumerate().map(|(i, x)| (x, i)));
    let new_input_len = new_inputs.len();
    let new_gate_num = visited.len();

    // println!("new_inputs: {:?}", new_inputs);
    // println!("visited: {:?}", visited);

    (
        Circuit::<T>::new(
            T::try_from(new_input_len).unwrap(),
            visited
                .iter()
                .map(|(orig, _)| {
                    let g = gates[*orig - input_len];
                    let gi0 = usize::try_from(g.i0).unwrap();
                    let gi1 = usize::try_from(g.i1).unwrap();
                    let g = Gate {
                        func: g.func,
                        i0: T::try_from(if gi0 >= input_len {
                            if let Some(idx) = visited.get(&gi0) {
                                idx + new_input_len
                            } else {
                                new_inputs[&gi0]
                            }
                        } else {
                            new_inputs[&gi0]
                        })
                        .unwrap(),
                        i1: T::try_from(if gi1 >= input_len {
                            if let Some(idx) = visited.get(&gi1) {
                                idx + new_input_len
                            } else {
                                new_inputs[&gi1]
                            }
                        } else {
                            new_inputs[&gi1]
                        })
                        .unwrap(),
                    };
                    //println!("Gate g: {:?}", g);
                    g
                })
                .collect::<Vec<_>>(),
            roots.iter().map(|x| {
                (
                    T::try_from(if *x >= input_len {
                        if let Some(idx) = visited.get(x) {
                            idx + new_input_len
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_circuit_seq() {
        let circuit = Circuit::new(
            5,
            [
                Gate::new_and(0, 1),     // 5
                Gate::new_and(0, 2),     // 6
                Gate::new_and(2, 3),     // 7
                Gate::new_and(3, 4),     // 8
                Gate::new_nor(5, 6),     // 9
                Gate::new_nor(7, 8),     // 10
                Gate::new_nor(9, 10),    // 11
                Gate::new_and(0, 3),     // 12
                Gate::new_and(1, 4),     // 13
                Gate::new_and(2, 4),     // 14
                Gate::new_and(1, 3),     // 15
                Gate::new_xor(12, 13),   // 16
                Gate::new_xor(14, 15),   // 17
                Gate::new_xor(16, 17),   // 18
                Gate::new_and(1, 2),     // 19
                Gate::new_nor(0, 4),     // 20
                Gate::new_nimpl(3, 1),   // 21
                Gate::new_nimpl(2, 4),   // 22
                Gate::new_xor(19, 20),   // 23
                Gate::new_xor(21, 22),   // 24
                Gate::new_xor(23, 24),   // 25
                Gate::new_and(0, 11),    // 26
                Gate::new_and(11, 18),   // 27
                Gate::new_and(11, 25),   // 28
                Gate::new_and(2, 25),    // 29
                Gate::new_nimpl(26, 27), // 30
                Gate::new_nimpl(28, 29), // 31
                Gate::new_nor(30, 31),   // 32
                Gate::new_and(2, 11),    // 33
                Gate::new_and(3, 18),    // 34
                Gate::new_and(4, 25),    // 35
                Gate::new_and(1, 11),    // 36
                Gate::new_nimpl(33, 34), // 37
                Gate::new_nimpl(35, 36), // 38
                Gate::new_nor(37, 38),   // 39
                Gate::new_and(0, 32),    // 40
                Gate::new_and(1, 32),    // 41
                Gate::new_and(1, 39),    // 42
                Gate::new_and(4, 39),    // 43
                Gate::new_nimpl(40, 41), // 44
                Gate::new_nimpl(42, 43), // 45
                Gate::new_nor(44, 45),   // 46
                Gate::new_and(2, 32),    // 47
                Gate::new_and(2, 39),    // 48
                Gate::new_and(4, 32),    // 49
                Gate::new_and(0, 39),    // 50
                Gate::new_nimpl(47, 48), // 51
                Gate::new_nimpl(49, 50), // 52
                Gate::new_nor(51, 52),   // 53
            ],
            [(46, false), (53, false)],
        )
        .unwrap();

        assert_eq!(
            (
                Circuit::new(
                    6,
                    [
                        Gate::new_and(0, 4),
                        Gate::new_and(1, 4),
                        Gate::new_and(1, 5),
                        Gate::new_and(3, 5),
                        Gate::new_nimpl(6, 7),
                        Gate::new_nimpl(8, 9),
                        Gate::new_nor(10, 11),
                        Gate::new_and(2, 4),
                        Gate::new_and(2, 5),
                        Gate::new_and(3, 4),
                        Gate::new_and(0, 5),
                        Gate::new_nimpl(13, 14),
                        Gate::new_nimpl(15, 16),
                        Gate::new_nor(17, 18),
                    ],
                    [(12, false), (19, false)]
                )
                .unwrap(),
                vec![0, 1, 2, 4, 32, 39]
            ),
            separate_circuit_seq(&circuit, &[46, 53], 20, 2),
        );
    }
}

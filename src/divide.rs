use gatesim::*;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

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
fn calculate_gate_depths<T>(circuit: &Circuit<T>) -> Vec<Vec<T>>
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
    let gates = circuit.gates();
    let mut visited = HashSet::new();

    let mut depth_map = HashMap::new();
    let mut max_depth = 0;

    // key - circuit output, value - start depth
    let mut update_map = circuit
        .outputs()
        .iter()
        .map(|(x, _)| (*x, 0))
        .collect::<Vec<_>>();
    // sort update map by gate output index in reverse order
    update_map.sort();
    update_map.reverse();
    let mut new_update_map = HashMap::new();

    while !update_map.is_empty() {
        for (o, sd) in &update_map {
            if *o < input_len_t {
                continue;
            }
            let oidx = usize::try_from(*o).unwrap() - input_len;
            let mut stack = Vec::new();
            stack.push(StackEntry { node: oidx, way: 0 });

            while !stack.is_empty() {
                let new_depth_u = sd + stack.len() - 1;
                let new_depth = T::try_from(new_depth_u).unwrap();
                let top = stack.last_mut().unwrap();
                let node_index = top.node;
                let way = top.way;
                // gidx - circuit output index for gate (used in same gate inputs).
                let gidx = T::try_from(input_len + node_index).unwrap();
                let depth = depth_map.get(&gidx).copied().unwrap_or(T::default());

                if way == 0 {
                    if !visited.contains(&node_index) {
                        visited.insert(node_index);
                    } else {
                        if new_depth > depth {
                            // add new entry to new update map with new start depth=new_depth
                            if let Some(new_sd) = new_update_map.get_mut(&gidx) {
                                *new_sd = std::cmp::max(*new_sd, new_depth_u);
                            } else {
                                new_update_map.insert(gidx, new_depth_u);
                            }
                        }
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
                    stack.pop();
                    depth_map.insert(gidx, std::cmp::max(depth, new_depth));
                    max_depth = std::cmp::max(max_depth, new_depth_u);
                }
            }
        }
        // replace update map
        update_map = new_update_map
            .iter()
            .map(|(x, d)| (*x, *d))
            .collect::<Vec<_>>();
        // sort update map by gate output index in reverse order
        update_map.sort();
        update_map.reverse();
        new_update_map.clear();
        // clear visited
        visited.clear();
    }

    let mut depths = vec![vec![]; max_depth + 1];
    for (v, d) in depth_map {
        depths[usize::try_from(d).unwrap()].push(v);
    }
    for nodes in &mut depths {
        nodes.sort();
    }
    depths
}

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
        if depth >= min_depth && visited.len() + cur_nodes.len() > max_gates {
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

    // println!("new_inputs: {:?}", new_inputs);
    // println!("visited: {:?}", visited);

    let old_outputs = circuit.outputs();
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
                    match old_outputs
                        .binary_search_by_key(&T::try_from(*x).unwrap(), |(idx, _)| *idx)
                    {
                        Ok(p) => old_outputs[p].1,
                        Err(_) => false,
                    },
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
    fn test_calculate_gate_depths() {
        assert_eq!(
            vec![vec![10], vec![8, 9], vec![4, 5, 6, 7]],
            calculate_gate_depths(
                &Circuit::new(
                    4,
                    [
                        Gate::new_and(0, 1),
                        Gate::new_and(1, 2),
                        Gate::new_and(2, 3),
                        Gate::new_nimpl(3, 1),
                        Gate::new_nor(4, 5),
                        Gate::new_nor(6, 7),
                        Gate::new_nor(8, 9),
                    ],
                    [(10, false)]
                )
                .unwrap()
            )
        );
        assert_eq!(
            vec![vec![10, 11, 12], vec![8, 9], vec![4, 5, 6, 7]],
            calculate_gate_depths(
                &Circuit::new(
                    4,
                    [
                        Gate::new_and(0, 1),
                        Gate::new_and(1, 2),
                        Gate::new_and(2, 3),
                        Gate::new_nimpl(3, 1),
                        Gate::new_nor(4, 5),
                        Gate::new_nor(6, 7),
                        Gate::new_xor(5, 6),
                        Gate::new_nor(8, 9),
                        Gate::new_nor(5, 7),
                    ],
                    [(10, false), (11, false), (12, false)]
                )
                .unwrap()
            )
        );
        assert_eq!(
            vec![vec![10], vec![8, 9], vec![4, 5, 6, 7]],
            calculate_gate_depths(
                &Circuit::new(
                    4,
                    [
                        Gate::new_and(0, 1),
                        Gate::new_and(1, 2),
                        Gate::new_and(2, 3),
                        Gate::new_nimpl(3, 1),
                        Gate::new_nor(4, 5),
                        Gate::new_nor(6, 7),
                        Gate::new_nor(8, 9),
                    ],
                    [(8, false), (10, false)]
                )
                .unwrap()
            )
        );
        assert_eq!(
            vec![vec![10, 12], vec![8, 11], vec![4, 5, 9], vec![6, 7]],
            calculate_gate_depths(
                &Circuit::new(
                    4,
                    [
                        Gate::new_and(0, 1),
                        Gate::new_and(1, 2),
                        Gate::new_and(2, 3),
                        Gate::new_nimpl(3, 1),
                        Gate::new_nor(4, 5),
                        Gate::new_nor(6, 7),
                        Gate::new_nor(8, 9),
                        Gate::new_nor(5, 9),
                        Gate::new_xor(6, 11),
                    ],
                    [(6, false), (8, false), (10, false), (12, false)]
                )
                .unwrap()
            )
        );
        assert_eq!(
            vec![vec![4, 8, 10, 11], vec![7, 9], vec![5, 6]],
            calculate_gate_depths(
                &Circuit::new(
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
                .unwrap()
            )
        );
    }

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
            [(46, true), (53, false)],
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
                    [(12, true), (19, false)]
                )
                .unwrap(),
                vec![0, 1, 2, 4, 32, 39]
            ),
            separate_circuit_seq(&circuit, &[46, 53], 18, 2),
        );

        assert_eq!(
            (
                Circuit::new(
                    8,
                    [
                        Gate::new_and(0, 5),
                        Gate::new_and(5, 6),
                        Gate::new_and(5, 7),
                        Gate::new_and(2, 7),
                        Gate::new_nimpl(8, 9),
                        Gate::new_nimpl(10, 11),
                        Gate::new_nor(12, 13),
                        Gate::new_and(2, 5),
                        Gate::new_and(3, 6),
                        Gate::new_and(4, 7),
                        Gate::new_and(1, 5),
                        Gate::new_nimpl(15, 16),
                        Gate::new_nimpl(17, 18),
                        Gate::new_nor(19, 20),
                    ],
                    [(14, false), (21, false)]
                )
                .unwrap(),
                vec![0, 1, 2, 3, 4, 11, 18, 25]
            ),
            separate_circuit_seq(&circuit, &[32, 39], 18, 2),
        );

        let circuit = Circuit::new(
            6,
            [
                Gate::new_and(0, 1),     // 6
                Gate::new_nor(0, 2),     // 7
                Gate::new_nor(6, 7),     // 8
                Gate::new_and(0, 3),     // 9
                Gate::new_xor(0, 4),     // 10
                Gate::new_nor(9, 10),    // 11
                Gate::new_nimpl(8, 11),  // 12
                Gate::new_and(0, 5),     // 13
                Gate::new_nor(1, 2),     // 14
                Gate::new_nimpl(13, 14), // 15
                Gate::new_and(1, 3),     // 16
                Gate::new_xor(1, 4),     // 17
                Gate::new_nor(16, 17),   // 18
                Gate::new_nimpl(15, 18), // 19
                Gate::new_xor(12, 19),   // 20
                Gate::new_and(1, 5),     // 21
                Gate::new_nor(2, 3),     // 22
                Gate::new_xor(21, 22),   // 23
                Gate::new_and(2, 4),     // 24
                Gate::new_xor(2, 5),     // 25
                Gate::new_nor(24, 25),   // 26
                Gate::new_nimpl(23, 26), // 27
                Gate::new_and(3, 4),     // 28
                Gate::new_nor(3, 5),     // 29
                Gate::new_and(28, 29),   // 30
                Gate::new_and(4, 5),     // 31
                Gate::new_xor(4, 5),     // 32
                Gate::new_nor(31, 32),   // 33
                Gate::new_nimpl(30, 33), // 34
                Gate::new_xor(27, 34),   // 35
                Gate::new_xor(20, 35),   // 36
            ],
            [(36, false)],
        )
        .unwrap();

        assert_eq!(
            (
                Circuit::new(
                    16,
                    [
                        Gate::new_nor(0, 1),
                        Gate::new_nor(2, 3),
                        Gate::new_nimpl(16, 17),
                        Gate::new_nimpl(4, 5),
                        Gate::new_nor(6, 7),
                        Gate::new_nimpl(19, 20),
                        Gate::new_xor(18, 21),
                        Gate::new_xor(8, 9),
                        Gate::new_nor(10, 11),
                        Gate::new_nimpl(23, 24),
                        Gate::new_and(12, 13),
                        Gate::new_nor(14, 15),
                        Gate::new_nimpl(26, 27),
                        Gate::new_xor(25, 28),
                        Gate::new_xor(22, 29)
                    ],
                    [(30, false)]
                )
                .unwrap(),
                vec![6, 7, 9, 10, 13, 14, 16, 17, 21, 22, 24, 25, 28, 29, 31, 32]
            ),
            separate_circuit_seq(&circuit, &[36], 18, 2),
        );
    }
}

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

// IDEA: from circuit input to last circuit outputs:
// move some circuit outputs that will be used later to last outputs by copying.

// IDEA:
// division layout:
// cseq0 cseq1 (cpar20 tback20 cpar21 tback21 cpar22 ...) cseq3 ....
// cseqX - sequential part of circuit
// cpartXX - parallel part of sequential part of circuit
// tbackXX - empty circuit that move back output of previous cparXX to input of next cparXY.

// returns (min_depths, max_depths) - list of list of depths.
// index of main list: depth (from circuit inputs).
fn calculate_gate_depths<T>(circuit: &Circuit<T>) -> (Vec<Vec<T>>, Vec<Vec<T>>)
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
    let mut visited = HashSet::new();

    let mut depth_map = vec![(usize::MAX, 0); gate_num];
    let mut max_depth = 0;
    // min max depth - maximal depth of min depth of gate
    let mut min_max_depth = 0;

    // key - circuit output, value - start depth
    let mut update_map = circuit
        .outputs()
        .iter()
        .map(|(x, _)| (*x, (usize::MAX, 0)))
        .collect::<Vec<_>>();
    // sort update map by gate output index in reverse order
    update_map.sort();
    update_map.reverse();
    let mut new_update_map = HashMap::new();

    while !update_map.is_empty() {
        for (o, (min_sd, sd)) in &update_map {
            if *o < input_len_t {
                continue;
            }
            let oidx = usize::try_from(*o).unwrap() - input_len;
            let mut stack = Vec::new();
            stack.push(StackEntry { node: oidx, way: 0 });

            while !stack.is_empty() {
                let min_new_depth_u = if *min_sd != usize::MAX {
                    min_sd + stack.len() - 1
                } else {
                    sd + stack.len() - 1
                };
                let new_depth_u = sd + stack.len() - 1;
                let top = stack.last_mut().unwrap();
                let node_index = top.node;
                let way = top.way;
                // gidx - circuit output index for gate (used in same gate inputs).
                let gidx = T::try_from(input_len + node_index).unwrap();
                let (lower_depth, upper_depth) = depth_map[node_index];

                if way == 0 {
                    if !visited.contains(&node_index) {
                        visited.insert(node_index);
                    } else {
                        if min_new_depth_u < lower_depth || new_depth_u > upper_depth {
                            // add new entry to new update map with new start depth=new_depth
                            if let Some((new_min_sd, new_sd)) = new_update_map.get_mut(&gidx) {
                                *new_min_sd = std::cmp::min(*new_min_sd, min_new_depth_u);
                                *new_sd = std::cmp::max(*new_sd, new_depth_u);
                            } else {
                                new_update_map.insert(gidx, (min_new_depth_u, new_depth_u));
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
                    depth_map[node_index].0 = std::cmp::min(lower_depth, min_new_depth_u);
                    depth_map[node_index].1 = std::cmp::max(upper_depth, new_depth_u);
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

    let min_max_depth = depth_map
        .iter()
        .filter_map(|(x, _)| if *x != usize::MAX { Some(*x) } else { None })
        .max()
        .unwrap_or(0);
    let mut min_depths = vec![vec![]; min_max_depth + 1];
    let mut depths = vec![vec![]; max_depth + 1];
    for (v, (ld, ud)) in depth_map.into_iter().enumerate() {
        if ld != usize::MAX {
            min_depths[min_max_depth - ld].push(T::try_from(input_len + v).unwrap());
        }
        depths[max_depth - ud].push(T::try_from(input_len + v).unwrap());
    }
    for nodes in &mut min_depths {
        nodes.sort();
    }
    for nodes in &mut depths {
        nodes.sort();
    }
    (min_depths, depths)
}

// separate circuit sequentially - using depths instead circuit
// fn separate_circuit_seq<T>(
//     circuit: &Circuit<T>,
//     roots: &[usize],
//     max_gates: usize,
//     min_depth: usize,
// ) -> (Circuit<T>, Vec<usize>)

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
            (
                vec![vec![4, 5, 6, 7], vec![8, 9], vec![10]],
                vec![vec![4, 5, 6, 7], vec![8, 9], vec![10]],
            ),
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
            (
                vec![vec![4], vec![5, 6, 7, 8, 9], vec![10, 11, 12]],
                vec![vec![4, 5, 6, 7], vec![8, 9], vec![10, 11, 12]],
            ),
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
            (
                vec![vec![6, 7], vec![4, 5, 9], vec![8, 10]],
                vec![vec![4, 5, 6, 7], vec![8, 9], vec![10]],
            ),
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
            (
                vec![vec![7], vec![4, 5, 9, 11], vec![6, 8, 10, 12]],
                vec![vec![6, 7], vec![4, 5, 9], vec![8, 11], vec![10, 12]],
            ),
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
            (
                vec![
                    vec![16],
                    vec![6, 7, 8, 9, 17],
                    vec![10, 11, 13, 18, 21],
                    vec![12, 14, 19, 22, 25],
                    vec![15, 20, 23, 24, 26],
                ],
                vec![
                    vec![6, 7, 8, 9],
                    vec![10, 11],
                    vec![12],
                    vec![16],
                    vec![17],
                    vec![13, 18, 21],
                    vec![14, 19, 22, 25],
                    vec![15, 20, 23, 24, 26],
                ],
            ),
            calculate_gate_depths(
                &Circuit::new(
                    6,
                    [
                        Gate::new_xor(0, 1),    // 6
                        Gate::new_xor(1, 2),    // 7
                        Gate::new_xor(3, 4),    // 8
                        Gate::new_xor(4, 5),    // 9
                        Gate::new_xor(6, 7),    // 10
                        Gate::new_xor(8, 9),    // 11
                        Gate::new_and(10, 11),  // 12
                        Gate::new_and(1, 12),   // 13
                        Gate::new_and(1, 13),   // 14
                        Gate::new_and(0, 14),   // 15
                        Gate::new_nor(1, 12),   // 16
                        Gate::new_nor(0, 16),   // 17
                        Gate::new_nor(1, 17),   // 18
                        Gate::new_nor(1, 18),   // 19
                        Gate::new_nor(0, 19),   // 20
                        Gate::new_nor(1, 12),   // 21
                        Gate::new_nor(1, 21),   // 22
                        Gate::new_nor(1, 22),   // 23
                        Gate::new_xor(0, 12),   // 24
                        Gate::new_nimpl(0, 12), // 25
                        Gate::new_nimpl(1, 25), // 26
                    ],
                    [
                        (15, false),
                        (20, false),
                        (23, false),
                        (24, false),
                        (26, false)
                    ]
                )
                .unwrap()
            )
        );
        assert_eq!(
            (
                vec![vec![5, 6, 7, 9], vec![4, 8, 10, 11]],
                vec![vec![5, 6], vec![7, 9], vec![4, 8, 10, 11]],
            ),
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
}

use gatesim::*;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) struct Placement {
    id: usize, // placement id (common for input and output placement (it can be buffer id)
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

// returns min_depths with max_depths - list of list of depths.
// index of main list: depth (from circuit inputs), value - list of gates with max depth
// (max depth from circuit input).
fn calculate_gate_depths<T>(circuit: &Circuit<T>) -> Vec<Vec<(T, usize)>>
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

    let mut depth_map = vec![(usize::MAX, 0); input_len + gate_num];
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

    // first, collect and calculate depths from circuit outputs to circuit inputs.
    while !update_map.is_empty() {
        for (o, (min_sd, sd)) in &update_map {
            if *o < input_len_t {
                let o = usize::try_from(*o).unwrap();
                let min_depth = depth_map[o].0;
                depth_map[o].0 = std::cmp::min(min_depth, min_sd + 1);
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
                let gidx_u = input_len + node_index;
                let gidx = T::try_from(gidx_u).unwrap();
                let (lower_depth, upper_depth) = depth_map[gidx_u];

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
                    } else {
                        let o = usize::try_from(gi0).unwrap();
                        let min_depth = depth_map[o].0;
                        depth_map[o].0 = std::cmp::min(min_depth, min_new_depth_u + 1);
                    }
                } else if way == 1 {
                    top.way += 1;
                    let gi1 = gates[node_index].i1;
                    if gi1 >= input_len_t {
                        stack.push(StackEntry {
                            node: usize::try_from(gi1).unwrap() - input_len,
                            way: 0,
                        });
                    } else {
                        let o = usize::try_from(gi1).unwrap();
                        let min_depth = depth_map[o].0;
                        depth_map[o].0 = std::cmp::min(min_depth, min_new_depth_u + 1);
                    }
                } else {
                    // allocate and use
                    stack.pop();
                    depth_map[gidx_u].0 = std::cmp::min(lower_depth, min_new_depth_u);
                    depth_map[gidx_u].1 = std::cmp::max(upper_depth, new_depth_u);
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
    max_depth += 1;
    // fix max_depth (from circuit outputs) for circuit inputs.
    for i in 0..input_len {
        depth_map[i].1 = max_depth;
    }

    let mut depths = vec![vec![]; max_depth + 1];
    // now put depths in reverse order (from circuit inputs to circuit outputs).
    for (v, (ld, ud)) in depth_map.into_iter().enumerate() {
        assert_ne!(ld, usize::MAX);
        depths[max_depth - ud].push((T::try_from(v).unwrap(), max_depth - ld));
    }
    for nodes in &mut depths {
        nodes.sort_by_key(|(node, _)| *node);
    }
    depths
}

// separate circuit sequentially - using depths instead circuit
fn divide_circuit_seq<T>(
    circuit: Circuit<T>,
    gate_depths: Vec<Vec<(T, usize)>>,
    max_gates: usize,
    min_depth: usize,
) -> Vec<DivCircuitEntry<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // shared gate outputs
    let mut shared_outputs = BTreeMap::<T, usize>::new();

    let mut gate_count = 0;
    let mut start_depth = 0;
    let depth_num = gate_depths.len();
    let mut cur_gates = BTreeSet::new();
    let mut circuits = Vec::<DivCircuitEntry<T>>::new();

    // TODO: include circuit inputs in depths
    for (depth, gates) in gate_depths
        .into_iter()
        .chain(std::iter::once(vec![]))
        .enumerate()
    {
        let depth_gate_num = gates.len();
        gate_count += depth_gate_num;
        if depth == depth_num || (depth - start_depth >= min_depth && gate_count > max_gates) {
            let end_depth = depth;
            // add all gates that will be used later
            shared_outputs.extend(
                gates
                    .iter()
                    .filter(|(_, max_depth)| *max_depth >= end_depth)
                    .copied(),
            );

            // create circuit

            // remove all shared outputs that are not after this region
            shared_outputs.retain(|_, max_depth| *max_depth >= end_depth);
            cur_gates.clear();
            start_depth = end_depth;
            gate_count = depth_gate_num;
        }
        cur_gates.extend(gates);
    }
    circuits
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
            vec![
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(4, 1), (5, 1), (6, 1), (7, 1)],
                vec![(8, 2), (9, 2)],
                vec![(10, 3)]
            ],
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
            vec![
                vec![(0, 0), (1, 1), (2, 1), (3, 1)],
                vec![(4, 1), (5, 2), (6, 2), (7, 2)],
                vec![(8, 2), (9, 2)],
                vec![(10, 3), (11, 3), (12, 3)]
            ],
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
            vec![
                vec![(0, 1), (1, 1), (2, 1), (3, 0)],
                vec![(4, 2), (5, 2), (6, 1), (7, 1)],
                vec![(8, 3), (9, 2)],
                vec![(10, 3)]
            ],
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
            vec![
                vec![(0, 2), (1, 2), (2, 3), (3, 3)],
                vec![(6, 4), (7, 2)],
                vec![(4, 3), (5, 3), (9, 3)],
                vec![(8, 4), (11, 3)],
                vec![(10, 4), (12, 4)]
            ],
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
            vec![
                vec![(0, 7), (1, 7), (2, 4), (3, 4), (4, 4), (5, 4)],
                vec![(6, 5), (7, 5), (8, 5), (9, 5)],
                vec![(10, 6), (11, 6)],
                vec![(12, 7)],
                vec![(16, 4)],
                vec![(17, 5)],
                vec![(13, 6), (18, 6), (21, 6)],
                vec![(14, 7), (19, 7), (22, 7), (25, 7)],
                vec![(15, 8), (20, 8), (23, 8), (24, 8), (26, 8)],
            ],
            calculate_gate_depths(
                &Circuit::new(
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
            vec![
                vec![(0, 2), (1, 1), (2, 2), (3, 1)],
                vec![(5, 2), (6, 2)],
                vec![(7, 2), (9, 2)],
                vec![(4, 3), (8, 3), (10, 3), (11, 3)]
            ],
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

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

// IDEA: dividing by traversing from circuit outputs to circuit inputs by using DFS.
// maybe this is better idea than dividing circuit sequantially (as chain subcircuits).
// this is preferred IDEA. while traversing include gate outputs usage.
// choose than way where is smallest gate output usage.
// algorithm concept:
// 1. from first enqueued output: traverse to first circuit input:
// 2. try connect other circut inputs to further gates by using smallest number of inputs.
// 3. if after trying we have some ways that not finished circuit inputs.
//    then one of it can be done:
//    3.1. divide into two parts
//    3.2. try connect other gate inputs to subcircuit that will be before current subcircuit.
//         only possible if next subcircuit doesn't have shared gates between
//         current subcircuit except way from next subcircuit is connected.
//    3.3. remove that ways where are overflows. and go to 1. to create next subcircuit
//         starting from end (to circuit output) of way where is overflow.
//    3.4. divide next subcircuit with overflow into two parts:
//         * part for current subcircuit (executed as first)
//         * part for next subcircuit (executed as second)
// hint: try use one buffer as input and output for one subcircuit. warning: RISKY!!!
// it is possible because code generator always load data first and store data at end of code.

// IDEA:
// division layout:
// cseq0 cseq1 (cpar20 tback20 cpar21 tback21 cpar22 ...) cseq3 ....
// cseqX - sequential part of circuit
// cpartXX - parallel part of sequential part of circuit
// tbackXX - empty circuit that move back output of previous cparXX to input of next cparXY.

// returns min_depths with max_depths - list of list of depths.
// index of main list: depth (from circuit inputs), value - list of gates with max depth
// (max depth from circuit input).
// max depth - maximal depth (from circuit input) where is used gate.
//   important notice: usage counts from X gate that have inputs connected to this gate
//   from minimal depth of X gate.
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

    // first, collect and calculate depths from circuit outputs to circuit inputs.
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
                let top = stack.last_mut().unwrap();
                let node_index = top.node;
                let way = top.way;
                // gidx - circuit output index for gate (used in same gate inputs).
                let gidx_u = input_len + node_index;
                let gidx = T::try_from(gidx_u).unwrap();
                let upper_depth = depth_map[gidx_u].1;

                if way == 0 {
                    if !visited.contains(&node_index) {
                        visited.insert(node_index);
                    } else {
                        if new_depth_u > upper_depth {
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

    // resolve min depth (from output) - by direct connection to gate
    for (i, g) in gates.iter().enumerate() {
        let gidx = input_len + i;
        let gi0 = usize::try_from(g.i0).unwrap();
        depth_map[gi0].0 = std::cmp::min(depth_map[gi0].0, depth_map[gidx].1 + 1);
        let gi1 = usize::try_from(g.i1).unwrap();
        depth_map[gi1].0 = std::cmp::min(depth_map[gi1].0, depth_map[gidx].1 + 1);
    }
    for (gidx, _) in circuit.outputs() {
        depth_map[usize::try_from(*gidx).unwrap()].0 = 0;
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
) -> Vec<Circuit<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    if circuit.len() <= max_gates || gate_depths.len() <= min_depth + 1 {
        return vec![circuit];
    }
    // shared gate outputs
    let mut shared_outputs = BTreeMap::<T, usize>::from_iter(gate_depths[0].iter().copied());

    let mut gate_count = 0;
    let mut start_depth = 1;
    let depth_num = gate_depths.len();
    let mut cur_gates = BTreeSet::new();
    let mut circuits = vec![];
    let gates = circuit.gates();
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();

    for (depth, slice_gates) in gate_depths
        .into_iter()
        .chain(std::iter::once(vec![]))
        .enumerate()
        .skip(1)
    {
        let depth_gate_num = slice_gates.len();
        gate_count += depth_gate_num;
        if depth == depth_num || (depth - start_depth >= min_depth && gate_count > max_gates) {
            let end_depth = depth;
            println!(
                "DepthRange {}: {}-{}",
                circuits.len(),
                start_depth,
                end_depth
            );
            let last_subc = end_depth == depth_num;
            // create circuit
            let mut node_map = BTreeMap::<T, T>::from_iter(
                shared_outputs
                    .keys()
                    .enumerate()
                    .map(|(i, x)| (*x, T::try_from(i).unwrap())),
            );
            let subc_input_len = shared_outputs.len();
            let mut subc_gates = vec![];
            // create gates for subcircuit
            for (i, (gidx, _)) in cur_gates.iter().enumerate() {
                if *gidx < input_len_t {
                    continue;
                }
                let g = gates[usize::try_from(*gidx).unwrap() - input_len];
                subc_gates.push(Gate {
                    i0: if g.i0 >= input_len_t {
                        node_map[&g.i0]
                    } else {
                        g.i0
                    },
                    i1: if g.i1 >= input_len_t {
                        node_map[&g.i1]
                    } else {
                        g.i1
                    },
                    func: g.func,
                });
                // add new gate node map
                node_map.insert(*gidx, T::try_from(i + subc_input_len).unwrap());
            }

            println!("NodeMap: {:?}", node_map);
            println!("SharedOutputs: {:?}", shared_outputs);
            // remove all shared outputs that are not after this region
            shared_outputs.retain(|_, max_depth| *max_depth + 1 >= end_depth);
            // add all gates that will be used later
            shared_outputs.extend(
                cur_gates
                    .iter()
                    .filter(|(_, max_depth)| *max_depth + 1 >= end_depth)
                    .copied(),
            );
            println!("SharedOutputs 2: {:?}", shared_outputs);
            // create outputs for subcircuit
            let subc_outputs = if last_subc {
                circuit
                    .outputs()
                    .iter()
                    .map(|(x, n)| {
                        if *x >= input_len_t {
                            (node_map[x], *n)
                        } else {
                            (*x, *n)
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                shared_outputs
                    .iter()
                    .map(|(x, _)| {
                        if *x >= input_len_t {
                            (node_map[x], false)
                        } else {
                            (*x, false)
                        }
                    })
                    .collect::<Vec<_>>()
            };
            println!(
                "Circuit: {} {:?} {:?}",
                subc_input_len, subc_gates, subc_outputs
            );
            circuits.push(
                Circuit::new(
                    T::try_from(subc_input_len).unwrap(),
                    subc_gates,
                    subc_outputs,
                )
                .unwrap(),
            );
            // clean up
            cur_gates.clear();
            start_depth = end_depth;
            gate_count = depth_gate_num;
        }
        cur_gates.extend(slice_gates.into_iter());
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
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
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
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(4, 1), (5, 1), (6, 1), (7, 1)],
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
                vec![(0, 1), (1, 1), (2, 1), (3, 0)],
                vec![(6, 4), (7, 1)],
                vec![(4, 2), (5, 2), (9, 3)],
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
                vec![(0, 1), (1, 1), (2, 4), (3, 0)],
                vec![(6, 4), (7, 1)],
                vec![(4, 2), (5, 2), (9, 3)],
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
                    [(6, false), (8, false), (10, false), (12, false), (2, false)]
                )
                .unwrap()
            )
        );
        assert_eq!(
            vec![
                vec![(0, 7), (1, 7), (2, 0), (3, 0), (4, 0), (5, 0)],
                vec![(6, 1), (7, 1), (8, 1), (9, 1)],
                vec![(10, 2), (11, 2)],
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
                        (26, false),
                        (23, false),
                        (24, false)
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

    #[test]
    fn test_divide_circuit_seq() {
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
        let depths = calculate_gate_depths(&circuit);
        assert_eq!(
            vec![
                Circuit::new(
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
                    [(0, false), (1, false), (12, false)]
                )
                .unwrap(),
                Circuit::new(
                    3,
                    [
                        Gate::new_and(1, 2), // 13
                        Gate::new_nor(1, 2), // 16
                        Gate::new_nor(4, 0), // 17
                        Gate::new_nor(1, 5), // 18
                        Gate::new_nor(1, 2), // 21
                    ],
                    [
                        (0, false),
                        (1, false),
                        (2, false),
                        (3, false),
                        (6, false),
                        (7, false)
                    ]
                )
                .unwrap(),
                Circuit::new(
                    6,
                    [
                        Gate::new_and(1, 3),   // 14
                        Gate::new_nor(1, 4),   // 19
                        Gate::new_nor(1, 5),   // 22
                        Gate::new_nimpl(2, 0), // 25
                    ],
                    [
                        (0, false),
                        (1, false),
                        (2, false),
                        (6, false),
                        (7, false),
                        (8, false),
                        (9, false)
                    ]
                )
                .unwrap(),
                Circuit::new(
                    7,
                    [
                        Gate::new_and(3, 0),   // 15
                        Gate::new_nor(4, 0),   // 20
                        Gate::new_nor(1, 5),   // 23
                        Gate::new_xor(2, 0),   // 24
                        Gate::new_nimpl(1, 6), // 26
                    ],
                    [(7, false), (8, true), (11, true), (9, false), (10, true)]
                )
                .unwrap(),
            ],
            divide_circuit_seq(circuit.clone(), depths.clone(), 7, 1)
        );
        assert_eq!(
            vec![
                Circuit::new(
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
                    [(0, false), (1, false), (12, false)]
                )
                .unwrap(),
                Circuit::new(
                    3,
                    [
                        Gate::new_and(1, 2), // 13
                        Gate::new_nor(1, 2), // 16
                        Gate::new_nor(4, 0), // 17
                        Gate::new_nor(1, 5), // 18
                        Gate::new_nor(1, 2), // 21
                    ],
                    [
                        (0, false),
                        (1, false),
                        (2, false),
                        (3, false),
                        (6, false),
                        (7, false)
                    ]
                )
                .unwrap(),
                Circuit::new(
                    6,
                    [
                        Gate::new_and(1, 3),    // 14
                        Gate::new_and(6, 0),    // 15
                        Gate::new_nor(1, 4),    // 19
                        Gate::new_nor(8, 0),    // 20
                        Gate::new_nor(1, 5),    // 22
                        Gate::new_nor(1, 10),   // 23
                        Gate::new_xor(2, 0),    // 24
                        Gate::new_nimpl(2, 0),  // 25
                        Gate::new_nimpl(1, 13), // 26
                    ],
                    [(7, false), (9, true), (14, true), (11, false), (12, true)]
                )
                .unwrap(),
            ],
            divide_circuit_seq(circuit.clone(), depths.clone(), 7, 2)
        );
        assert_eq!(
            vec![circuit.clone()],
            divide_circuit_seq(circuit.clone(), depths.clone(), 30, 2)
        );
        assert_eq!(
            vec![circuit.clone()],
            divide_circuit_seq(circuit.clone(), depths.clone(), 7, 8)
        );
    }
}

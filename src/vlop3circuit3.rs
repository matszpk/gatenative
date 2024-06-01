use gatesim::*;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::vbinopcircuit::*;
use crate::vcircuit::VGateFunc;
use crate::VNegs::*;

use crate::vlop3circuit::*;
use crate::vlop3circuit2::*;

impl<T> VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from_lop3nodes(
        circuit: VBinOpCircuit<T>,
        enableds: Vec<bool>,
        lop3nodes: Vec<LOP3Node<T>>,
    ) -> VLOP3Circuit<T> {
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let mut new_gates: Vec<VLOP3Gate<T>> = vec![];
        let gates = &circuit.gates;
        let mut trans_tbl = vec![T::default(); input_len + gates.len()];
        for (i, (en, lop3node)) in enableds.into_iter().zip(lop3nodes.into_iter()).enumerate() {
            if !en {
                continue;
            }
            let gi = input_len + i;
            let newgi = input_len + new_gates.len();
            trans_tbl[gi] = T::try_from(newgi).unwrap();
            if !lop3node.tree_paths[0].is_empty()
                && lop3node.tree_paths[1].is_empty()
                && lop3node.tree_paths[2].is_empty()
            {
                // single gate
                let func = match gates[i].0.func {
                    VGateFunc::And => VLOP3GateFunc::And,
                    VGateFunc::Or => VLOP3GateFunc::Or,
                    VGateFunc::Xor => VLOP3GateFunc::Xor,
                    _ => {
                        panic!("Unexpected!");
                    }
                };
                new_gates.push(VLOP3Gate {
                    i0: gates[i].0.i0,
                    i1: gates[i].0.i1,
                    i2: gates[i].0.i1,
                    func,
                    negs: gates[i].1,
                });
            } else {
                // more complex LOP3
                let tree = get_small_tree(&circuit, T::try_from(input_len + i).unwrap());
                let mut calcs = [
                    0b11110000u8,
                    0b11001100u8,
                    0b10101010u8,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                let mut level_start = 3;
                let mut level_end = 7;
                // calculate values for tree nodes
                let lop3moves = &lop3node.tree_paths;
                for _ in 0..3 {
                    for l in level_start..level_end {
                        let calc_l = 3 + 7 - l - 1;
                        if let Some(t) = tree[l] {
                            calcs[calc_l] = if lop3node.args[0] == t {
                                calcs[0]
                            } else if lop3node.args[1] == t {
                                calcs[1]
                            } else if lop3node.args[2] == t {
                                calcs[2]
                            } else if t >= circuit.input_len && !lop3moves[l].is_empty() {
                                let tgi = usize::try_from(t).unwrap() - input_len;
                                let l_arg0 = (l << 1) + 1;
                                let l_arg1 = (l << 1) + 2;
                                let va0 = if l_arg0 < 7 && !lop3moves[l_arg0].is_empty() {
                                    calcs[3 + 7 - l_arg0 - 1]
                                } else if gates[tgi].0.i0 == lop3node.args[0] {
                                    calcs[0]
                                } else if gates[tgi].0.i0 == lop3node.args[1] {
                                    calcs[1]
                                } else {
                                    calcs[2]
                                };
                                let va1 = if l_arg1 < 7 && !lop3moves[l_arg1].is_empty() {
                                    calcs[3 + 7 - l_arg1 - 1]
                                } else if gates[tgi].0.i1 == lop3node.args[0] {
                                    calcs[0]
                                } else if gates[tgi].0.i1 == lop3node.args[1] {
                                    calcs[1]
                                } else {
                                    calcs[2]
                                };
                                let va1 = if gates[tgi].1 == NegInput1 {
                                    va1 ^ 0xff
                                } else {
                                    va1
                                };
                                let vop = match gates[tgi].0.func {
                                    VGateFunc::And => va0 & va1,
                                    VGateFunc::Or => va0 | va1,
                                    VGateFunc::Xor => va0 ^ va1,
                                    _ => {
                                        panic!("Unexpected gate");
                                    }
                                };
                                if gates[tgi].1 == NegOutput {
                                    vop ^ 0xff
                                } else {
                                    vop
                                }
                            } else {
                                0
                            };
                        }
                    }
                    level_start >>= 1;
                    level_end >>= 1;
                }
                new_gates.push(VLOP3Gate {
                    i0: lop3node.args[0],
                    i1: lop3node.args[1],
                    i2: lop3node.args[2],
                    func: VLOP3GateFunc::LOP3(*calcs.last().unwrap()),
                    negs: NoNegs,
                });
            }
        }
        // translation of gate arguments
        for g in new_gates.iter_mut() {
            g.i0 = if g.i0 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i0).unwrap()]
            } else {
                g.i0
            };
            g.i1 = if g.i1 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i1).unwrap()]
            } else {
                g.i1
            };
            g.i2 = if g.i2 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i2).unwrap()]
            } else {
                g.i2
            };
        }

        Self {
            input_len: circuit.input_len,
            gates: new_gates,
            outputs: circuit
                .outputs
                .into_iter()
                .map(|(o, n)| {
                    if o >= circuit.input_len {
                        (trans_tbl[usize::try_from(o).unwrap()], n)
                    } else {
                        (o, n)
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl<T> From<Circuit<T>> for VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: Circuit<T>) -> Self {
        // for especially NVIDIA LOP3 - enabled by default
        let mut vcircuit = VBinOpCircuit::from(circuit.clone());
        vcircuit.optimize_negs();
        Self::from(vcircuit)
    }
}

impl<T> From<VBinOpCircuit<T>> for VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: VBinOpCircuit<T>) -> Self {
        println!("Conv start");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let circuit_outputs = HashSet::<T>::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3enableds = vec![false; gates.len()];
        let mut mtuareas = subtrees
            .iter()
            .map(|s| {
                let mut mtuarea = MTUArea::<T>::empty_with_root(s.root());
                if circuit_outputs.contains(&s.root()) {
                    mtuarea.nodes.push((s.root(), vec![]));
                }
                mtuarea
            })
            .collect::<Vec<_>>();
        let mut lop3nodes = vec![LOP3Node::<T>::default(); gates.len()];
        // generate lop3nodes
        for i in (0..subtrees.len()).rev() {
            //println!("Subtrees: {}", i);
            let subtree = &subtrees[i];
            mtuareas[i].improve_and_optimize_and_gen_lop3nodes(
                &circuit,
                &mut lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
            );
            let (farest_nodes, nonfarest_nodes) = mtuareas[i].farest_nonfarest_nodes(&circuit);
            // get nonfarest nodes
            for nidx in subtree
                .gates()
                .iter()
                .map(|(x, _)| *x)
                .chain(std::iter::once(subtree.root()))
                // skip all nonfarest nodes in MTUAreaview
                .filter(|nidx| nonfarest_nodes.iter().all(|x| *x != *nidx))
            {
                let gidx = usize::try_from(nidx).unwrap() - input_len;
                // get preferred nodes from mtuareas
                let preferred_nodes =
                    get_preferred_nodes_from_mtuareas(&circuit, &mtuareas, &cov, nidx);
                lop3nodes[gidx] = find_best_lop3node(
                    &circuit,
                    &lop3nodes,
                    &cov,
                    &subtrees,
                    &circuit_outputs,
                    nidx,
                    &preferred_nodes,
                );
            }
            // filter out current mtublock
            filter_lop3nodes_in_mtuarea(
                input_len,
                &mut lop3enableds,
                &lop3nodes,
                &farest_nodes,
                subtree,
            );
            update_mtuareas_from_lop3node(
                &circuit,
                &mut mtuareas,
                &cov,
                subtree,
                &lop3enableds,
                &lop3nodes,
            );
        }
        // convert inputs in lop3nodes
        let mut circuit = Self::from_lop3nodes(circuit, lop3enableds, lop3nodes);
        // optimize can be called later
        // println!("xLOP3end");
        circuit.optimize_negs();
        println!("optneg end");
        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vcircuit::*;
    use crate::VNegs;

    fn vgate<T: Clone + Copy>(
        func: VLOP3GateFunc,
        i0: T,
        i1: T,
        i2: T,
        negs: VNegs,
    ) -> VLOP3Gate<T> {
        VLOP3Gate {
            i0,
            i1,
            i2,
            func,
            negs,
        }
    }

    fn vgate_and<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::And, i0, i1, i1, negs)
    }
    fn vgate_or<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::Or, i0, i1, i1, negs)
    }
    fn vgate_xor<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::Xor, i0, i1, i1, negs)
    }
    fn vgate_lop3<T: Clone + Copy>(i0: T, i1: T, i2: T, f: u8) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::LOP3(f), i0, i1, i2, NoNegs)
    }

    fn vbgate<T: Clone + Copy>(func: VGateFunc, i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        (VGate { i0, i1, func: func }, negs)
    }

    fn vbgate_and<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::And, i0, i1, negs)
    }
    fn vbgate_or<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::Or, i0, i1, negs)
    }
    fn vbgate_xor<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::Xor, i0, i1, negs)
    }

    fn call_vlop3circuit_from_lopnodes(
        circuit: VBinOpCircuit<u32>,
        lop3nodes_and_enableds: Vec<(LOP3Node<u32>, bool)>,
    ) -> VLOP3Circuit<u32> {
        let (lop3nodes, enableds): (Vec<_>, Vec<_>) = lop3nodes_and_enableds.into_iter().unzip();
        VLOP3Circuit::from_lop3nodes(circuit, enableds, lop3nodes)
    }

    fn lop3node_1(arg0: u32, arg1: u32, arg2: u32) -> LOP3Node<u32> {
        LOP3Node {
            args: [arg0, arg1, arg2],
            tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
            mtu_cost: MTU_COST_BASE + 1,
        }
    }

    fn lop3node_mmask(arg0: u32, arg1: u32, arg2: u32, move_mask: u8) -> LOP3Node<u32> {
        let mut tree_paths = LOP3_SUBTREE_PATHS_DEFAULT;
        for i in 0..7 {
            tree_paths[i] = PathMove(u8::from(((move_mask >> i) & 1) != 0) * 3);
        }
        LOP3Node {
            args: [arg0, arg1, arg2],
            tree_paths,
            mtu_cost: MTU_COST_BASE + 1,
        }
    }

    fn to_paths(paths: [u8; 7]) -> LOP3SubTreePaths {
        paths.map(|x| PathMove(x))
    }

    #[test]
    fn test_vlop3circuit_from_lop3nodes() {
        let a0 = 0b11110000u8;
        let a1 = 0b11001100u8;
        let a2 = 0b10101010u8;
        assert_eq!(
            VLOP3Circuit {
                input_len: 2,
                gates: vec![
                    vgate_and(0, 1, NoNegs),
                    vgate_and(0, 2, NegInput1),
                    vgate_and(0, 3, NegOutput),
                    vgate_or(2, 3, NoNegs),
                    vgate_or(2, 1, NegInput1),
                    vgate_or(0, 1, NegOutput),
                    vgate_xor(4, 2, NoNegs),
                    vgate_xor(0, 1, NegInput1),
                    vgate_xor(0, 1, NegOutput),
                ],
                outputs: vec![
                    (0, false),
                    (2, true),
                    (3, false),
                    (4, true),
                    (5, false),
                    (6, true),
                    (7, false),
                    (8, true),
                    (9, false),
                    (10, true),
                ],
            },
            VLOP3Circuit::from_lop3nodes(
                VBinOpCircuit {
                    input_len: 2,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),
                        vbgate_and(0, 2, NegInput1),
                        vbgate_and(0, 3, NegOutput),
                        vbgate_or(2, 3, NoNegs),
                        vbgate_or(2, 1, NegInput1),
                        vbgate_or(0, 1, NegOutput),
                        vbgate_xor(4, 2, NoNegs),
                        vbgate_xor(0, 1, NegInput1),
                        vbgate_xor(0, 1, NegOutput),
                    ],
                    outputs: vec![
                        (0, false),
                        (2, true),
                        (3, false),
                        (4, true),
                        (5, false),
                        (6, true),
                        (7, false),
                        (8, true),
                        (9, false),
                        (10, true),
                    ],
                },
                std::iter::repeat(true).take(9).collect::<Vec<_>>(),
                std::iter::repeat(lop3node_1(0, 1, 0))
                    .take(9)
                    .collect::<Vec<_>>(),
            )
        );
        // LOP3Nodes
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, !((a0 & a1) | (a0 & !a2))),
                    vgate_lop3(0, 1, 2, !(a0 & a1) ^ (a2 & !a1)),
                    vgate_lop3(0, 1, 2, !(a0 | a1) | !(a1 ^ !a2)),
                    vgate_lop3(2, 1, 0, !((a2 | !a0) & (a0 ^ a1))),
                ],
                outputs: vec![(3, false), (4, false), (5, true), (6, false)],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),      // 3
                        vbgate_and(0, 2, NegInput1),   // 4
                        vbgate_or(3, 4, NegOutput),    // 5
                        vbgate_and(0, 1, NegOutput),   // 6
                        vbgate_and(2, 1, NegInput1),   // 7
                        vbgate_xor(6, 7, NoNegs),      // 8
                        vbgate_or(0, 2, NegOutput),    // 9
                        vbgate_xor(1, 2, NegInput1),   // 10
                        vbgate_or(9, 10, NegInput1),   // 11
                        vbgate_or(0, 2, NegInput1),    // 12
                        vbgate_xor(2, 1, NoNegs),      // 13
                        vbgate_and(12, 13, NegOutput), // 14
                    ],
                    outputs: vec![(5, false), (8, false), (11, true), (14, false)],
                },
                vec![
                    (lop3node_1(0, 1, 0), false),               // 3
                    (lop3node_1(0, 2, 0), false),               // 4
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 5
                    (lop3node_1(0, 1, 0), false),               // 6
                    (lop3node_1(2, 1, 2), false),               // 7
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 8
                    (lop3node_1(0, 2, 0), false),               // 9
                    (lop3node_1(1, 2, 1), false),               // 10
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 11
                    (lop3node_1(0, 2, 0), false),               // 12
                    (lop3node_1(2, 1, 2), false),               // 13
                    (lop3node_mmask(2, 1, 0, 0b0000111), true), // 14
                ],
            )
        );
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, a2 | (a0 & !a1)),
                    vgate_lop3(0, 1, 2, (a0 & a1) | (a2 & !a1)),
                    vgate_lop3(0, 1, 2, a1 | !(a2 & (a0 & !a1))),
                    vgate_lop3(2, 0, 1, ((a0 & !a2) & a1) | !a0),
                    vgate_lop3(0, 1, 2, (a0 & !(a2 | a1)) ^ ((a0 ^ a1) & !a2)),
                    vgate_lop3(3, 4, 5, !(a2 | (a0 ^ a1))),
                    vgate_lop3(6, 7, 3, a2 & !(a0 & a1)),
                    vgate_lop3(0, 1, 2, (a2 & !a1) | !a0),
                    vgate_lop3(0, 1, 2, ((a0 & !a1) ^ (a2 & !a1)) | !(a0 & !a1)),
                    vgate_lop3(1, 2, 1, !(a0 & a1) ^ a1),
                    vgate_lop3(4, 5, 7, ((a0 & a1) | (a1 & !a2)) ^ ((a2 & !a0) | (a0 ^ a1))),
                ],
                outputs: vec![
                    (8, true),
                    (9, false),
                    (10, true),
                    (11, false),
                    (12, true),
                    (13, true)
                ],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 3
                        vbgate_or(2, 3, NoNegs),     // 4
                        //
                        vbgate_and(0, 1, NoNegs),    // 5
                        vbgate_and(2, 1, NegInput1), // 6
                        vbgate_xor(5, 6, NoNegs),    // 7
                        //
                        vbgate_and(0, 1, NegInput1), // 8
                        vbgate_and(2, 8, NoNegs),    // 9
                        vbgate_or(1, 9, NegInput1),  // 10
                        //
                        vbgate_and(2, 1, NegInput1), // 11
                        vbgate_and(11, 0, NoNegs),   // 12
                        vbgate_or(12, 2, NegInput1), // 13
                        //
                        vbgate_xor(0, 1, NoNegs),     // 14
                        vbgate_or(2, 1, NegOutput),   // 15
                        vbgate_and(0, 15, NoNegs),    // 16
                        vbgate_and(14, 2, NegInput1), // 17
                        vbgate_xor(16, 17, NoNegs),   // 18
                        //
                        vbgate_xor(4, 7, NoNegs),     // 19
                        vbgate_or(10, 19, NegOutput), // 20
                        vbgate_and(13, 18, NoNegs),   // 21
                        vbgate_and(4, 21, NegInput1), // 22
                        //
                        vbgate_and(2, 1, NegInput1), // 23
                        vbgate_or(23, 0, NegInput1), // 24
                        //
                        vbgate_xor(3, 6, NoNegs),    // 25
                        vbgate_or(25, 3, NegInput1), // 26
                        //
                        vbgate_and(1, 2, NegOutput), // 27
                        vbgate_xor(27, 2, NoNegs),   // 28
                        //
                        vbgate_and(7, 10, NoNegs),     // 29
                        vbgate_and(10, 18, NegInput1), // 30
                        vbgate_and(18, 7, NegInput1),  // 31
                        vbgate_xor(7, 10, NoNegs),     // 32
                        vbgate_or(29, 30, NoNegs),     // 33
                        vbgate_or(31, 32, NoNegs),     // 34
                        vbgate_xor(33, 34, NoNegs),    // 35
                    ],
                    outputs: vec![
                        (20, true),
                        (22, false),
                        (24, true),
                        (26, false),
                        (28, true),
                        (35, true)
                    ],
                },
                vec![
                    (lop3node_1(0, 1, 0), false),                  // 3
                    (lop3node_mmask(0, 1, 2, 0b0000101), true),    // 4
                    (lop3node_1(0, 1, 0), false),                  // 5
                    (lop3node_1(2, 1, 2), false),                  // 6
                    (lop3node_mmask(0, 1, 2, 0b0000111), true),    // 7
                    (lop3node_1(0, 1, 0), false),                  // 8
                    (lop3node_1(2, 8, 2), false),                  // 9
                    (lop3node_mmask(0, 1, 2, 0b1000101), true),    // 10
                    (lop3node_1(2, 1, 2), false),                  // 11
                    (lop3node_1(11, 0, 11), false),                // 12
                    (lop3node_mmask(2, 0, 1, 0b0001011), true),    // 13
                    (lop3node_1(0, 1, 0), false),                  // 14
                    (lop3node_1(2, 1, 2), false),                  // 15
                    (lop3node_mmask(0, 1, 2, 0b0000101), false),   // 16
                    (lop3node_mmask(0, 1, 2, 0b0000011), false),   // 17
                    (lop3node_mmask(0, 1, 2, 0b0110111), true),    // 18
                    (lop3node_1(4, 7, 4), false),                  // 19
                    (lop3node_mmask(4, 7, 10, 0b0000101), true),   // 20
                    (lop3node_1(13, 18, 13), false),               // 21
                    (lop3node_mmask(13, 18, 4, 0b0000101), true),  // 22
                    (lop3node_1(2, 1, 2), false),                  // 23
                    (lop3node_mmask(0, 1, 2, 0b0000011), true),    // 24
                    (lop3node_1(3, 6, 3), false),                  // 25
                    (lop3node_mmask(0, 1, 2, 0b0011111), true),    // 26
                    (lop3node_1(1, 2, 1), false),                  // 27
                    (lop3node_mmask(1, 2, 1, 0b0000011), true),    // 28
                    (lop3node_1(7, 10, 7), false),                 // 29
                    (lop3node_1(10, 18, 10), false),               // 30
                    (lop3node_1(18, 7, 18), false),                // 31
                    (lop3node_1(7, 10, 7), false),                 // 32
                    (lop3node_mmask(7, 10, 18, 0b0000111), false), // 33
                    (lop3node_mmask(7, 10, 18, 0b0000111), false), // 34
                    (lop3node_mmask(7, 10, 18, 0b1111111), true),  // 35
                ],
            )
        );
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_and(0, 1, NoNegs),    // 3
                    vgate_and(0, 2, NegInput1), // 4
                    vgate_and(1, 2, NegOutput), // 5
                    vgate_or(2, 1, NoNegs),     // 6
                    vgate_or(2, 0, NegInput1),  // 7
                    vgate_or(0, 1, NegOutput),  // 8
                    vgate_xor(0, 2, NoNegs),    // 9
                    vgate_xor(2, 1, NegInput1), // 10
                    vgate_xor(1, 0, NegOutput), // 11
                    vgate_lop3(3, 4, 5, (!(a2 & a0) | (a1 & !a0)) ^ !a2),
                    vgate_lop3(6, 7, 8, !(a0 ^ ((a1 & a2) & (a2 | !a1)))),
                    vgate_lop3(9, 10, 11, a1 | !(!(a1 & a2) & a0)),
                    vgate_lop3(4, 7, 4, !(a0 ^ (a0 & a1))),
                ],
                outputs: vec![(12, false), (13, true), (14, true), (15, false)],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),    // 3
                        vbgate_and(0, 2, NegInput1), // 4
                        vbgate_and(1, 2, NegOutput), // 5
                        vbgate_or(2, 1, NoNegs),     // 6
                        vbgate_or(2, 0, NegInput1),  // 7
                        vbgate_or(0, 1, NegOutput),  // 8
                        vbgate_xor(0, 2, NoNegs),    // 9
                        vbgate_xor(2, 1, NegInput1), // 10
                        vbgate_xor(1, 0, NegOutput), // 11
                        //
                        vbgate_and(4, 3, NegInput1), // 12
                        vbgate_and(5, 3, NegOutput), // 13
                        vbgate_or(13, 12, NoNegs),   // 14
                        vbgate_and(7, 8, NoNegs),    // 15
                        vbgate_or(8, 7, NegInput1),  // 16
                        vbgate_and(15, 16, NoNegs),  // 17
                        //
                        vbgate_xor(14, 5, NegInput1), // 18
                        vbgate_xor(6, 17, NegOutput), // 19
                        //
                        vbgate_and(10, 11, NegOutput), // 20
                        vbgate_and(20, 9, NoNegs),     // 21
                        vbgate_or(10, 21, NegInput1),  // 22
                        //
                        vbgate_and(4, 7, NoNegs),     // 23
                        vbgate_xor(4, 23, NegOutput), // 24
                    ],
                    outputs: vec![(18, false), (19, true), (22, true), (24, false)],
                },
                vec![
                    (lop3node_1(0, 1, 0), true),                  // 3
                    (lop3node_1(0, 2, 0), true),                  // 4
                    (lop3node_1(1, 2, 1), true),                  // 5
                    (lop3node_1(2, 1, 2), true),                  // 6
                    (lop3node_1(2, 0, 2), true),                  // 7
                    (lop3node_1(0, 1, 0), true),                  // 8
                    (lop3node_1(0, 2, 0), true),                  // 9
                    (lop3node_1(2, 1, 2), true),                  // 10
                    (lop3node_1(1, 0, 1), true),                  // 11
                    (lop3node_1(4, 3, 4), false),                 // 12
                    (lop3node_1(5, 3, 5), false),                 // 13
                    (lop3node_1(12, 13, 12), false),              // 14
                    (lop3node_1(7, 8, 7), false),                 // 15
                    (lop3node_1(8, 7, 8), false),                 // 16
                    (lop3node_1(15, 16, 15), false),              // 17
                    (lop3node_mmask(3, 4, 5, 0b0011011), true),   // 18
                    (lop3node_mmask(6, 7, 8, 0b1100101), true),   // 19
                    (lop3node_1(10, 11, 10), false),              // 20
                    (lop3node_1(20, 9, 20), false),               // 21
                    (lop3node_mmask(9, 10, 11, 0b0100101), true), // 22
                    (lop3node_1(4, 7, 4), false),                 // 23
                    (lop3node_mmask(4, 7, 4, 0b0000101), true),   // 24
                ],
            )
        );
    }
}

use gatesim::*;

use crate::vcircuit::*;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::VNegs::{self, *};

#[derive(Clone, PartialEq, Eq, Debug)]
struct VBinOpCircuit<T: Clone + Copy> {
    input_len: T,
    gates: Vec<(VGate<T>, VNegs)>,
    outputs: Vec<(T, bool)>,
}

impl<T: Clone + Copy> From<Circuit<T>> for VBinOpCircuit<T> {
    fn from(circuit: Circuit<T>) -> VBinOpCircuit<T> {
        VBinOpCircuit {
            input_len: circuit.input_len(),
            gates: circuit
                .gates()
                .into_iter()
                .map(|g| VGate::from(*g).to_binop())
                .collect::<Vec<_>>(),
            outputs: circuit.outputs().to_vec(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VOccur<T: Clone + Copy> {
    Gate(T),
    GateDouble(T),
    Output(T), // circuit output index
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SubTree<T> {
    root: T,
    // gates: gate entry: first - index in circuit gates
    // second - index in circuit gates of successor
    gates: Vec<(T, T)>,
}

impl<T: Clone + Copy + Ord> SubTree<T> {
    fn sort_and_dedup(&mut self) {
        self.gates.sort();
        self.gates.dedup();
    }
}
impl<T> SubTree<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // gates must be ordered by index. last gate MUST BE in root.
    fn fill_successors(&mut self, circuit: &VBinOpCircuit<T>) {
        let input_len = usize::try_from(circuit.input_len).unwrap();
        for i in 0..self.gates.len() {
            let oi = usize::try_from(self.gates[i].0).unwrap() - input_len;
            let g = circuit.gates[oi].0;
            if g.i0 >= circuit.input_len {
                if let Ok(p) = self.gates[0..i].binary_search(&(g.i0, T::default())) {
                    self.gates[p].1 = self.gates[i].0;
                }
            }
            if g.i1 >= circuit.input_len {
                if let Ok(p) = self.gates[0..i].binary_search(&(g.i1, T::default())) {
                    self.gates[p].1 = self.gates[i].0;
                }
            }
        }
        self.gates.pop();
    }

    #[inline]
    fn find_index(&self, t: T) -> Option<usize> {
        if t == self.root {
            Some(self.gates.len())
        } else {
            self.gates.binary_search_by_key(&t, |(x, _)| *x).ok()
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SubTreeCopy<'a, T: Clone + Copy> {
    input_len: T,
    subtree: &'a SubTree<T>,
    gates: Vec<(VGate<T>, VNegs)>,
}

impl<'a, T> SubTreeCopy<'a, T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn new(circuit: &VBinOpCircuit<T>, subtree: &'a SubTree<T>) -> Self {
        let input_len = usize::try_from(circuit.input_len).unwrap();
        Self {
            input_len: circuit.input_len,
            subtree,
            gates: subtree
                .gates
                .iter()
                .copied()
                .chain(std::iter::once((subtree.root, T::default())))
                .map(|(x, _)| circuit.gates[usize::try_from(x).unwrap() - input_len])
                .collect::<Vec<_>>(),
        }
    }

    #[inline]
    fn gate_index(&self, t: T) -> Option<usize> {
        self.subtree.find_index(t)
    }

    #[inline]
    fn circuit_index(&self, index: usize) -> T {
        if index < self.subtree.gates.len() {
            self.subtree.gates[index].0
        } else {
            self.subtree.root
        }
    }

    fn optimize_negs(&mut self) {
        let input_len = usize::try_from(self.input_len).unwrap();
        for (oi, (i, next_i)) in self.subtree.gates.iter().enumerate() {
            let next_oi = self.gate_index(*next_i).unwrap();
            let (g, neg) = self.gates[oi];
            let (next_g, next_neg) = self.gates[next_oi];
            if neg != NoNegs {
                let (new_g, new_neg) = g.binop_neg(neg);
                let (new_next_g, new_next_neg) = if next_g.i0 == *i {
                    next_g.binop_neg_args(next_neg, true, false)
                } else {
                    next_g.binop_neg_args(next_neg, false, true)
                };
                let old_neg_count = usize::from(neg != NoNegs) + usize::from(next_neg != NoNegs);
                let new_neg_count =
                    usize::from(new_neg != NoNegs) + usize::from(new_next_neg != NoNegs);
                if old_neg_count > new_neg_count
                    || (old_neg_count == new_neg_count
                        && (new_next_neg != NegInput1 || next_neg != NegOutput))
                {
                    // apply if negation reduction or negation move forward.
                    self.gates[oi] = (new_g, new_neg);
                    self.gates[next_oi] = (new_next_g, new_next_neg);
                }
            }
            // check single reduction subtree.
            let (g, neg) = self.gates[oi];
            let (next_g, next_neg) = self.gates[next_oi];
            let (roi, in_next) = if (next_neg == NegInput1) && *i == next_g.i1 {
                // if this fork (from negated input)
                (Some(oi), true)
            } else if next_neg == NegOutput {
                (Some(next_oi), false)
            } else {
                (None, false)
            };

            if let Some(roi) = roi {
                let (rg, rneg) = self.gates[roi];
                if rg.i0 < self.input_len || rg.i1 < self.input_len {
                    continue;
                }
                let rg_oi0 = self.gate_index(rg.i0).unwrap();
                let rg_oi1 = self.gate_index(rg.i1).unwrap();
                let (rg0g, rg0neg) = self.gates[rg_oi0];
                let (rg1g, rg1neg) = self.gates[rg_oi1];
                if rg.func == VGateFunc::Xor
                    || rg0g.func == VGateFunc::Xor
                    || rg1g.func == VGateFunc::Xor
                {
                    continue;
                }

                if rg0neg == NegInput1 && rg1neg == NegInput1 {
                    // found - just change subtree.
                    self.gates[rg_oi0] = rg0g.binop_neg(rg0neg);
                    self.gates[rg_oi1] = rg1g.binop_neg(rg1neg);
                    self.gates[roi] = rg.binop_neg_args(rneg, true, true);
                    if in_next {
                        // propagate to next after ROI
                        self.gates[roi].1 = NoNegs;
                        self.gates[next_oi].1 = NoNegs;
                    }
                    continue;
                }
                if rg0g.i0 < self.input_len
                    || rg0g.i1 < self.input_len
                    || rg1g.i0 < self.input_len
                    || rg1g.i1 < self.input_len
                {
                    continue;
                }
                let rg_oi00 = self.gate_index(rg0g.i0).unwrap();
                let rg_oi01 = self.gate_index(rg0g.i1).unwrap();
                let rg_oi10 = self.gate_index(rg1g.i0).unwrap();
                let rg_oi11 = self.gate_index(rg1g.i1).unwrap();
                let (rg00g, rg00neg) = self.gates[rg_oi00];
                let (rg01g, rg01neg) = self.gates[rg_oi01];
                let (rg10g, rg10neg) = self.gates[rg_oi10];
                let (rg11g, rg11neg) = self.gates[rg_oi11];
                if rg00g.func == VGateFunc::Xor
                    || rg01g.func == VGateFunc::Xor
                    || rg10g.func == VGateFunc::Xor
                    || rg11g.func == VGateFunc::Xor
                {
                    continue;
                }
                if rg00neg == NegInput1
                    && rg01neg == NegInput1
                    && rg10neg == NegInput1
                    && rg11neg == NegInput1
                    && rg0neg == NoNegs
                    && rg1neg == NoNegs
                {
                    // found - just change subtree.
                    self.gates[rg_oi00] = rg00g.binop_neg(rg00neg);
                    self.gates[rg_oi01] = rg01g.binop_neg(rg01neg);
                    self.gates[rg_oi10] = rg10g.binop_neg(rg10neg);
                    self.gates[rg_oi11] = rg11g.binop_neg(rg11neg);
                    self.gates[rg_oi0] = rg0g.binop_neg_args(rg0neg, true, true);
                    self.gates[rg_oi1] = rg1g.binop_neg_args(rg1neg, true, true);
                    let (rg0g, rg0neg) = self.gates[rg_oi0];
                    let (rg1g, rg1neg) = self.gates[rg_oi1];
                    self.gates[rg_oi0] = rg0g.binop_neg(rg0neg);
                    self.gates[rg_oi1] = rg1g.binop_neg(rg1neg);
                    self.gates[roi] = rg.binop_neg_args(rneg, true, true);
                    if in_next {
                        // propagate to next after ROI
                        self.gates[roi].1 = NoNegs;
                        self.gates[next_oi].1 = NoNegs;
                    }
                }
            }
        }
    }

    // arguments: self - original (no changes), opt1 - if option1 changed to true.
    // opt2 - if option2 changed to true.
    fn is_independent_optimize_negs(&self, opt1: &Self, opt2: &Self) -> bool {
        let mut diff1 = HashSet::new();
        let mut diff2 = HashSet::new();
        for i in 0..self.gates.len() {
            if self.gates[i] != opt1.gates[i] {
                diff1.insert(self.circuit_index(i));
            }
        }
        for i in 0..self.gates.len() {
            if self.gates[i] != opt2.gates[i] {
                diff2.insert(self.circuit_index(i));
            }
        }
        /// TODO: its requires other checking: single reduction subtree collisions and others
        diff1.is_disjoint(&diff2)
    }
}

impl<T> VBinOpCircuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // return map of Xor gates: key - XOR gate index, value - root XOR gate index.
    fn subtrees(&self) -> (BTreeMap<T, T>, Vec<SubTree<T>>) {
        // println!("XorSubtreeStart");
        let input_len = usize::try_from(self.input_len).unwrap();
        let mut usage = vec![0u8; self.gates.len()];
        for (g, _) in &self.gates {
            if g.i0 >= self.input_len {
                let i0 = usize::try_from(g.i0).unwrap() - input_len;
                if usage[i0] < 2 {
                    usage[i0] += 1;
                }
            }
            if g.i1 >= self.input_len {
                let i1 = usize::try_from(g.i1).unwrap() - input_len;
                if usage[i1] < 2 {
                    usage[i1] += 1;
                }
            }
        }

        for (o, _) in self.outputs.iter() {
            if *o >= self.input_len {
                let o = usize::try_from(*o).unwrap() - input_len;
                if usage[o] < 2 {
                    usage[o] += 1;
                }
            }
        }

        #[derive(Clone, Copy)]
        struct StackEntry<T> {
            node: usize,
            way: usize,
            subtree_index: Option<T>,
        }
        let gate_num = self.gates.len();
        let mut visited = vec![false; gate_num];
        let mut subtree_map = BTreeMap::new();
        let mut subtree_object_map = BTreeMap::<T, SubTree<T>>::new();

        // traverse through circuit
        for (o, _) in self.outputs.iter() {
            if *o < self.input_len {
                continue;
            }
            let oidx = usize::try_from(*o).unwrap() - input_len;
            let mut stack = Vec::<StackEntry<T>>::new();
            stack.push(StackEntry {
                node: oidx,
                way: 0,
                subtree_index: None,
            });

            while !stack.is_empty() {
                let top = stack.last_mut().unwrap();
                let node_index = top.node;
                let way = top.way;

                let node_out_index = T::try_from(node_index + input_len).unwrap();
                if way == 0 {
                    if !visited[node_index] {
                        visited[node_index] = true;
                    } else {
                        stack.pop();
                        continue;
                    }

                    top.way += 1;
                    let gi0 = self.gates[node_index].0.i0;
                    if gi0 >= self.input_len {
                        let new_node_index = usize::try_from(gi0).unwrap() - input_len;
                        // println!("To next 0: {:?} {:?} {:?}", node_index + input_len, gi0,
                        //          self.gates[node_index].0);
                        // determine subtree_index
                        let subtree_index = if let Some(subtree_index) = top.subtree_index {
                            // propagate only to gate with usage<2
                            if usage[new_node_index] < 2 {
                                Some(subtree_index)
                            } else {
                                None
                            }
                        } else if usage[new_node_index] < 2 {
                            // if without subtree_index then its node index is subtree_index
                            Some(node_out_index)
                        } else {
                            None
                        };
                        stack.push(StackEntry {
                            node: new_node_index,
                            way: 0,
                            subtree_index,
                        });
                    }
                } else if way == 1 {
                    top.way += 1;
                    let gi1 = self.gates[node_index].0.i1;
                    if gi1 >= self.input_len {
                        // println!("To next 1: {:?} {:?}: {:?}", node_index + input_len, gi1,
                        //          self.gates[node_index].0);
                        let new_node_index = usize::try_from(gi1).unwrap() - input_len;
                        // determine subtree_index
                        let subtree_index = if let Some(subtree_index) = top.subtree_index {
                            // propagate xor only to gate with usage<2
                            if usage[new_node_index] < 2 {
                                Some(subtree_index)
                            } else {
                                None
                            }
                        } else if usage[new_node_index] < 2 {
                            // if without subtree_index then its node index is subtree_index
                            Some(node_out_index)
                        } else {
                            None
                        };
                        stack.push(StackEntry {
                            node: new_node_index,
                            way: 0,
                            subtree_index,
                        });
                    }
                } else {
                    // println!("Top node: {:?} {:?} {:?}", top.node + input_len,
                    //          top.way, top.subtree_index);
                    if let Some(subtree_index) = top.subtree_index {
                        // add to subtree_map
                        subtree_map.insert(node_out_index, subtree_index);
                    } else {
                        // add same root
                        subtree_map.insert(node_out_index, node_out_index);
                    }
                    stack.pop();
                }
            }
        }

        let mut subtree_object_map = BTreeMap::<T, SubTree<T>>::new();
        for (&x, &root) in &subtree_map {
            if let Some(st) = subtree_object_map.get_mut(&root) {
                st.gates.push((x, T::default()));
            } else {
                subtree_object_map.insert(
                    root,
                    SubTree {
                        root,
                        gates: vec![(x, T::default())],
                    },
                );
            }
        }

        (
            subtree_map,
            subtree_object_map
                .into_iter()
                .map(|(_, mut v)| {
                    v.sort_and_dedup();
                    v.fill_successors(self);
                    v
                })
                .collect::<Vec<_>>(),
        )
    }

    fn apply_subtree(&mut self, subtree: SubTreeCopy<T>) {
        let input_len = usize::try_from(self.input_len).unwrap();
        for i in 0..subtree.gates.len() {
            let oi = if i < subtree.gates.len() - 1 {
                subtree.subtree.gates[i].0
            } else {
                subtree.subtree.root
            };
            let oi = usize::try_from(oi).unwrap() - input_len;
            self.gates[oi] = subtree.gates[i];
        }
    }

    fn optimize_negs(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vgate<T: Clone + Copy>(func: VGateFunc, i0: T, i1: T) -> VGate<T> {
        VGate { i0, i1, func: func }
    }

    fn vgate_and<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::And, i0, i1)
    }
    fn vgate_or<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Or, i0, i1)
    }
    fn vgate_xor<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Xor, i0, i1)
    }

    #[test]
    fn test_vbinopcircuit_from() {
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_xor(2, 3), NoNegs),
                    (vgate_and(2, 3), NegInput1),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(5, 6), NegOutput),
                ],
                outputs: vec![(4, true), (7, false)],
            },
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn test_vbinopcircuit_subtrees() {
        assert_eq!(
            (
                BTreeMap::from_iter([(4, 4), (3, 4)]),
                vec![SubTree {
                    root: 4,
                    gates: vec![(3, 4)]
                }]
            ),
            VBinOpCircuit::from(
                Circuit::new(3, [Gate::new_xor(0, 1), Gate::new_xor(2, 3)], [(4, true)]).unwrap()
            )
            .subtrees()
        );

        assert_eq!(
            (
                BTreeMap::from_iter([(3, 3), (4, 4), (5, 7), (6, 7), (7, 7)]),
                vec![
                    SubTree {
                        root: 3,
                        gates: vec![],
                    },
                    SubTree {
                        root: 4,
                        gates: vec![],
                    },
                    SubTree {
                        root: 7,
                        gates: vec![(5, 7), (6, 7)],
                    },
                ]
            ),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .subtrees()
        );

        assert_eq!(
            (
                BTreeMap::from_iter([(3, 3), (4, 4), (5, 7), (6, 7), (7, 7)]),
                vec![
                    SubTree {
                        root: 3,
                        gates: vec![],
                    },
                    SubTree {
                        root: 4,
                        gates: vec![],
                    },
                    SubTree {
                        root: 7,
                        gates: vec![(5, 7), (6, 7)],
                    },
                ]
            ),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(3, 2),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .subtrees()
        );

        assert_eq!(
            (
                BTreeMap::from_iter([(3, 7), (4, 7), (5, 7), (6, 7), (7, 7)]),
                vec![SubTree {
                    root: 7,
                    gates: vec![(3, 4), (4, 5), (5, 7), (6, 7)]
                }]
            ),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 4),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(7, false)],
                )
                .unwrap()
            )
            .subtrees()
        );

        assert_eq!(
            (
                BTreeMap::from_iter([
                    (3, 4),
                    (4, 4),
                    (5, 7),
                    (6, 7),
                    (7, 7),
                    (8, 10),
                    (9, 10),
                    (10, 10)
                ]),
                vec![
                    SubTree {
                        root: 4,
                        gates: vec![(3, 4)]
                    },
                    SubTree {
                        root: 7,
                        gates: vec![(5, 7), (6, 7)]
                    },
                    SubTree {
                        root: 10,
                        gates: vec![(8, 10), (9, 10)]
                    },
                ]
            ),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(4, 2),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                        Gate::new_nor(2, 7),
                        Gate::new_xor(1, 7),
                        Gate::new_and(8, 9),
                    ],
                    [(4, true), (10, false)],
                )
                .unwrap()
            )
            .subtrees()
        );
    }

    #[test]
    fn test_vbinopcircuit_apply_subtree() {
        let mut circuit = VBinOpCircuit::from(
            Circuit::new(
                3,
                [
                    Gate::new_nimpl(0, 1),
                    Gate::new_xor(3, 2),
                    Gate::new_nimpl(4, 2),
                    Gate::new_and(0, 1),
                    Gate::new_nor(5, 6),
                    Gate::new_nor(2, 7),
                    Gate::new_xor(1, 7),
                    Gate::new_and(8, 9),
                ],
                [(4, true), (10, false)],
            )
            .unwrap(),
        );
        let (subtree_map, subtrees) = circuit.subtrees();
        let mut subtree_copies = subtrees
            .iter()
            .map(|st| SubTreeCopy::new(&circuit, st))
            .collect::<Vec<_>>();
        subtree_copies[0].gates[0] = (vgate_or(0, 1), NegInput1);
        subtree_copies[1].gates[2] = (vgate_and(5, 6), NoNegs);
        subtree_copies[2].gates[1] = (vgate_xor(1, 7), NegOutput);
        for st in subtree_copies {
            circuit.apply_subtree(st);
        }
        assert_eq!(
            circuit,
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_xor(3, 2), NoNegs),
                    (vgate_and(4, 2), NegInput1),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_and(5, 6), NoNegs),
                    (vgate_or(2, 7), NegOutput),
                    (vgate_xor(1, 7), NegOutput),
                    (vgate_and(8, 9), NoNegs),
                ],
                outputs: vec![(4, true), (10, false)],
            }
        );
    }

    fn vbinop_optimize_negs_in_subtree<T>(circuit: VBinOpCircuit<T>) -> VBinOpCircuit<T>
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash + Debug,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        let mut circuit = circuit.clone();
        let subtree = circuit.subtrees().1.pop().unwrap();
        let mut subtree_copy = SubTreeCopy::new(&circuit, &subtree);
        subtree_copy.optimize_negs();
        circuit.apply_subtree(subtree_copy);
        circuit
    }

    #[test]
    fn test_vbinopcircuit_optimize_negs_in_subtree() {
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NoNegs), (vgate_and(2, 3), NoNegs)],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NoNegs), (vgate_and(2, 3), NoNegs)],
                outputs: vec![(4, false)]
            })
        );

        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NoNegs), (vgate_and(2, 3), NegInput1)],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegOutput), (vgate_and(2, 3), NoNegs)],
                outputs: vec![(4, false)]
            })
        );
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NoNegs), (vgate_and(2, 3), NegInput1)],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegOutput), (vgate_and(3, 2), NoNegs)],
                outputs: vec![(4, false)]
            })
        );
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NoNegs), (vgate_or(3, 2), NegInput1),],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegOutput), (vgate_and(2, 3), NegOutput)],
                outputs: vec![(4, false)]
            })
        );
        // ignore this move
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegInput1), (vgate_and(2, 3), NegOutput)],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegInput1), (vgate_and(2, 3), NegOutput)],
                outputs: vec![(4, false)]
            })
        );
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_or(1, 0), NegInput1), (vgate_or(3, 2), NegOutput)],
                outputs: vec![(4, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![(vgate_and(0, 1), NegInput1), (vgate_and(3, 2), NegInput1)],
                outputs: vec![(4, false)]
            })
        );

        // more complex examples
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NoNegs),
                    (vgate_or(2, 4), NoNegs)
                ],
                outputs: vec![(5, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegOutput),
                    (vgate_or(2, 4), NegInput1)
                ],
                outputs: vec![(5, false)]
            })
        );
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(1, 0), NegInput1),
                    (vgate_and(3, 2), NoNegs),
                    (vgate_and(2, 4), NoNegs)
                ],
                outputs: vec![(5, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(3, 2), NegInput1),
                    (vgate_and(2, 4), NegInput1)
                ],
                outputs: vec![(5, false)]
            })
        );

        assert_eq!(
            VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NoNegs),
                    (vgate_or(9, 10), NoNegs),
                    (vgate_and(4, 11), NegInput1),
                    (vgate_and(12, 5), NoNegs),
                    (vgate_and(13, 6), NoNegs),
                    (vgate_and(14, 7), NoNegs),
                    (vgate_and(15, 8), NegOutput),
                ],
                outputs: vec![(16, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NegOutput),
                    (vgate_or(2, 3), NegOutput),
                    (vgate_and(9, 10), NegOutput),
                    (vgate_or(11, 4), NegInput1),
                    (vgate_or(12, 5), NegInput1),
                    (vgate_or(13, 6), NegInput1),
                    (vgate_or(14, 7), NegInput1),
                    (vgate_or(15, 8), NegInput1),
                ],
                outputs: vec![(16, false)]
            })
        );

        assert_eq!(
            VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NoNegs),
                    (vgate_or(10, 9), NegInput1),
                    (vgate_or(4, 11), NoNegs),
                    (vgate_and(12, 5), NegInput1),
                    (vgate_or(6, 13), NoNegs),
                    (vgate_and(14, 7), NegInput1),
                    (vgate_or(8, 15), NoNegs),
                ],
                outputs: vec![(16, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NoNegs),
                    (vgate_and(9, 10), NegInput1),
                    (vgate_or(4, 11), NegInput1),
                    (vgate_or(5, 12), NegInput1),
                    (vgate_or(6, 13), NegInput1),
                    (vgate_or(7, 14), NegInput1),
                    (vgate_or(8, 15), NegInput1),
                ],
                outputs: vec![(16, false)]
            })
        );

        // move negation through XOR subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NoNegs),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_or(6, 7), NegInput1),
                    (vgate_xor(8, 9), NoNegs),
                    (vgate_xor(10, 11), NoNegs),
                    (vgate_xor(12, 13), NoNegs),
                    (vgate_xor(14, 15), NegOutput),
                ],
                outputs: vec![(16, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NegOutput),
                    (vgate_or(2, 3), NegOutput),
                    (vgate_and(4, 5), NegOutput),
                    (vgate_or(6, 7), NegInput1),
                    (vgate_xor(8, 9), NoNegs),
                    (vgate_xor(10, 11), NoNegs),
                    (vgate_xor(12, 13), NoNegs),
                    (vgate_xor(14, 15), NoNegs),
                ],
                outputs: vec![(16, false)]
            })
        );
        // move negation through XOR subtree 2
        assert_eq!(
            VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_or(6, 7), NegInput1),
                    (vgate_xor(8, 9), NoNegs),
                    (vgate_xor(10, 11), NoNegs),
                    (vgate_xor(12, 13), NoNegs),
                    (vgate_xor(14, 15), NoNegs),
                ],
                outputs: vec![(16, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 9,
                gates: vec![
                    (vgate_or(0, 1), NegOutput),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                    (vgate_or(6, 7), NegInput1),
                    (vgate_xor(8, 9), NoNegs),
                    (vgate_xor(10, 11), NoNegs),
                    (vgate_xor(12, 13), NoNegs),
                    (vgate_xor(14, 15), NoNegs),
                ],
                outputs: vec![(16, false)]
            })
        );
        // single reduce subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(1, 0), NegInput1),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                ],
                outputs: vec![(6, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                ],
                outputs: vec![(6, false)]
            })
        );
        // no single reduce subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_xor(0, 1), NoNegs),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                ],
                outputs: vec![(6, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_xor(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                ],
                outputs: vec![(6, false)]
            })
        );
        // no single reduce subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                ],
                outputs: vec![(6, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NoNegs),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                ],
                outputs: vec![(6, false)]
            })
        );
        // single reduce subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(1, 0), NegInput1),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(7, 6), NoNegs),
                ],
                outputs: vec![(8, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(7, 6), NegInput1),
                ],
                outputs: vec![(8, false)]
            })
        );
        // no single reduce subtree - neg input at bad root gate input
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(6, 7), NegInput1),
                ],
                outputs: vec![(8, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(6, 7), NegInput1),
                ],
                outputs: vec![(8, false)]
            })
        );
        // single reduce subtree
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(1, 0), NegInput1),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(7, 6), NoNegs),
                ],
                outputs: vec![(8, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(4, 5), NegOutput),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(7, 6), NoNegs),
                ],
                outputs: vec![(8, false)]
            })
        );
        // single reduce subtree level 2
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(1, 0), NegInput1),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(2, 0), NegInput1),
                    (vgate_and(3, 1), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_and(6, 7), NoNegs),
                    (vgate_and(8, 9), NoNegs),
                ],
                outputs: vec![(10, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(0, 2), NegInput1),
                    (vgate_or(1, 3), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(6, 7), NoNegs),
                    (vgate_or(8, 9), NegOutput),
                ],
                outputs: vec![(10, false)]
            })
        );
        // no single reduce subtree level 2
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(0, 2), NoNegs),
                    (vgate_or(1, 3), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(6, 7), NoNegs),
                    (vgate_or(8, 9), NegOutput),
                ],
                outputs: vec![(10, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(0, 2), NoNegs),
                    (vgate_or(1, 3), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(6, 7), NoNegs),
                    (vgate_or(8, 9), NegOutput),
                ],
                outputs: vec![(10, false)]
            })
        );
        // single reduce subtree level 2 - in_next
        assert_eq!(
            VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_or(1, 0), NegInput1),
                    (vgate_and(3, 2), NegInput1),
                    (vgate_or(2, 0), NegInput1),
                    (vgate_and(3, 1), NegInput1),
                    (vgate_and(4, 5), NoNegs),
                    (vgate_and(6, 7), NoNegs),
                    (vgate_and(8, 9), NoNegs),
                    (vgate_or(0, 3), NoNegs),
                    (vgate_and(11, 10), NoNegs),
                ],
                outputs: vec![(12, false)]
            },
            vbinop_optimize_negs_in_subtree(VBinOpCircuit {
                input_len: 4,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_or(2, 3), NegInput1),
                    (vgate_and(0, 2), NegInput1),
                    (vgate_or(1, 3), NegInput1),
                    (vgate_or(4, 5), NoNegs),
                    (vgate_or(6, 7), NoNegs),
                    (vgate_or(8, 9), NoNegs),
                    (vgate_or(0, 3), NoNegs),
                    (vgate_and(11, 10), NegInput1),
                ],
                outputs: vec![(12, false)]
            })
        );
    }
}

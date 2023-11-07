use gatesim::*;

use crate::vcircuit::*;

use std::collections::{BTreeMap, HashMap};
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
            subtree,
            gates: subtree
                .gates
                .iter()
                .map(|(x, _)| circuit.gates[usize::try_from(*x).unwrap() - input_len])
                .chain(std::iter::once(
                    circuit.gates[usize::try_from(subtree.root).unwrap() - input_len],
                ))
                .collect::<Vec<_>>(),
        }
    }

    #[inline]
    fn gate_index(&self, t: T) -> Option<usize> {
        self.subtree.find_index(t)
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
    fn xor_subtree_map(&self) -> HashMap<T, T> {
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
            xor_index: Option<T>,
        }
        let gate_num = self.gates.len();
        let mut visited = vec![false; gate_num];
        let mut xor_map = HashMap::new();
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
                xor_index: None,
            });

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
                    let gi0 = self.gates[node_index].0.i0;
                    if gi0 >= self.input_len {
                        // println!("To next 0: {:?} {:?} {:?}", node_index + input_len, gi0,
                        //          self.gates[node_index].0);
                        let new_node_index = usize::try_from(gi0).unwrap() - input_len;
                        // determine xor_index
                        let xor_index = if let Some(xor_index) = top.xor_index {
                            // propagate xor only to XOR gate with usage<2
                            if self.gates[new_node_index].0.func == VGateFunc::Xor
                                && usage[new_node_index] < 2
                            {
                                Some(xor_index)
                            } else {
                                None
                            }
                        } else if self.gates[node_index].0.func == VGateFunc::Xor
                            && self.gates[new_node_index].0.func == VGateFunc::Xor
                            && usage[new_node_index] < 2
                        {
                            // if xor without xor_index then its node index is xor_index
                            Some(T::try_from(node_index + input_len).unwrap())
                        } else {
                            None
                        };
                        stack.push(StackEntry {
                            node: new_node_index,
                            way: 0,
                            xor_index,
                        });
                    }
                } else if way == 1 {
                    top.way += 1;
                    let gi1 = self.gates[node_index].0.i1;
                    if gi1 >= self.input_len {
                        // println!("To next 1: {:?} {:?}: {:?}", node_index + input_len, gi1,
                        //          self.gates[node_index].0);
                        let new_node_index = usize::try_from(gi1).unwrap() - input_len;
                        // determine xor_index
                        let xor_index = if let Some(xor_index) = top.xor_index {
                            // propagate xor only to XOR gate with usage<2
                            if self.gates[new_node_index].0.func == VGateFunc::Xor
                                && usage[new_node_index] < 2
                            {
                                Some(xor_index)
                            } else {
                                None
                            }
                        } else if self.gates[node_index].0.func == VGateFunc::Xor
                            && self.gates[new_node_index].0.func == VGateFunc::Xor
                            && usage[new_node_index] < 2
                        {
                            // if xor without xor_index then its node index is xor_index
                            Some(T::try_from(node_index + input_len).unwrap())
                        } else {
                            None
                        };
                        stack.push(StackEntry {
                            node: new_node_index,
                            way: 0,
                            xor_index,
                        });
                    }
                } else {
                    let node_out_index = T::try_from(node_index + input_len).unwrap();
                    // println!("Top node: {:?} {:?} {:?}", top.node + input_len,
                    //          top.way, top.xor_index);
                    if let Some(xor_index) = top.xor_index {
                        // add to xor_map
                        xor_map.insert(node_out_index, xor_index);
                    } else if self.gates[node_index].0.func == VGateFunc::Xor {
                        // add same root
                        xor_map.insert(node_out_index, node_out_index);
                    }
                    stack.pop();
                }
            }
        }
        xor_map
    }

    // returns list of occurrences for all gates (gate index as output index).
    fn occurrences(&self) -> Vec<Vec<VOccur<T>>> {
        let input_len = usize::try_from(self.input_len).unwrap();
        let mut occurs = vec![vec![]; self.gates.len()];
        for (i, (g, _)) in self.gates.iter().enumerate() {
            if g.i0 >= self.input_len {
                let i0 = usize::try_from(g.i0).unwrap() - input_len;
                occurs[i0].push(VOccur::Gate(T::try_from(i + input_len).unwrap()));
            }
            if g.i1 >= self.input_len {
                let i1 = usize::try_from(g.i1).unwrap() - input_len;
                let oidx = T::try_from(i + input_len).unwrap();
                if g.i0 != g.i1 {
                    occurs[i1].push(VOccur::Gate(oidx));
                } else {
                    *occurs[i1].last_mut().unwrap() = VOccur::GateDouble(oidx);
                }
            }
        }

        for (i, (o, _)) in self.outputs.iter().enumerate() {
            if *o >= self.input_len {
                let o = usize::try_from(*o).unwrap() - input_len;
                occurs[o].push(VOccur::Output(T::try_from(i).unwrap()));
            }
        }
        occurs
    }

    fn optimize_negs_to_occurs(&mut self, occurs: &[Vec<VOccur<T>>], xor_map: HashMap<T, T>) {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        enum HashKey<T> {
            Gate(T),
            Output(T),
        }
        // println!("Start optnegs");
        let input_len = usize::try_from(self.input_len).unwrap();
        for i in 0..self.gates.len() {
            let oi = T::try_from(i + input_len).unwrap();
            if let Some(doi) = xor_map.get(&oi) {
                if *doi != oi {
                    // if gate is part of xor subtree and not root.
                    // because that gate must have one occurrence to xor root
                    // therefore negation reduction is useless.
                    continue;
                }
            }

            let g_negs = self.gates[i].1;
            assert!(g_negs != NegInput1 || self.gates[i].0.func != VGateFunc::Xor);
            // println!("  Start: {:?}: {:?}: {:?}", oi, self.gates[i], occurs[i]);
            // check whether same type of occurrence (negation)
            let mut occurs_changed = HashMap::<HashKey<T>, (bool, bool)>::new();
            for occur in &occurs[i] {
                let (x_key, neg_i0, neg_i1) = match occur {
                    VOccur::Gate(x) => {
                        if let Some(xx) = xor_map.get(x) {
                            (HashKey::Gate(*xx), true, false)
                        } else {
                            let xs = usize::try_from(*x).unwrap() - input_len;
                            if self.gates[xs].0.i0 == oi {
                                (HashKey::Gate(*x), true, false)
                            } else {
                                (HashKey::Gate(*x), false, true)
                            }
                        }
                    }
                    VOccur::GateDouble(x) => {
                        if let Some(xx) = xor_map.get(x) {
                            (HashKey::Gate(*xx), true, true)
                        } else {
                            (HashKey::Gate(*x), true, true)
                        }
                    }
                    VOccur::Output(x) => (HashKey::Output(*x), true, false),
                };

                if let Some((occur_n0, occur_n1)) = occurs_changed.get_mut(&x_key) {
                    *occur_n0 ^= neg_i0;
                    *occur_n1 ^= neg_i1;
                } else {
                    occurs_changed.insert(x_key, (neg_i0, neg_i1));
                }
            }

            // println!("  OccursChanged: {:?}: {:?}", oi, occurs_changed);
            // calculate balance of removed negations
            let negs_removed = occurs_changed
                .iter()
                .map(|(k, (neg_i0, neg_i1))| {
                    // return number of removed negations in occurrence gate.
                    match k {
                        HashKey::Gate(x) => {
                            let (occur_g, occur_negs) =
                                &self.gates[usize::try_from(*x).unwrap() - input_len];
                            let (_, new_negs) =
                                occur_g.binop_neg_args(*occur_negs, *neg_i0, *neg_i1);
                            isize::from(*occur_negs != NoNegs) - isize::from(new_negs != NoNegs)
                        }
                        HashKey::Output(x) => {
                            if *neg_i0 {
                                let on = self.outputs[usize::try_from(*x).unwrap()].1;
                                // on=false -> -1, on=true -> 1
                                (isize::from(on) << 1) - 1
                            } else {
                                0
                            }
                        }
                    }
                })
                .sum::<isize>();

            let min_removed = match g_negs {
                NoNegs => 2,
                NegInput1 => 1,
                NegOutput => -1,
            };

            // println!("  NegsRemoved: {:?}: {}", oi, negs_removed);
            if negs_removed >= min_removed {
                // apply changes if change remove more negations than added negations.
                self.gates[i] = self.gates[i].0.binop_neg(self.gates[i].1);
                for (k, (neg_i0, neg_i1)) in occurs_changed.into_iter() {
                    match k {
                        HashKey::Gate(x) => {
                            let xi = usize::try_from(x).unwrap() - input_len;
                            let (occur_g, occur_negs) = self.gates[xi];
                            self.gates[xi] = occur_g.binop_neg_args(occur_negs, neg_i0, neg_i1);
                            // println!("    Change: {:?}: {:?}: {:?} -> {:?}",
                            //          oi, x, (occur_g, occur_negs), self.gates[xi]);
                        }
                        HashKey::Output(x) => {
                            if neg_i0 {
                                let out_negs = &mut self.outputs[usize::try_from(x).unwrap()].1;
                                *out_negs = !*out_negs;
                            }
                        }
                    }
                }
            }
        }
    }

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

    fn optimize_negs_in_subtree(&mut self, subtree: &SubTree<T>) {
        let input_len = usize::try_from(self.input_len).unwrap();
        for (i, next_i) in &subtree.gates {
            let oi = usize::try_from(*i).unwrap() - input_len;
            let next_oi = usize::try_from(*next_i).unwrap() - input_len;
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
                let rg_oi0 = usize::try_from(rg.i0).unwrap() - input_len;
                let rg_oi1 = usize::try_from(rg.i1).unwrap() - input_len;
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
                let rg_oi00 = usize::try_from(rg0g.i0).unwrap() - input_len;
                let rg_oi01 = usize::try_from(rg0g.i1).unwrap() - input_len;
                let rg_oi10 = usize::try_from(rg1g.i0).unwrap() - input_len;
                let rg_oi11 = usize::try_from(rg1g.i1).unwrap() - input_len;
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
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    // arguments: self - original (no changes), opt1 - if option1 changed to true.
    // opt2 - if option2 changed to true.
    fn is_independent_optimize_neg(&self, opt1: &Self, opt2: &Self) -> bool {
        false
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
    fn vgate_nand<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Nand, i0, i1)
    }
    fn vgate_or<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Or, i0, i1)
    }
    fn vgate_nor<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Nor, i0, i1)
    }
    fn vgate_impl<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Impl, i0, i1)
    }
    fn vgate_nimpl<T: Clone + Copy>(i0: T, i1: T) -> VGate<T> {
        vgate(VGateFunc::Nimpl, i0, i1)
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
    fn test_vbinopcircuit_xor_subtree_map() {
        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 4)]),
            VBinOpCircuit::from(
                Circuit::new(3, [Gate::new_xor(0, 1), Gate::new_xor(2, 3)], [(4, true)]).unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 4)]),
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
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 3)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1), // used more than once!
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 3)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1), // used more than once!
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 4)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 4),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 4)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(2, 4),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 4)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_and(0, 1), // not XOR
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 4)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_and(0, 1), // not XOR
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 4), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3), // used by output (used more than once)
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(4, 5),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 4), (3, 4), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3), // used by output (used more than once)
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(5, 4),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 6), (3, 6), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(4, 5),
                        Gate::new_nor(5, 6),
                    ],
                    [(7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 6), (3, 6), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(5, 4),
                        Gate::new_nor(5, 6),
                    ],
                    [(7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 6), (3, 6), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(5, 4),
                        Gate::new_nor(5, 6),
                    ],
                    [(7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        assert_eq!(
            HashMap::from_iter([(4, 6), (3, 3), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1), // used more than once
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(4, 5),
                        Gate::new_nor(5, 6),
                    ],
                    [(3, false), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
        assert_eq!(
            HashMap::from_iter([(4, 6), (3, 3), (6, 6)]),
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1), // used more than once
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(1, 2),
                        Gate::new_xor(4, 5),
                        Gate::new_nor(5, 6),
                    ],
                    [(3, false), (7, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );

        // bigger sample
        assert_eq!(
            HashMap::from_iter([
                (13, 19),
                (14, 19),
                (15, 19),
                (16, 19),
                (17, 19),
                (18, 19),
                (19, 19),
                (27, 31),
                (28, 31),
                (31, 31),
                (29, 32),
                (30, 32),
                (32, 32),
                (33, 36),
                (35, 36),
                (36, 36),
            ]),
            VBinOpCircuit::from(
                Circuit::new(
                    4,
                    [
                        Gate::new_and(0, 1),   // 4
                        Gate::new_nimpl(0, 1), // 5
                        Gate::new_nor(0, 1),   // 6
                        Gate::new_and(0, 2),   // 7
                        Gate::new_nimpl(0, 2), // 8
                        Gate::new_nor(0, 2),   // 9
                        Gate::new_and(1, 2),   // 10
                        Gate::new_nimpl(1, 2), // 11
                        Gate::new_nor(1, 2),   // 12
                        Gate::new_xor(4, 5),   // 13
                        Gate::new_xor(6, 7),   // 14
                        Gate::new_xor(8, 9),   // 15
                        Gate::new_xor(10, 11), // 16
                        Gate::new_xor(13, 14), // 17
                        Gate::new_xor(15, 16), // 18
                        Gate::new_xor(17, 18), // 19
                        Gate::new_and(1, 3),   // 20
                        Gate::new_nimpl(1, 3), // 21
                        Gate::new_nor(1, 3),   // 22
                        Gate::new_and(2, 3),   // 23
                        Gate::new_nimpl(2, 3), // 24
                        Gate::new_nor(2, 3),   // 25
                        Gate::new_nor(0, 3),   // 26
                        Gate::new_xor(12, 20), // 27
                        Gate::new_xor(21, 22), // 28
                        Gate::new_xor(23, 24), // 29
                        Gate::new_xor(25, 26), // 30
                        Gate::new_xor(27, 28), // 31
                        Gate::new_xor(29, 30), // 32
                        Gate::new_xor(31, 32), // 33
                        Gate::new_and(2, 32),  // 34
                        Gate::new_xor(3, 34),  // 35
                        Gate::new_xor(33, 35), // 36
                    ],
                    [(19, false), (31, false), (36, false)],
                )
                .unwrap()
            )
            .xor_subtree_map()
        );
    }

    #[test]
    fn test_vbinopcircuit_occurrences() {
        assert_eq!(
            vec![
                vec![VOccur::Gate(4), VOccur::Gate(5)],
                vec![VOccur::Output(0)],
                vec![VOccur::Gate(7)],
                vec![VOccur::Gate(7)],
                vec![VOccur::Output(1)],
            ],
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
            .occurrences()
        );

        assert_eq!(
            vec![
                vec![VOccur::Gate(4), VOccur::Gate(5)],
                vec![VOccur::GateDouble(6), VOccur::Output(0)],
                vec![VOccur::Gate(7)],
                vec![VOccur::Gate(7)],
                vec![VOccur::Output(1)],
            ],
            VBinOpCircuit::from(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_nimpl(2, 3),
                        Gate::new_and(4, 4),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap()
            )
            .occurrences()
        );
    }

    #[test]
    fn test_vbinopcircuit_optimize_negs_to_occurs() {
        let mut circuit = VBinOpCircuit::from(
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
            .unwrap(),
        );
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegInput1),
                    (vgate_xor(2, 3), NoNegs),
                    (vgate_and(2, 3), NegInput1),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(5, 6), NoNegs),
                ],
                outputs: vec![(4, true), (7, true)],
            },
            circuit
        );

        // NegInput1 negation
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegInput1),
                (vgate_xor(2, 3), NegOutput),
                (vgate_and(2, 3), NegInput1),
                (vgate_and(0, 1), NoNegs),
                (vgate_or(5, 6), NegOutput),
            ],
            outputs: vec![(4, true), (7, true)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_or(1, 0), NegInput1), // force reduction
                    (vgate_xor(2, 3), NoNegs),
                    (vgate_and(2, 3), NoNegs),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(5, 6), NoNegs),
                ],
                outputs: vec![(4, true), (7, false)],
            },
            circuit
        );

        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NoNegs),
                (vgate_xor(2, 3), NegOutput),
                (vgate_and(2, 3), NegInput1),
                (vgate_and(0, 1), NoNegs),
                (vgate_or(5, 6), NegOutput),
            ],
            outputs: vec![(4, true), (7, true)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegOutput), // force reduction
                    (vgate_xor(2, 3), NoNegs),
                    (vgate_and(2, 3), NoNegs),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(5, 6), NoNegs),
                ],
                outputs: vec![(4, true), (7, false)],
            },
            circuit
        );

        // optimize not(or(not,not)) -> and(..,..)
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegOutput),
                (vgate_or(1, 2), NegOutput),
                (vgate_or(3, 4), NegOutput),
                (vgate_xor(0, 1), NoNegs),
                (vgate_xor(2, 6), NegOutput),
            ],
            outputs: vec![(5, false), (7, true)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(1, 2), NoNegs),
                    (vgate_and(3, 4), NoNegs),
                    (vgate_xor(0, 1), NoNegs),
                    (vgate_xor(2, 6), NoNegs),
                ],
                outputs: vec![(5, false), (7, false)],
            },
            circuit
        );

        // reduce by adding negation
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NoNegs), // forced reduction
                (vgate_or(1, 2), NoNegs),  // forced reduction
                (vgate_and(3, 4), NegOutput),
                (vgate_or(3, 4), NegOutput),
                (vgate_xor(0, 3), NegOutput),
                (vgate_xor(1, 3), NegOutput),
                (vgate_or(5, 6), NoNegs),
                (vgate_or(7, 9), NoNegs),
                (vgate_or(8, 10), NoNegs),
            ],
            outputs: vec![(11, false)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegOutput),
                    (vgate_or(1, 2), NegOutput),
                    (vgate_or(3, 4), NoNegs),
                    (vgate_and(3, 4), NoNegs),
                    (vgate_xor(0, 3), NoNegs),
                    (vgate_xor(1, 3), NoNegs),
                    (vgate_or(5, 6), NoNegs),
                    (vgate_or(7, 9), NoNegs),
                    (vgate_or(8, 10), NoNegs),
                ],
                outputs: vec![(11, false)],
            },
            circuit
        );

        // omit negation if too many negation added
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegOutput), // too many negations added
                (vgate_or(1, 2), NegOutput),  // too many negations added
                (vgate_or(1, 3), NoNegs),
                (vgate_and(2, 3), NoNegs),
                (vgate_and(1, 4), NoNegs),
                (vgate_or(2, 4), NoNegs),
            ],
            outputs: vec![(5, false), (6, false), (7, false), (8, false)],
        };
        let orig_circuit = circuit.clone();
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(orig_circuit, circuit);

        // handling XOR subtrees
        for t in [false, true] {
            let mut circuit = VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegOutput),
                    (vgate_or(0, 2), NegOutput),
                    (vgate_and(0, 2), if t { NegOutput } else { NoNegs }),
                    (vgate_or(1, 2), NegOutput),
                    (vgate_xor(3, 4), NoNegs),
                    (vgate_xor(5, 6), NoNegs),
                    (vgate_xor(7, 8), NoNegs),
                ],
                outputs: vec![(9, true)],
            };
            let xor_map = circuit.xor_subtree_map();
            let occurs = circuit.occurrences();
            circuit.optimize_negs_to_occurs(&occurs, xor_map);
            assert_eq!(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        (vgate_and(0, 1), NoNegs),
                        (vgate_or(0, 2), NoNegs),
                        (vgate_and(0, 2), NoNegs),
                        (vgate_or(1, 2), NoNegs),
                        (vgate_xor(3, 4), NoNegs),
                        (vgate_xor(5, 6), NoNegs),
                        (vgate_xor(7, 8), NoNegs),
                    ],
                    outputs: vec![(9, t)],
                },
                circuit
            );
        }

        // optimize not(or(not,not)) -> and(..,..)
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegOutput),
                (vgate_or(3, 3), NegOutput), // double usage
                (vgate_xor(0, 1), NoNegs),
                (vgate_xor(2, 5), NegOutput),
            ],
            outputs: vec![(4, false), (6, true)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NoNegs),
                    (vgate_and(3, 3), NoNegs),
                    (vgate_xor(0, 1), NoNegs),
                    (vgate_xor(2, 5), NoNegs),
                ],
                outputs: vec![(4, false), (6, false)],
            },
            circuit
        );

        // omit negation if too many negation added except gate 4
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegOutput), // too many negations added
                (vgate_or(1, 2), NegOutput),  // it reduces one negation
                (vgate_or(1, 3), NoNegs),
                (vgate_and(2, 3), NoNegs),
                (vgate_and(1, 4), NoNegs),
                (vgate_or(2, 4), NoNegs),
            ],
            outputs: vec![
                (4, true),
                (4, true),
                (5, false),
                (6, false),
                (7, false),
                (8, false),
            ],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegOutput), // too many negations added
                    (vgate_or(1, 2), NoNegs),     // too many negations added
                    (vgate_or(1, 3), NoNegs),
                    (vgate_and(2, 3), NoNegs),
                    (vgate_and(1, 4), NegInput1),
                    (vgate_or(2, 4), NegInput1),
                ],
                outputs: vec![
                    (4, false),
                    (4, false),
                    (5, false),
                    (6, false),
                    (7, false),
                    (8, false)
                ],
            },
            circuit
        );

        // handling XOR subtrees
        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NegOutput),
                (vgate_or(0, 2), NegOutput),
                (vgate_and(0, 2), NoNegs),
                (vgate_or(1, 2), NegOutput),
                (vgate_and(1, 2), NegOutput),
                (vgate_xor(3, 4), NoNegs),
                (vgate_xor(5, 6), NoNegs),
                (vgate_xor(8, 9), NoNegs),
                (vgate_xor(3, 5), NoNegs),
                (vgate_xor(6, 7), NoNegs),
                (vgate_xor(11, 12), NegOutput),
            ],
            outputs: vec![(10, true), (13, false)],
        };
        let xor_map = circuit.xor_subtree_map();
        let occurs = circuit.occurrences();
        circuit.optimize_negs_to_occurs(&occurs, xor_map);
        assert_eq!(
            VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(0, 2), NoNegs),
                    (vgate_and(0, 2), NoNegs),
                    (vgate_or(1, 2), NegOutput),
                    (vgate_and(1, 2), NoNegs),
                    (vgate_xor(3, 4), NoNegs),
                    (vgate_xor(5, 6), NoNegs),
                    (vgate_xor(8, 9), NoNegs),
                    (vgate_xor(3, 5), NoNegs),
                    (vgate_xor(6, 7), NoNegs),
                    (vgate_xor(11, 12), NoNegs),
                ],
                outputs: vec![(10, true), (13, true)],
            },
            circuit
        );

        // handling XOR subtrees
        for t in [false, true] {
            let mut circuit = VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    (vgate_and(0, 1), NegOutput),
                    (vgate_or(0, 2), NegOutput),
                    (vgate_and(0, 2), if t { NegOutput } else { NoNegs }),
                    (vgate_xor(3, 4), NoNegs),
                    (vgate_xor(5, 3), NoNegs),
                    (vgate_xor(6, 7), NoNegs),
                ],
                outputs: vec![(8, true)],
            };
            let xor_map = circuit.xor_subtree_map();
            let occurs = circuit.occurrences();
            circuit.optimize_negs_to_occurs(&occurs, xor_map);
            assert_eq!(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        (vgate_and(0, 1), NoNegs),
                        (vgate_or(0, 2), NoNegs),
                        (vgate_and(0, 2), NoNegs),
                        (vgate_xor(3, 4), NoNegs),
                        (vgate_xor(5, 3), NoNegs),
                        (vgate_xor(6, 7), NoNegs),
                    ],
                    outputs: vec![(8, t)],
                },
                circuit
            );
        }
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
        circuit.optimize_negs_in_subtree(&subtree);
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

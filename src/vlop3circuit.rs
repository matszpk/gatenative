use gatesim::*;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::vbinopcircuit::*;
use crate::vcircuit::*;
use crate::VNegs::{self, *};

// Next idea: after conversion to lop3 with binop: reduce negations by using connection
// between lop3 and binop - by moving negation into lop3.
// Idea: always convert to LOP3 two gates. Do not convert to LOP3 single gate even with negation.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum VLOP3GateFunc {
    And(VNegs),
    Or(VNegs),
    Xor(VNegs),
    LOP3(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct VLOP3Gate<T: Clone + Copy> {
    pub(crate) i0: T,
    pub(crate) i1: T,
    pub(crate) i2: T,
    pub(crate) func: VLOP3GateFunc,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct VLOP3Circuit<T: Clone + Copy> {
    pub(crate) input_len: T,
    pub(crate) gates: Vec<VLOP3Gate<T>>,
    pub(crate) outputs: Vec<(T, bool)>,
}

#[derive(Clone)]
struct MTUArea<T> {
    nodes: Vec<T>,
    extra_cost: usize,
}

impl<T> Default for MTUArea<T> {
    #[inline]
    fn default() -> Self {
        Self {
            nodes: vec![],
            extra_cost: 0,
        }
    }
}

impl<T> MTUArea<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // and improve - fix other TouchNodes to make better result if possible
    fn add_node(&mut self, wire_index: T) {}

    fn calc_lop3nodes(&self, lop3nodes: &mut [LOP3Node<T>]) {}

    fn improve_and_optimize_and_gen_lop3nodes(
        &mut self,
        circuit: &VBinOpCircuit<T>,
        lop3node: &mut [LOP3Node<T>],
    ) {
    }

    fn nonfarest_nodes(&self) -> Vec<T> {
        vec![]
    }
}

// instead LOP3Boundary use path penetration form:
// entry: 0 - nothing, 1 - go left, 2 - go right, 3 - go left and right
// and encode in bits to save memory.
#[derive(Clone, Copy, PartialEq, Eq)]
struct PathMove(u8);

impl PathMove {
    #[inline]
    fn is_first(self) -> bool {
        (self.0 & 1) != 0
    }
    #[inline]
    fn is_second(self) -> bool {
        (self.0 & 2) != 0
    }
    #[inline]
    fn go_first(self) -> Self {
        Self(self.0 | 1)
    }
    #[inline]
    fn go_second(self) -> Self {
        Self(self.0 | 2)
    }
}

// tree moves organization:
//       /--------0-------\
//   /---1---\       /----2---\
//   3       4       5        6
// 0 - root, 1 - first level start, 3 - second level start
// leaves are deepest LOP3 gates.
type LOP3SubTreePaths = [PathMove; 7];

#[derive(Clone)]
struct LOP3Node<T> {
    args: [T; 3],                 // arguments, also leaves of LOP3 subtree
    tree_paths: LOP3SubTreePaths, // LOP3 subtree paths
    mtu_cost: usize,
}

impl<T> Default for LOP3Node<T>
where
    T: Clone + Copy + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            args: [T::default(); 3],
            tree_paths: [PathMove(0); 7],
            mtu_cost: 0,
        }
    }
}

fn find_best_lop3node<T>(
    circuit: &VBinOpCircuit<T>,
    lop3nodes: &[LOP3Node<T>],
    wire_index: T,
    preferred_nodes: &[T],
) -> LOP3Node<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gates = &circuit.gates;
    // generate tree to explore
    let tree = {
        let mut tree = [None; 7];
        let mut old_level_start = 0;
        let mut level_start = 1;
        tree[0] = Some(wire_index);
        for level in 1..3 {
            for pos in 0..level_start - old_level_start {
                if let Some(t) = tree[old_level_start + pos] {
                    if t >= input_len_t {
                        let gi = usize::try_from(t).unwrap();
                        let g = gates[gi - input_len].0;
                        tree[level_start + (pos << 1)] = Some(g.i0);
                        tree[level_start + (pos << 1) + 1] = Some(g.i1);
                    }
                }
            }
            old_level_start = level_start;
            level_start += level_start + 1;
        }
        tree
    };
    // algorithm: simple batch of combinations with difference
    #[derive(Clone, Copy)]
    enum CombBatchEntry {
        // fields: node to operate, to execute test
        AddNode(u8, bool),
        RemoveNode(u8, bool),
    }
    use CombBatchEntry::*;
    const COMB_BATCH: [CombBatchEntry; 31] = [
        AddNode(0, false), // (R) *
        AddNode(1, true),  // (R,C0)
        //
        AddNode(3, true),     // (R,C0,C00)
        AddNode(4, true),     // (R,C0,C00,C01)
        RemoveNode(3, true),  // (R,C0,C01)
        RemoveNode(4, false), // (R,C0) *
        //
        AddNode(2, true), // (R,C0,C1)
        //
        AddNode(3, true),     // (R,C0,C1,C00)
        AddNode(4, true),     // (R,C0,C1,C00,C01)
        RemoveNode(3, true),  // (R,C0,C1,C01)
        RemoveNode(3, false), // (R,C0,C1) *
        AddNode(5, true),     // (R,C0,C1,C10)
        AddNode(3, true),     // (R,C0,C1,C00,C10)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C10)
        RemoveNode(3, true),  // (R,C0,C1,C01,C10)
        RemoveNode(3, false), // (R,C0,C1,C10) *
        AddNode(6, true),     // (R,C0,C1,C10,C11)
        AddNode(3, true),     // (R,C0,C1,C00,C10,C11)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C10,C11)
        RemoveNode(3, true),  // (R,C0,C1,C01,C10,C11)
        RemoveNode(3, false), // (R,C0,C1,C10,C11) *
        RemoveNode(5, true),  // (R,C0,C1,C11)
        AddNode(3, true),     // (R,C0,C1,C00,C11)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C11)
        RemoveNode(3, true),  // (R,C0,C1,C01,C11)
        RemoveNode(3, false), // (R,C0,C1,C11) *
        RemoveNode(6, false), // (R,C0,C1) *
        //
        RemoveNode(1, true), // (R,C1)
        //
        AddNode(5, true),    // (R,C1,C10)
        AddNode(6, true),    // (R,C1,C10,C11)
        RemoveNode(5, true), // (R,C1,C11)
    ];
    let mut leaves: Vec<T> = vec![];
    for instr in COMB_BATCH {
        let ex = match instr {
            AddNode(i, ex) => {
                if let Some(tt) = tree[i as usize] {
                    if tt >= input_len_t {
                        let t = usize::try_from(tt).unwrap();
                        leaves.retain(|x| *x != tt);
                        let g = gates[t - input_len].0;
                        let a0 = g.i0;
                        let a1 = g.i1;
                        if leaves.iter().all(|x| *x != a0) {
                            leaves.push(a0);
                        }
                        if leaves.iter().all(|x| *x != a1) {
                            leaves.push(a1);
                        }
                        ex
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            RemoveNode(i, ex) => {
                if let Some(tt) = tree[i as usize] {
                    if tt >= input_len_t {
                        let t = usize::try_from(tt).unwrap();
                        let g = gates[t - input_len].0;
                        let a0 = g.i0;
                        let a1 = g.i1;
                        leaves.retain(|x| *x != a0);
                        leaves.retain(|x| *x != a1);
                        leaves.push(tt);
                        ex
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        };
        if ex {
            if leaves.len() <= 3 {
                // register case
            }
        }
    }
    LOP3Node {
        args: [T::default(); 3],
        tree_paths: [PathMove(0); 7],
        mtu_cost: 0,
    }
}

fn update_mtuareas_from_lop3node<T>(
    mtuareas: &mut [MTUArea<T>],
    circuit: &VBinOpCircuit<T>,
    subtrees: &[SubTree<T>],
    lop3node: &LOP3Node<T>,
) where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
}

// MTU graph and coverage: index - gate index, value - subtree index
fn gen_subtree_coverage<T>(circuit: &VBinOpCircuit<T>, subtrees: &[SubTree<T>]) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len).unwrap();
    let mut coverage = vec![T::default(); circuit.gates.len()];
    for (i, st) in subtrees.iter().enumerate() {
        for (gi, _) in st
            .gates()
            .iter()
            .copied()
            .chain(std::iter::once((st.root(), T::default())))
        {
            let gi = usize::try_from(gi).unwrap();
            coverage[gi - input_len] = T::try_from(i).unwrap();
        }
    }
    coverage
}

// THINK: really needed?? just pass in reverse order by subtrees.
// argument - return from subtree_dependencies
// return: index - subtree index, value - list of subtree's children (subtree index)
fn gen_rev_subtree_dependencies<T>(deps: Vec<Vec<(T, T, bool)>>) -> Vec<Vec<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let mut revdeps = vec![vec![]; deps.len()];
    for (i, deplist) in deps.into_iter().enumerate() {
        for (ni, _, _) in deplist {
            revdeps[usize::try_from(ni).unwrap()].push(T::try_from(i).unwrap());
        }
    }
    for dep in &mut revdeps {
        dep.sort();
        dep.dedup();
    }
    revdeps
}

fn get_preferred_nodes_from_mtuareas<T>(
    circuit: &VBinOpCircuit<T>,
    mtuareas: &[MTUArea<T>],
    circuit_outputs: &HashSet<T>,
    nidx: T,
) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    vec![]
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
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut mtuareas = vec![MTUArea::<T>::default(); subtrees.len()];
        let mut lop3nodes = vec![LOP3Node::<T>::default(); gates.len()];
        let circuit_outputs = HashSet::<T>::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        // generate lop3nodes
        for i in (0..subtrees.len()).rev() {
            let subtree = &subtrees[i];
            mtuareas[i].improve_and_optimize_and_gen_lop3nodes(&circuit, &mut lop3nodes);
            let nonfarest_nodes: Vec<T> = mtuareas[i].nonfarest_nodes();
            // get nonfarest nodes
            for (i, nidx) in subtree
                .gates()
                .iter()
                .map(|(x, _)| *x)
                .chain(std::iter::once(subtree.root()))
                .enumerate()
                // skip all nonfarest nodes in MTUAreaview
                .filter(|(_, nidx)| nonfarest_nodes.iter().all(|x| *x != *nidx))
            {
                let gidx = usize::try_from(nidx).unwrap() - input_len;
                // get preferred nodes from mtuareas
                let preferred_nodes =
                    get_preferred_nodes_from_mtuareas(&circuit, &mtuareas, &circuit_outputs, nidx);
                lop3nodes[gidx] = find_best_lop3node(&circuit, &lop3nodes, nidx, &preferred_nodes);
                update_mtuareas_from_lop3node(&mut mtuareas, &circuit, &subtrees, &lop3nodes[gidx]);
            }
        }
        // filter lop3nodes
        // convert inputs in lop3nodes
        let mut out = Self::from_lop3nodes(circuit, lop3nodes);
        out.optimize_negs();
        out
    }
}

impl<T> VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from_lop3nodes(circuit: VBinOpCircuit<T>, lop3nodes: Vec<LOP3Node<T>>) -> VLOP3Circuit<T> {
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }

    // return true if negation of argument is needed
    fn reduce_neg_from_lop3_input(&mut self, gi0: usize, usage: &[u8]) -> bool {
        let input_len = usize::try_from(self.input_len).unwrap();
        let g0 = self.gates[gi0 - input_len];
        match g0.func {
            VLOP3GateFunc::And(negs) => {
                if negs == NegOutput {
                    self.gates[gi0 - input_len].func = VLOP3GateFunc::And(NoNegs);
                    true
                } else if negs == NegInput1 {
                    // check LOP3(and(LOP3,!v1)) and convert to: LOP3(!or(LOP3_neg,v1))
                    let g0i0 = usize::try_from(g0.i0).unwrap();
                    if g0i0 >= input_len && usage[g0i0 - input_len] < 2 {
                        let g00 = self.gates[gi0 - input_len];
                        if let VLOP3GateFunc::LOP3(f) = g00.func {
                            self.gates[gi0 - input_len].func = VLOP3GateFunc::Or(NoNegs);
                            self.gates[g0i0 - input_len].func = VLOP3GateFunc::LOP3(!f);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            VLOP3GateFunc::Or(negs) => {
                if negs == NegOutput {
                    self.gates[gi0 - input_len].func = VLOP3GateFunc::Or(NoNegs);
                    true
                } else if negs == NegInput1 {
                    // check LOP3(or(LOP3,!v1)) and convert to: LOP3(!and(LOP3_neg,v1))
                    let g0i0 = usize::try_from(g0.i0).unwrap();
                    if g0i0 >= input_len && usage[g0i0 - input_len] < 2 {
                        let g00 = self.gates[gi0 - input_len];
                        if let VLOP3GateFunc::LOP3(f) = g00.func {
                            self.gates[gi0 - input_len].func = VLOP3GateFunc::And(NoNegs);
                            self.gates[g0i0 - input_len].func = VLOP3GateFunc::LOP3(!f);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            VLOP3GateFunc::Xor(negs) => {
                if negs == NegOutput || negs == NegInput1 {
                    self.gates[gi0 - input_len].func = VLOP3GateFunc::Xor(NoNegs);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    // optimize negations in 2-input gates that neighbors with LOP3 gates.
    fn optimize_negs(&mut self) {
        let input_len = usize::try_from(self.input_len).unwrap();
        // calculate usage to avoids multiple usages
        let mut usage = vec![0u8; self.gates.len()];
        let mut usage_by_gates = vec![0u8; self.gates.len()];
        for g in &self.gates {
            if g.i0 >= self.input_len {
                let i0 = usize::try_from(g.i0).unwrap() - input_len;
                if usage[i0] < 2 {
                    usage[i0] += 1;
                }
                if usage_by_gates[i0] < 2 {
                    usage_by_gates[i0] += 1;
                }
            }
            if g.i1 >= self.input_len {
                let i1 = usize::try_from(g.i1).unwrap() - input_len;
                if usage[i1] < 2 {
                    usage[i1] += 1;
                }
                if usage_by_gates[i1] < 2 {
                    usage_by_gates[i1] += 1;
                }
            }
            if matches!(g.func, VLOP3GateFunc::LOP3(_)) {
                if g.i2 >= self.input_len {
                    let i2 = usize::try_from(g.i2).unwrap() - input_len;
                    if usage[i2] < 2 {
                        usage[i2] += 1;
                    }
                    if usage_by_gates[i2] < 2 {
                        usage_by_gates[i2] += 1;
                    }
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
        // optimize negations
        let input_len = usize::try_from(self.input_len).unwrap();
        for i in 0..self.gates.len() {
            let g = self.gates[i];
            // apply optimizations rules
            match g.func {
                VLOP3GateFunc::And(negs) => {
                    if negs == NegInput1 {
                        let gi1 = usize::try_from(g.i1).unwrap();
                        if gi1 >= input_len && usage[gi1 - input_len] < 2 {
                            let g1 = self.gates[gi1 - input_len];
                            if let VLOP3GateFunc::LOP3(f) = g1.func {
                                // negate lop3 and remove negation from second input
                                self.gates[gi1 - input_len].func = VLOP3GateFunc::LOP3(!f);
                                self.gates[i].func = VLOP3GateFunc::And(NoNegs);
                            }
                        }
                    }
                }
                VLOP3GateFunc::Or(negs) => {
                    if negs == NegInput1 {
                        let gi1 = usize::try_from(g.i1).unwrap();
                        if gi1 >= input_len && usage[gi1 - input_len] < 2 {
                            let g1 = self.gates[gi1 - input_len];
                            if let VLOP3GateFunc::LOP3(f) = g1.func {
                                // negate lop3 and remove negation from second input
                                self.gates[gi1 - input_len].func = VLOP3GateFunc::LOP3(!f);
                                self.gates[i].func = VLOP3GateFunc::Or(NoNegs);
                            }
                        }
                    }
                }
                VLOP3GateFunc::Xor(negs) => {
                    if negs == NegInput1 {
                        let gi1 = usize::try_from(g.i1).unwrap();
                        if gi1 >= input_len && usage[gi1 - input_len] < 2 {
                            let g1 = self.gates[gi1 - input_len];
                            if let VLOP3GateFunc::LOP3(f) = g1.func {
                                // negate lop3 and remove negation from second input
                                self.gates[gi1 - input_len].func = VLOP3GateFunc::LOP3(!f);
                                self.gates[i].func = VLOP3GateFunc::Xor(NoNegs);
                            }
                        }
                    }
                }
                VLOP3GateFunc::LOP3(f) => {
                    let mut f = f;
                    let gi0 = usize::try_from(g.i0).unwrap();
                    if gi0 >= input_len && usage[gi0 - input_len] < 2 {
                        if self.reduce_neg_from_lop3_input(gi0, &usage) {
                            f = (f >> 4) | (f << 4);
                        }
                    }
                    let gi1 = usize::try_from(g.i1).unwrap();
                    if gi1 >= input_len && usage[gi1 - input_len] < 2 {
                        if self.reduce_neg_from_lop3_input(gi1, &usage) {
                            f = ((f >> 2) & 0x33) | ((f << 2) & 0xcc);
                        }
                    }
                    let gi2 = usize::try_from(g.i2).unwrap();
                    if gi2 >= input_len && usage[gi2 - input_len] < 2 {
                        if self.reduce_neg_from_lop3_input(gi2, &usage) {
                            f = ((f >> 1) & 0x55) | ((f << 1) & 0xaa);
                        }
                    }
                    self.gates[i].func = VLOP3GateFunc::LOP3(f);
                }
            }
        }
        // collect outputs by wire indices
        let mut node_outputs_negs = HashMap::<T, (Vec<usize>, Vec<usize>)>::new();
        for (i, (o, n)) in self.outputs.iter().enumerate() {
            if *o < self.input_len {
                continue;
            }
            if let Some((pos, negs)) = node_outputs_negs.get_mut(o) {
                if *n {
                    negs.push(i);
                } else {
                    pos.push(i);
                }
            } else {
                node_outputs_negs.insert(
                    *o,
                    if *n {
                        (vec![], vec![i])
                    } else {
                        (vec![i], vec![])
                    },
                );
            }
        }
        // optimize negs in outputs
        for (o, (pos, negs)) in node_outputs_negs {
            let o = usize::try_from(o).unwrap();
            if usage_by_gates[o - input_len] != 0 {
                // because it changes result on other gates!
                continue;
            }
            if let VLOP3GateFunc::LOP3(f) = self.gates[o - input_len].func {
                // if positive outputs is less than negative outputs
                if pos.len() < negs.len() {
                    // reduce negation: negate LOP3
                    self.gates[o - input_len].func = VLOP3GateFunc::LOP3(!f);
                    // negate outputs
                    for p in pos.iter().chain(negs.iter()) {
                        self.outputs[*p].1 = !self.outputs[*p].1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_subtree_coverage_from_circuit(circuit: Circuit<u32>) -> Vec<u32> {
        let binop_circuit = VBinOpCircuit::from(circuit.clone());
        let subtrees = binop_circuit.subtrees();
        gen_subtree_coverage(&binop_circuit, &subtrees)
    }

    #[test]
    fn test_gen_subtree_coverage() {
        assert_eq!(
            vec![0, 1, 0, 1, 0, 1, 2, 3, 4, 5, 5, 5, 5, 6, 5, 6, 6, 6, 6],
            gen_subtree_coverage_from_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1),   // 4
                        Gate::new_and(0, 3),     // 5
                        Gate::new_xor(1, 4),     // 6
                        Gate::new_and(3, 5),     // 7
                        Gate::new_xor(2, 6),     // 8
                        Gate::new_xor(3, 7),     // 9
                        Gate::new_nor(8, 9),     // 10
                        Gate::new_and(8, 9),     // 11
                        Gate::new_nimpl(8, 9),   // 12
                        Gate::new_nor(0, 10),    // 13
                        Gate::new_nor(1, 11),    // 14
                        Gate::new_xor(2, 12),    // 15
                        Gate::new_xor(13, 14),   // 16
                        Gate::new_and(0, 10),    // 17 tree4
                        Gate::new_nor(15, 16),   // 18 tree3
                        Gate::new_nimpl(1, 12),  // 19 tree4
                        Gate::new_nimpl(11, 17), // 20
                        Gate::new_nimpl(3, 19),  // 21
                        Gate::new_xor(20, 21),   // 22
                    ],
                    [(18, true), (22, false)],
                )
                .unwrap()
            )
        );

        assert_eq!(
            vec![0, 0, 1, 1, 1, 2, 2, 2],
            gen_subtree_coverage_from_circuit(
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
        );
    }
}

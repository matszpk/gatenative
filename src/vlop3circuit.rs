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
    And,
    Or,
    Xor,
    LOP3(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct VLOP3Gate<T: Clone + Copy> {
    pub(crate) i0: T,
    pub(crate) i1: T,
    pub(crate) i2: T,
    pub(crate) func: VLOP3GateFunc,
    pub(crate) negs: VNegs,
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

    // return true if operation done for all LOP3s as nodes, return false if done nothing
    fn negate_lop3_arg_except(&mut self, arg: T, successors: &[T], except: Option<T>) -> bool {
        // skip second successors because except2==first_gate and successors2=[first_gate]
        self.negate_lop3_arg_except2(
            arg,
            arg,
            successors,
            &[self.input_len],
            except,
            Some(self.input_len),
        )
    }

    // negate except including two various changes
    fn negate_lop3_arg_except2(
        &mut self,
        arg: T,
        arg2: T,
        successors: &[T],
        successors2: &[T],
        except: Option<T>,
        except2: Option<T>,
    ) -> bool {
        let input_len = usize::try_from(self.input_len).unwrap();
        let sets = [(successors, arg, except), (successors2, arg2, except2)];
        // do if all is LOP3s except excepted node and if successor is not empty
        if sets.iter().all(|(successors, arg, except)| {
            !successors.is_empty()
                && successors.iter().all(|x| {
                    let xu = usize::try_from(*x).unwrap();
                    if matches!(self.gates[xu - input_len].func, VLOP3GateFunc::LOP3(_)) {
                        true
                    } else if let Some(except) = except {
                        *except == *x
                    } else {
                        false
                    }
                })
        }) {
            for (successors, arg, except) in sets {
                for t in successors {
                    if except.map(|x| x == *t).unwrap_or(false) {
                        // skip except
                        continue;
                    }
                    let t = usize::try_from(*t).unwrap();
                    if let VLOP3GateFunc::LOP3(f) = self.gates[t - input_len].func {
                        let mut f = f;
                        if self.gates[t - input_len].i0 == arg {
                            f = (f << 4) | (f >> 4);
                        }
                        if self.gates[t - input_len].i1 == arg {
                            f = ((f << 2) & 0xcc) | ((f >> 2) & 0x33);
                        }
                        if self.gates[t - input_len].i2 == arg {
                            f = ((f << 1) & 0xaa) | ((f >> 1) & 0x55);
                        }
                        self.gates[t - input_len].func = VLOP3GateFunc::LOP3(f);
                    } else {
                        panic!("Unexpected!");
                    }
                }
            }
            true
        } else {
            false
        }
    }

    // return true if negation of argument is needed
    fn reduce_neg_from_lop3_input(&mut self, gi0: usize, successors: &[Vec<T>]) {
        let input_len = usize::try_from(self.input_len).unwrap();
        let g0 = self.gates[gi0 - input_len];
        let g0n = T::try_from(gi0).unwrap();
        match g0.func {
            VLOP3GateFunc::And | VLOP3GateFunc::Or => {
                let is_and = matches!(g0.func, VLOP3GateFunc::And);
                if g0.negs == NegOutput {
                    if self.negate_lop3_arg_except(g0n, &successors[gi0 - input_len], None) {
                        self.gates[gi0 - input_len].negs = NoNegs;
                    }
                } else if g0.negs == NegInput1 {
                    // check LOP3(and(LOP3,!v1)) and convert to: LOP3(!or(LOP3_neg,v1))
                    let g0i0 = usize::try_from(g0.i0).unwrap();
                    if g0i0 >= input_len {
                        let g00 = self.gates[g0i0 - input_len];
                        if let VLOP3GateFunc::LOP3(f) = g00.func {
                            if self.negate_lop3_arg_except2(
                                g0.i0,
                                g0n,
                                &successors[g0i0 - input_len],
                                &successors[gi0 - input_len],
                                Some(g0n),
                                None,
                            ) {
                                self.gates[gi0 - input_len].func = if is_and {
                                    VLOP3GateFunc::Or
                                } else {
                                    VLOP3GateFunc::And
                                };
                                self.gates[gi0 - input_len].negs = NoNegs;
                                self.gates[g0i0 - input_len].func = VLOP3GateFunc::LOP3(!f);
                            }
                        }
                    }
                }
            }
            VLOP3GateFunc::Xor => {
                if g0.negs == NegOutput || g0.negs == NegInput1 {
                    if self.negate_lop3_arg_except(g0n, &successors[gi0 - input_len], None) {
                        self.gates[gi0 - input_len].negs = NoNegs;
                    }
                }
            }
            _ => (),
        }
    }

    fn successors_and_usage(&self) -> (Vec<Vec<T>>, Vec<u8>) {
        let input_len = usize::try_from(self.input_len).unwrap();
        // calculate usage to avoids multiple usages
        // successors to negate its arguments: only for LOP3s
        // info: entry is empty list if change of this node is prohibited (no all LOP3s)
        let mut successors = vec![vec![]; self.gates.len()];
        let mut usage_by_gates = vec![0u8; self.gates.len()];
        for (i, g) in self.gates.iter().enumerate() {
            if g.i0 >= self.input_len {
                let i0 = usize::try_from(g.i0).unwrap() - input_len;
                successors[i0].push(T::try_from(i + input_len).unwrap());
                if usage_by_gates[i0] < 2 {
                    usage_by_gates[i0] += 1;
                }
            }
            if g.i1 >= self.input_len {
                let i1 = usize::try_from(g.i1).unwrap() - input_len;
                successors[i1].push(T::try_from(i + input_len).unwrap());
                if usage_by_gates[i1] < 2 {
                    usage_by_gates[i1] += 1;
                }
            }
            if matches!(g.func, VLOP3GateFunc::LOP3(_)) {
                if g.i2 >= self.input_len {
                    let i2 = usize::try_from(g.i2).unwrap() - input_len;
                    successors[i2].push(T::try_from(i + input_len).unwrap());
                    if usage_by_gates[i2] < 2 {
                        usage_by_gates[i2] += 1;
                    }
                }
            }
        }
        for (o, _) in self.outputs.iter() {
            if *o >= self.input_len {
                let o = usize::try_from(*o).unwrap() - input_len;
                // clear usage_by_gates because used by circuit outputs
                successors[o].clear();
            }
        }
        // sort and dedup successors
        for succ in &mut successors {
            succ.sort();
            succ.dedup();
        }
        (successors, usage_by_gates)
    }

    // optimize negations in 2-input gates that neighbors with LOP3 gates.
    fn optimize_negs(&mut self) {
        let (successors, usage_by_gates) = self.successors_and_usage();
        // optimize negations
        let input_len = usize::try_from(self.input_len).unwrap();
        for i in 0..self.gates.len() {
            let gi = T::try_from(i + input_len).unwrap();
            let g = self.gates[i];
            // apply optimizations rules
            match g.func {
                VLOP3GateFunc::And | VLOP3GateFunc::Or | VLOP3GateFunc::Xor => {
                    if g.negs == NegInput1 {
                        let gi1 = usize::try_from(g.i1).unwrap();
                        if gi1 >= input_len && !successors[gi1 - input_len].is_empty() {
                            let g1 = self.gates[gi1 - input_len];
                            if let VLOP3GateFunc::LOP3(f) = g1.func {
                                // negate lop3 and remove negation from second input
                                if self.negate_lop3_arg_except(
                                    g.i1,
                                    &successors[gi1 - input_len],
                                    Some(gi),
                                ) {
                                    self.gates[gi1 - input_len].func = VLOP3GateFunc::LOP3(!f);
                                    self.gates[i].negs = NoNegs;
                                }
                            }
                        }
                    }
                }
                VLOP3GateFunc::LOP3(f) => {
                    let gi0 = usize::try_from(g.i0).unwrap();
                    if gi0 >= input_len {
                        self.reduce_neg_from_lop3_input(gi0, &successors);
                    }
                    let gi1 = usize::try_from(g.i1).unwrap();
                    // compare with previous args to avoid double negations
                    if gi0 != gi1 && gi1 >= input_len {
                        self.reduce_neg_from_lop3_input(gi1, &successors);
                    }
                    let gi2 = usize::try_from(g.i2).unwrap();
                    // compare with previous args to avoid double negations
                    if gi0 != gi2 && gi1 != gi2 && gi2 >= input_len {
                        self.reduce_neg_from_lop3_input(gi2, &successors);
                    }
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

    #[test]
    fn test_vlop3circuit_negate_lop3_arg_except() {
        for (ci, (circuit, testcases)) in [
            // circuit 0
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_lop3(1, 2, 3, 0b00010010),
                        vgate_lop3(2, 3, 4, 0b10111101),
                    ],
                    outputs: vec![(5, false)],
                },
                vec![
                    (
                        2,
                        &[5][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00010010),
                                vgate_lop3(2, 3, 4, 0b11011011),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        3,
                        &[5][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00010010),
                                vgate_lop3(2, 3, 4, 0b11100111),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        4,
                        &[5][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00010010),
                                vgate_lop3(2, 3, 4, 0b01111110),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        1,
                        &[4][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00100001),
                                vgate_lop3(2, 3, 4, 0b10111101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        2,
                        &[4][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b01001000),
                                vgate_lop3(2, 3, 4, 0b10111101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        3,
                        &[4][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00100001),
                                vgate_lop3(2, 3, 4, 0b10111101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        2,
                        &[4][..],
                        Some(4),
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110),
                                vgate_lop3(1, 2, 3, 0b00010010), // ignored
                                vgate_lop3(2, 3, 4, 0b10111101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                ],
            ),
            // circuit 1 - more argument combinations
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 0, 1, 0b11011001),
                        vgate_lop3(2, 3, 3, 0b11001011),
                        vgate_lop3(4, 3, 4, 0b01101101),
                    ],
                    outputs: vec![(5, false)],
                },
                vec![
                    (
                        0,
                        &[3][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 0, 1, 0b01100111),
                                vgate_lop3(2, 3, 3, 0b11001011),
                                vgate_lop3(4, 3, 4, 0b01101101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        1,
                        &[3][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 0, 1, 0b11100110),
                                vgate_lop3(2, 3, 3, 0b11001011),
                                vgate_lop3(4, 3, 4, 0b01101101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        3,
                        &[4][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 0, 1, 0b11011001),
                                vgate_lop3(2, 3, 3, 0b00111101),
                                vgate_lop3(4, 3, 4, 0b01101101),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                    (
                        4,
                        &[5][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 0, 1, 0b11011001),
                                vgate_lop3(2, 3, 3, 0b11001011),
                                vgate_lop3(4, 3, 4, 0b11101001),
                            ],
                            outputs: vec![(5, false)],
                        },
                    ),
                ],
            ),
            // circuit 2 - do or nothing
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110), // 3
                        vgate_lop3(1, 2, 3, 0b00010010), // 4
                        vgate_lop3(2, 3, 4, 0b10111101), // 5
                        vgate_and(0, 4, NegOutput),      // 6
                        vgate_lop3(0, 2, 5, 0b10010110), // 7
                        vgate_lop3(0, 7, 5, 0b00101000), // 8
                    ],
                    outputs: vec![(6, false), (7, true), (8, false)],
                },
                vec![
                    (
                        3,
                        &[4, 5][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00100001), // 4 changed
                                vgate_lop3(2, 3, 4, 0b11100111), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        3,
                        &[4, 5][..],
                        Some(4),
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4 skipped
                                vgate_lop3(2, 3, 4, 0b11100111), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        4,
                        &[5, 6][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        4,
                        &[5, 6][..],
                        Some(5), // skip LOP3 - do nothing
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        4,
                        &[5, 6][..],
                        Some(6), // skp and - do
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b01111110), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        5,
                        &[7, 8][..],
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b01101001), // 7 changed
                                vgate_lop3(0, 7, 5, 0b00010100), // 8 changed
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        5,
                        &[7, 8][..],
                        Some(8),
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b01101001), // 7 changed
                                vgate_lop3(0, 7, 5, 0b00101000), // 8 skipped
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                ],
            ),
        ]
        .into_iter()
        .enumerate()
        {
            for (ti, (arg, succs, except, expres)) in testcases.into_iter().enumerate() {
                let mut circuit = circuit.clone();
                println!("Testcase {} {}", ci, ti);
                circuit.negate_lop3_arg_except(arg, succs, except);
                assert_eq!(expres, circuit, "{} {}", ci, ti);
            }
        }
    }

    #[test]
    fn test_vlop3circuit_negate_lop3_arg_except2() {
        for (ci, (circuit, testcases)) in [
            // circuit 0
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110), // 3
                        vgate_lop3(1, 2, 3, 0b00010010), // 4
                        vgate_lop3(2, 3, 4, 0b10111101), // 5
                        vgate_and(0, 4, NegOutput),      // 6
                        vgate_lop3(0, 2, 5, 0b10010110), // 7
                        vgate_lop3(0, 7, 5, 0b00101000), // 8
                    ],
                    outputs: vec![(6, false), (7, true), (8, false)],
                },
                vec![
                    (
                        2,
                        3,
                        &[3, 4, 5][..],
                        &[4, 5][..],
                        None,
                        None,
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11011001), // 3 changed
                                vgate_lop3(1, 2, 3, 0b10000100), // 4 changed
                                vgate_lop3(2, 3, 4, 0b01111110), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        2,
                        3,
                        &[3, 4, 5][..],
                        &[4, 5][..],
                        Some(5),
                        Some(4),
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11011001), // 3 changed
                                vgate_lop3(1, 2, 3, 0b01001000), // 4 changed
                                vgate_lop3(2, 3, 4, 0b11100111), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        3,
                        4,
                        &[4, 5][..],
                        &[5, 6][..],
                        None,
                        None,
                        // no changes
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        4,
                        3,
                        &[5, 6][..],
                        &[4, 5][..],
                        None,
                        None,
                        // no changes
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        3,
                        4,
                        &[4, 5][..],
                        &[5, 6][..],
                        Some(5),
                        None,
                        // no changes
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00010010), // 4
                                vgate_lop3(2, 3, 4, 0b10111101), // 5
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                    (
                        3,
                        4,
                        &[4, 5][..],
                        &[5, 6][..],
                        None,
                        Some(6),
                        VLOP3Circuit {
                            input_len: 3,
                            gates: vec![
                                vgate_lop3(0, 1, 2, 0b11100110), // 3
                                vgate_lop3(1, 2, 3, 0b00100001), // 4 changed
                                vgate_lop3(2, 3, 4, 0b11011011), // 5 changed
                                vgate_and(0, 4, NegOutput),      // 6
                                vgate_lop3(0, 2, 5, 0b10010110), // 7
                                vgate_lop3(0, 7, 5, 0b00101000), // 8
                            ],
                            outputs: vec![(6, false), (7, true), (8, false)],
                        },
                    ),
                ],
            ),
        ]
        .into_iter()
        .enumerate()
        {
            for (ti, (arg, arg2, succs, succs2, except, except2, expres)) in
                testcases.into_iter().enumerate()
            {
                let mut circuit = circuit.clone();
                println!("Testcase {} {}", ci, ti);
                circuit.negate_lop3_arg_except2(arg, arg2, succs, succs2, except, except2);
                assert_eq!(expres, circuit, "{} {}", ci, ti);
            }
        }
    }

    #[test]
    fn test_vlop3circuit_successors_and_usage_by_gates() {
        assert_eq!(
            (
                vec![vec![4, 5], vec![5, 6], vec![7, 8], vec![], vec![], vec![]],
                vec![2, 2, 2, 0, 1, 0]
            ),
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, 0b11100110), // 3
                    vgate_lop3(1, 2, 3, 0b00010010), // 4
                    vgate_lop3(2, 3, 4, 0b10111101), // 5
                    vgate_and(0, 4, NegOutput),      // 6
                    vgate_lop3(0, 2, 5, 0b10010110), // 7
                    vgate_lop3(0, 7, 5, 0b00101000), // 8
                ],
                outputs: vec![(6, false), (7, true), (8, false)],
            }
            .successors_and_usage()
        );
        assert_eq!(
            (
                vec![
                    vec![4, 5, 6],
                    vec![5, 6, 7],
                    vec![],
                    vec![],
                    vec![8],
                    vec![]
                ],
                vec![2, 2, 2, 0, 1, 0]
            ),
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, 0b11100110), // 3
                    vgate_lop3(1, 2, 3, 0b00010010), // 4
                    vgate_lop3(2, 3, 4, 0b10111101), // 5
                    vgate_or(3, 4, NegOutput),       // 6
                    vgate_lop3(4, 2, 5, 0b10010110), // 7
                    vgate_xor(5, 7, NoNegs),         // 8
                ],
                outputs: vec![(6, false), (5, true), (8, false)],
            }
            .successors_and_usage()
        );
    }

    #[test]
    fn test_vlop3circuit_reduce_neg_from_lop3_input() {
        for (ci, (circuit, testcases)) in [
            // circuit 0
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b01111110),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 1
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegInput1),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b01111110),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 2
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NoNegs),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b10111101),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 3
            (
                // no change because some successors are not LOP3
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(4, 5, NoNegs),
                        vgate_and(1, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NegOutput),
                            vgate_lop3(2, 3, 4, 0b10111101),
                            vgate_and(4, 5, NoNegs),
                            vgate_and(1, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // circuit 4
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_lop3(0, 4, 5, 0b00100110),
                        vgate_and(1, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b01111110),
                            vgate_lop3(0, 4, 5, 0b10001001),
                            vgate_and(1, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // circuit 5
            (
                // no change because xor in output
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(4, false), (6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_xor(2, 3, NegOutput),
                            vgate_lop3(2, 3, 4, 0b10111101),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(4, false), (6, false)],
                    },
                )],
            ),
            // and gate to change
            // circuit 6
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_lop3(0, 4, 5, 0b00100110),
                        vgate_and(1, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_and(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b01111110),
                            vgate_lop3(0, 4, 5, 0b10001001),
                            vgate_and(1, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // circuit 7
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_or(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_lop3(0, 4, 5, 0b00100110),
                        vgate_and(1, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_or(2, 3, NoNegs),
                            vgate_lop3(2, 3, 4, 0b01111110),
                            vgate_lop3(0, 4, 5, 0b10001001),
                            vgate_and(1, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // two negations on gate
            // circuit 8
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(3, 2, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b00011001),
                            vgate_or(3, 2, NoNegs),
                            vgate_lop3(1, 2, 4, 0b01111110),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 9
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_or(3, 2, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b00011001),
                            vgate_and(3, 2, NoNegs),
                            vgate_lop3(1, 2, 4, 0b01111110),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 10
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(2, 3, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    // no changes because connected to negated input1
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_and(2, 3, NegInput1),
                            vgate_lop3(1, 2, 4, 0b10111101),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 11
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(3, 2, NegInput1),
                        vgate_lop3(1, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                vec![(
                    4,
                    // changes 1 and 2 argument in gate 5 (LOP3).
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b00011001),
                            vgate_or(3, 2, NoNegs),
                            vgate_lop3(1, 3, 4, 0b11011011),
                            vgate_and(1, 5, NoNegs),
                        ],
                        outputs: vec![(6, false)],
                    },
                )],
            ),
            // circuit 12
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(3, 2, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_or(1, 3, NoNegs),
                        vgate_and(5, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    // no changes because one of successor is not LOP3
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_and(3, 2, NegInput1),
                            vgate_lop3(1, 2, 4, 0b10111101),
                            vgate_or(1, 3, NoNegs),
                            vgate_and(5, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // circuit 13
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(3, 2, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_or(1, 4, NoNegs),
                        vgate_and(5, 6, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    // no changes because one of successor is not LOP3
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b11100110),
                            vgate_and(3, 2, NegInput1),
                            vgate_lop3(1, 2, 4, 0b10111101),
                            vgate_or(1, 4, NoNegs),
                            vgate_and(5, 6, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
            // circuit 14
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_and(3, 2, NegInput1),
                        vgate_lop3(1, 2, 4, 0b10111101),
                        vgate_lop3(1, 2, 3, 0b10000110),
                        vgate_lop3(4, 2, 6, 0b01001000),
                        vgate_and(5, 7, NoNegs),
                    ],
                    outputs: vec![(7, false)],
                },
                vec![(
                    4,
                    VLOP3Circuit {
                        input_len: 3,
                        gates: vec![
                            vgate_lop3(0, 1, 2, 0b00011001),
                            vgate_or(3, 2, NoNegs),
                            vgate_lop3(1, 2, 4, 0b01111110),
                            vgate_lop3(1, 2, 3, 0b01001001),
                            vgate_lop3(4, 2, 6, 0b10000100),
                            vgate_and(5, 7, NoNegs),
                        ],
                        outputs: vec![(7, false)],
                    },
                )],
            ),
        ]
        .into_iter()
        .enumerate()
        {
            for (ti, (lop3, expres)) in testcases.into_iter().enumerate() {
                let mut circuit = circuit.clone();
                println!("Testcase {} {}", ci, ti);
                let successors = circuit.successors_and_usage().0;
                circuit.reduce_neg_from_lop3_input(lop3, &successors);
                assert_eq!(expres, circuit, "{} {}", ci, ti);
            }
        }
    }

    #[test]
    fn test_vlop3circuit_optimize_negs() {
        for (ci, (circuit, expres)) in [
            // circuit 0
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                    ],
                    outputs: vec![(6, false), (7, false), (8, false)],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b11100010),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NoNegs),
                        vgate_xor(2, 5, NoNegs),
                    ],
                    outputs: vec![(6, false), (7, false), (8, false)],
                },
            ),
            // circuit 1
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_lop3(3, 1, 2, 0b00011001),
                        vgate_lop3(4, 1, 2, 0b11010001),
                        vgate_lop3(5, 1, 2, 0b00110111),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_and(0, 6, NoNegs),
                        vgate_or(1, 7, NoNegs),
                        vgate_xor(2, 8, NoNegs),
                    ],
                    outputs: vec![
                        (9, false),
                        (10, false),
                        (11, false),
                        (12, false),
                        (13, false),
                        (14, false),
                    ],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b11100010),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_lop3(3, 1, 2, 0b10010001),
                        vgate_lop3(4, 1, 2, 0b00011101),
                        vgate_lop3(5, 1, 2, 0b01110011),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NoNegs),
                        vgate_xor(2, 5, NoNegs),
                        vgate_and(0, 6, NoNegs),
                        vgate_or(1, 7, NoNegs),
                        vgate_xor(2, 8, NoNegs),
                    ],
                    outputs: vec![
                        (9, false),
                        (10, false),
                        (11, false),
                        (12, false),
                        (13, false),
                        (14, false),
                    ],
                },
            ),
            // circuit 2
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_or(3, 1, NoNegs),
                        vgate_or(4, 1, NoNegs),
                        vgate_xor(5, 1, NoNegs),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_and(0, 6, NoNegs),
                        vgate_or(1, 7, NoNegs),
                        vgate_xor(2, 8, NoNegs),
                    ],
                    outputs: vec![
                        (9, false),
                        (10, false),
                        (11, false),
                        (12, false),
                        (13, false),
                        (14, false),
                    ],
                },
                // no changes because changes normal gates
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_or(3, 1, NoNegs),
                        vgate_or(4, 1, NoNegs),
                        vgate_xor(5, 1, NoNegs),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_and(0, 6, NoNegs),
                        vgate_or(1, 7, NoNegs),
                        vgate_xor(2, 8, NoNegs),
                    ],
                    outputs: vec![
                        (9, false),
                        (10, false),
                        (11, false),
                        (12, false),
                        (13, false),
                        (14, false),
                    ],
                },
            ),
            // circuit 3
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(2, 3, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NoNegs),
                        vgate_lop3(2, 3, 4, 0b01111110),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
            ),
            // optimize negs at circuit outputs
            // circuit 4
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_lop3(6, 7, 8, 0b11100010),
                        vgate_lop3(6, 7, 8, 0b00100110),
                    ],
                    outputs: vec![(6, true), (7, true), (8, true), (9, true), (10, true)],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b11100010),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NoNegs),
                        vgate_xor(2, 5, NoNegs),
                        vgate_lop3(6, 7, 8, 0b00011101),
                        vgate_lop3(6, 7, 8, 0b11011001),
                    ],
                    outputs: vec![(6, true), (7, true), (8, true), (9, false), (10, false)],
                },
            ),
            // circuit 5
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_lop3(6, 7, 8, 0b11100010),
                        vgate_lop3(6, 7, 8, 0b00100110),
                    ],
                    outputs: vec![
                        (6, true),
                        (7, true),
                        (8, true),
                        (9, true),
                        (9, false),
                        (10, true),
                        (10, true),
                        (10, false),
                    ],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b11100010),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NoNegs),
                        vgate_xor(2, 5, NoNegs),
                        vgate_lop3(6, 7, 8, 0b11100010),
                        vgate_lop3(6, 7, 8, 0b11011001),
                    ],
                    outputs: vec![
                        (6, true),
                        (7, true),
                        (8, true),
                        (9, true),
                        (9, false),
                        (10, false),
                        (10, false),
                        (10, true),
                    ],
                },
            ),
            // circuit 6
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                        vgate_lop3(6, 7, 8, 0b11100010),
                        vgate_lop3(6, 7, 8, 0b00100110),
                        vgate_and(2, 10, NoNegs),
                    ],
                    outputs: vec![
                        (6, true),
                        (7, true),
                        (8, true),
                        (9, true),
                        (10, true),
                        (11, true),
                    ],
                },
                // gate 10 - not changed because used by other gate
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b11100010),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NoNegs),
                        vgate_xor(2, 5, NoNegs),
                        vgate_lop3(6, 7, 8, 0b00011101),
                        vgate_lop3(6, 7, 8, 0b00100110), // not changed
                        vgate_and(2, 10, NoNegs),
                    ],
                    outputs: vec![
                        (6, true),
                        (7, true),
                        (8, true),
                        (9, false),
                        (10, true),
                        (11, true),
                    ],
                },
            ),
            // circuit 7 - circuit 0 with blocked some change at 4 gate
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b10010001),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b11011000),
                        vgate_and(0, 3, NegInput1),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NegInput1),
                    ],
                    outputs: vec![(6, false), (7, false), (8, false), (4, false)],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b01101110),
                        vgate_lop3(0, 1, 2, 0b00011101),
                        vgate_lop3(0, 1, 2, 0b00100111),
                        vgate_and(0, 3, NoNegs),
                        vgate_or(1, 4, NegInput1),
                        vgate_xor(2, 5, NoNegs),
                    ],
                    outputs: vec![(6, false), (7, false), (8, false), (4, false)],
                },
            ),
            // other test with reduce_neg_from_lop3_input
            // circuit 8
            (
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NegOutput),
                        vgate_lop3(3, 4, 4, 0b10111101),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
                VLOP3Circuit {
                    input_len: 3,
                    gates: vec![
                        vgate_lop3(0, 1, 2, 0b11100110),
                        vgate_xor(2, 3, NoNegs),
                        vgate_lop3(3, 4, 4, 0b11011011),
                        vgate_and(1, 5, NoNegs),
                    ],
                    outputs: vec![(6, false)],
                },
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let mut circuit = circuit.clone();
            circuit.optimize_negs();
            assert_eq!(expres, circuit, "{}", ci);
        }
    }
}

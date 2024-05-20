use gatesim::*;

use std::collections::HashSet;
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

fn mtu_area_view_calc_costs<T>(mtuaview: &MTUArea<T>) -> (Vec<(T, LOP3SubTreePaths)>, usize) {
    (vec![], 0)
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
                let preferred_nodes = vec![];
                lop3nodes[gidx] = find_best_lop3node(&circuit, &lop3nodes, nidx, &preferred_nodes);
                update_mtuareas_from_lop3node(&mut mtuareas, &circuit, &subtrees, &lop3nodes[gidx]);
            }
        }
        // filter lop3nodes
        // convert inputs in lop3nodes
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }
}

use gatesim::*;

use std::fmt::Debug;

use crate::vcircuit::*;
use crate::VNegs::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum VLop3GateFunc {
    And,
    Or,
    Nimpl,
    Xor,
    Lop3(u8),
}

impl TryFrom<VGateFunc> for VLop3GateFunc {
    type Error = String;
    #[inline]
    fn try_from(gf: VGateFunc) -> Result<Self, Self::Error> {
        match gf {
            VGateFunc::And => Ok(VLop3GateFunc::And),
            VGateFunc::Or => Ok(VLop3GateFunc::Or),
            VGateFunc::Nimpl => Ok(VLop3GateFunc::Nimpl),
            VGateFunc::Xor => Ok(VLop3GateFunc::Xor),
            _ => Err("Unsupported!".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct VLop3Gate<T: Clone + Copy> {
    pub(crate) i0: T,
    pub(crate) i1: T,
    pub(crate) i2: T,
    pub(crate) func: VLop3GateFunc,
}

impl<T: Clone + Copy + Default> TryFrom<VGate<T>> for VLop3Gate<T> {
    type Error = String;
    fn try_from(g: VGate<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            i0: g.i0,
            i1: g.i1,
            i2: T::default(),
            func: VLop3GateFunc::try_from(g.func)?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct VLop3Circuit<T: Clone + Copy> {
    pub(crate) input_len: T,
    pub(crate) gates: Vec<VLop3Gate<T>>,
    pub(crate) outputs: Vec<(T, bool)>,
}

// IDEA:
// Use conversion to clauses to find literal duplicates and collect into LOP3.
// With clauses it possible to better choosing other clauses to collect into LOP3.
// PREFERRED: Simpler: just use gates and short-trees to optimize to LOP3.

struct SubTree {
    nodes: [Option<VGate<usize>>; 31],
    extra_cond_node_indices: [Option<usize>; 16],
}

struct Lop3SubTree {
    nodes: [Option<(VLop3Gate<usize>, usize)>; 31],
    orig_indices: [Option<usize>; 31],
}

impl SubTree {
    fn optimize(self) -> Lop3SubTree {
        Lop3SubTree {
            nodes: [None; 31],
            orig_indices: [None; 31],
        }
    }
}

#[derive(Clone, Copy)]
struct Lop3NodeVariant<T> {
    orig_nodes: [T; 3],
    lop3: u8,
    total_cost: usize,
    shared_extra_cost: usize,
    next_variant: Option<usize>,
}

// var_usage - table of var usage:
// entry: (count_to_2, circuit_output):
// count_to_2 - 0, 1 or 2 or more usages
// circuit_output - if used by circuit's output

fn get_lop3_best_variants<T>(
    node_variants: &[Lop3NodeVariant<T>],
    var_usage: &[(u8, bool)],
    node: T,
    boundaries: Option<&[T]>,
) -> Vec<Lop3NodeVariant<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    vec![]
}

fn calc_length_in_lop3s<T>(root: T, children: &[T]) -> usize
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    0
}

impl<T> From<Circuit<T>> for VLop3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: Circuit<T>) -> Self {
        Self::from(VCircuit::to_op_and_ximpl_circuit(circuit, true))
    }
}

impl<T> From<VCircuit<T>> for VLop3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: VCircuit<T>) -> Self {
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }
}

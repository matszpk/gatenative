use gatesim::*;

use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::rc::{Rc, Weak};

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

#[derive(Clone)]
struct MTUAreaView<T> {
    node: T, // MTU node
    touch_nodes: RefCell<Vec<Rc<GraphTouchNode<T>>>>,
    nodes_in_mtu: RefCell<Vec<T>>,
    extra_cost: Cell<usize>,
}

#[derive(Clone)]
struct GraphTouchNode<T> {
    node: T, // touch node
    children: RefCell<Vec<Rc<GraphTouchNode<T>>>>,
    parents: RefCell<Vec<Weak<GraphTouchNode<T>>>>,
    disjoint_cost: Cell<usize>,
    total_cost: Cell<usize>,
    mtu_views: RefCell<Vec<Rc<MTUAreaView<T>>>>,
}

#[derive(Clone)]
struct MTUView<T> {
    touch_nodes: RefCell<Vec<Rc<GraphTouchNode<T>>>>,
    mtu_views: RefCell<Vec<Rc<MTUAreaView<T>>>>,
}

impl<T> MTUView<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn new(vcircuit: &VCircuit<T>, mtu_view: Rc<MTUView<T>>, node: T) -> Option<Rc<MTUView<T>>> {
        None
    }

    // update current mtuview with data from new_mtuview
    // fn update_current(self: Rc<MTUView<T>>, new_mtu_view: Rc<MTUView<T>>) -> Rc<MTUView<T>> {
    //     None
    // }

    // join parent mtuview with children mtuview
    // fn join_to_parent(self: Rc<MTUView<T>>, child_mtu_view: Rc<MTUView<T>>) -> Rc<MTUView<T>> {
    //     None
    // }
}

#[derive(Clone)]
struct LOP3Node<T> {
    node: T,                  // node in original circuit graph
    mtu_view: Rc<MTUView<T>>, // by default it can be empty MTUView
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

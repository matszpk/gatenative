use gatesim::*;

use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::rc::{Rc, Weak};

use crate::vcircuit::*;
use crate::VNegs::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum VLOP3GateFunc {
    And,
    Or,
    Nimpl,
    Xor,
    LOP3(u8),
}

impl TryFrom<VGateFunc> for VLOP3GateFunc {
    type Error = String;
    #[inline]
    fn try_from(gf: VGateFunc) -> Result<Self, Self::Error> {
        match gf {
            VGateFunc::And => Ok(VLOP3GateFunc::And),
            VGateFunc::Or => Ok(VLOP3GateFunc::Or),
            VGateFunc::Nimpl => Ok(VLOP3GateFunc::Nimpl),
            VGateFunc::Xor => Ok(VLOP3GateFunc::Xor),
            _ => Err("Unsupported!".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct VLOP3Gate<T: Clone + Copy> {
    pub(crate) i0: T,
    pub(crate) i1: T,
    pub(crate) i2: T,
    pub(crate) func: VLOP3GateFunc,
}

impl<T: Clone + Copy + Default> TryFrom<VGate<T>> for VLOP3Gate<T> {
    type Error = String;
    fn try_from(g: VGate<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            i0: g.i0,
            i1: g.i1,
            i2: T::default(),
            func: VLOP3GateFunc::try_from(g.func)?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct VLOP3Circuit<T: Clone + Copy> {
    pub(crate) input_len: T,
    pub(crate) gates: Vec<VLOP3Gate<T>>,
    pub(crate) outputs: Vec<(T, bool)>,
}

#[derive(Clone)]
struct MTUAreaView<T> {
    node: T, // MTU node
    touch_nodes: RefCell<Vec<Weak<TouchNode<T>>>>,
    nodes_in_mtu: RefCell<Vec<T>>,
    extra_cost: Cell<usize>,
}

#[derive(Clone)]
struct TouchNode<T> {
    node: T, // touch node
    mtu_cost: Cell<usize>,
    mtu_views: RefCell<Vec<Weak<MTUAreaView<T>>>>,
}

impl<T> MTUAreaView<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // and improve - fix other TouchNodes to make better result if possible

    // update current mtuview with data from new_mtuview
    // fn update_current(
    //     self: Rc<MTUAreaView<T>>,
    //     new_mtu_view: Rc<MTUAreaView<T>>,
    // ) -> Rc<RefCell<MTUView<T>>> {
    //     Rc::new(RefCell::new(MTUView {
    //         touch_nodes: vec![],
    //         mtu_views: vec![],
    //     }))
    // }
}

#[derive(Clone)]
struct MTUView<T> {
    touch_nodes: Vec<Rc<TouchNode<T>>>,
    mtu_views: Vec<Rc<MTUAreaView<T>>>,
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
    fn update_current(
        self: Rc<MTUView<T>>,
        new_mtu_view: Rc<MTUView<T>>,
    ) -> Rc<RefCell<MTUView<T>>> {
        Rc::new(RefCell::new(MTUView {
            touch_nodes: vec![],
            mtu_views: vec![],
        }))
    }
}

#[derive(Clone)]
struct LOP3Boundary<T> {
    boundary_levels: [u8; 8], // boundary levels
    // boundaries from left to right (first to last argument)
    boundaries: [T; 8], // boundaries are parents of arguments
    boundary_len: u8,   // boundary length
}

#[derive(Clone)]
struct LOP3Node<T> {
    node: T,                          // node in original circuit graph
    args: [T; 3],                     // arguments, also leaves of LOP3 subtree
    boundary: LOP3Boundary<T>,        // LOP3 subtree boundary
    mtu_view: Option<Rc<MTUView<T>>>, // by default it can be empty MTUView
}

impl<T> From<Circuit<T>> for VLOP3Circuit<T>
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

impl<T> From<VCircuit<T>> for VLOP3Circuit<T>
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

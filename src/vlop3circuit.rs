use gatesim::*;

use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};

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
// /-3-\   /-4-\   /-5-\    /-6-\
// 7   8   9   10  11  12  13   14
// 0 - root, 1 - first level start, 3 - second level start, 7 - third level start
type LOP3SubTreePaths = [PathMove; 15];

#[derive(Clone)]
struct LOP3Node<T> {
    node: T,                          // node in original circuit graph
    args: [T; 3],                     // arguments, also leaves of LOP3 subtree
    tree_paths: LOP3SubTreePaths,     // LOP3 subtree paths
    mtu_view: Option<Rc<MTUView<T>>>, // by default it can be empty MTUView
    mtu_cost: usize,
}

// fn find_best_lop3node<T>(lop3nodes: [&LOP3Node<T>], wire_index: T) -> LOP3Node {
//     LOP
// }

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
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }
}

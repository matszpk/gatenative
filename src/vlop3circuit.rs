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
// 15 - fourth level start
// leaves are LOP3 inputs.
type LOP3SubTreePaths = [PathMove; 31];

#[derive(Clone)]
struct LOP3Node<T> {
    node: T,                          // node in original circuit graph
    args: [T; 3],                     // arguments, also leaves of LOP3 subtree
    tree_paths: LOP3SubTreePaths,     // LOP3 subtree paths
    mtu_view: Option<Rc<MTUView<T>>>, // by default it can be empty MTUView
    mtu_cost: usize,
}

fn find_best_lop3node<T>(
    circuit: VBinOpCircuit<T>,
    lop3nodes: &[LOP3Node<T>],
    wire_index: T,
    preferred_leaves: Option<&[T]>,
) -> LOP3Node<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    // generate tree to explore
    let tree = {
        let input_len_t = circuit.input_len;
        let input_len = usize::try_from(input_len_t).unwrap();
        let gates = &circuit.gates;
        let mut tree = [None; 15];
        let mut old_level_start = 0;
        let mut level_start = 1;
        tree[0] = Some(wire_index);
        for level in 1..5 {
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
        }
        tree
    };
    // algorithm: brute force with cutting by number of unique leaves (if l>3 then cut way).
    LOP3Node {
        node: wire_index,
        args: [T::default(); 3],
        tree_paths: [PathMove(0); 31],
        mtu_view: None,
        mtu_cost: 0,
    }
}

fn mtu_area_view_calc_costs<T>(mtuaview: &MTUAreaView<T>) -> (Vec<(T, LOP3SubTreePaths)>, usize) {
    (vec![], 0)
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
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }
}

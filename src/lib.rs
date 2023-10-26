use gatesim::*;

use int_enum::IntEnum;

use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Or = 1,
    Impl = 2,
    Nimpl = 3,
    Xor = 4,
}

pub trait CodeWriter {
    /// It returns bit mask of where bit position is InstrOp integer value - support
    // Instr Ops.
    fn supported_ops(&self) -> u64;
    // Returns Word length in bits. Single variable have word length.
    fn word_len(&self) -> u32;
    // Returns maximal possible variable number in words.
    fn max_var_num(&self) -> usize;
    // Returns preferred variable number in words.
    fn preferred_var_num(&self) -> usize;
    /// Generates prolog.
    fn prolog(&self, out: &mut Vec<u8>);
    /// Generates epilog;
    fn epilog(&self, out: &mut Vec<u8>);
    /// Generates function start with definition.
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize);
    /// Generates function end.
    fn func_end(&self, out: &mut Vec<u8>, name: &str);
    /// Generates allocation of local variables to make operations.
    fn alloc_vars(&self, out: &mut Vec<u8>, var_num: usize);

    /// Generates Load instruction from input.
    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize);
    /// Generates operation.
    fn gen_op(
        &self,
        out: &mut Vec<u8>,
        op: InstrOp,
        negs: VNegs,
        dst_arg: usize,
        arg1: usize,
        arg2: usize,
    );
    /// Generates Store instruction into output.
    fn gen_store(&self, out: &mut Vec<u8>, neg: bool, output: usize, reg: usize);
}

pub struct VarAllocator {
    free_list: BinaryHeap<std::cmp::Reverse<usize>>,
    alloc_map: Vec<bool>,
}

impl VarAllocator {
    fn new() -> Self {
        Self {
            free_list: BinaryHeap::new(),
            alloc_map: vec![],
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.alloc_map.len()
    }

    fn alloc(&mut self) -> usize {
        if let Some(std::cmp::Reverse(index)) = self.free_list.pop() {
            self.alloc_map[index] = true;
            index
        } else {
            let index = self.alloc_map.len();
            self.alloc_map.push(true);
            index
        }
    }

    fn free(&mut self, index: usize) -> bool {
        assert!(index < self.len());
        if self.alloc_map[index] {
            self.free_list.push(std::cmp::Reverse(index));
            self.alloc_map[index] = false;
            true
        } else {
            false
        }
    }

    #[inline]
    fn is_alloc(&self, index: usize) -> bool {
        self.alloc_map[index]
    }
}

// var usage - just counter of var usage.

// TODO: binary double-not optimization.
// if only one type of occurrence of variable: reduce in both sides.
// if either two type of occurrence of variable:

struct VarUsage {
    index: usize,
    bnot: bool, // if single operation is boolean negation of original gate output.
}

use VNegs::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VGateFunc {
    And,
    Nand,
    Or,
    Nor,
    Impl,
    Nimpl,
    Xor,
}

impl From<GateFunc> for VGateFunc {
    #[inline]
    fn from(gf: GateFunc) -> Self {
        match gf {
            GateFunc::And => VGateFunc::And,
            GateFunc::Nor => VGateFunc::Nor,
            GateFunc::Nimpl => VGateFunc::Nimpl,
            GateFunc::Xor => VGateFunc::Xor,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct VGate<T: Clone + Copy> {
    i0: T,
    i1: T,
    func: VGateFunc,
}

impl<T: Clone + Copy> From<Gate<T>> for VGate<T> {
    fn from(g: Gate<T>) -> Self {
        Self {
            i0: g.i0,
            i1: g.i1,
            func: VGateFunc::from(g.func),
        }
    }
}

impl<T: Clone + Copy> VGate<T> {
    // conversion to operation (and,or,xor,not,impl)
    fn to_binop_and_impl(self) -> (VGate<T>, bool) {
        match self.func {
            VGateFunc::Nand => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::And,
                },
                true,
            ),
            VGateFunc::Nimpl => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::Impl,
                },
                true,
            ),
            VGateFunc::Nor => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::Or,
                },
                true,
            ),
            _ => (self, false),
        }
    }

    // negate argument i0 in gate with conversion to op_and_impl or op_and_nimpl
    fn to_binop_and_ximpl_neg_args(
        self,
        nimpl: bool,
        neg_i0: bool,
        neg_i1: bool,
    ) -> (VGate<T>, bool) {
        let (impl_func, impl_neg) = if nimpl {
            (VGateFunc::Nimpl, true)
        } else {
            (VGateFunc::Impl, false)
        };
        match self.func {
            VGateFunc::And | VGateFunc::Nand => {
                let neg = self.func == VGateFunc::Nand;
                match (neg_i0, neg_i1) {
                    (false, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        neg,
                    ),
                    (true, false) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: impl_func,
                        },
                        !neg ^ impl_neg,
                    ),
                    (false, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: impl_func,
                        },
                        !neg ^ impl_neg,
                    ),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        !neg,
                    ),
                }
            }
            VGateFunc::Or | VGateFunc::Nor => {
                let neg = self.func == VGateFunc::Nor;
                match (neg_i0, neg_i1) {
                    (false, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        neg,
                    ),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: impl_func,
                        },
                        neg ^ impl_neg,
                    ),
                    (false, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: impl_func,
                        },
                        neg ^ impl_neg,
                    ),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        !neg,
                    ),
                }
            }
            VGateFunc::Impl | VGateFunc::Nimpl => {
                let neg = self.func == VGateFunc::Nimpl;
                match (neg_i0, neg_i1) {
                    (false, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: impl_func,
                        },
                        neg ^ impl_neg,
                    ),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        neg,
                    ),
                    (false, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        !neg,
                    ),
                    (true, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: impl_func,
                        },
                        neg ^ impl_neg,
                    ),
                }
            }
            VGateFunc::Xor => (self, neg_i0 ^ neg_i1),
        }
    }

    // conversion to operation (and,or,xor,not,nimpl)
    fn to_binop_and_nimpl(self) -> (VGate<T>, bool) {
        match self.func {
            VGateFunc::Nand => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::And,
                },
                true,
            ),
            VGateFunc::Impl => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::Nimpl,
                },
                true,
            ),
            VGateFunc::Nor => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::Or,
                },
                true,
            ),
            _ => (self, false),
        }
    }

    // conversion to operation (and,or,xor,not)
    fn to_binop(self) -> (VGate<T>, VNegs) {
        match self.func {
            VGateFunc::Nand => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::And,
                },
                NegOutput,
            ),
            VGateFunc::Nor => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::Or,
                },
                NegOutput,
            ),
            VGateFunc::Impl => (
                VGate {
                    i0: self.i1,
                    i1: self.i0,
                    func: VGateFunc::Or,
                },
                NegInput1,
            ),
            VGateFunc::Nimpl => (
                VGate {
                    i0: self.i0,
                    i1: self.i1,
                    func: VGateFunc::And,
                },
                NegInput1,
            ),
            _ => (self, NoNegs),
        }
    }

    fn binop_neg(self, negs: VNegs) -> (VGate<T>, VNegs) {
        assert!(matches!(
            self.func,
            VGateFunc::And | VGateFunc::Or | VGateFunc::Xor
        ));
        match negs {
            NoNegs => (self, NegOutput),
            NegInput1 => {
                match self.func {
                    VGateFunc::And => (
                        // not and(..,not ..) <-> or(not ..., ...)
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::Or,
                        },
                        NegInput1,
                    ),
                    VGateFunc::Or => (
                        // not or(..,not ..) <-> and(not ..., ...)
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::And,
                        },
                        NegInput1,
                    ),
                    VGateFunc::Xor => (self, NoNegs), // not xor(..., not...) <-> xor(...,...)
                    _ => {
                        panic!("Unsupported");
                    }
                }
            }
            NegOutput => (self, NoNegs),
        }
    }

    fn binop_neg_args(self, negs: VNegs, neg_i0: bool, neg_i1: bool) -> (VGate<T>, VNegs) {
        assert!(matches!(
            self.func,
            VGateFunc::And | VGateFunc::Or | VGateFunc::Xor
        ));
        match negs {
            NoNegs => match self.func {
                VGateFunc::And => match (neg_i0, neg_i1) {
                    (false, false) => (self, NoNegs),
                    (true, false) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::And,
                        },
                        NegInput1,
                    ),
                    (false, true) => (self, NegInput1),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        NegOutput,
                    ),
                },
                VGateFunc::Or => match (neg_i0, neg_i1) {
                    (false, false) => (self, NoNegs),
                    (true, false) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::Or,
                        },
                        NegInput1,
                    ),
                    (false, true) => (self, NegInput1),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        NegOutput,
                    ),
                },
                VGateFunc::Xor => (self, if neg_i0 ^ neg_i1 { NegOutput } else { NoNegs }),
                _ => {
                    panic!("Unsupported");
                }
            },
            NegInput1 => match self.func {
                VGateFunc::And => match (neg_i0, neg_i1) {
                    (false, false) => (self, NegInput1),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        NegOutput,
                    ),
                    (false, true) => (self, NoNegs),
                    (true, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::And,
                        },
                        NegInput1,
                    ),
                },
                VGateFunc::Or => match (neg_i0, neg_i1) {
                    (false, false) => (self, NegInput1),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        NegOutput,
                    ),
                    (false, true) => (self, NoNegs),
                    (true, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::Or,
                        },
                        NegInput1,
                    ),
                },
                VGateFunc::Xor => (self, if !neg_i0 ^ neg_i1 { NegOutput } else { NoNegs }),
                _ => {
                    panic!("Unsupported");
                }
            },
            NegOutput => match self.func {
                VGateFunc::And => match (neg_i0, neg_i1) {
                    (false, false) => (self, NegOutput),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        NegInput1,
                    ),
                    (false, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::Or,
                        },
                        NegInput1,
                    ),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::Or,
                        },
                        NoNegs,
                    ),
                },
                VGateFunc::Or => match (neg_i0, neg_i1) {
                    (false, false) => (self, NegOutput),
                    (true, false) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        NegInput1,
                    ),
                    (false, true) => (
                        VGate {
                            i0: self.i1,
                            i1: self.i0,
                            func: VGateFunc::And,
                        },
                        NegInput1,
                    ),
                    (true, true) => (
                        VGate {
                            i0: self.i0,
                            i1: self.i1,
                            func: VGateFunc::And,
                        },
                        NoNegs,
                    ),
                },
                VGateFunc::Xor => (self, if !neg_i0 ^ neg_i1 { NegOutput } else { NoNegs }),
                _ => {
                    panic!("Unsupported");
                }
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct VCircuit<T: Clone + Copy> {
    input_len: T,
    gates: Vec<VGate<T>>,
    outputs: Vec<(T, bool)>,
}

impl<T> VCircuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn to_op_and_ximpl_circuit(circuit: Circuit<T>, nimpl: bool) -> VCircuit<T> {
        #[derive(Clone, Copy)]
        struct StackEntry {
            node: usize,
            way: usize,
        }

        let input_len_t = circuit.input_len();
        let input_len = usize::try_from(input_len_t).unwrap();
        let gate_num = circuit.len();
        let mut gates = circuit
            .gates()
            .into_iter()
            .map(|g| (VGate::from(*g), false))
            .collect::<Vec<_>>();

        let mut visited = vec![false; gate_num];
        for (o, _) in circuit.outputs().into_iter() {
            if *o < input_len_t {
                continue;
            }
            let oidx = usize::try_from(*o).unwrap() - input_len;
            let mut stack = Vec::new();
            stack.push(StackEntry { node: oidx, way: 0 });

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
                    let gi0 = gates[node_index].0.i0;
                    if gi0 >= input_len_t {
                        stack.push(StackEntry {
                            node: usize::try_from(gi0).unwrap() - input_len,
                            way: 0,
                        });
                    }
                } else if way == 1 {
                    top.way += 1;
                    let gi1 = gates[node_index].0.i1;
                    if gi1 >= input_len_t {
                        stack.push(StackEntry {
                            node: usize::try_from(gi1).unwrap() - input_len,
                            way: 0,
                        });
                    }
                } else {
                    // resolve
                    assert!(!gates[node_index].1);
                    let (newg, n) = if nimpl {
                        gates[node_index].0.to_binop_and_nimpl()
                    } else {
                        gates[node_index].0.to_binop_and_impl()
                    };
                    let ni0 = if newg.i0 >= input_len_t {
                        gates[usize::try_from(newg.i0).unwrap() - input_len].1
                    } else {
                        false
                    };
                    let ni1 = if newg.i1 >= input_len_t {
                        gates[usize::try_from(newg.i1).unwrap() - input_len].1
                    } else {
                        false
                    };
                    gates[node_index] = {
                        let (newg2, n2) = newg.to_binop_and_ximpl_neg_args(nimpl, ni0, ni1);
                        (newg2, n ^ n2)
                    };
                    stack.pop();
                }
            }
        }

        let outputs = circuit
            .outputs()
            .into_iter()
            .map(|(o, on)| {
                if *o >= input_len_t {
                    (*o, on ^ gates[usize::try_from(*o).unwrap() - input_len].1)
                } else {
                    (*o, *on)
                }
            })
            .collect::<Vec<_>>();
        VCircuit {
            input_len: circuit.input_len(),
            gates: gates.into_iter().map(|(g, _)| g).collect::<Vec<_>>(),
            outputs,
        }
    }
}

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
        println!("Start optnegs");
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
            };
            let g_negs = self.gates[i].1;
            if g_negs == NegInput1 {
                continue;
            }
            println!("  Start: {:?}: {:?}: {:?}", oi, self.gates[i], occurs[i]);
            // check whether same type of occurrence (negation)
            let mut occurs_changed = HashMap::<HashKey<T>, (bool, bool)>::new();
            for occur in &occurs[i] {
                let (x_key, neg_i0, neg_i1) = match occurs[i].first().unwrap() {
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
            println!("  OccursChanged: {:?}: {:?}", oi, occurs_changed);
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
            let min_removed = if g_negs == NegOutput {
                -1 // can just propagate negation to further gates
            } else {
                2 // must reduce negation
            };
            println!("  NegsRemoved: {:?}: {}", oi, negs_removed);
            if negs_removed >= min_removed {
                // apply changes if change remove more negations than added negations.
                self.gates[i].1 = if g_negs == NegOutput {
                    NoNegs
                } else {
                    NegOutput
                };
                for (k, (neg_i0, neg_i1)) in occurs_changed.into_iter() {
                    match k {
                        HashKey::Gate(x) => {
                            let (occur_g, occur_negs) =
                                &self.gates[usize::try_from(x).unwrap() - input_len];
                            self.gates[usize::try_from(x).unwrap() - input_len] =
                                occur_g.binop_neg_args(*occur_negs, neg_i0, neg_i1);
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

    fn optimize_negs(&mut self) {}
}

pub fn generate_code<CW: CodeWriter, T>(writer: &CW, circuit: Circuit<T>)
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_allocator() {
        let mut vacc = VarAllocator::new();
        assert_eq!(0, vacc.alloc());
        assert_eq!(1, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(3, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert!(vacc.free(2));
        assert!(!vacc.free(2));
        assert!(vacc.free(1));
        assert!(!vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(0));
        assert!(!vacc.free(0));
        assert_eq!(0, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(5, vacc.alloc());
        assert!(vacc.free(4));
        assert!(vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(3));
        assert_eq!(3, vacc.alloc());
        assert!(vacc.free(2));
        assert_eq!(2, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert_eq!(6, vacc.alloc());
    }

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
    fn test_vgate() {
        assert_eq!(vgate_and(3, 4), VGate::from(Gate::new_and(3, 4)));
        assert_eq!(vgate_nor(3, 5), VGate::from(Gate::new_nor(3, 5)));
        assert_eq!(vgate_nimpl(2, 4), VGate::from(Gate::new_nimpl(2, 4)));
        assert_eq!(vgate_xor(2, 6), VGate::from(Gate::new_xor(2, 6)));
    }

    #[test]
    fn test_vgate_to_binop_and_ximpl() {
        // binop_and_impl
        assert_eq!(
            (vgate_and(3, 5), false),
            vgate_and(3, 5).to_binop_and_impl()
        );
        assert_eq!(
            (vgate_and(3, 5), true),
            vgate_nand(3, 5).to_binop_and_impl()
        );
        assert_eq!((vgate_or(3, 5), false), vgate_or(3, 5).to_binop_and_impl());
        assert_eq!((vgate_or(3, 5), true), vgate_nor(3, 5).to_binop_and_impl());
        assert_eq!(
            (vgate_impl(3, 5), false),
            vgate_impl(3, 5).to_binop_and_impl()
        );
        assert_eq!(
            (vgate_impl(3, 5), true),
            vgate_nimpl(3, 5).to_binop_and_impl()
        );
        assert_eq!(
            (vgate_xor(3, 5), false),
            vgate_xor(3, 5).to_binop_and_impl()
        );

        // binop_and_nimpl
        assert_eq!(
            (vgate_and(3, 5), false),
            vgate_and(3, 5).to_binop_and_nimpl()
        );
        assert_eq!(
            (vgate_and(3, 5), true),
            vgate_nand(3, 5).to_binop_and_nimpl()
        );
        assert_eq!((vgate_or(3, 5), false), vgate_or(3, 5).to_binop_and_nimpl());
        assert_eq!((vgate_or(3, 5), true), vgate_nor(3, 5).to_binop_and_nimpl());
        assert_eq!(
            (vgate_nimpl(3, 5), true),
            vgate_impl(3, 5).to_binop_and_nimpl()
        );
        assert_eq!(
            (vgate_nimpl(3, 5), false),
            vgate_nimpl(3, 5).to_binop_and_nimpl()
        );
        assert_eq!(
            (vgate_xor(3, 5), false),
            vgate_xor(3, 5).to_binop_and_nimpl()
        );

        // to_binop
        assert_eq!((vgate_and(3, 5), NoNegs), vgate_and(3, 5).to_binop());
        assert_eq!((vgate_and(3, 5), NegOutput), vgate_nand(3, 5).to_binop());
        assert_eq!((vgate_or(3, 5), NoNegs), vgate_or(3, 5).to_binop());
        assert_eq!((vgate_or(3, 5), NegOutput), vgate_nor(3, 5).to_binop());
        assert_eq!((vgate_or(5, 3), NegInput1), vgate_impl(3, 5).to_binop());
        assert_eq!((vgate_and(3, 5), NegInput1), vgate_nimpl(3, 5).to_binop());
        assert_eq!((vgate_xor(3, 5), NoNegs), vgate_xor(3, 5).to_binop());
    }

    fn vgate_eval<T: Clone + Copy>(g: VGate<T>, neg_i0: bool, neg_i1: bool) -> u8
    where
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        let i0 = usize::try_from(g.i0).unwrap();
        let i1 = usize::try_from(g.i1).unwrap();
        let neg_i0_mask = if neg_i0 { 0b1111 } else { 0 };
        let neg_i1_mask = if neg_i1 { 0b1111 } else { 0 };
        let v0 = (if i0 == 0 { 0b1010 } else { 0b1100 }) ^ neg_i0_mask;
        let v1 = (if i1 == 0 { 0b1010 } else { 0b1100 }) ^ neg_i1_mask;
        let out = match g.func {
            VGateFunc::And => v0 & v1,
            VGateFunc::Nand => !(v0 & v1),
            VGateFunc::Or => v0 | v1,
            VGateFunc::Nor => !(v0 | v1),
            VGateFunc::Impl => !v0 | v1,
            VGateFunc::Nimpl => !(!v0 | v1),
            VGateFunc::Xor => v0 ^ v1,
        };
        out & 0b1111
    }

    #[test]
    fn test_vgate_to_binop_and_ximpl_neg_args() {
        for func in [
            VGateFunc::And,
            VGateFunc::Nand,
            VGateFunc::Or,
            VGateFunc::Nor,
            VGateFunc::Impl,
            VGateFunc::Nimpl,
            VGateFunc::Xor,
        ] {
            for t in 0..8 {
                let neg_i0 = (t & 1) != 0;
                let neg_i1 = (t & 2) != 0;
                let nimpl = (t & 4) != 0;
                let orig = VGate { i0: 0, i1: 1, func };
                let exp_value = vgate_eval(orig, neg_i0, neg_i1);
                let (trans, neg) = orig.to_binop_and_ximpl_neg_args(nimpl, neg_i0, neg_i1);
                let res_value = vgate_eval(trans, false, false) ^ (if neg { 0b1111 } else { 0 });
                assert_eq!(
                    exp_value,
                    res_value,
                    "func:{:?} nimpl:{} n0:{} n1:{}: trans:{:?}",
                    func,
                    nimpl,
                    neg_i0,
                    neg_i1,
                    (trans, neg)
                );
                let ximpl_func = if nimpl {
                    VGateFunc::Nimpl
                } else {
                    VGateFunc::Impl
                };
                assert!(
                    matches!(trans.func, VGateFunc::And | VGateFunc::Or | VGateFunc::Xor)
                        || trans.func == ximpl_func,
                    "func:{:?} nimpl:{} n0:{} n1:{}: trans:{:?}",
                    func,
                    nimpl,
                    neg_i0,
                    neg_i1,
                    (trans, neg)
                );
            }
        }
    }

    #[test]
    fn test_vgate_binop_neg() {
        for func in [VGateFunc::And, VGateFunc::Or, VGateFunc::Xor] {
            for negs in [NoNegs, NegInput1, NegOutput] {
                let orig = VGate { i0: 0, i1: 1, func };
                let exp_value = vgate_eval(orig, false, matches!(negs, NegInput1))
                    ^ (if matches!(negs, NegOutput) { 0b1111 } else { 0 })
                    // vvv - apply negation
                    ^ 0b1111;
                let (trans, tnegs) = orig.binop_neg(negs);
                let res_value = vgate_eval(trans, false, matches!(tnegs, NegInput1))
                    ^ (if matches!(tnegs, NegOutput) {
                        0b1111
                    } else {
                        0
                    });
                assert_eq!(
                    exp_value, res_value,
                    "func:{:?} negs:{:?}: trans:{:?} tnegs:{:?}",
                    func, negs, trans, tnegs,
                );
                assert!(
                    matches!(trans.func, VGateFunc::And | VGateFunc::Or | VGateFunc::Xor),
                    "func:{:?} negs:{:?}: trans:{:?} tnegs:{:?}",
                    func,
                    negs,
                    trans,
                    tnegs,
                );
            }
        }
    }

    #[test]
    fn test_vgate_binop_neg_args() {
        for func in [VGateFunc::And, VGateFunc::Or, VGateFunc::Xor] {
            for negs in [NoNegs, NegInput1, NegOutput] {
                for t in 0..4 {
                    let neg_i0 = (t & 1) != 0;
                    let neg_i1 = (t & 2) != 0;
                    let orig = VGate { i0: 0, i1: 1, func };
                    let exp_value = vgate_eval(orig, neg_i0, neg_i1 ^ matches!(negs, NegInput1))
                        ^ (if matches!(negs, NegOutput) { 0b1111 } else { 0 });
                    let (trans, tnegs) = orig.binop_neg_args(negs, neg_i0, neg_i1);
                    let res_value = vgate_eval(trans, false, matches!(tnegs, NegInput1))
                        ^ (if matches!(tnegs, NegOutput) {
                            0b1111
                        } else {
                            0
                        });
                    assert_eq!(
                        exp_value, res_value,
                        "func:{:?} negs:{:?} ni0:{} ni1:{}: trans:{:?} tnegs:{:?}",
                        func, negs, neg_i0, neg_i1, trans, tnegs,
                    );
                    assert!(
                        matches!(trans.func, VGateFunc::And | VGateFunc::Or | VGateFunc::Xor),
                        "func:{:?} negs:{:?} ni0:{} ni1:{}: trans:{:?} tnegs:{:?}",
                        func,
                        negs,
                        neg_i0,
                        neg_i1,
                        trans,
                        tnegs,
                    );
                }
            }
        }
    }

    #[test]
    fn test_vcircuit_to_op_and_ximpl_circuit() {
        for (nimpl, gate, vgate, vout_neg) in [
            (false, Gate::new_nimpl(0, 1), vgate_impl(0, 1), true),
            (false, Gate::new_nor(0, 1), vgate_or(0, 1), true),
            (true, Gate::new_nimpl(0, 1), vgate_nimpl(0, 1), false),
            (true, Gate::new_nor(0, 1), vgate_or(0, 1), true),
        ] {
            assert_eq!(
                VCircuit {
                    input_len: 2,
                    gates: vec![vgate],
                    outputs: vec![(2, vout_neg)],
                },
                VCircuit::to_op_and_ximpl_circuit(
                    Circuit::new(2, [gate], [(2, false)],).unwrap(),
                    nimpl
                ),
                "{} {:?} {:?} {}",
                nimpl,
                gate,
                vgate,
                vout_neg
            );
        }

        assert_eq!(
            VCircuit {
                input_len: 3,
                gates: vec![
                    vgate_xor(0, 1),
                    vgate_xor(2, 3),
                    vgate_and(2, 3),
                    vgate_and(0, 1),
                    vgate_or(5, 6),
                ],
                outputs: vec![(4, false), (7, false)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    3,
                    [
                        Gate::new_xor(0, 1),
                        Gate::new_xor(2, 3),
                        Gate::new_and(2, 3),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, false), (7, true)],
                )
                .unwrap(),
                false
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 3,
                gates: vec![
                    vgate_impl(0, 1),
                    vgate_xor(2, 3),
                    vgate_and(2, 3),
                    vgate_and(0, 1),
                    vgate_or(5, 6),
                ],
                outputs: vec![(4, false), (7, true)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1), // not impl(0,1)
                        Gate::new_xor(2, 3),   // xor(2,not impl(0,1)) = not xor(2,3)
                        Gate::new_nimpl(2, 3), // nimpl(2,not impl(0,1)) = and(2,3)
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                    ],
                    [(4, true), (7, false)],
                )
                .unwrap(),
                false
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 3,
                gates: vec![
                    vgate_nimpl(0, 1),
                    vgate_xor(2, 3),
                    vgate_nimpl(2, 3),
                    vgate_and(0, 1),
                    vgate_or(5, 6),
                ],
                outputs: vec![(4, true), (7, true)],
            },
            VCircuit::to_op_and_ximpl_circuit(
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
                true
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![
                    vgate_impl(0, 1),
                    vgate_or(2, 3),
                    vgate_or(1, 3),
                    vgate_xor(1, 4),
                    vgate_xor(5, 7),
                    vgate_xor(6, 8),
                    vgate_impl(3, 9),
                ],
                outputs: vec![(7, false), (8, false), (9, false), (10, true)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_nor(2, 3),
                        Gate::new_nor(1, 3),
                        Gate::new_xor(1, 4), // xor(not,..) = not xor(..,..)
                        Gate::new_xor(5, 7), // xor(not...,xor(not,..)) = xor(xor(..),..)
                        Gate::new_xor(6, 8), // xor(not..,xor(not xor(...) = not xor(...)
                        Gate::new_and(3, 9), // and(..,not..) = not impl(...,...)
                    ],
                    [(7, true), (8, false), (9, true), (10, false)],
                )
                .unwrap(),
                false
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![
                    vgate_nimpl(0, 1),
                    vgate_or(2, 3),
                    vgate_or(1, 3),
                    vgate_xor(1, 4),
                    vgate_xor(5, 7),
                    vgate_xor(6, 8),
                    vgate_and(3, 9),
                ],
                outputs: vec![(7, true), (8, true), (9, true), (10, false)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_nor(2, 3),
                        Gate::new_nor(1, 3),
                        Gate::new_xor(1, 4),
                        Gate::new_xor(5, 7),
                        Gate::new_xor(6, 8),
                        Gate::new_and(3, 9),
                    ],
                    [(7, true), (8, false), (9, true), (10, false)],
                )
                .unwrap(),
                true
            )
        );

        // next testcases
        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![
                    vgate_impl(0, 1),
                    vgate_or(2, 3),
                    vgate_or(1, 3),
                    vgate_and(1, 4),
                    vgate_or(5, 7),
                    vgate_and(6, 8),
                    vgate_xor(3, 4),
                    vgate_or(4, 9),
                ],
                outputs: vec![
                    (5, true),
                    (7, true),
                    (8, true),
                    (9, true),
                    (10, true),
                    (11, false)
                ],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1), // -> not :impl(0, 1)
                        Gate::new_nor(2, 3),   // -> not :or(2, 3)
                        Gate::new_nor(1, 3),   // -> not :or(1, 3)
                        Gate::new_nimpl(1, 4), // -> not: or(not 1, not 4) -> and(1, 4)
                        Gate::new_nimpl(5, 7), // -> not :impl(not 5, 7) -> not: or(5, 7)
                        Gate::new_nor(6, 8),   // -> not :or(not 6, not 8) -> and(6, 8)
                        Gate::new_xor(3, 4),   // -> xor(3, not 4) -> not: xor(3, 4)
                        Gate::new_nimpl(4, 9), // -> not: impl(not 4, 9) -> not: or(4, 9)
                    ],
                    [
                        (5, false),
                        (7, true),
                        (8, false),
                        (9, true),
                        (10, false),
                        (11, true)
                    ],
                )
                .unwrap(),
                false
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![
                    vgate_nimpl(0, 1),
                    vgate_or(2, 3),
                    vgate_or(1, 3),
                    vgate_nimpl(1, 4),
                    vgate_or(5, 7),
                    vgate_and(6, 8),
                    vgate_xor(3, 4),
                    vgate_nimpl(4, 9),
                ],
                outputs: vec![
                    (5, true),
                    (7, true),
                    (8, true),
                    (9, true),
                    (10, false),
                    (11, true)
                ],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_nor(2, 3), // -> not :or(2, 3)
                        Gate::new_nor(1, 3), // -> not :or(1, 3)
                        Gate::new_nimpl(1, 4),
                        Gate::new_nimpl(5, 7), // -> nimpl(not 5, 7) -> not: or(5, 7)
                        Gate::new_nor(6, 8),   // -> not :or(not 6, not 8) -> and(6, 8)
                        Gate::new_xor(3, 4),
                        Gate::new_nimpl(4, 9),
                    ],
                    [
                        (5, false),
                        (7, true),
                        (8, false),
                        (9, true),
                        (10, false),
                        (11, true)
                    ],
                )
                .unwrap(),
                true
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![vgate_impl(0, 1), vgate_or(2, 3), vgate_impl(5, 4)],
                outputs: vec![(4, true), (5, false), (6, true)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1), // -> not :impl(0, 1)
                        Gate::new_nor(2, 3),   // -> not :or(2, 3)
                        Gate::new_nimpl(4, 5), // -> not: impl(not 4, not 5) -> impl(4, 5)
                    ],
                    [(4, false), (5, true), (6, false)],
                )
                .unwrap(),
                false
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![vgate_nimpl(0, 1), vgate_or(2, 3), vgate_and(4, 5)],
                outputs: vec![(4, false), (5, false), (6, false)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1), // -> nimpl(0, 1)
                        Gate::new_nor(2, 3),   // -> not :or(2, 3)
                        Gate::new_nimpl(4, 5), // -> nimpl(4, not 5) -> and(4, 5)
                    ],
                    [(4, false), (5, true), (6, false)],
                )
                .unwrap(),
                true
            )
        );

        assert_eq!(
            VCircuit {
                input_len: 4,
                gates: vec![
                    vgate_impl(0, 1),
                    vgate_or(2, 3),
                    vgate_impl(5, 4),
                    vgate_xor(4, 5)
                ],
                outputs: vec![(4, true), (5, false), (6, true), (7, false)],
            },
            VCircuit::to_op_and_ximpl_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1), // -> not :impl(0, 1)
                        Gate::new_nor(2, 3),   // -> not :or(2, 3)
                        Gate::new_nimpl(4, 5), // -> not: impl(not 4, not 5) -> impl(4, 5)
                        Gate::new_xor(4, 5),   // -> not: impl(not 4, not 5) -> impl(4, 5)
                    ],
                    [(4, false), (5, true), (6, false), (7, false)],
                )
                .unwrap(),
                false
            )
        );
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

    fn swap_gate_inputs<T: Clone + Copy>(g: Gate<T>, swap: bool) -> Gate<T> {
        if swap {
            Gate {
                i0: g.i1,
                i1: g.i0,
                func: g.func,
            }
        } else {
            g
        }
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
                    (vgate_and(0, 1), NegInput1),
                    (vgate_xor(2, 3), NoNegs),
                    (vgate_and(2, 3), NegInput1),
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(5, 6), NoNegs),
                ],
                outputs: vec![(4, false), (7, false)],
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

        let mut circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                (vgate_and(0, 1), NoNegs),
                (vgate_or(1, 2), NoNegs),
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
                    (vgate_and(0, 1), NoNegs),
                    (vgate_or(1, 2), NoNegs),
                    (vgate_and(3, 4), NoNegs),
                    (vgate_or(3, 4), NoNegs),
                    (vgate_xor(0, 3), NoNegs),
                    (vgate_xor(1, 3), NoNegs),
                    (vgate_and(6, 5), NoNegs),
                    (vgate_and(9, 7), NoNegs),
                    (vgate_and(10, 8), NoNegs),
                ],
                outputs: vec![(11, true)],
            },
            circuit
        );
    }
}

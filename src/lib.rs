use gatesim::*;

use int_enum::IntEnum;

use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Or = 1,
    Impl = 2,
    Nimpl = 3,
    Xor = 4,
    BNot = 5,
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
        dst_arg: usize,
        arg1: usize,
        arg2: Option<usize>,
    );
    /// Generates Store instruction into output.
    fn gen_store(&self, out: &mut Vec<u8>, output: usize, reg: usize);
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
    #[inline]
    fn to_binop_and_impl(self: VGate<T>) -> (VGate<T>, bool) {
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
    #[inline]
    fn to_binop_and_ximpl_neg_args(
        self: VGate<T>,
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
                    (false, false) => (self, neg),
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
                    (false, false) => (self, neg),
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
                let neg = self.func == VGateFunc::Nor;
                match (neg_i0, neg_i1) {
                    (false, false) => (self, neg),
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
                        !neg ^ impl_neg,
                    ),
                }
            }
            VGateFunc::Xor => (self, neg_i0 ^ neg_i1),
            _ => (self, false),
        }
    }

    // conversion to operation (and,or,xor,not,nimpl)
    #[inline]
    fn to_binop_and_nimpl(self: VGate<T>) -> (VGate<T>, bool) {
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
}

#[derive(Clone)]
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
        for (o, on) in circuit.outputs().into_iter() {
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
                    gates[node_index] = if ni0 || ni1 {
                        let (newg2, n2) = newg.to_binop_and_ximpl_neg_args(nimpl, ni0, ni1);
                        (newg2, n ^ n2)
                    } else {
                        (newg, n)
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum VNegs {
    NoNegs,
    NegArg1,
    NegOut,
    NegsArg1Out,
}

#[derive(Clone)]
struct VCircuitStdOp<T: Clone + Copy> {
    input_len: T,
    gates: Vec<(VGate<T>, VNegs)>,
    outputs: Vec<(T, bool)>,
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
}

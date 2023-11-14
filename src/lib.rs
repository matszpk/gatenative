use gatesim::*;

use int_enum::IntEnum;

use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

mod vcircuit;
use vcircuit::*;
mod vbinopcircuit;
use vbinopcircuit::*;

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

fn gen_var_usage<T>(circuit: &Circuit<T>) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let mut var_usage = vec![T::default(); input_len + circuit.len()];
    for (i, g) in circuit.gates().iter().enumerate() {
        let gi0 = usize::try_from(g.i0).unwrap();
        let gi1 = usize::try_from(g.i1).unwrap();
        let var_usage_0 = usize::try_from(var_usage[gi0]).unwrap() + 1;
        var_usage[gi0] = T::try_from(var_usage_0).unwrap();
        let var_usage_1 = usize::try_from(var_usage[gi1]).unwrap() + 1;
        var_usage[gi1] = T::try_from(var_usage_1).unwrap();
    }
    var_usage
}

fn gen_var_allocs<T>(circuit: &Circuit<T>, var_usage: &mut [usize]) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    #[derive(Clone, Copy)]
    struct StackEntry {
        node: usize,
        way: usize,
    }
    let input_len_t = circuit.input_len();
    let input_len = usize::try_from(input_len_t).unwrap();
    let gate_num = circuit.len();
    let gates = circuit.gates();
    let mut var_allocs: Vec<Option<T>> = vec![None; input_len + gate_num];
    
    let mut visited = vec![false; gate_num];
    for (o, _) in circuit.outputs().iter() {
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
                let gi0 = gates[node_index].i0;
                if gi0 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi0).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else if way == 1 {
                top.way += 1;
                let gi1 = gates[node_index].i1;
                if gi1 >= input_len_t {
                    stack.push(StackEntry {
                        node: usize::try_from(gi1).unwrap() - input_len,
                        way: 0,
                    });
                }
            } else {
                // resolve
            }
        }
    }
    var_allocs.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>()
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
}

use gatesim::*;

use int_enum::IntEnum;

use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Nand = 1,
    Or = 2,
    Nor = 3,
    Impl = 4,
    Nimpl = 5,
    Xor = 6,
    Equal = 7,
    BNot = 8,
}

pub trait CodeWriter {
    /// It returns bit mask of where bit position is InstrOp integer value - support
    // Instr Ops.
    fn supported_ops(&self) -> u64;
    // Returns Word length in bits
    fn word_len(&self) -> u32;
    // Returns maximal possible allocation size in words.
    fn max_alloc_size(&self) -> usize;
    // Returns preferred allocation size in words.
    fn preferred_alloc_size(&self) -> usize;
    /// Generates prolog.
    fn prolog(&self, out: &mut Vec<u8>);
    /// Generates epilog;
    fn epilog(&self, out: &mut Vec<u8>);
    /// Generates function start with definition.
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize);
    /// Generates function end.
    fn func_end(&self, out: &mut Vec<u8>, name: &str);
    /// Generates allocation of local words to make operations.
    fn alloc(&self, out: &mut Vec<u8>, alloc_size: usize);

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

pub struct WordAllocator {
    free_list: BinaryHeap<std::cmp::Reverse<usize>>,
    alloc_map: Vec<bool>,
}

impl WordAllocator {
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
    fn test_word_allocator() {
        let mut wacc = WordAllocator::new();
        assert_eq!(0, wacc.alloc());
        assert_eq!(1, wacc.alloc());
        assert_eq!(2, wacc.alloc());
        assert_eq!(3, wacc.alloc());
        assert_eq!(4, wacc.alloc());
        assert!(wacc.free(2));
        assert!(!wacc.free(2));
        assert!(wacc.free(1));
        assert!(!wacc.free(1));
        assert_eq!(1, wacc.alloc());
        assert!(wacc.free(0));
        assert!(!wacc.free(0));
        assert_eq!(0, wacc.alloc());
        assert_eq!(2, wacc.alloc());
        assert_eq!(5, wacc.alloc());
        assert!(wacc.free(4));
        assert!(wacc.free(1));
        assert_eq!(1, wacc.alloc());
        assert!(wacc.free(3));
        assert_eq!(3, wacc.alloc());
        assert!(wacc.free(2));
        assert_eq!(2, wacc.alloc());
        assert_eq!(4, wacc.alloc());
        assert_eq!(6, wacc.alloc());
    }
}

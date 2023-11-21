use gatesim::Circuit;

use int_enum::IntEnum;

use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_writer;
pub mod cpu_build_exec;
pub mod gencode;
mod vbinopcircuit;
mod vcircuit;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Or = 1,
    Impl = 2,
    Nimpl = 3,
    Xor = 4,
}

pub const INSTR_OP_VALUE_AND: u64 = 0;
pub const INSTR_OP_VALUE_OR: u64 = 1;
pub const INSTR_OP_VALUE_IMPL: u64 = 2;
pub const INSTR_OP_VALUE_NIMPL: u64 = 3;
pub const INSTR_OP_VALUE_XOR: u64 = 4;

pub trait FuncWriter {
    fn func_start(&mut self);
    /// Generates function end.
    fn func_end(&mut self);
    /// Generates allocation of local variables to make operations.
    fn alloc_vars(&mut self, var_num: usize);

    /// Generates Load instruction from input.
    fn gen_load(&mut self, reg: usize, input: usize);
    /// Generates operation.
    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize);
    /// Generates Store instruction into output.
    fn gen_store(&mut self, neg: bool, output: usize, reg: usize);
}

pub trait CodeWriter<'a, FW: FuncWriter> {
    /// It returns bit mask of where bit position is InstrOp integer value - support Instr Ops.
    fn supported_ops(&self) -> u64;
    /// Returns Word length in bits. Single variable have word length.
    fn word_len(&self) -> u32;
    /// Returns maximal possible variable number in words.
    fn max_var_num(&self) -> usize;
    /// Returns preferred variable number in words.
    fn preferred_var_num(&self) -> usize;
    /// Generates prolog.
    fn prolog(&mut self);
    /// Generates epilog.
    fn epilog(&mut self);
    /// Get function writer.
    /// The input_placement and output_placement - real input and output area defintion:
    /// first field - list of real indices. second field - real length.
    fn func_writer(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'a [usize], usize)>,
        output_placement: Option<(&'a [usize], usize)>,
    ) -> FW;

    fn out(self) -> Vec<u8>;
}

pub trait DataHolder {
    fn get(&self) -> &[u32];
    fn get_mut(&mut self) -> &mut [u32];
    fn release(self) -> Vec<u32>;
    fn free(self);
}

pub trait Executor<D: DataHolder> {
    type ErrorType;
    fn input_len(&self) -> usize;
    fn output_len(&self) -> usize;
    fn real_input_len(&self) -> usize;
    fn real_output_len(&self) -> usize;
    fn execute(&mut self, input: &D) -> Result<D, Self::ErrorType>;
    fn execute_reuse(&mut self, input: &D, output: &mut D) -> Result<(), Self::ErrorType>;
    fn new_data(&mut self, len: usize) -> D;
    fn new_data_from(&mut self, data: &[u32]) -> D;
}

pub trait Builder<D: DataHolder, E: Executor<D>> {
    type ErrorType;
    fn add<T>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
    ) where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;
    fn build(self) -> Result<Vec<E>, Self::ErrorType>;
    fn word_len(&self) -> u32;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

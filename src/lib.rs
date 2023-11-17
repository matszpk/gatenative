use gatesim::Circuit;

use int_enum::IntEnum;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_writer;
mod cpu_build_exec;
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

pub trait CodeWriter {
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
    /// Generates function start with definition.
    fn func_start(&mut self, name: &str, input_len: usize, output_len: usize);
    /// Generates function end.
    fn func_end(&mut self, name: &str);
    /// Generates allocation of local variables to make operations.
    fn alloc_vars(&mut self, var_num: usize);

    /// Generates Load instruction from input.
    fn gen_load(&mut self, reg: usize, input: usize);
    /// Generates operation.
    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize);
    /// Generates Store instruction into output.
    fn gen_store(&mut self, neg: bool, output: usize, reg: usize);
}

pub trait Executor {
    type ErrorType;
    fn input_len(&self) -> usize;
    fn output_len(&self) -> usize;
    fn execute(&mut self, input: &[u32]) -> Result<Vec<u32>, Self::ErrorType>;
}

pub trait Builder<E: Executor> {
    type ErrorType;
    fn build<T: Clone + Copy>(&mut self, circuit: Circuit<T>) -> Result<E, Self::ErrorType>;
    fn word_len(&self) -> u32;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

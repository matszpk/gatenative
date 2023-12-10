use gatesim::Circuit;

use int_enum::IntEnum;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeFrom};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_writer;
pub mod cpu_build_exec;
pub mod div_build_exec;
mod divide;
pub mod gencode;
pub mod opencl_build_exec;
mod utils;
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

fn check_placements(
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
) -> bool {
    if let Some((placement, len)) = input_placement {
        if placement.iter().any(|x| *x >= len) {
            return false;
        }
    }
    if let Some((placement, len)) = output_placement {
        if placement.iter().any(|x| *x >= len) {
            return false;
        }
    }
    return true;
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
    /// The arg_inputs - list of circuit inputs that will be set by integer argument
    /// (where bits just set input values).
    unsafe fn func_writer_internal(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'a [usize], usize)>,
        output_placement: Option<(&'a [usize], usize)>,
        arg_inputs: Option<&'a [usize]>,
        single_buffer: bool,
    ) -> FW;

    fn func_writer_ext(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'a [usize], usize)>,
        output_placement: Option<(&'a [usize], usize)>,
        arg_inputs: Option<&'a [usize]>,
        single_buffer: bool,
    ) -> FW {
        // for checking requirements for single_buffer
        let real_input_len = if let Some((_, len)) = input_placement {
            len
        } else {
            input_len - arg_inputs.map(|x| x.len()).unwrap_or(0)
        };
        let real_output_len = if let Some((_, len)) = output_placement {
            len
        } else {
            output_len
        };
        // check requirements for single buffer
        assert!(!single_buffer || real_input_len == real_output_len);
        assert!(check_placements(input_placement, output_placement));
        if let Some(arg_inputs) = arg_inputs {
            assert!(arg_inputs.len() <= 32);
            assert!(arg_inputs.iter().all(|x| *x < input_len));
        }

        unsafe {
            self.func_writer_internal(
                name,
                input_len,
                output_len,
                input_placement,
                output_placement,
                arg_inputs,
                single_buffer,
            )
        }
    }

    fn func_writer(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'a [usize], usize)>,
        output_placement: Option<(&'a [usize], usize)>,
        arg_inputs: Option<&'a [usize]>,
    ) -> FW {
        self.func_writer_ext(
            name,
            input_len,
            output_len,
            input_placement,
            output_placement,
            arg_inputs,
            false,
        )
    }

    fn func_writer_simple(&'a mut self, name: &'a str, input_len: usize, output_len: usize) -> FW {
        self.func_writer(name, input_len, output_len, None, None, None)
    }

    fn out(self) -> Vec<u8>;
}

pub trait DataReader {
    fn get(&self) -> &[u32];
}

pub trait DataWriter {
    fn get_mut(&mut self) -> &mut [u32];
}

pub trait DataHolder<'a, DR: DataReader, DW: DataWriter> {
    fn len(&self) -> usize;
    // set range
    fn set_range(&'a mut self, range: Range<usize>);
    #[inline]
    fn set_range_from(&'a mut self, range: RangeFrom<usize>) {
        self.set_range(range.start..usize::MAX);
    }
    fn get(&'a self) -> DR;
    fn get_mut(&'a mut self) -> DW;
    /// release underlying data
    fn release(self) -> Vec<u32>;
    // free
    fn free(self);
}

pub trait Executor<'a, DR: DataReader, DW: DataWriter, D: DataHolder<'a, DR, DW>> {
    type ErrorType;
    /// Get circuit input length (number of inputs)
    fn input_len(&self) -> usize;
    /// Get circuit output length (number of outputs)
    fn output_len(&self) -> usize;
    /// Get real input length (number of entries in area of input placements)
    fn real_input_len(&self) -> usize;
    /// Get real output length (number of entries in area of output placements)
    fn real_output_len(&self) -> usize;

    unsafe fn execute_internal(&mut self, input: &D, arg_input: u32) -> Result<D, Self::ErrorType>;
    fn execute(&mut self, input: &D, arg_input: u32) -> Result<D, Self::ErrorType> {
        assert!(!self.is_single_buffer());
        unsafe { self.execute_internal(input, arg_input) }
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u32,
        output: &mut D,
    ) -> Result<(), Self::ErrorType>;
    fn execute_reuse(
        &mut self,
        input: &D,
        arg_input: u32,
        output: &mut D,
    ) -> Result<(), Self::ErrorType> {
        assert!(!self.is_single_buffer());
        unsafe { self.execute_reuse_internal(input, arg_input, output) }
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u32,
    ) -> Result<(), Self::ErrorType>;
    fn execute_single(&mut self, output: &mut D, arg_input: u32) -> Result<(), Self::ErrorType> {
        assert!(self.is_single_buffer());
        unsafe { self.execute_single_internal(output, arg_input) }
    }
    /// Create new data - length is number of 32-bit words
    fn new_data(&mut self, len: usize) -> D;
    /// Create new datafrom vector.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// try clone executor if possible
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized;
    // returns true if executor with single_buffer
    fn is_single_buffer(&self) -> bool;
}

pub trait Builder<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    type ErrorType;

    // Add new circuit to built. arg_inputs - input that will be set by argument arg_input.
    fn add<T>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
        arg_inputs: Option<&[usize]>,
    ) where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.add_ext(
            name,
            circuit,
            input_placement,
            output_placement,
            arg_inputs,
            false,
        );
    }

    fn add_ext<T>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
        arg_inputs: Option<&[usize]>,
        single_buffer: bool,
    ) where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    fn add_simple<T>(&mut self, name: &str, circuit: Circuit<T>)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.add(name, circuit, None, None, None);
    }

    fn build(self) -> Result<Vec<E>, Self::ErrorType>;
    /// word length in bits
    fn word_len(&self) -> u32;
    // if no added circuit to built
    fn is_empty(&self) -> bool;
    /// executor can be used per thread
    fn is_executor_per_thread() -> bool;
    // preferred input count for this builder
    fn preferred_input_count(&self) -> usize;
}

// TODO: redesign mapper interface
pub trait MapperExecutor<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
{
    type ErrorType;

    // function: F(input data, output data, arg_input)
    fn execute<Out, F>(&mut self, input: &D, f: F) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(&D, &D, u32) -> Out;
}

pub trait MapperBuilder<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: MapperExecutor<'a, DR, DW, D>,
{
    type ErrorType;

    fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    fn build(self) -> Result<Vec<E>, Self::ErrorType>;

    /// word length in bits
    fn word_len(&self) -> u32;
}

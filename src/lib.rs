use gatesim::Circuit;

use int_enum::IntEnum;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeFrom};

// TODO: Add special Builder that for arg_input execute differently optimized circuit
// instead same - for 000xxxx - use circuit000, for 001xxxx use circuit001
// TODO: Add ability to build once circuits for many these same builders.
// TODO: add ability to execute in kernel circuit multiply times until some bit is not set.
// TODO: Add elem_index input bits (value are index of element).
// TODO: elem_inputs - add ability to set elem_index for all circuit inputs
// TODO: add elem_inputs to mappers (including ParSeqMapper).
// TODO: Add output aggregator to execution.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_writer;
pub mod cpu_build_exec;
pub mod cpu_data_transform;
pub mod cpu_machine;
pub mod div_build_exec;
mod divide;
pub mod gencode;
pub mod mapper;
pub mod opencl_build_exec;
pub mod opencl_data_transform;
pub mod opencl_machine;
pub mod parseq_mapper;
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
        elem_inputs: Option<&'a [usize]>,
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
        elem_inputs: Option<&'a [usize]>,
        single_buffer: bool,
    ) -> FW {
        // for checking requirements for single_buffer
        let real_input_len = if let Some((_, len)) = input_placement {
            len
        } else {
            input_len
                - arg_inputs.map(|x| x.len()).unwrap_or(0)
                - elem_inputs.map(|x| x.len()).unwrap_or(0)
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
            assert!(arg_inputs.len() <= 64);
            assert!(arg_inputs.iter().all(|x| *x < input_len));
        }
        if let Some(elem_inputs) = elem_inputs {
            assert!(elem_inputs.iter().all(|x| *x < input_len));
        }
        // check whether arg_input and elem_input have common inputs
        if let Some(arg_inputs) = arg_inputs {
            if let Some(elem_inputs) = elem_inputs {
                use std::collections::HashSet;
                let arg_input_set = HashSet::<usize>::from_iter(arg_inputs.iter().copied());
                let elem_input_set = HashSet::from_iter(elem_inputs.iter().copied());
                assert_eq!(arg_input_set.intersection(&elem_input_set).count(), 0);
            }
        }

        unsafe {
            self.func_writer_internal(
                name,
                input_len,
                output_len,
                input_placement,
                output_placement,
                arg_inputs,
                elem_inputs,
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
            None,
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
    fn get(&'a self) -> DR;
    fn get_mut(&'a mut self) -> DW;
    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out;
    fn process_mut<F, Out>(&mut self, f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out;
    /// release underlying data
    fn release(self) -> Vec<u32>;
    // free
    fn free(self);
}

pub trait RangedData {
    // set range
    fn set_range(&mut self, range: Range<usize>);
    #[inline]
    fn set_range_from(&mut self, range: RangeFrom<usize>) {
        self.set_range(range.start..usize::MAX);
    }
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

    unsafe fn execute_internal(&mut self, input: &D, arg_input: u64) -> Result<D, Self::ErrorType>;
    fn execute(&mut self, input: &D, arg_input: u64) -> Result<D, Self::ErrorType> {
        assert!(!self.is_single_buffer());
        unsafe { self.execute_internal(input, arg_input) }
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
    ) -> Result<(), Self::ErrorType>;
    fn execute_reuse(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
    ) -> Result<(), Self::ErrorType> {
        assert!(!self.is_single_buffer());
        unsafe { self.execute_reuse_internal(input, arg_input, output) }
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType>;
    fn execute_single(&mut self, output: &mut D, arg_input: u64) -> Result<(), Self::ErrorType> {
        assert!(self.is_single_buffer());
        unsafe { self.execute_single_internal(output, arg_input) }
    }
    /// Create new data - length is number of 32-bit words
    fn new_data(&mut self, len: usize) -> D;
    /// Create new data from vector.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// Create new data from slice.
    fn new_data_from_slice(&mut self, data: &[u32]) -> D;
    /// try clone executor if possible
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized;
    // returns true if executor with single_buffer
    fn is_single_buffer(&self) -> bool;

    fn word_len(&self) -> u32;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        if self.real_input_len() != 0 {
            assert_eq!(elem_num % (self.word_len() as usize), 0);
            (elem_num * self.real_input_len()) >> 5
        } else {
            1
        }
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        (elem_num * self.real_output_len()) >> 5
    }

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }
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
            None,
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
        elem_inputs: Option<&[usize]>,
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
    /// data holder can be used between any executor
    fn is_data_holder_global() -> bool;
    /// data holder can be used between any executor created by one builder
    fn is_data_holder_in_builder() -> bool;
    // preferred input count for this builder
    fn preferred_input_count(&self) -> usize;
}

pub trait MapperExecutor<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
{
    type ErrorType;

    /// Get circuit input length (number of inputs)
    fn input_len(&self) -> usize;
    /// Get real input length (number of entries in area of input placements)
    fn real_input_len(&self) -> usize;
    /// Get circuit output length (number of outputs)
    fn output_len(&self) -> usize;
    // function: F - main reduce function: F(input data, output data, arg_input)
    fn execute<Out, F, Stop>(
        &mut self,
        input: &D,
        init: Out,
        f: F,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, u64) -> Out,
        Stop: FnMut(&Out) -> bool;

    // function: F - main reduce function: F(input data, output data, arg_input)
    fn execute_direct<'b, Out: Clone, F, Stop>(
        &mut self,
        input: &'b D,
        init: Out,
        mut f: F,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &[u32], &[u32], u64) -> Out,
        Stop: FnMut(&Out) -> bool,
    {
        self.execute(
            input,
            init,
            |out, input, output, arg_input| {
                input.process(|inputx| {
                    output.process(|outputx| f(out.clone(), inputx, outputx, arg_input))
                })
            },
            stop,
        )
    }
    /// Create new data - length is number of 32-bit words
    fn new_data(&mut self, len: usize) -> D;
    /// Create new data from vector.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// Create new data from slice.
    fn new_data_from_slice(&mut self, data: &[u32]) -> D;

    fn word_len(&self) -> u32;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        (elem_num * self.real_input_len()) >> 5
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        (elem_num * self.output_len()) >> 5
    }

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }
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

    /// data holder can be used between any executor
    fn is_data_holder_global() -> bool;
    /// data holder can be used between any executor created by one builder
    fn is_data_holder_in_builder() -> bool;
    // preferred input count for this builder
    fn preferred_input_count(&self) -> usize;
}

pub trait ParMapperExecutor<'a, DR, DW, D>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
{
    type ErrorType;

    /// Get circuit input length (number of inputs)
    fn input_len(&self) -> usize;
    /// Get real input length (number of entries in area of input placements)
    fn real_input_len(&self) -> usize;
    /// Get circuit output length (number of outputs)
    fn output_len(&self) -> usize;
    /// execute. F - main reduce function: F(input data, output data, arg_input)
    /// G - main join function
    fn execute<Out, F, G, Stop>(
        &mut self,
        input: &D,
        init: Out,
        f: F,
        g: G,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: Fn(&D, &D, u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Stop: Fn(&Out) -> bool + Send + Sync,
        Out: Clone + Send + Sync;

    /// execute. F - main reduce function: F(input data, output data, arg_input)
    /// G - main join function
    fn execute_direct<'b, Out: Clone, F, G, Stop>(
        &mut self,
        input: &D,
        init: Out,
        f: F,
        g: G,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: Fn(&[u32], &[u32], u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Stop: Fn(&Out) -> bool + Send + Sync,
        Out: Clone + Send + Sync,
    {
        self.execute(
            input,
            init,
            |input, output, arg_input| {
                input.process(|inputx| output.process(|outputx| f(inputx, outputx, arg_input)))
            },
            g,
            stop,
        )
    }
    /// Create new data - length is number of 32-bit words
    fn new_data(&mut self, len: usize) -> D;
    /// Create new data from vector.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// Create new data from slice.
    fn new_data_from_slice(&mut self, data: &[u32]) -> D;

    fn word_len(&self) -> u32;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        (elem_num * self.real_input_len()) >> 5
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        (elem_num * self.output_len()) >> 5
    }

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }
}

pub trait ParMapperBuilder<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: ParMapperExecutor<'a, DR, DW, D>,
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

    /// data holder can be used between any executor
    fn is_data_holder_global() -> bool;
    /// data holder can be used between any executor created by one builder
    fn is_data_holder_in_builder() -> bool;
    // preferred input count for this builder
    fn preferred_input_count(&self) -> usize;
}

pub trait DataTransformer<'a, DR: DataReader, DW: DataWriter, D: DataHolder<'a, DR, DW>> {
    type ErrorType;

    fn transform(&mut self, input: &D) -> Result<D, Self::ErrorType>;
    fn transform_reuse(&mut self, input: &D, output: &mut D) -> Result<(), Self::ErrorType>;

    fn output_data_len(&self, len: usize) -> usize {
        assert_eq!(
            usize::try_from(
                (u128::try_from(len).unwrap() * u128::try_from(self.output_elem_len()).unwrap())
                    % u128::try_from(self.input_elem_len()).unwrap(),
            )
            .unwrap(),
            0
        );
        usize::try_from(
            (u128::try_from(len).unwrap() * u128::try_from(self.output_elem_len()).unwrap())
                / u128::try_from(self.input_elem_len()).unwrap(),
        )
        .unwrap()
    }

    // input elem length in bits
    fn input_elem_len(&self) -> usize;
    // output elem length in bits
    fn output_elem_len(&self) -> usize;
}

pub trait DataTransforms<'a, DR, DW, D, IDT, ODT>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    IDT: DataTransformer<'a, DR, DW, D>,
    ODT: DataTransformer<'a, DR, DW, D>,
{
    type ErrorType;

    fn input_tx(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType>;
    fn output_tx(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<ODT, Self::ErrorType>;
}

//
// Dynamic Circuit Machine traits
//
// Model execution:
// Execution unit as circuit. Circuit treat as sequential circuit:
// input: [state,mem_cell]
// output: [state,mem_cell,memrw,mem_address,mem_type,unsat,stop]
// Memory types:
// * private - (optional) smallest,
// * group - greater private
// * global - for all machine
// Dynamic execution: circuit type can choosen by using information in global memory:
// circuit_info (first bytes of global memory):
// 1. current_circuit - n-bit
// 2. register_circuit - n-bit - MAX if no registration
// 3. dump_circuit - n-bit - if not MAX - then circuit under given id will be placed in
//    circuit_data
// 4. if registration: circuit_data_address - n-bit
//
// Accepted cell lengths: 1, 2, 4, 8, 16, 32, 64, 128, 256.
//

pub struct MachineConfig {
    pub word_len: u32,
    pub address_len: u32,
    pub private_address_len: u32,
    pub group_address_len: u32,
    pub global_address_len: u32,
    pub cell_len_bits: u32,
    pub fast_group_len: u32,
    pub group_len: u32,
    pub machine_len: u32,
    pub circuit_info_word_len: u32,
}

pub trait MachineBuilder {
    fn config(&self) -> &MachineConfig;
}

pub trait MachineMemoryHandler<'a, DR: DataReader, DW: DataWriter, D: DataHolder<'a, DR, DW>> {
    fn read_mems(
        state_len: usize,
        mem_cell_start: usize,
        mem_address_start: usize,
        states: &mut D,
        private_mems: &D,
        group_mems: &D,
        global_mem: &D,
    );
    fn write_mems(
        state_len: usize,
        mem_cell_start: usize,
        mem_address_start: usize,
        states: &D,
        private_mems: &mut D,
        group_mems: &mut D,
        global_mem: &mut D,
    );
}

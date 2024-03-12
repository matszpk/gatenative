use gatesim::Circuit;

use int_enum::IntEnum;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeFrom};

// TODO: Add special Builder that for arg_input execute differently optimized circuit
// instead same - for 000xxxx - use circuit000, for 001xxxx use circuit001
// TODO: Add ability to build once circuits for many these same builders.
// TODO: add ability to execute in kernel circuit multiply times until some bit is not set.
// TODO: Add (transforming to CLang).
// TODO: Add output aggregation with same original output.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_transform;
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
pub mod utils;
mod vbinopcircuit;
mod vcircuit;

#[derive(Clone, Copy)]
pub struct CodeConfig<'a> {
    // determine place of circuit input bits in input bits and its length.
    // first: index - circuit input bit, value - destination input bit. second - input length.
    pub input_placement: Option<(&'a [usize], usize)>,
    // determine place of circuit output bits in input bits and its length.
    // first: index - circuit output bit, value - destination output bit. second - output length.
    pub output_placement: Option<(&'a [usize], usize)>,
    // determine what circuit input bits is assigned to argument passed to execute.
    pub arg_inputs: Option<&'a [usize]>,
    // determine what circuit input bits is assigned to element index.
    pub elem_inputs: Option<&'a [usize]>,
    // use single buffer to store input and output.
    pub single_buffer: bool,
    pub init_code: Option<&'a str>,
    // populate input code
    pub pop_input_code: Option<&'a str>,
    pub pop_input_len: Option<usize>,
    // aggregated output code - aggregates all outputs into single output.
    pub aggr_output_code: Option<&'a str>,
    pub aggr_output_len: Option<usize>, // length in 32-bit words
    // if some then some choosen circuit inputs populated from buffer.
    pub pop_from_buffer: Option<&'a [usize]>,
    // if some then aggregate some choosen circuit outputs to buffer.
    // and keep storing circuit outputs to original output buffer.
    pub aggr_to_buffer: Option<&'a [usize]>,
}

impl<'a> CodeConfig<'a> {
    pub fn new() -> Self {
        Self {
            input_placement: None,
            output_placement: None,
            arg_inputs: None,
            elem_inputs: None,
            single_buffer: false,
            init_code: None,
            pop_input_code: None,
            pop_input_len: None,
            aggr_output_code: None,
            aggr_output_len: None,
            pop_from_buffer: None,
            aggr_to_buffer: None,
        }
    }

    pub fn input_placement(mut self, p: Option<(&'a [usize], usize)>) -> Self {
        self.input_placement = p;
        self
    }
    pub fn output_placement(mut self, p: Option<(&'a [usize], usize)>) -> Self {
        self.output_placement = p;
        self
    }
    pub fn arg_inputs(mut self, arg: Option<&'a [usize]>) -> Self {
        self.arg_inputs = arg;
        self
    }
    pub fn elem_inputs(mut self, elem: Option<&'a [usize]>) -> Self {
        self.elem_inputs = elem;
        self
    }
    pub fn single_buffer(mut self, s: bool) -> Self {
        self.single_buffer = s;
        self
    }
    pub fn init_code(mut self, init: Option<&'a str>) -> Self {
        self.init_code = init;
        self
    }
    pub fn pop_input_code(mut self, pop: Option<&'a str>) -> Self {
        self.pop_input_code = pop;
        self
    }
    pub fn pop_input_len(mut self, pop: Option<usize>) -> Self {
        self.pop_input_len = pop;
        self
    }
    pub fn aggr_output_code(mut self, aggr: Option<&'a str>) -> Self {
        self.aggr_output_code = aggr;
        self
    }
    pub fn aggr_output_len(mut self, aggr: Option<usize>) -> Self {
        self.aggr_output_len = aggr;
        self
    }
    pub fn pop_from_buffer(mut self, pop: Option<&'a [usize]>) -> Self {
        self.pop_from_buffer = pop;
        self
    }
    pub fn aggr_to_buffer(mut self, aggr: Option<&'a [usize]>) -> Self {
        self.aggr_to_buffer = aggr;
        self
    }
}

pub fn default_aggr_output_len(word_len: u32) -> usize {
    (word_len as usize) >> 5
}
pub fn default_pop_input_len(word_len: u32) -> usize {
    (word_len as usize) >> 5
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
    /// Generates NOT operation.
    fn gen_not(&mut self, dst_arg: usize, arg: usize);
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
        let mut psorted = placement.to_vec();
        psorted.sort();
        psorted.dedup();
        assert_eq!(psorted.len(), placement.len());
    }
    if let Some((placement, len)) = output_placement {
        if placement.iter().any(|x| *x >= len) {
            return false;
        }
        let mut psorted = placement.to_vec();
        psorted.sort();
        psorted.dedup();
        assert_eq!(psorted.len(), placement.len());
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
    /// user definitions
    fn user_defs(&mut self, user_defs: &str);
    /// add transform helpers
    fn transform_helpers(&mut self);
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
        code_config: CodeConfig<'a>,
        output_vars: Option<Vec<usize>>,
    ) -> FW;

    fn func_writer_with_config(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        code_config: CodeConfig<'a>,
        output_vars: Option<Vec<usize>>,
    ) -> FW {
        // for checking requirements for single_buffer
        let real_input_len = if let Some((_, len)) = code_config.input_placement {
            len
        } else {
            input_len
                - code_config.arg_inputs.map(|x| x.len()).unwrap_or(0)
                - code_config.elem_inputs.map(|x| x.len()).unwrap_or(0)
        };
        let real_output_len = if let Some((_, len)) = code_config.output_placement {
            len
        } else {
            output_len
        };
        // check requirements for single buffer
        if !(code_config.pop_input_code.is_some()
            && code_config.aggr_output_code.is_some()
            && code_config.single_buffer)
        {
            assert!(!code_config.single_buffer || real_input_len == real_output_len);
        }
        assert!(check_placements(
            code_config.input_placement,
            code_config.output_placement
        ));
        if let Some(arg_inputs) = code_config.arg_inputs {
            assert!(arg_inputs.len() <= 64);
            assert!(arg_inputs.iter().all(|x| *x < input_len));
            let mut psorted = arg_inputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), arg_inputs.len());
        }
        if let Some(elem_inputs) = code_config.elem_inputs {
            assert!(elem_inputs.iter().all(|x| *x < input_len));
            let mut psorted = elem_inputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), elem_inputs.len());
        }
        // check whether arg_input and elem_input have common inputs
        if let Some(arg_inputs) = code_config.arg_inputs {
            if let Some(elem_inputs) = code_config.elem_inputs {
                use std::collections::HashSet;
                let arg_input_set = HashSet::<usize>::from_iter(arg_inputs.iter().copied());
                let elem_input_set = HashSet::from_iter(elem_inputs.iter().copied());
                assert_eq!(arg_input_set.intersection(&elem_input_set).count(), 0);
            }
        }

        if code_config.pop_input_code.is_some()
            && code_config.aggr_output_code.is_some()
            && code_config.single_buffer
        {
            assert_eq!(
                code_config.pop_input_len.unwrap(),
                code_config.aggr_output_len.unwrap()
            );
        } else if (code_config.pop_input_code.is_some() && code_config.pop_from_buffer.is_none())
            || (code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_none())
        {
            assert!(!code_config.single_buffer);
        }

        unsafe { self.func_writer_internal(name, input_len, output_len, code_config, output_vars) }
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
        self.func_writer_with_config(
            name,
            input_len,
            output_len,
            CodeConfig::new()
                .input_placement(input_placement)
                .output_placement(output_placement)
                .arg_inputs(arg_inputs),
            None,
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

    fn elem_count(&self, input_len: usize) -> usize;

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

    fn input_is_populated(&self) -> bool;
    fn output_is_aggregated(&self) -> bool;

    fn aggr_output_len(&self) -> Option<usize>;
    fn pop_input_len(&self) -> Option<usize>;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        if self.input_is_populated() {
            self.pop_input_len().unwrap()
        } else if self.real_input_len() != 0 {
            assert_eq!(elem_num % (self.word_len() as usize), 0);
            (elem_num * self.real_input_len()) >> 5
        } else {
            1
        }
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        if self.output_is_aggregated() {
            self.aggr_output_len().unwrap()
        } else {
            (elem_num * self.real_output_len()) >> 5
        }
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

    fn user_defs(&mut self, user_defs: &str);
    /// add transform helpers
    fn transform_helpers(&mut self);

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
        self.add_with_config(
            name,
            circuit,
            CodeConfig::new()
                .input_placement(input_placement)
                .output_placement(output_placement)
                .arg_inputs(arg_inputs),
        );
    }

    fn add_with_config<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
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

    fn elem_count(&self, input_len: usize) -> usize;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        if self.input_is_populated() {
            self.pop_input_len().unwrap()
        } else if self.real_input_len() != 0 {
            assert_eq!(elem_num % (self.word_len() as usize), 0);
            (elem_num * self.real_input_len()) >> 5
        } else {
            1
        }
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        if self.output_is_aggregated() {
            self.aggr_output_len().unwrap()
        } else {
            (elem_num * self.output_len()) >> 5
        }
    }

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }

    fn output_is_aggregated(&self) -> bool;
    fn input_is_populated(&self) -> bool;

    fn aggr_output_len(&self) -> Option<usize>;
    fn pop_input_len(&self) -> Option<usize>;
}

pub trait MapperBuilder<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: MapperExecutor<'a, DR, DW, D>,
{
    type ErrorType;

    fn user_defs(&mut self, user_defs: &str);
    /// add transform helpers
    fn transform_helpers(&mut self);

    unsafe fn add_internal<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    fn add_with_config<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        assert!(code_config.input_placement.is_none());
        assert!(code_config.output_placement.is_none());
        assert!(!code_config.single_buffer);
        assert!(code_config.arg_inputs.is_some());
        unsafe {
            self.add_internal(name, circuit, code_config);
        }
    }

    fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.add_with_config(
            name,
            circuit,
            CodeConfig::new().arg_inputs(Some(arg_inputs)),
        );
    }

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

    fn elem_count(&self, input_len: usize) -> usize;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        if self.input_is_populated() {
            self.pop_input_len().unwrap()
        } else if self.real_input_len() != 0 {
            assert_eq!(elem_num % (self.word_len() as usize), 0);
            (elem_num * self.real_input_len()) >> 5
        } else {
            1
        }
    }

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        if self.output_is_aggregated() {
            self.aggr_output_len().unwrap()
        } else {
            (elem_num * self.output_len()) >> 5
        }
    }

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }

    fn output_is_aggregated(&self) -> bool;
    fn input_is_populated(&self) -> bool;

    fn aggr_output_len(&self) -> Option<usize>;
    fn pop_input_len(&self) -> Option<usize>;
}

pub trait ParMapperBuilder<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: ParMapperExecutor<'a, DR, DW, D>,
{
    type ErrorType;

    fn user_defs(&mut self, user_defs: &str);
    /// add transform helpers
    fn transform_helpers(&mut self);

    unsafe fn add_internal<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    fn add_with_config<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        assert!(code_config.input_placement.is_none());
        assert!(code_config.output_placement.is_none());
        assert!(!code_config.single_buffer);
        assert!(code_config.arg_inputs.is_some());
        unsafe {
            self.add_internal(name, circuit, code_config);
        }
    }

    fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.add_with_config(
            name,
            circuit,
            CodeConfig::new().arg_inputs(Some(arg_inputs)),
        );
    }

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

    fn input_transformer(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType>;
    fn output_transformer(
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

use gatesim::Circuit;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeFrom};

// TODO: Add special Builder that for arg_input execute differently optimized circuit
// instead same - for 000xxxx - use circuit000, for 001xxxx use circuit001
// TODO: Add ability to build once circuits for many these same builders.
// TODO: add ability to execute in kernel circuit multiply times until some bit is not set.
// TODO: CLangWriter: add handling array-like types to handle longer words and
// groupped executions. Hint: Add parameter to configs, write transparent to CLangWriterConfigs.
// Next hint: use embed array to structure to better handling types in C.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    NoNegs,
    NegInput1, // second input in gate
    NegOutput,
}

pub mod clang_data_transform;
pub mod clang_transform;
pub mod clang_writer;
pub mod cpu_build_exec;
pub mod cpu_data_transform;
pub mod div_build_exec;
mod divide;
pub mod gencode;
pub mod mapper;
pub mod opencl_build_exec;
pub mod opencl_data_transform;
pub mod parseq_mapper;
pub mod utils;
mod vbinopcircuit;
mod vcircuit;
mod vlop3circuit;
mod vlop3circuit2;
mod vlop3circuit3;

pub use opencl3;

#[derive(Clone, Copy, Debug)]
pub struct CodeConfig<'a> {
    // determine place of circuit input bits in input bits and its length.
    // first: index - circuit input bit, value - destination input bit. second - input length.
    pub input_placement: Option<(&'a [usize], usize)>,
    // determine place of circuit output bits in input bits and its length.
    // first: index - circuit output bit, value - destination output bit. second - output length.
    pub output_placement: Option<(&'a [usize], usize)>,
    // determine what circuit input bits are assigned to argument passed to execute.
    pub arg_inputs: Option<&'a [usize]>,
    // determine what circuit input bits are assigned to element index.
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
    // exclude outputs
    pub exclude_outputs: Option<&'a [usize]>,
    // applied to BasicMapper and ParSeqMapper - if true then aggregated output buffer
    // will not be cleared before single execution (to save time) and content of this buffer
    // will be kept to later use.
    pub dont_clear_outputs: bool,
    // Enable inner loop. pop_input_code and aggr_output_code are inside loop.
    // Loop adds stop and iter variables to code:
    // stop - can be set by user supplied code. If have nonzero value then
    //   loop should be stopped.
    // iter - current loop iteration number starts from 0.
    // iter_max - read-only - number of iterations
    // Supplied parameter is max iteration number.
    // If pop_input_code and aggr_output_code are executed for all iterations.
    // User should put own code in required conditional blocks if it is needed.
    // Aggr_output_code is executed before input update and iteration check and
    // this code can have modify 'stop' variable to stop iteration.
    // Because circuit are executed for word unconditionally then any conditional
    // loop stopping at circuit must be implemented at same circuit.
    pub inner_loop: Option<u32>,
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
            exclude_outputs: None,
            dont_clear_outputs: false,
            inner_loop: None,
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
    pub fn exclude_outputs(mut self, excl: Option<&'a [usize]>) -> Self {
        self.exclude_outputs = excl;
        self
    }
    pub fn aggr_only_to_buffer(mut self, aggr: Option<&'a [usize]>) -> Self {
        self.aggr_to_buffer = aggr;
        self.exclude_outputs = aggr;
        self
    }
    pub fn dont_clear_outputs(mut self, ignore: bool) -> Self {
        self.dont_clear_outputs = ignore;
        self
    }
    pub fn inner_loop(mut self, l: Option<u32>) -> Self {
        self.inner_loop = l;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CodeConfigCopy {
    pub input_placement: Option<(Vec<usize>, usize)>,
    pub output_placement: Option<(Vec<usize>, usize)>,
    pub arg_inputs: Option<Vec<usize>>,
    pub elem_inputs: Option<Vec<usize>>,
    pub single_buffer: bool,
    pub init_code: Option<String>,
    pub pop_input_code: Option<String>,
    pub pop_input_len: Option<usize>,
    pub aggr_output_code: Option<String>,
    pub aggr_output_len: Option<usize>, // length in 32-bit words
    pub pop_from_buffer: Option<Vec<usize>>,
    pub aggr_to_buffer: Option<Vec<usize>>,
    pub exclude_outputs: Option<Vec<usize>>,
    pub dont_clear_outputs: bool,
    pub inner_loop: Option<u32>,
}

impl CodeConfigCopy {
    pub fn to_ref(&self) -> CodeConfig {
        CodeConfig {
            input_placement: self.input_placement.as_ref().map(|x| (x.0.as_slice(), x.1)),
            output_placement: self
                .output_placement
                .as_ref()
                .map(|x| (x.0.as_slice(), x.1)),
            arg_inputs: self.arg_inputs.as_ref().map(|x| x.as_slice()),
            elem_inputs: self.elem_inputs.as_ref().map(|x| x.as_slice()),
            single_buffer: self.single_buffer,
            init_code: self.init_code.as_ref().map(|x| x.as_str()),
            pop_input_code: self.pop_input_code.as_ref().map(|x| x.as_str()),
            pop_input_len: self.pop_input_len,
            aggr_output_code: self.aggr_output_code.as_ref().map(|x| x.as_str()),
            aggr_output_len: self.aggr_output_len,
            pop_from_buffer: self.pop_from_buffer.as_ref().map(|x| x.as_slice()),
            aggr_to_buffer: self.aggr_to_buffer.as_ref().map(|x| x.as_slice()),
            exclude_outputs: self.exclude_outputs.as_ref().map(|x| x.as_slice()),
            dont_clear_outputs: self.dont_clear_outputs,
            inner_loop: self.inner_loop,
        }
    }

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
            exclude_outputs: None,
            dont_clear_outputs: false,
            inner_loop: None,
        }
    }

    pub fn input_placement(mut self, p: Option<(Vec<usize>, usize)>) -> Self {
        self.input_placement = p;
        self
    }
    pub fn output_placement(mut self, p: Option<(Vec<usize>, usize)>) -> Self {
        self.output_placement = p;
        self
    }
    pub fn arg_inputs(mut self, arg: Option<Vec<usize>>) -> Self {
        self.arg_inputs = arg;
        self
    }
    pub fn elem_inputs(mut self, elem: Option<Vec<usize>>) -> Self {
        self.elem_inputs = elem;
        self
    }
    pub fn single_buffer(mut self, s: bool) -> Self {
        self.single_buffer = s;
        self
    }
    pub fn init_code(mut self, init: Option<String>) -> Self {
        self.init_code = init;
        self
    }
    pub fn pop_input_code(mut self, pop: Option<String>) -> Self {
        self.pop_input_code = pop;
        self
    }
    pub fn pop_input_len(mut self, pop: Option<usize>) -> Self {
        self.pop_input_len = pop;
        self
    }
    pub fn aggr_output_code(mut self, aggr: Option<String>) -> Self {
        self.aggr_output_code = aggr;
        self
    }
    pub fn aggr_output_len(mut self, aggr: Option<usize>) -> Self {
        self.aggr_output_len = aggr;
        self
    }
    pub fn pop_from_buffer(mut self, pop: Option<Vec<usize>>) -> Self {
        self.pop_from_buffer = pop;
        self
    }
    pub fn aggr_to_buffer(mut self, aggr: Option<Vec<usize>>) -> Self {
        self.aggr_to_buffer = aggr;
        self
    }
    pub fn exclude_outputs(mut self, excl: Option<Vec<usize>>) -> Self {
        self.exclude_outputs = excl;
        self
    }
    pub fn aggr_only_to_buffer(mut self, aggr: Option<Vec<usize>>) -> Self {
        self.aggr_to_buffer = aggr.clone();
        self.exclude_outputs = aggr;
        self
    }
    pub fn dont_clear_outputs(mut self, ignore: bool) -> Self {
        self.dont_clear_outputs = ignore;
        self
    }
    pub fn inner_loop(mut self, l: Option<u32>) -> Self {
        self.inner_loop = l;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstrOp {
    And,
    Or,
    Impl,
    Nimpl,
    Xor,
    Lop3(u8),
}

impl InstrOp {
    pub fn int_value(self) -> usize {
        match self {
            InstrOp::And => 0,
            InstrOp::Or => 1,
            InstrOp::Impl => 2,
            InstrOp::Nimpl => 3,
            InstrOp::Xor => 4,
            InstrOp::Lop3(_) => 5,
        }
    }
    pub fn arg_num(self) -> usize {
        if matches!(self, InstrOp::Lop3(_)) {
            3
        } else {
            2
        }
    }
}

pub const INSTR_OP_VALUE_AND: u64 = 0;
pub const INSTR_OP_VALUE_OR: u64 = 1;
pub const INSTR_OP_VALUE_IMPL: u64 = 2;
pub const INSTR_OP_VALUE_NIMPL: u64 = 3;
pub const INSTR_OP_VALUE_XOR: u64 = 4;
pub const INSTR_OP_VALUE_LOP3: u64 = 5;

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
    /// Generates operation.
    fn gen_op3(&mut self, op: InstrOp, dst_arg: usize, arg0: usize, arg1: usize, arg2: usize);
    /// Generates NOT operation.
    fn gen_not(&mut self, dst_arg: usize, arg: usize);
    /// Generates Store instruction into output.
    fn gen_store(&mut self, neg: bool, output: usize, reg: usize);
    /// Generates copy to register
    fn gen_set(&mut self, dst_arg: usize, arg: usize);

    /// Generates conditional for start of loop
    fn gen_if_loop_start(&mut self);
    /// Generates conditional for end of loop
    fn gen_if_loop_end(&mut self);
    /// Generates conditional for end of loop
    fn gen_else(&mut self);
    /// Generates end of conditional
    fn gen_end_if(&mut self);
    /// Generates aggr_output_code
    fn gen_aggr_output_code(&mut self);
}

fn check_placements(
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&[usize], usize)>,
    output_placement: Option<(&[usize], usize)>,
) -> bool {
    if let Some((placement, len)) = input_placement {
        if placement.len() != input_len {
            return false;
        }
        if placement.iter().any(|x| *x >= len) {
            return false;
        }
        let mut psorted = placement.to_vec();
        psorted.sort();
        psorted.dedup();
        assert_eq!(psorted.len(), placement.len());
    }
    if let Some((placement, len)) = output_placement {
        if placement.len() != output_len {
            return false;
        }
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
        output_vars: Option<Vec<(usize, usize)>>,
    ) -> FW;

    fn func_writer_with_config(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        code_config: CodeConfig<'a>,
        output_vars: Option<Vec<(usize, usize)>>,
    ) -> FW {
        if code_config.pop_input_code.is_some() && code_config.aggr_output_code.is_some() {
            assert!(code_config.pop_from_buffer.is_some() == code_config.aggr_to_buffer.is_some());
        }
        // for checking requirements for single_buffer
        let input_len_after_removal = input_len
            - code_config.arg_inputs.map(|x| x.len()).unwrap_or(0)
            - code_config.elem_inputs.map(|x| x.len()).unwrap_or(0)
            - code_config.pop_from_buffer.map(|x| x.len()).unwrap_or(0);
        let real_input_len = if let Some((_, len)) = code_config.input_placement {
            len
        } else {
            input_len_after_removal
        };
        let output_len_after_removal =
            output_len - code_config.exclude_outputs.map(|x| x.len()).unwrap_or(0);
        let real_output_len = if let Some((_, len)) = code_config.output_placement {
            len
        } else {
            output_len_after_removal
        };
        let pop_input_same =
            code_config.pop_input_code.is_some() && code_config.pop_from_buffer.is_none();
        let aggr_output_same =
            code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_none();
        // check requirements for single buffer
        if !(pop_input_same && aggr_output_same && code_config.single_buffer) {
            assert!(!code_config.single_buffer || real_input_len == real_output_len);
        }
        assert!(check_placements(
            input_len_after_removal,
            output_len_after_removal,
            code_config.input_placement,
            code_config.output_placement
        ));
        if let Some(exclude_outputs) = code_config.exclude_outputs {
            assert_ne!(exclude_outputs.len(), 0);
            assert!(exclude_outputs.iter().all(|x| *x < output_len));
            let mut psorted = exclude_outputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), exclude_outputs.len());
        }
        if let Some(arg_inputs) = code_config.arg_inputs {
            assert_ne!(arg_inputs.len(), 0);
            assert!(arg_inputs.len() <= 64);
            assert!(arg_inputs.iter().all(|x| *x < input_len));
            let mut psorted = arg_inputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), arg_inputs.len());
        }
        if let Some(elem_inputs) = code_config.elem_inputs {
            assert_ne!(elem_inputs.len(), 0);
            assert!(elem_inputs.iter().all(|x| *x < input_len));
            let mut psorted = elem_inputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), elem_inputs.len());
        }
        if let Some(pop_inputs) = code_config.pop_from_buffer {
            assert_ne!(pop_inputs.len(), 0);
            assert!(pop_inputs.iter().all(|x| *x < input_len));
            let mut psorted = pop_inputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), pop_inputs.len());
        }
        if let Some(aggr_outputs) = code_config.aggr_to_buffer {
            assert_ne!(aggr_outputs.len(), 0);
            assert!(aggr_outputs.iter().all(|x| *x < output_len));
            let mut psorted = aggr_outputs.to_vec();
            psorted.sort();
            psorted.dedup();
            assert_eq!(psorted.len(), aggr_outputs.len());
        }
        // check whether arg_input and elem_input have common inputs
        if let Some(arg_inputs) = code_config.arg_inputs {
            if let Some(elem_inputs) = code_config.elem_inputs {
                let arg_input_set = HashSet::<usize>::from_iter(arg_inputs.iter().copied());
                let elem_input_set = HashSet::from_iter(elem_inputs.iter().copied());
                assert_eq!(arg_input_set.intersection(&elem_input_set).count(), 0);
            }
            if let Some(pop_inputs) = code_config.pop_from_buffer {
                let arg_input_set = HashSet::<usize>::from_iter(arg_inputs.iter().copied());
                let pop_input_set = HashSet::from_iter(pop_inputs.iter().copied());
                assert_eq!(arg_input_set.intersection(&pop_input_set).count(), 0);
            }
        }
        if let Some(elem_inputs) = code_config.elem_inputs {
            if let Some(pop_inputs) = code_config.pop_from_buffer {
                let elem_input_set = HashSet::<usize>::from_iter(elem_inputs.iter().copied());
                let pop_input_set = HashSet::from_iter(pop_inputs.iter().copied());
                assert_eq!(elem_input_set.intersection(&pop_input_set).count(), 0);
            }
        }

        if pop_input_same && aggr_output_same && code_config.single_buffer {
            assert_eq!(
                code_config.pop_input_len.unwrap(),
                code_config.aggr_output_len.unwrap()
            );
        } else if pop_input_same || aggr_output_same {
            assert!(!code_config.single_buffer);
        }

        // inner loop checking
        if let Some(max_iter) = code_config.inner_loop {
            assert!(max_iter >= 1);
            assert_eq!(input_len_after_removal, output_len_after_removal);
            // check mathing placements
            if let Some((input_p, _)) = code_config.input_placement {
                if let Some((output_p, _)) = code_config.output_placement {
                    for idx in input_p.iter() {
                        assert!(output_p.iter().find(|oidx| **oidx == *idx).is_some());
                    }
                } else {
                    for idx in 0..input_len_after_removal {
                        assert!(input_p.iter().find(|iidx| **iidx == idx).is_some());
                    }
                }
            } else if let Some((output_p, _)) = code_config.output_placement {
                for idx in 0..input_len_after_removal {
                    assert!(output_p.iter().find(|oidx| **oidx == idx).is_some());
                }
            }
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
    fn copy(&self) -> Self;
    fn fill(&mut self, value: u32);
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
        assert!(!(self.input_is_populated() && self.is_populated_from_buffer()));
        assert!(!(self.output_is_aggregated() && self.is_aggregated_to_buffer()));
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
        assert!(!(self.input_is_populated() && self.is_populated_from_buffer()));
        assert!(!(self.output_is_aggregated() && self.is_aggregated_to_buffer()));
        unsafe { self.execute_reuse_internal(input, arg_input, output) }
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType>;
    fn execute_single(&mut self, output: &mut D, arg_input: u64) -> Result<(), Self::ErrorType> {
        assert!(self.is_single_buffer());
        assert!(!(self.input_is_populated() && self.is_populated_from_buffer()));
        assert!(!(self.output_is_aggregated() && self.is_aggregated_to_buffer()));
        unsafe { self.execute_single_internal(output, arg_input) }
    }

    // executes for additional buffers
    unsafe fn execute_buffer_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<D, Self::ErrorType>;
    fn execute_buffer(
        &mut self,
        input: &D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<D, Self::ErrorType> {
        assert!(!self.is_single_buffer());
        assert!(
            (self.input_is_populated() && self.is_populated_from_buffer())
                || (self.output_is_aggregated() && self.is_aggregated_to_buffer())
        );
        unsafe { self.execute_buffer_internal(input, arg_input, buffer) }
    }

    unsafe fn execute_buffer_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType>;
    fn execute_buffer_reuse(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType> {
        assert!(!self.is_single_buffer());
        assert!(
            (self.input_is_populated() && self.is_populated_from_buffer())
                || (self.output_is_aggregated() && self.is_aggregated_to_buffer())
        );
        unsafe { self.execute_buffer_reuse_internal(input, arg_input, output, buffer) }
    }

    unsafe fn execute_buffer_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType>;
    fn execute_buffer_single(
        &mut self,
        output: &mut D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType> {
        assert!(self.is_single_buffer());
        assert!(
            (self.input_is_populated() && self.is_populated_from_buffer())
                || (self.output_is_aggregated() && self.is_aggregated_to_buffer())
        );
        unsafe { self.execute_buffer_single_internal(output, arg_input, buffer) }
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
    fn is_populated_from_buffer(&self) -> bool;
    fn output_is_aggregated(&self) -> bool;
    fn is_aggregated_to_buffer(&self) -> bool;

    fn aggr_output_len(&self) -> Option<usize>;
    fn pop_input_len(&self) -> Option<usize>;

    // in 32-bit words
    fn input_data_len(&self, elem_num: usize) -> usize {
        if self.input_is_populated() && !self.is_populated_from_buffer() {
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
        if self.output_is_aggregated() && !self.is_aggregated_to_buffer() {
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

    fn dont_clear_outputs(&self) -> bool;

    fn need_clear_outputs(&self) -> bool {
        self.output_is_aggregated() && !self.is_aggregated_to_buffer() && !self.dont_clear_outputs()
    }

    // return true if sequential execution
    fn is_sequential_execution(&self) -> bool;

    fn inner_loop(&self) -> Option<u32>;
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
    /// type length in bits (includes only type length not word length if group_vec enabled).
    fn type_len(&self) -> u32;
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
    // executes for additional buffers
    fn execute_buffer<Out, F, Stop>(
        &mut self,
        input: &D,
        buffer: &mut D,
        init: Out,
        f: F,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, &D, u64) -> Out,
        Stop: FnMut(&Out) -> bool;

    fn execute_buffer_direct<'b, Out: Clone, F, Stop>(
        &mut self,
        input: &'b D,
        buffer: &'b mut D,
        init: Out,
        mut f: F,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &[u32], &[u32], &[u32], u64) -> Out,
        Stop: FnMut(&Out) -> bool,
    {
        self.execute_buffer(
            input,
            buffer,
            init,
            |out, input, output, buf_output, arg_input| {
                input.process(|inputx| {
                    output.process(|outputx| {
                        buf_output.process(|buf_outputx| {
                            f(out.clone(), inputx, outputx, buf_outputx, arg_input)
                        })
                    })
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
    fn input_data_len(&self, elem_num: usize) -> usize;

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize;

    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }

    fn output_is_aggregated(&self) -> bool;
    fn is_aggregated_to_buffer(&self) -> bool;
    fn input_is_populated(&self) -> bool;
    fn is_populated_from_buffer(&self) -> bool;

    fn aggr_output_len(&self) -> Option<usize>;
    fn pop_input_len(&self) -> Option<usize>;

    // return true if sequential execution in single execution call of inner executor
    fn is_sequential_execution(&self) -> bool;

    fn inner_loop(&self) -> Option<u32>;
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
    /// type length in bits (includes only type length not word length if group_vec enabled).
    fn type_len(&self) -> u32;

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
    fn input_data_len(&self, elem_num: usize) -> usize;

    // in 32-bit words
    fn output_data_len(&self, elem_num: usize) -> usize;

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

    // return true if sequential execution in single execution call of inner executor
    fn is_sequential_execution(&self) -> bool;

    fn inner_loop(&self) -> Option<u32>;
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
    /// type length in bits (includes only type length not word length if group_vec enabled).
    fn type_len(&self) -> u32;

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

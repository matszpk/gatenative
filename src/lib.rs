#![cfg_attr(docsrs, feature(doc_cfg))]
//! The library allows to execute simulation of the Gate circuit on CPU or GPU (by using OpenCL).
//! It provides complex configuration of execution including passing data circuit's inputs.
//! This library executes parallel simulation of circuit that runs many circuits.
//! The execution organized as threads (elements) in single execution. One circuit simulation
//! mapped to one bit of word of processor. For modern CPU a word can have 64 to 512 bits.
//! The library uses vector processing instruction to run simulation efficiently on CPU.
//!
//! Additional feature is ability to write code to process passing and processing inputs before
//! calling the simulation and write code to process outputs after calling the simulation.
//! The code that process inputs called as 'populating code' and the code that process
//! outputs called as 'aggregating code'.
//!
//! Next feature is ability to make loop for execution of simulation. Inside that loop
//! is possible to process inputs and outputs between simulations and use a GPU local memory.
//! Just it possible to call multiple simulation executions (iterations) under single
//! kernel call.
//!
//! This library organizes simulation's execution in two steps: in the first step build code to
//! execute simulation that native for processor or GPU, in second step just execute built code
//! to make simulation.
//!
//! The organization of simulation is shown under image:
//! ```text
//! +---------------------------------------------------------------------------------+
//! | SIM(0)....SIM(N-1) | SIM(N)....SIM(2*N-1) | ..... | SIM((K-1)*N)....SIM((K)*N-1)|
//! +---------------------------------------------------------------------------------+
//! ```
//! In this image `SIM(X)` is Xth simulation. Simulation are groupped into processor words
//! with length N bits and all execution contains K*N simulation. If a processor has
//! 256-bit word then can execute 256 simulations under single thread.
//! A GPU uses only 32-bit words, however simulation also groupped by group size (group length).
//! that can have even 256 threads. A special option treat whole group as processor's word,
//! however in many cases is not usable.
//!
//! Circuit's inputs and circuit's outputs are organized as pack of processor words that
//! groupped in greater stream. Data can contain more that packs if number of elements
//! is greater than number of bits of processor word.
//!
//! ```text
//! +----------------------------------------------------------------------------------------+
//! |D(0)(0)B(0)...D(0)(0)B(N)|D(0)(1)B(0)...D(0)(1)B(N)|............|D(0)(1)B0...D(0)(1)B(N)|
//! +----------------------------------------------------------------------------------------+
//! |D(1)(0)B(0)...D(1)(0)B(N)|D(1)(1)B(0)...D(1)(1)B(N)|............|D(1)(1)B0...D(1)(1)B(N)|
//! +----------------------------------------------------------------------------------------+
//! |........................................................................................|
//! +----------------------------------------------------------------------------------------+
//! |D(T)(0)B(0)...D(T)(0)B(N)|D(T)(1)B(0)...D(T)(1)B(N)|............|D(T)(1)B0...D(T)(1)B(N)|
//! +----------------------------------------------------------------------------------------+
//! ```
//! D(I)(X)B(Y) - Yth bit in Xth pack element in Ith group. That bit assigned to I*N+Y element
//! (thread).
//!
//! By default ith pack element assigned to ith circuit's input or ith circuit's output.
//! It can be changed by using input placement or output placement. Number of element in single
//! execution should be divisible by number of bit of word processor.
//!
//! Input data or output data organized as bits in processor word. One bit per one
//! element (thread). If you want convert data organized as packs from/to data organized per
//! bit you should use data transformer.
//!
//! In this library it used terminology:
//! * Builder - object to built code for simulate circuits. Builder can hold many
//!   simulation configurations for same circuit.
//! * Code configuration - It holds circuit's inputs and outputs configuration,
//!   populating and aggregating code, loop setup, etc.
//! * Execution - execution of simulations with specified size that will be run under
//!   single execution on GPU (as single execution of kernel) or CPU.
//! * Executor - object to call simulation. Single executor per single circuit.
//! * MapperBuilder - builder to simplify multiple execution with more elements
//!   than can have single simulation.
//! * MapperExecutor - executor that execute multiple simulations.
//! * Data holder - object that holds data used while simulation
//!   (as input or output or other data). Data will be in device that will run simulation.
//! * Data reader - object that allows read data from data holder.
//! * Data writer - object that allows write data in data holder.
//! * Populating input code - code in the C language (or OpenCL C) that generate data to
//!   populate for some specified circuit's inputs.
//! * Aggregating output code - code in the C language (or OpenCL C) that process output
//!   data from some specified circuit's outputs.
//! * Word - generally is processor's word , however if `group_len` is set then
//!   it is multipla: group_len*word_len.
//! * Type in Code - processor's word used while executing simulation. It is used in native code
//!   and in a populating and an aggregating code.
//! * Element - single simulation.
//! * Element index - index of simulation.
//! * Element input - circuit's input that value is element index.
//! * Argument input - circuit's input that obtained from argument from execution call.
//! * FuncWriter - trait defines object to write native code of function.
//! * CodeWriter - trait defines object to write native code.
//! * Data transformer - object to convert input data for circuit simulation and
//!   convert output data from circuit simulation. Because single simulation done by one bit
//!   of word then data should be converted into packs (pack per element).
//! * Pack element - part of data that assigned to one circuit's input or circuit's output.
//!
//! Program should make few steps to run simulation:
//! 1. Create builder.
//! 2. Add circuits and their configurations that includes input and output setup, additional
//!    code to process input and output data before and after simulation.
//! 3. Built executors.
//! 4. Prepare input data by using input data transformer if needed. Also it can put additional
//!    data for populating code in buffers.
//! 5. Execute simulation.
//! 6. Retrieve output data by using output data transformer if needed. Also it can get additional
//!    data processed by aggregating code.
//!
//! The library reads environment variable to get important setup:
//! * `GATE_SYS_DUMP_SOURCE` - if set to 1 then GateNative prints source code for simulation.
//! * `GATE_SYS_CC` - path to C compiler that will be used while building code for simulation.

use gatesim::Circuit;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Range, RangeFrom};

// TODO: Add special Builder that for arg_input execute differently optimized circuit
// instead same - for 000xxxx - use circuit000, for 001xxxx use circuit001
// TODO: Add ability to build once circuits for many these same builders.
// TODO: add ability to execute in kernel circuit multiply times until some bit is not set.
// TODO: CLangWriter: add handling array-like types to handle longer words and
// groupped executions. Hint: Add parameter to configs, write transparent to CLangWriterConfigs.
// Next hint: use embed array to structure to better handling types in C.

/// Type negations for mainy internal usage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VNegs {
    /// No negation.
    NoNegs,
    /// Negate second input of gate.
    NegInput1,
    /// Negate output of gate.
    NegOutput,
}

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

pub use gatesim;
pub use libloading;
pub use opencl3;
pub use rayon;

/// Main structure to describe code configuration.
///
/// This structure provides assignment for circuit's inputs and circuit's outputs,
/// a polulating code, an aggregating code, loop setup, buffer setup.
///
/// Circuits inputs assigned to following sources:
/// * provided data through execution call (as data).
/// * single argument value provided through execution call.
/// * element index (thread index).
/// * populating code that can process data or data in additional buffer.
///
/// Circuit's outputs assigned to output data returned by execution call (as data).
/// Some circuit's outputs can be excluded if aggregation code uses some outputs.
///
/// All four sources for circuit's inputs must be defined exclusively (no shared circuit's inputs).
///
/// Input placement is setup refers to circuit's inputs that don't have assginment to
/// other sources than assignment to provided data. Input placement contains list and number of
/// total number of pack elements of input data. Map is list where index is circuit's input index,
/// and value is destination pack element. Similary, output placement contains list of
/// placement and total number of pack elements of output data.
/// Circuit inputs and circuit's outputs are numbered from 0 in original order in that list
/// (if circuit have 5 inputs and 1,3 are assigned to element index then inputs 0,2,4 have
/// numberes 0,1,3 after removal).
///
/// For example (&[5, 1, 4, 2], 7) - data have 7 pack elements, and circuit's input 0
/// assigned to pack element 1 (starting from 0), circuit's input 1 to pack element 1,
/// circuit's input 2 to pack element 4 and circuit's input 3 to pack element 2.
///
/// In the most cases no reasons to use input/output placement.
///
/// A populating input code (`pop_input_code`) is code supplied by user and written in
/// C language (or OpenCL C) to obtain circuit's input data from some source. Next `pop_from_buffer`
/// field is list of circuit's inputs that obtained by a pop_input_code. If `pop_from_buffer`
/// is not supplied then all circuit's inputs (except circuit's inputs assigned to other sources)
/// are obtained by pop_input_code and no other circuit's inputs are assigned to data.
/// If `pop_from_buffer` is supplied then only listed circuit's inputs will be obtained from
/// pop_input_code. If pop_from_buffer is supplied then pop_input_code should read data from
/// additional buffer, otherwise it should read data from input data buffer.
/// `pop_input_len` specifies length of source in 32-bit words. That length shouldn't be
/// exceeded.
///
/// An aggregating output code (`aggr_output_code`) is code supplied by user and written in
/// C language (or OpenCL C) to process circuit's outputs and store results into some destination.
/// Next `aggr_to_buffer` field is list of circuit's outputs that will be processed by
/// aggr_output_code. If `aggr_to_buffer` is not supplied then all circuit's outputs will be
/// processed by aggr_output_code and no other circuit's outputs are assigned to data.
/// If `aggr_to_buffer` is supplied then only listed circuit's outputs will be processed by
/// aggr_output_code. Special field `exclude_outputs` allows to exclude circuit's outputs
/// (mainly assigned to aggr_output_code). If aggr_to_buffer is supplied then aggr_output_code
/// it should write data to additional buffer, otherwise to output data buffer.
/// `aggr_output_len` specifies length of destination in 32-bit words. That length shouldn't be
/// exceeded.
///
/// Interface for pop_input_code and aggr_output_code is simple.
/// A variable `iX` refers to circuit's input X. A variable `oX` refers to circuit's output X.
/// Defined `TYPE_NAME` defines name of Type In Code. `TYPE_LEN` is length of type in bits.
/// * `input` is input data normally holds circuit's input data.
/// * `output` is output data holds circuit's output data.
/// * `buffer` is additional buffer for a pop_input_code or an aggr_output_code.
/// * `GET_U32(D,X,I)` gets ith 32-bit word stored in X variable of type Type In Code
/// and store to `D` - 32-bit unsigned integer.
/// * `GET_U32_ALL(D,X)` gets all words stored in X variable of type Type In Code and store to
/// * `D` - array of 32-bit unsigned integers.
/// * `SET_U32(X,S,I)` sets `S` value to ith 32-bit word in X variable of type Type In Code.
/// * `SET_U32_ALL(X,S)` sets `S` 32-bit words to X variable of type Type In Code.
/// * `idx` is index of pack in data.
/// * `arg` is lower half of 64-bit argument.
/// * `arg2` is higher half of 64-bit argument.
/// * `lidx` (only for OpenCL C) is index of local thread (word) in group.
///
/// About inner loop. Inner loop can be enabled by `inner_loop` field.
//  pop_input_code and aggr_output_code are inside loop.
/// Loop adds stop and iter variables to code:
/// * stop - can be set by user supplied code. If have nonzero value then loop should be stopped.
/// * iter - current loop iteration number starts from 0.
/// * iter_max - read-only - number of iterations
///
/// `inner_loop` supplied parameter is max iteration number.
/// If pop_input_code and aggr_output_code are executed for all iterations.
/// User should put own code in required conditional blocks if it is needed.
/// Aggr_output_code is executed before input update and iteration check and
/// this code can have modify 'stop' variable to stop iteration.
/// Because circuit are executed for word unconditionally then any conditional
/// loop stopping at circuit must be implemented at same circuit.
#[derive(Clone, Copy, Debug)]
pub struct CodeConfig<'a> {
    /// Input placement. See main description of structure.
    pub input_placement: Option<(&'a [usize], usize)>,
    /// Output placement. See main description of structure.
    pub output_placement: Option<(&'a [usize], usize)>,
    /// Arg inputs is list of circuit's inputs assigned to argument inputs. Index is bit of
    /// argument and value is cicrcuit's input index.
    pub arg_inputs: Option<&'a [usize]>,
    /// Elem inputs is list of circuit's inputs assigned to element index. Index is bit of
    /// element index and value is cicrcuit's input index.
    pub elem_inputs: Option<&'a [usize]>,
    /// Use single buffer that two buffers (for input and and output). In this case
    /// input placement and output placement must have these same number of pack elements.
    pub single_buffer: bool,
    /// Additional initialization code in C language (or OpenCL C).
    pub init_code: Option<&'a str>,
    /// A pop_input code that written in C language (or OpenCL C) that obtains data
    /// from additional source. See in main description of `CodeConfig`.
    pub pop_input_code: Option<&'a str>,
    /// Length of source for pop_input_code in 32-bit words. That length shouldn't be exceeded in
    /// pop_input_code code.
    pub pop_input_len: Option<usize>,
    /// An aggr_output_code written in C language (or OpenCL C) that process data
    /// and write results to additional destination.
    pub aggr_output_code: Option<&'a str>,
    /// Length of source for aggr_output_code in 32-bit words. That length shouldn't be
    /// exceeded in pop_input_code code.
    pub aggr_output_len: Option<usize>,
    /// List of circuit's inputs that will be populated from additional source by
    /// pop_input_code.
    pub pop_from_buffer: Option<&'a [usize]>,
    /// List of circuit's outputs that will be processed by aggr_output_code.
    pub aggr_to_buffer: Option<&'a [usize]>,
    /// List of circuit's outputs that will be excluded as output data.
    pub exclude_outputs: Option<&'a [usize]>,
    /// Applied to BasicMapper and ParSeqMapper - if true then aggregated output buffer
    /// will not be cleared before single execution (to save time) and content of this buffer
    /// will be kept to later use.
    pub dont_clear_outputs: bool,
    /// If some then it holds maximal number of iterations for loop in single execution.
    pub inner_loop: Option<u32>,
}

impl<'a> CodeConfig<'a> {
    /// Creates new CodeConfig with empty setup.
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

    /// Sets input placement.
    pub fn input_placement(mut self, p: Option<(&'a [usize], usize)>) -> Self {
        self.input_placement = p;
        self
    }
    /// Sets output placement.
    pub fn output_placement(mut self, p: Option<(&'a [usize], usize)>) -> Self {
        self.output_placement = p;
        self
    }
    /// Sets arg inputs.
    pub fn arg_inputs(mut self, arg: Option<&'a [usize]>) -> Self {
        self.arg_inputs = arg;
        self
    }
    /// Sets elem inputs.
    pub fn elem_inputs(mut self, elem: Option<&'a [usize]>) -> Self {
        self.elem_inputs = elem;
        self
    }
    /// Sets single buffer.
    pub fn single_buffer(mut self, s: bool) -> Self {
        self.single_buffer = s;
        self
    }
    /// Sets initialization code.
    pub fn init_code(mut self, init: Option<&'a str>) -> Self {
        self.init_code = init;
        self
    }
    /// Sets pop_input_code (a populating input code).
    pub fn pop_input_code(mut self, pop: Option<&'a str>) -> Self {
        self.pop_input_code = pop;
        self
    }
    /// Sets length of additional source for pop_input_code.
    pub fn pop_input_len(mut self, pop: Option<usize>) -> Self {
        self.pop_input_len = pop;
        self
    }
    /// Sets aggr_output_code (an aggregating output code).
    pub fn aggr_output_code(mut self, aggr: Option<&'a str>) -> Self {
        self.aggr_output_code = aggr;
        self
    }
    /// Sets length of additional destination for aggr_output_code.
    pub fn aggr_output_len(mut self, aggr: Option<usize>) -> Self {
        self.aggr_output_len = aggr;
        self
    }
    /// Sets list of circuit's inputs that will be populated from additional buffer.
    pub fn pop_from_buffer(mut self, pop: Option<&'a [usize]>) -> Self {
        self.pop_from_buffer = pop;
        self
    }
    /// Sets list of circuit's outputs that will be processed to additional buffer.
    pub fn aggr_to_buffer(mut self, aggr: Option<&'a [usize]>) -> Self {
        self.aggr_to_buffer = aggr;
        self
    }
    /// Sets lists of circuit's outputs that will be excluded from output data.
    pub fn exclude_outputs(mut self, excl: Option<&'a [usize]>) -> Self {
        self.exclude_outputs = excl;
        self
    }
    /// Sets list of circuit's outputs that will be processed to additional buffer and
    /// will not be in output data.
    pub fn aggr_only_to_buffer(mut self, aggr: Option<&'a [usize]>) -> Self {
        self.aggr_to_buffer = aggr;
        self.exclude_outputs = aggr;
        self
    }
    /// Sets don't clear outputs.
    pub fn dont_clear_outputs(mut self, ignore: bool) -> Self {
        self.dont_clear_outputs = ignore;
        self
    }
    /// Sets inner loop.
    pub fn inner_loop(mut self, l: Option<u32>) -> Self {
        self.inner_loop = l;
        self
    }
}

/// Helper for CodeConfig.
///
/// It is copy of code config that holds not references, but same copies.
/// Some more at [CodeConfig].
#[derive(Clone, Debug)]
pub struct CodeConfigCopy {
    /// Input placement. See main description of structure.
    pub input_placement: Option<(Vec<usize>, usize)>,
    /// Output placement. See main description of structure.
    pub output_placement: Option<(Vec<usize>, usize)>,
    /// Arg inputs is list of circuit's inputs assigned to argument inputs. Index is bit of
    /// argument and value is cicrcuit input index.
    pub arg_inputs: Option<Vec<usize>>,
    /// Elem inputs is list of circuit's inputs assigned to element index. Index is bit of
    /// element index and value is cicrcuit input index.
    pub elem_inputs: Option<Vec<usize>>,
    /// Use single buffer that two buffers (for input and and output). In this case
    /// input placement and output placement must have these same number of pack elements.
    pub single_buffer: bool,
    /// Additional initialization code in C language (or OpenCL C).
    pub init_code: Option<String>,
    /// A pop_input code that written in C language (or OpenCL C) that obtains data
    /// from additional source. See in main description of `CodeConfig`.
    pub pop_input_code: Option<String>,
    /// Length of source for pop_input_code in 32-bit words. That length shouldn't be exceeded in
    /// pop_input_code code.
    pub pop_input_len: Option<usize>,
    /// An aggr_output_code written in C language (or OpenCL C) that process data
    /// and write results to additional destination.
    pub aggr_output_code: Option<String>,
    /// Length of source for aggr_output_code in 32-bit words. That length shouldn't be
    /// exceeded in pop_input_code code.
    pub aggr_output_len: Option<usize>,
    /// List of circuit's inputs that will be populated from additional source by
    /// pop_input_code.
    pub pop_from_buffer: Option<Vec<usize>>,
    /// List of circuit's outputs that will be processed by aggr_output_code.
    pub aggr_to_buffer: Option<Vec<usize>>,
    /// List of circuit's outputs that will be excluded as output data.
    pub exclude_outputs: Option<Vec<usize>>,
    /// Applied to BasicMapper and ParSeqMapper - if true then aggregated output buffer
    /// will not be cleared before single execution (to save time) and content of this buffer
    /// will be kept to later use.
    pub dont_clear_outputs: bool,
    /// If true then inner loop enabled.
    pub inner_loop: Option<u32>,
}

impl CodeConfigCopy {
    /// Makes reference from this copy.
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

    /// Creates new CodeConfig with empty setup.
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

    /// Sets input placement.
    pub fn input_placement(mut self, p: Option<(Vec<usize>, usize)>) -> Self {
        self.input_placement = p;
        self
    }
    /// Sets output placement.
    pub fn output_placement(mut self, p: Option<(Vec<usize>, usize)>) -> Self {
        self.output_placement = p;
        self
    }
    /// Sets arg inputs.
    pub fn arg_inputs(mut self, arg: Option<Vec<usize>>) -> Self {
        self.arg_inputs = arg;
        self
    }
    /// Sets elem inputs.
    pub fn elem_inputs(mut self, elem: Option<Vec<usize>>) -> Self {
        self.elem_inputs = elem;
        self
    }
    /// Sets single buffer.
    pub fn single_buffer(mut self, s: bool) -> Self {
        self.single_buffer = s;
        self
    }
    /// Sets initialization code.
    pub fn init_code(mut self, init: Option<String>) -> Self {
        self.init_code = init;
        self
    }
    /// Sets pop_input_code (a populating input code).
    pub fn pop_input_code(mut self, pop: Option<String>) -> Self {
        self.pop_input_code = pop;
        self
    }
    /// Sets length of additional source for pop_input_code.
    pub fn pop_input_len(mut self, pop: Option<usize>) -> Self {
        self.pop_input_len = pop;
        self
    }
    /// Sets aggr_output_code (an aggregating output code).
    pub fn aggr_output_code(mut self, aggr: Option<String>) -> Self {
        self.aggr_output_code = aggr;
        self
    }
    /// Sets length of additional destination for aggr_output_code.
    pub fn aggr_output_len(mut self, aggr: Option<usize>) -> Self {
        self.aggr_output_len = aggr;
        self
    }
    /// Sets list of circuit's inputs that will be populated from additional buffer.
    pub fn pop_from_buffer(mut self, pop: Option<Vec<usize>>) -> Self {
        self.pop_from_buffer = pop;
        self
    }
    /// Sets list of circuit's outputs that will be processed to additional buffer.
    pub fn aggr_to_buffer(mut self, aggr: Option<Vec<usize>>) -> Self {
        self.aggr_to_buffer = aggr;
        self
    }
    /// Sets lists of circuit's outputs that will be excluded from output data.
    pub fn exclude_outputs(mut self, excl: Option<Vec<usize>>) -> Self {
        self.exclude_outputs = excl;
        self
    }
    /// Sets list of circuit's outputs that will be processed to additional buffer and
    /// will not be in output data.
    pub fn aggr_only_to_buffer(mut self, aggr: Option<Vec<usize>>) -> Self {
        self.aggr_to_buffer = aggr.clone();
        self.exclude_outputs = aggr;
        self
    }
    /// Sets don't clear outputs.
    pub fn dont_clear_outputs(mut self, ignore: bool) -> Self {
        self.dont_clear_outputs = ignore;
        self
    }
    /// Sets inner loop.
    pub fn inner_loop(mut self, l: Option<u32>) -> Self {
        self.inner_loop = l;
        self
    }
}

/// Returns default length of additional destination for aggr_output_code.
pub fn default_aggr_output_len(word_len: u32) -> usize {
    (word_len as usize) >> 5
}
/// Returns default length of additional source for pop_input_code.
pub fn default_pop_input_len(word_len: u32) -> usize {
    (word_len as usize) >> 5
}

/// Type of instruction operation. It is used by function writers.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstrOp {
    /// Boolean And operation
    And,
    /// Boolean Or operation
    Or,
    /// Boolean implication operation
    Impl,
    /// Boolean negated implication operation
    Nimpl,
    /// Boolean XOR operation.
    Xor,
    /// NVIDIA LOP3 operation.
    Lop3(u8),
}

impl InstrOp {
    /// Returns integer of value for self enumeration.
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
    /// Returns argument passed to instruction.
    pub fn arg_num(self) -> usize {
        if matches!(self, InstrOp::Lop3(_)) {
            3
        } else {
            2
        }
    }
}

/// Integer value for And operation.
pub const INSTR_OP_VALUE_AND: u64 = 0;
/// Integer value for Or operation.
pub const INSTR_OP_VALUE_OR: u64 = 1;
/// Integer value for implication operation.
pub const INSTR_OP_VALUE_IMPL: u64 = 2;
/// Integer value for negated implication operation.
pub const INSTR_OP_VALUE_NIMPL: u64 = 3;
/// Integer value for Xor operation.
pub const INSTR_OP_VALUE_XOR: u64 = 4;
/// Integer value for LOP3 operation.
pub const INSTR_OP_VALUE_LOP3: u64 = 5;

/// Function writer that writes function to execute simulation.
pub trait FuncWriter {
    /// Generates function start.
    fn func_start(&mut self);
    /// Generates function end.
    fn func_end(&mut self);
    /// Generates allocation of local variables to make operations.
    fn alloc_vars(&mut self, var_num: usize);

    /// Generates Load instruction from input.
    fn gen_load(&mut self, reg: usize, input: usize);
    /// Generates operation. `negs` determines place where is negation. `dst_arg`
    /// is destination.
    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize);
    /// Generates operation. `dst_arg` is destination.
    fn gen_op3(&mut self, op: InstrOp, dst_arg: usize, arg0: usize, arg1: usize, arg2: usize);
    /// Generates NOT operation. `dst_arg` is destination.
    fn gen_not(&mut self, dst_arg: usize, arg: usize);
    /// Generates Store instruction into output. `output` is index in output data.
    /// `neg` is negation (if true then value will be negated).
    fn gen_store(&mut self, neg: bool, output: usize, reg: usize);
    /// Generates copy register to register. `dst_arg` is destination.
    fn gen_set(&mut self, dst_arg: usize, arg: usize);

    /// Generates conditional for start of loop.
    fn gen_if_loop_start(&mut self);
    /// Generates conditional for end of loop.
    fn gen_if_loop_end(&mut self);
    /// Generates conditional for end of loop.
    fn gen_else(&mut self);
    /// Generates end of conditional.
    fn gen_end_if(&mut self);
    /// Generates aggr_output_code.
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

/// CodeWriter is main trait that defines code generator for simulation.
///
/// CodeWriter trait provides basic logic to implement code generator. Main it is
/// checking of code configuration. Mainly, CodeWriter is used internally by this library.
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
    /// Gets function writer - unsafe version used to internal purpose only.
    unsafe fn func_writer_internal(
        &'a mut self,
        name: &'a str,
        input_len: usize,
        output_len: usize,
        code_config: CodeConfig<'a>,
        output_vars: Option<Vec<(usize, usize)>>,
    ) -> FW;

    /// Gets function writer. `input_len` is number of circuit inputs or if input placement
    /// is provided a total number of pack elements for input data.
    /// `output_len` is number of circuit outputs or if output placement
    /// is provided a total number of pack elements for output data.
    /// `code_config` is code configuration. `output_vars` is list of bit_mapping
    /// circuit's wires to variables. This version checks code configuration before call
    /// internal version.
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

    /// Gets function writer. `input_len` is number of circuit inputs or if input placement
    /// is provided a total number of pack elements for input data.
    /// `output_len` is number of circuit outputs or if output placement
    /// is provided a total number of pack elements for output data.
    /// `input_placement`, `output_placement` and `arg_inputs` are part of code configuration.
    /// `output_vars` is list of bit_mapping
    /// circuit's wires to variables. This version checks code configuration before call
    /// internal version.
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

    /// Gets function writer. `input_len` is number of circuit inputs or if input placement
    /// is provided a total number of pack elements for input data.
    /// `output_len` is number of circuit outputs or if output placement
    /// is provided a total number of pack elements for output data.
    /// This version checks code configuration before call
    /// internal version.
    fn func_writer_simple(&'a mut self, name: &'a str, input_len: usize, output_len: usize) -> FW {
        self.func_writer(name, input_len, output_len, None, None, None)
    }

    /// Returns content of source code as bytes.
    fn out(self) -> Vec<u8>;
}

/// DataReader is simple trait for accessing to data from Data holder.
pub trait DataReader {
    /// Returns slice with data from Data holder.
    fn get(&self) -> &[u32];
}

/// DataWriter is simple trait for writing data to Data holder's data.
pub trait DataWriter {
    /// Returns mutable slice with data from Data holder.
    fn get_mut(&mut self) -> &mut [u32];
}

/// DataHolder is object to holds data.
///
/// DataHolder is object that provides access to data that can be placed in other device
/// than CPU. It provides simple interface to read and write data from Data Holder.
/// This is ability to write input data from simulation and read result data after simulation.
///
/// Data is stored as 32-bit words.
pub trait DataHolder<'a, DR: DataReader, DW: DataWriter> {
    /// Returns current length of data.
    fn len(&self) -> usize;
    /// Returns data reader to read data in data holder.
    fn get(&'a self) -> DR;
    /// Returns data writer to write data to data holder.
    fn get_mut(&'a mut self) -> DW;
    /// Processes data in data holder by given function `f` and returns its output value.
    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out;
    /// Processes data in data holder by given function `f` and returns its output value.
    /// Function `f` can modify data.
    fn process_mut<F, Out>(&mut self, f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out;
    /// Make copy of DataHolder with data that holds Data holder.
    fn copy(&self) -> Self;
    /// Fills data in data holder by given 32-bit value.
    fn fill(&mut self, value: u32);
    /// Release data from data holder.
    fn release(self) -> Vec<u32>;
    /// Free data holder with same data.
    fn free(self);
}

/// Trait provides additional property to set range of data.
///
/// That property is range of index that can be accessed by user. Any data beyond will be
/// unavailable while reading, writing and processing by simulation. To clear range and
/// set availability all data back, `set_range_from(0)` should be call.
pub trait RangedData {
    /// Sets range of available data.
    fn set_range(&mut self, range: Range<usize>);
    /// Sets range of available data.
    #[inline]
    fn set_range_from(&mut self, range: RangeFrom<usize>) {
        self.set_range(range.start..usize::MAX);
    }
}

// ParentDataHolder with specified range that honored while changing range.
/// This Data holder is wrapper to another data holder.
///
/// This Data holder behaves as data holder imposed range.  While using that data holder
/// is not possible to revert range and it is make this data holder as safe.
pub struct ParentDataHolder<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW> + RangedData,
{
    range: Range<usize>,
    child: D,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

impl<'a, DR, DW, D> ParentDataHolder<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW> + RangedData,
{
    /// Creates new parent data holder with imposed range.
    pub fn new(range: Range<usize>, mut child: D) -> Self {
        child.set_range(range.clone());
        Self {
            range,
            child,
            dr: PhantomData,
            dw: PhantomData,
        }
    }
}

impl<'a, DR, DW, D> DataHolder<'a, DR, DW> for ParentDataHolder<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW> + RangedData,
{
    fn len(&self) -> usize {
        self.child.len()
    }
    fn get(&'a self) -> DR {
        self.child.get()
    }
    fn get_mut(&'a mut self) -> DW {
        self.child.get_mut()
    }
    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        self.child.process(f)
    }
    fn process_mut<F, Out>(&mut self, f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        self.child.process_mut(f)
    }
    fn copy(&self) -> Self {
        let c = self.child.copy();
        Self::new(0..c.len(), c)
    }
    fn fill(&mut self, value: u32) {
        self.child.fill(value)
    }
    fn release(self) -> Vec<u32> {
        self.child.release()
    }
    fn free(self) {
        self.child.free()
    }
}

impl<'a, DR, DW, D> RangedData for ParentDataHolder<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW> + RangedData,
{
    // set range
    fn set_range(&mut self, range: Range<usize>) {
        let range_len = self.range.end - self.range.start;
        assert!(range.start <= range_len);
        let end = std::cmp::min(range.end, range_len);
        self.child
            .set_range(self.range.start + range.start..self.range.start + end);
    }
}

/// This trait determines interface for executor of simulation.
///
/// Executor provides basic interface to run single execution. It should provides following
/// methods to run execution:
/// * `execute` - basic execution only for code configuration without single buffer and
///   additional buffer (`pop_from_buffer` and `aggr_to_buffer` shouldn't be set).
///   Output data will be returned by method.
/// * `execute_reuse` - basic execution only for code configuration without single buffer and
///   additional buffer (`pop_from_buffer` and `aggr_to_buffer` shouldn't be set).
///   Output data will be stored in `output` data holder.
/// * `execute_single` - execution only for code configuration with single buffer and
///   additional buffer (`pop_from_buffer` and `aggr_to_buffer` shouldn't be set).
/// * `execute_buffer` - execute only for code configuration with additional buffer
///   (`pop_from_buffer` or `aggr_to_buffer` should be set). Data will be returned.
/// * `execute_buffer_reuse` - execute only for code configuration without single buffer,
///   with additional buffer (`pop_from_buffer` or `aggr_to_buffer` should be set).
///   Data will be stored in `output` data holder.
/// * `execute_buffer_single ` - execute only for code configuration with single buffer
///   and additional buffer (`pop_from_buffer` or `aggr_to_buffer` should be set).
///   Data will be stored in `output` data holder.
///
/// Executor provides additional methods to create data holders:
/// `new_data`, `new_data_from_vec`, `new_data_input_elems` and `new_data_output_elems`.
/// These methods simplify creation of data.
///
/// Basic unit in simulation is element (thread) that is part of pack. Pack contains
/// N elements. N is number of bits of processor word. Number of element in single
/// execution should be divisible by number of bit of word processor.
/// If given data holder provides data for circuit's inputs or outputs directly then
//  data must match to element count. `new_data_input_elems` simplifies creation of data holder
/// with correct length.
pub trait Executor<'a, DR: DataReader, DW: DataWriter, D: DataHolder<'a, DR, DW>> {
    /// Error type used if error encountered while execution.
    type ErrorType;
    /// Returns number of circuit's inputs.
    fn input_len(&self) -> usize;
    /// Returns number of circuit's outputs.
    fn output_len(&self) -> usize;
    /// Returns number of pack elements for input data (for assigned circuit's inputs).
    fn real_input_len(&self) -> usize;
    /// Returns number of pack elements for output data (for assigned circuit's outputs).
    fn real_output_len(&self) -> usize;

    /// Returns element count based on input length in 32-bit words.
    fn elem_count(&self, input_len: usize) -> usize;

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must not have single buffer, `pop_from_buffer` and `aggr_to_buffer`.
    unsafe fn execute_internal(&mut self, input: &D, arg_input: u64) -> Result<D, Self::ErrorType>;
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must not have single buffer, `pop_from_buffer` and `aggr_to_buffer`.
    fn execute(&mut self, input: &D, arg_input: u64) -> Result<D, Self::ErrorType> {
        assert!(!self.is_single_buffer());
        assert!(!(self.input_is_populated() && self.is_populated_from_buffer()));
        assert!(!(self.output_is_aggregated() && self.is_aggregated_to_buffer()));
        unsafe { self.execute_internal(input, arg_input) }
    }

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must not have single buffer, `pop_from_buffer` and `aggr_to_buffer`.
    unsafe fn execute_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
    ) -> Result<(), Self::ErrorType>;
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must not have single buffer, `pop_from_buffer` and `aggr_to_buffer`.
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

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `output` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must have single buffer, and it must not have
    /// `pop_from_buffer` and `aggr_to_buffer`.
    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType>;
    /// Executes simulation. Input data passed by `output` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must have single buffer, and it must not have
    /// `pop_from_buffer` and `aggr_to_buffer`.
    fn execute_single(&mut self, output: &mut D, arg_input: u64) -> Result<(), Self::ErrorType> {
        assert!(self.is_single_buffer());
        assert!(!(self.input_is_populated() && self.is_populated_from_buffer()));
        assert!(!(self.output_is_aggregated() && self.is_aggregated_to_buffer()));
        unsafe { self.execute_single_internal(output, arg_input) }
    }

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must not have single buffer and it must have `pop_from_buffer` or
    /// `aggr_to_buffer`.
    unsafe fn execute_buffer_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<D, Self::ErrorType>;
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method returns data holder with output data.
    ///
    /// Code configuration must not have single buffer and it must have `pop_from_buffer` or
    /// `aggr_to_buffer`.
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

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must not have single buffer and it must have `pop_from_buffer` or
    /// `aggr_to_buffer`.
    unsafe fn execute_buffer_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u64,
        output: &mut D,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType>;
    /// Executes simulation. Input data passed by `input` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must not have single buffer and it must have `pop_from_buffer` or
    /// `aggr_to_buffer`.
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

    /// Only for implementation.
    ///
    /// Executes simulation. Input data passed by `output` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must have single buffer, have `pop_from_buffer` or
    /// `aggr_to_buffer`.
    unsafe fn execute_buffer_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u64,
        buffer: &mut D,
    ) -> Result<(), Self::ErrorType>;
    /// Executes simulation. Input data passed by `output` argument as data holder.
    /// Additionaly if some circuit's inputs assigned to arg input then `arg_input` will be used,
    /// otherwise `arg_input` will be ignored. Additional `buffer` data holder holds
    /// data for `pop_input_code` or can be stored by `aggr_output_code`.
    /// If execution successfully finished then method store output data into `output` data
    /// holder and returns Ok.
    ///
    /// Code configuration must have single buffer, have `pop_from_buffer` or
    /// `aggr_to_buffer`.
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

    /// Creates new data. It returns data holder with zeroed data with length `len` 32-bit words.
    fn new_data(&mut self, len: usize) -> D;
    /// Creates new data. It returns data holder with data supplied by vector `data`.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// Create new data from slice.
    fn new_data_from_slice(&mut self, data: &[u32]) -> D;
    /// Try clone executor if possible.
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized;
    /// Returns true if executor have single buffer property.
    fn is_single_buffer(&self) -> bool;

    /// Returns processor word length.
    fn word_len(&self) -> u32;

    /// Returns true if input data will be populated by `pop_input_code`.
    fn input_is_populated(&self) -> bool;
    /// Returns true if input data will be populated from additional buffer.
    fn is_populated_from_buffer(&self) -> bool;
    /// Returns true if output data will be processed by `aggr_output_code`.
    fn output_is_aggregated(&self) -> bool;
    /// Returns true if output data will be processed by `aggr_output_code` and stored
    /// to additional buffer.
    fn is_aggregated_to_buffer(&self) -> bool;

    /// Returns length of additional buffer in 32-bit words for `aggr_output_code`.
    fn aggr_output_len(&self) -> Option<usize>;
    /// Returns length of additional buffer in 32-bit words for `pop_input_code`.
    fn pop_input_len(&self) -> Option<usize>;

    /// Returns input data (for circuit's inputs) length in 32-bit words for given
    /// number of elements.
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

    /// Returns input data (for circuit's outputs) length in 32-bit words for given
    /// number of elements.
    fn output_data_len(&self, elem_num: usize) -> usize {
        assert_eq!(elem_num % (self.word_len() as usize), 0);
        if self.output_is_aggregated() && !self.is_aggregated_to_buffer() {
            self.aggr_output_len().unwrap()
        } else {
            (elem_num * self.real_output_len()) >> 5
        }
    }

    /// Returns input data holder (for circuit's inputs) with zeroed data with length matched to
    /// given number of elements.
    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    /// Returns output data holder (for circuit's outputs) with zeroed data with length matched to
    /// given number of elements.
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }

    /// Returns true if dont_clear_outputs set.
    fn dont_clear_outputs(&self) -> bool;

    /// Returns true if data should be cleared manually.
    fn need_clear_outputs(&self) -> bool {
        self.output_is_aggregated() && !self.is_aggregated_to_buffer() && !self.dont_clear_outputs()
    }

    /// Returns true if executor executes simulation in sequentially (not parallel way).
    fn is_sequential_execution(&self) -> bool;

    /// Returns inner loop maximal number of iterations.
    fn inner_loop(&self) -> Option<u32>;
}

/// This trait determines interface for builder.
///
/// Usage of builder is simple: first step is adding circuits to builder. Next step is building
/// executors by using `build` method. Additional methods adds helpers and an user defined code.
/// Builder after building should returns same number of executor as number of added
/// simulation configurations.
pub trait Builder<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    /// Error type used if error encountered while execution.
    type ErrorType;

    /// Adds additional user definition to code of simulations.
    fn user_defs(&mut self, user_defs: &str);

    /// Adds transform helpers.
    ///
    /// Transform helpers provides macros that helps to transform data between form used while
    /// simulating circuit and external usage. They can be used in pop_input_code and
    /// aggr_output_code.
    /// * Macro `INPUT_TRANSFORM_BXX(D0,...,DXX,S)` transforms data in X-bit integers stored as
    /// 32-bit words to form fetched by simulation code. `DX` is output single pack element X,
    /// `S` array of 32-bit words.
    /// * Macro `OUTPUT_TRANSFORM_BXX(D,S0,....,SXX)` transforms from form fetched by simulation
    /// code to data in X-bit integers stored as 32-bit words. `D` is output data array of
    /// 32-bit words, `SX` is input pack element X.
    ///
    /// Transform helpers are much faster than data transformers.
    fn transform_helpers(&mut self);

    /// Adds circuit to builder. `name` is name of function, `circuit` is
    /// circuit to simulate. `input_placement`, `output_placement` and `arg_inputs` are
    /// part of code configuration of this simulation configuration.
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

    /// Adds circuit to builder. `name` is name of function, `circuit` is
    /// circuit to simulate, `code_config` is code configuration.
    fn add_with_config<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    /// Adds circuit to builder. `name` is name of function, `circuit` is
    /// circuit to simulate.
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

    /// Build code to simulations. If build succeeded then returns executors for simulations
    /// in addition order.
    fn build(self) -> Result<Vec<E>, Self::ErrorType>;
    /// Returns length processor word in bits.
    fn word_len(&self) -> u32;
    /// Returns type length in bits (includes only type length not word length if
    /// group_vec enabled).
    fn type_len(&self) -> u32;
    /// Returns true if nothing added to build.
    fn is_empty(&self) -> bool;
    /// Returns true if any executor can be used per native thread.
    fn is_executor_per_thread() -> bool;
    /// Returns true if any data holder is global and it can be shared between any
    /// executors from any builder of that type.
    fn is_data_holder_global() -> bool;
    /// Returns true if any data holder is global and it can be shared between any
    /// executors from this builder.
    fn is_data_holder_in_builder() -> bool;
    /// Returns hint about preferred count of input.
    fn preferred_input_count(&self) -> usize;
}

/// Executor of sequential mapper executes simulation multiple times.
///
/// This executor comes from MapperBuilder. Arg input is counter of execution of simulations
/// and will be passed to circuit's input assigned to arg input.
/// Simulations are independents and they will be executed sequentially. Output data for each
/// simulation will be processed by supplied function that returns output. `stop` functions
/// determines whether stop execution of simulations.
pub trait MapperExecutor<'a, DR, DW, D>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
{
    /// Error type used if error encountered while execution.
    type ErrorType;

    /// Returns number of circuit's inputs.
    fn input_len(&self) -> usize;
    /// Returns number of pack elements for input data (for assigned circuit's inputs).
    fn real_input_len(&self) -> usize;
    /// Returns number of circuit's outputs.
    fn output_len(&self) -> usize;

    /// Executes many simulations. Input data passed by `input`. `init` is initial
    /// output. `f` is function that process output data from single execution and
    /// `stop` checks whether whole execution should be stopped (then function should return
    /// true in this case). Function `f` read data from data holders. Arg input value is counter
    /// that increase for every single execution.
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

    /// Executes many simulations. Input data passed by `input`. `init` is initial
    /// output. `f` is function that process output data from single execution and
    /// `stop` checks whether whole execution should be stopped (then function should return
    /// true in this case). Function `f` read data from slice. Arg input value is counter
    /// that increase for every single execution.
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

    /// Executes many simulations. Input data passed by `input`. `init` is initial
    /// output. `buffer` is additional buffer (for pop_input_code and aggr_output_code).
    /// `f` is function that process output data from single execution and
    /// `stop` checks whether whole execution should be stopped (then function should return
    /// true in this case). Function `f` read data from data holders. Arg input value is counter
    /// that increase for every single execution.
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

    /// Executes many simulations. Input data passed by `input`. `init` is initial
    /// output. `buffer` is additional buffer (for pop_input_code and aggr_output_code).
    /// `f` is function that process output data from single execution and
    /// `stop` checks whether whole execution should be stopped (then function should return
    /// true in this case). Function `f` read data from slice. Arg input value is counter
    /// that increase for every single execution.
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
    /// Creates new data. It returns data holder with zeroed data with length `len` 32-bit words.
    fn new_data(&mut self, len: usize) -> D;
    /// Creates new data. It returns data holder with data supplied by vector `data`.
    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D;
    /// Create new data from slice.
    fn new_data_from_slice(&mut self, data: &[u32]) -> D;

    /// Returns processor word length.
    fn word_len(&self) -> u32;

    /// Returns element count for given input length in 32-bit words.
    fn elem_count(&self, input_len: usize) -> usize;

    /// Returns input data (for circuit's inputs) length in 32-bit words for given
    /// number of elements.
    fn input_data_len(&self, elem_num: usize) -> usize;

    /// Returns input data (for circuit's outputs) length in 32-bit words for given
    /// number of elements.
    fn output_data_len(&self, elem_num: usize) -> usize;

    /// Returns input data holder (for circuit's inputs) with zeroed data with length matched to
    /// given number of elements.
    fn new_data_input_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.input_data_len(elem_num))
    }
    /// Returns output data holder (for circuit's outputs) with zeroed data with length matched to
    /// given number of elements.
    fn new_data_output_elems(&mut self, elem_num: usize) -> D {
        self.new_data(self.output_data_len(elem_num))
    }

    /// Returns true if output data will be processed by `aggr_output_code`.
    fn output_is_aggregated(&self) -> bool;
    /// Returns true if output data will be processed by `aggr_output_code` and stored
    /// to additional buffer.
    fn is_aggregated_to_buffer(&self) -> bool;
    /// Returns true if input data will be populated by `pop_input_code`.
    fn input_is_populated(&self) -> bool;
    /// Returns true if input data will be populated from additional buffer.
    fn is_populated_from_buffer(&self) -> bool;

    /// Returns length of additional buffer in 32-bit words for `aggr_output_code`.
    fn aggr_output_len(&self) -> Option<usize>;
    /// Returns length of additional buffer in 32-bit words for `pop_input_code`.
    fn pop_input_len(&self) -> Option<usize>;

    /// Returns true if executor executes simulation in sequentially (not parallel way).
    fn is_sequential_execution(&self) -> bool;

    /// Returns inner loop maximal number of iterations.
    fn inner_loop(&self) -> Option<u32>;
}

/// Trait defines builder for sequential mapper.
///
/// Usage of builder is simple: first step is adding circuits to builder. Next step is building
/// executors by using `build` method. Additional methods adds helpers and an user defined code.
/// Builder after building should returns same number of executor as number of added
/// simulation configurations. This builder returns MapperExecutors.
pub trait MapperBuilder<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: MapperExecutor<'a, DR, DW, D>,
{
    /// Error type used if error encountered while execution.
    type ErrorType;

    /// Adds additional user definition to code of simulations.
    fn user_defs(&mut self, user_defs: &str);
    /// Adds transform helpers.
    ///
    /// Transform helpers provides macros that helps to transform data between form used while
    /// simulating circuit and external usage. They can be used in pop_input_code and
    /// aggr_output_code.
    /// * Macro `INPUT_TRANSFORM_BXX(D0,...,DXX,S)` transforms data in X-bit integers stored as
    /// 32-bit words to form fetched by simulation code. `DX` is output single pack element X,
    /// `S` array of 32-bit words.
    /// * Macro `OUTPUT_TRANSFORM_BXX(D,S0,....,SXX)` transforms from form fetched by simulation
    /// code to data in X-bit integers stored as 32-bit words. `D` is output data array of
    /// 32-bit words, `SX` is input pack element X.
    ///
    /// Transform helpers are much faster than data transformers.
    fn transform_helpers(&mut self);

    /// Only for implementation.
    ///
    /// Adds circuit to builder. `name` is name of function, `circuit is circuit to simulate,
    /// `code_config` is code configuration.
    unsafe fn add_internal<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug;

    /// Adds circuit to builder. `name` is name of function, `circuit is circuit to simulate,
    /// `code_config` is code configuration.
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

    /// Adds circuit to builder. `name` is name of function, `circuit is circuit to simulate,
    /// `arg_inputs` are circuit's inputs to be assigned to arg input.
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

    /// Build code to simulations. If build succeeded then returns executors for simulations
    /// in addition order.
    fn build(self) -> Result<Vec<E>, Self::ErrorType>;

    /// Returns length processor word in bits.
    fn word_len(&self) -> u32;
    /// Returns type length in bits (includes only type length not word length if
    /// group_vec enabled).
    fn type_len(&self) -> u32;

    /// Returns true if any data holder is global and it can be shared between any
    /// executors from any builder of that type.
    fn is_data_holder_global() -> bool;
    /// Returns true if any data holder is global and it can be shared between any
    /// executors from this builder.
    fn is_data_holder_in_builder() -> bool;
    /// Returns hint about preferred count of input.
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

// About using DataTransformers and DataTransforms.
// These data transformations designed to be light and used only once,
// because they are not too much fast.
// If transformations used many times then it recommeded to use
// INPUT_TRANSFORM and OUTPUT_TRANSFORM inside populated input code or aggregated output code.
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

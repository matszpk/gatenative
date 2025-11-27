#![cfg_attr(docsrs, feature(doc_cfg))]
//! Simulation execution on CPU.
//!
//! The module provides builder and executors to run simulation on CPU. It uses CLangWriter
//! to generate code for specific instruction set extension for CPU. The Builder executes
//! external compiler to build simulation code in C language. `GATE_SYS_CC` is environment
//! variable that holds path to C compiler. By default is `clang` compiler from LLVM package.
//! Source code compiled to shared library will be loaded after built and to call
//! simulation code.
//!
//! `CPU_EXTENSION` holds current recognized CPU instruction set extension that will be
//! by default while creating CPUBuilder.
//!
//! Additional structure is Shared that allows creating of shared library.

use crate::clang_writer::*;
use crate::cpu_data_transform::*;
use crate::gencode::generate_code_with_config_and_wire_order;
use crate::utils::{dump_source_code, get_timestamp};
use crate::*;
use libloading::{Library, Symbol};
use rayon::prelude::*;
use static_init::dynamic;
use thiserror::Error;

use std::convert::Infallible;
use std::env::{self, temp_dir};
use std::fmt::Debug;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

#[derive(Error, Debug)]
enum DetectCPUError {
    #[error("IO error {0}")]
    IOError(#[from] io::Error),
}

/// Build error type.
#[derive(Error, Debug)]
pub enum BuildError {
    /// I/O error.
    #[error("IO error {0}")]
    IOError(#[from] io::Error),
    /// Error while compiling.
    #[error("Compile error {0}")]
    CompileError(String),
    /// Error while loading library.
    #[error("LibLoading error {0}")]
    LibLoadingError(#[from] libloading::Error),
}

/// Type for CPU instruction set extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUExtension {
    /// No extension in CPU.
    NoExtension,
    /// CPU has Intel MMX extensions.
    IntelMMX,
    /// CPU has Intel SSE extensions.
    IntelSSE,
    /// CPU has Intel SSE2 extensions.
    IntelSSE2,
    /// CPU has Intel AVX extensions.
    IntelAVX,
    /// CPU has Intel AVX2 extensions.
    IntelAVX2,
    /// CPU has Intel AVX512 extensions.
    IntelAVX512,
    /// CPU has ARM NEON extensions.
    ARMNEON,
}

fn detect_cpu_from_file(file: impl BufRead) -> Result<CPUExtension, DetectCPUError> {
    let mut have_fp = false;
    let mut is_armv8 = false;
    let mut have_flags = false;
    let untested = *utils::GATE_SYS_UNTESTED;
    for rl in file.lines() {
        let line = rl?;
        if line.starts_with("flags") || line.starts_with("Features") {
            if line.find(" avx512").is_some() && untested {
                return Ok(CPUExtension::IntelAVX512);
            } else if line.find(" avx2").is_some() {
                return Ok(CPUExtension::IntelAVX2);
            } else if line.find(" avx").is_some() {
                return Ok(CPUExtension::IntelAVX);
            } else if line.find(" sse2").is_some() {
                return Ok(CPUExtension::IntelSSE2);
            } else if line.find(" sse").is_some() {
                return Ok(CPUExtension::IntelSSE);
            } else if line.find(" mmx").is_some() {
                return Ok(CPUExtension::IntelMMX);
            } else if line.find(" neon").is_some() && untested {
                return Ok(CPUExtension::ARMNEON);
            } else if line.find(" fp").is_some() {
                have_fp = true;
            }
            have_flags = true;
        } else if line.starts_with("CPU architecture: 8") {
            is_armv8 = true;
        }
        if have_flags && is_armv8 {
            if have_fp {
                // NEON is default for ARMv8 with FP.
                return Ok(CPUExtension::ARMNEON);
            } else {
                return Ok(CPUExtension::NoExtension);
            }
        }
    }
    Ok(CPUExtension::NoExtension)
}

fn detect_cpu_by_cpuidrs() -> CPUExtension {
    use cpuidrs::*;
    let cpuinfo = cpuidrs::get_cpu_info();
    let untested = *utils::GATE_SYS_UNTESTED;
    if untested && cpuinfo.has_feature(InstructionSet::AVX512F) {
        CPUExtension::IntelAVX512
    } else if cpuinfo.has_feature(InstructionSet::AVX2) {
        CPUExtension::IntelAVX2
    } else if cpuinfo.has_feature(InstructionSet::AVX) {
        CPUExtension::IntelAVX
    } else if cpuinfo.has_feature(InstructionSet::SSE2) {
        CPUExtension::IntelSSE2
    } else if cpuinfo.has_feature(InstructionSet::SSE) {
        CPUExtension::IntelSSE
    } else if cpuinfo.has_feature(InstructionSet::MMX) {
        CPUExtension::IntelMMX
    } else if untested && cpuinfo.has_feature(InstructionSet::NEON) {
        CPUExtension::ARMNEON
    } else {
        CPUExtension::NoExtension
    }
}

fn detect_cpu() -> Result<CPUExtension, DetectCPUError> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(file) = File::open("/proc/cpuinfo") {
            detect_cpu_from_file(BufReader::new(file))
        } else {
            Ok(detect_cpu_by_cpuidrs())
        }
    }
    #[cfg(not(target_os = "linux"))]
    Ok(detect_cpu_by_cpuidrs())
}

/// It holds currently recognized CPU instruction set extension.
#[dynamic]
pub static CPU_EXTENSION: CPUExtension = detect_cpu().unwrap_or(CPUExtension::NoExtension);

// configurations

struct BuildConfig<'a> {
    writer_config: &'a CLangWriterConfig<'a>,
    extra_flags: &'a [&'a str],
}

#[cfg(target_pointer_width = "32")]
const BUILD_CONFIG_U32: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_U32,
    extra_flags: &[],
};

#[cfg(target_pointer_width = "64")]
const BUILD_CONFIG_U64: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_U64,
    extra_flags: &[],
};

const BUILD_CONFIG_INTEL_MMX: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_MMX,
    extra_flags: &["-mmmx"],
};

const BUILD_CONFIG_INTEL_SSE: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_SSE,
    extra_flags: &["-msse"],
};

const BUILD_CONFIG_INTEL_SSE2: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_SSE2,
    extra_flags: &["-msse2"],
};

const BUILD_CONFIG_INTEL_AVX: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX,
    extra_flags: &["-mavx"],
};

const BUILD_CONFIG_INTEL_AVX2: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX2,
    extra_flags: &["-mavx2"],
};

const BUILD_CONFIG_INTEL_AVX512: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX512,
    extra_flags: &["-mavx512f"],
};

const BUILD_CONFIG_ARM_NEON: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_ARM_NEON,
    extra_flags: &["-mfpu=neon"],
};

fn get_build_config(cpu_ext: CPUExtension) -> BuildConfig<'static> {
    match cpu_ext {
        CPUExtension::NoExtension => {
            #[cfg(target_pointer_width = "32")]
            {
                BUILD_CONFIG_U32
            }
            #[cfg(target_pointer_width = "64")]
            {
                BUILD_CONFIG_U64
            }
        }
        CPUExtension::IntelMMX => BUILD_CONFIG_INTEL_MMX,
        CPUExtension::IntelSSE => BUILD_CONFIG_INTEL_SSE,
        CPUExtension::IntelSSE2 => BUILD_CONFIG_INTEL_SSE2,
        CPUExtension::IntelAVX => BUILD_CONFIG_INTEL_AVX,
        CPUExtension::IntelAVX2 => BUILD_CONFIG_INTEL_AVX2,
        CPUExtension::IntelAVX512 => BUILD_CONFIG_INTEL_AVX512,
        CPUExtension::ARMNEON => BUILD_CONFIG_ARM_NEON,
    }
}

// shared library object

/// It holds current value of `GATE_SYS_CC` environment variable.
#[dynamic]
pub static GATE_SYS_CC: String = env::var("GATE_SYS_CC").unwrap_or("clang".to_string());

/// Object to create and manage shared library.
pub struct SharedLib {
    cpu_ext: CPUExtension,
    source_path: PathBuf,
    shared_library_path: PathBuf,
}

impl SharedLib {
    /// Creates shared library to built with including given CPU instruction extension in
    /// `cpu_ext`.
    pub fn new_with_cpu_ext(cpu_ext: CPUExtension) -> Self {
        let temp_dir_path = temp_dir();
        let unix_time = get_timestamp();
        Self {
            cpu_ext,
            source_path: temp_dir_path.join(format!("gate_x4x_source_{}.c", unix_time)),
            shared_library_path: temp_dir_path.join(format!("gate_x4x_lib_{}.so", unix_time)),
        }
    }

    /// Builds source code to shared library. It succeded then it returns library object.
    pub fn build(self, source: &[u8]) -> Result<Library, BuildError> {
        fs::write(&self.source_path, source)?;
        let extra_flags = get_build_config(self.cpu_ext).extra_flags;
        let args = {
            let mut args = vec![];
            args.extend(extra_flags);
            args.extend([
                "-shared",
                "-fPIC",
                "-o",
                self.shared_library_path.to_str().unwrap(),
                "-Wall",
                "-fno-strict-aliasing",
                "-O2",
                self.source_path.to_str().unwrap(),
            ]);
            args
        };

        let output = Command::new(&*GATE_SYS_CC).args(args).output()?;
        if !output.status.success() {
            return Err(BuildError::CompileError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        let lib = unsafe { Library::new(&self.shared_library_path)? };
        if fs::remove_file(&self.source_path).is_err() {
            eprintln!("File {:?} doesn't exist", self.source_path);
        }
        if fs::remove_file(&self.shared_library_path).is_err() {
            eprintln!("File {:?} doesn't exist", self.shared_library_path);
        }
        Ok(lib)
    }
}

impl Drop for SharedLib {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.source_path);
        let _ = fs::remove_file(&self.shared_library_path);
    }
}

// CPU Builder

/// CPU Data reader.
///
/// See more in [DataReader].
pub struct CPUDataReader<'a> {
    buffer: &'a [u32],
}

impl<'a> DataReader for CPUDataReader<'a> {
    #[inline]
    fn get(&self) -> &[u32] {
        self.buffer
    }
}

/// CPU Data writer.
///
/// See more in [DataWriter].
pub struct CPUDataWriter<'a> {
    buffer: &'a mut [u32],
}

impl<'a> DataWriter for CPUDataWriter<'a> {
    #[inline]
    fn get_mut(&mut self) -> &mut [u32] {
        self.buffer
    }
}

/// CPU Data holder.
///
/// See more in [DataHolder].
pub struct CPUDataHolder {
    buffer: Vec<u32>,
    range: Range<usize>,
}

impl CPUDataHolder {
    /// Creates new CPU data holder with data from vector.
    #[inline]
    pub fn new(data: Vec<u32>) -> Self {
        let len = data.len();
        Self {
            buffer: data,
            range: 0..len,
        }
    }
    /// Creates new CPU data holder with data from slice.
    pub fn new_from_slice(data: &[u32]) -> Self {
        let len = data.len();
        Self {
            buffer: data.to_vec(),
            range: 0..len,
        }
    }
}

impl<'a> DataHolder<'a, CPUDataReader<'a>, CPUDataWriter<'a>> for CPUDataHolder {
    #[inline]
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }
    fn get(&'a self) -> CPUDataReader<'a> {
        CPUDataReader {
            buffer: &self.buffer[self.range.clone()],
        }
    }
    fn get_mut(&'a mut self) -> CPUDataWriter<'a> {
        CPUDataWriter {
            buffer: &mut self.buffer[self.range.clone()],
        }
    }
    fn process<F, Out>(&self, mut f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        f(&self.buffer[self.range.clone()])
    }
    fn process_mut<F, Out>(&mut self, mut f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        f(&mut self.buffer[self.range.clone()])
    }
    fn copy(&self) -> Self {
        Self::new_from_slice(&self.buffer[self.range.clone()])
    }
    fn fill(&mut self, value: u32) {
        self.buffer[self.range.clone()].fill(value);
    }
    fn release(self) -> Vec<u32> {
        if self.range.start == 0 && self.range.end == self.buffer.len() {
            self.buffer
        } else {
            self.buffer[self.range.clone()].to_vec()
        }
    }
    fn free(self) {}
}

impl RangedData for CPUDataHolder {
    fn set_range(&mut self, range: Range<usize>) {
        self.range = std::cmp::min(self.buffer.len(), range.start)
            ..std::cmp::min(self.buffer.len(), range.end);
        if self.range.start >= self.range.end {
            self.range = 0..0;
        }
    }
}

/// Structure holds CPU builder configuration.
#[derive(Clone, Debug)]
pub struct CPUBuilderConfig {
    /// If true then code generator can optimize negation while creating code to simulate circuit.
    pub optimize_negs: bool,
    // if some then parallel - value is parallel chunk length
    /// Sets parallel mode. If value supplied then value is chunk length in processor words.
    /// Input and output data simulation will be divided in chunks that will be run
    /// parallel way.
    pub parallel: Option<usize>,
    /// If set then value is length of longer vector types as processor words. It preferred to be
    /// a power of 2. In normal cases this property is not needed to be set.
    pub array_len: Option<usize>,
    /// If set then ordering of instruction based on wire (input or gate) index
    /// (original ordering), otherwise ordering based on tree traversal from outputs to inputs.
    /// For many cases this option is not recommended.
    pub wire_order: bool,
}

impl CPUBuilderConfig {
    /// Creates new empty CPU buoilder configuration.
    pub fn new() -> Self {
        Self {
            optimize_negs: false,
            parallel: None,
            array_len: None,
            wire_order: false,
        }
    }
    /// Sets optimize negations.
    pub fn optimize_negs(mut self, optimize_negs: bool) -> Self {
        self.optimize_negs = optimize_negs;
        self
    }
    /// Sets parallelism.
    pub fn parallel(mut self, parallel: Option<usize>) -> Self {
        self.parallel = parallel;
        self
    }
    /// Sets array_len (longer vector types).
    pub fn array_len(mut self, array_len: Option<usize>) -> Self {
        self.array_len = array_len;
        self
    }
    /// Sets wire_order.
    pub fn wire_order(mut self, wire_order: bool) -> Self {
        self.wire_order = wire_order;
        self
    }
}

/// Default CPU builder configuration.
pub const CPU_BUILDER_CONFIG_DEFAULT: CPUBuilderConfig = CPUBuilderConfig {
    optimize_negs: true,
    parallel: None,
    array_len: None,
    wire_order: false,
};

/// Main CPU executor.
///
/// This executor provides data transformers by [DataTransforms]. See more in [Executor].
#[derive(Clone)]
pub struct CPUExecutor {
    input_len: usize,
    output_len: usize,
    real_input_len: usize,
    real_output_len: usize,
    words_per_real_word: usize,
    arg_input_len: Option<usize>,
    elem_input_num: usize,
    library: Arc<Library>,
    sym_name: String,
    single_buffer: bool,
    aggregated_output: bool,
    aggregated_to_buffer: bool,
    aggr_output_len: Option<usize>,
    populated_input: bool,
    populated_from_buffer: bool,
    pop_input_len: Option<usize>,
    pop_input_len_from_buffer: Option<usize>,
    // parallel chunk length
    parallel: Option<usize>,
    dont_clear_outputs: bool,
    inner_loop: Option<u32>,
}

impl CPUExecutor {
    fn call_execute_internal(
        &self,
        num: usize,
        input: &[u32],
        output: &mut [u32],
        real_input_words: usize,
        real_output_words: usize,
        arg_input: u64,
    ) -> Result<(), libloading::Error> {
        if let Some(par_chunk_len) = self.parallel {
            // parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    let chunk_num = (num + par_chunk_len - 1) / par_chunk_len;
                    (0..chunk_num).par_bridge().for_each(|ch_idx| {
                        let output_ptr = output[..].as_ptr();
                        unsafe {
                            let output_ptr = output_ptr.cast_mut();
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                (symbol)(
                                    input[i * real_input_words..].as_ptr(),
                                    output_ptr,
                                    (arg_input & 0xffffffff) as u32,
                                    (arg_input >> 32) as u32,
                                    i,
                                );
                            }
                        }
                    });
                } else {
                    output
                        .chunks_mut(par_chunk_len * real_output_words)
                        .enumerate()
                        .par_bridge()
                        .for_each(|(ch_idx, out_chunk)| {
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                unsafe {
                                    (symbol)(
                                        input[i * real_input_words..].as_ptr(),
                                        out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                        (arg_input & 0xffffffff) as u32,
                                        (arg_input >> 32) as u32,
                                        i,
                                    );
                                }
                            }
                        });
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    let chunk_num = (num + par_chunk_len - 1) / par_chunk_len;
                    (0..chunk_num).par_bridge().for_each(|ch_idx| {
                        let output_ptr = output[..].as_ptr();
                        unsafe {
                            let output_ptr = output_ptr.cast_mut();
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                (symbol)(input[i * real_input_words..].as_ptr(), output_ptr, i);
                            }
                        }
                    });
                } else {
                    output
                        .chunks_mut(par_chunk_len * real_output_words)
                        .enumerate()
                        .par_bridge()
                        .for_each(|(ch_idx, out_chunk)| {
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                unsafe {
                                    (symbol)(
                                        input[i * real_input_words..].as_ptr(),
                                        out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                        i,
                                    );
                                }
                            }
                        });
                }
            }
        } else {
            // non-parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                input[i * real_input_words..].as_ptr(),
                                output[..].as_mut_ptr(),
                                (arg_input & 0xffffffff) as u32,
                                (arg_input >> 32) as u32,
                                i,
                            );
                        }
                    }
                } else {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                input[i * real_input_words..].as_ptr(),
                                output[i * real_output_words..].as_mut_ptr(),
                                (arg_input & 0xffffffff) as u32,
                                (arg_input >> 32) as u32,
                                i,
                            );
                        }
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                input[i * real_input_words..].as_ptr(),
                                output[..].as_mut_ptr(),
                                i,
                            );
                        }
                    }
                } else {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                input[i * real_input_words..].as_ptr(),
                                output[i * real_output_words..].as_mut_ptr(),
                                i,
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn call_execute_buffer_internal(
        &self,
        num: usize,
        input: &[u32],
        output: &mut [u32],
        buffer: &mut [u32],
        real_input_words: usize,
        real_output_words: usize,
        arg_input: u64,
    ) -> Result<(), libloading::Error> {
        if let Some(par_chunk_len) = self.parallel {
            // parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<
                    unsafe extern "C" fn(*const u32, *mut u32, u32, u32, *mut u32, usize),
                > = unsafe { self.library.get(self.sym_name.as_bytes())? };
                output
                    .chunks_mut(par_chunk_len * real_output_words)
                    .enumerate()
                    .par_bridge()
                    .for_each(|(ch_idx, out_chunk)| {
                        let buffer_ptr = buffer[..].as_ptr();
                        let start = ch_idx * par_chunk_len;
                        let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                        for i in start..end {
                            let buffer_ptr = buffer_ptr.cast_mut();
                            unsafe {
                                (symbol)(
                                    input[i * real_input_words..].as_ptr(),
                                    out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                    (arg_input & 0xffffffff) as u32,
                                    (arg_input >> 32) as u32,
                                    buffer_ptr,
                                    i,
                                );
                            }
                        }
                    });
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                output
                    .chunks_mut(par_chunk_len * real_output_words)
                    .enumerate()
                    .par_bridge()
                    .for_each(|(ch_idx, out_chunk)| {
                        let buffer_ptr = buffer[..].as_ptr();
                        let start = ch_idx * par_chunk_len;
                        let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                        for i in start..end {
                            let buffer_ptr = buffer_ptr.cast_mut();
                            unsafe {
                                (symbol)(
                                    input[i * real_input_words..].as_ptr(),
                                    out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                    buffer_ptr,
                                    i,
                                );
                            }
                        }
                    });
            }
        } else {
            // non-parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<
                    unsafe extern "C" fn(*const u32, *mut u32, u32, u32, *mut u32, usize),
                > = unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                            buffer[..].as_mut_ptr(),
                            i,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            buffer[..].as_mut_ptr(),
                            i,
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a> Executor<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder> for CPUExecutor {
    type ErrorType = libloading::Error;
    #[inline]
    fn input_len(&self) -> usize {
        self.input_len
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.output_len
    }
    #[inline]
    fn real_input_len(&self) -> usize {
        self.real_input_len
    }
    #[inline]
    fn real_output_len(&self) -> usize {
        self.real_output_len
    }

    fn elem_count(&self, input_len: usize) -> usize {
        if self.populated_input && !self.populated_from_buffer {
            1 << (self.input_len - self.arg_input_len.unwrap_or(0))
        } else if self.real_input_len != 0 {
            (input_len / self.real_input_len) << 5
        } else if self.elem_input_num != 0 {
            1 << self.elem_input_num
        } else if self.populated_input && self.populated_from_buffer {
            if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
                1 << pop_from_buffer_len
            } else {
                0
            }
        } else {
            0
        }
    }

    unsafe fn execute_internal(
        &mut self,
        input: &CPUDataHolder,
        arg_input: u64,
    ) -> Result<CPUDataHolder, Self::ErrorType> {
        let input_r = input.get();
        let input = input_r.get();
        let real_input_words = if !self.populated_input {
            self.real_input_len * self.words_per_real_word
        } else {
            0
        };
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output = if self.aggregated_output {
            vec![0; self.aggr_output_len.unwrap()]
        } else {
            vec![0; num * real_output_words]
        };
        self.call_execute_internal(
            num,
            input,
            &mut output,
            real_input_words,
            real_output_words,
            arg_input,
        )?;
        let out_len = output.len();
        Ok(CPUDataHolder {
            buffer: output.into(),
            range: 0..out_len,
        })
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &CPUDataHolder,
        arg_input: u64,
        output: &mut CPUDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let input_r = input.get();
        let input = input_r.get();
        let real_input_words = if !self.populated_input {
            self.real_input_len * self.words_per_real_word
        } else {
            0
        };
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            output_len / real_output_words
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        if !self.aggregated_output {
            assert!(output_len >= real_output_words * num);
        }
        self.call_execute_internal(
            num,
            input,
            output,
            real_input_words,
            real_output_words,
            arg_input,
        )
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut CPUDataHolder,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            output_len / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        if !self.aggregated_output {
            assert!(output_len >= real_output_words * num);
        }
        if let Some(par_chunk_len) = self.parallel {
            // parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    let chunk_num = (num + par_chunk_len - 1) / par_chunk_len;
                    (0..chunk_num).par_bridge().for_each(|ch_idx| {
                        let output_ptr = output[..].as_ptr();
                        unsafe {
                            let output_ptr = output_ptr.cast_mut();
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                (symbol)(
                                    output_ptr,
                                    (arg_input & 0xffffffff) as u32,
                                    (arg_input >> 32) as u32,
                                    i,
                                );
                            }
                        }
                    });
                } else {
                    output
                        .chunks_mut(par_chunk_len * real_output_words)
                        .enumerate()
                        .par_bridge()
                        .for_each(|(ch_idx, out_chunk)| {
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                unsafe {
                                    (symbol)(
                                        out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                        (arg_input & 0xffffffff) as u32,
                                        (arg_input >> 32) as u32,
                                        i,
                                    );
                                }
                            }
                        });
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    let chunk_num = (num + par_chunk_len - 1) / par_chunk_len;
                    (0..chunk_num).par_bridge().for_each(|ch_idx| {
                        let output_ptr = output[..].as_ptr();
                        unsafe {
                            let output_ptr = output_ptr.cast_mut();
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                (symbol)(output_ptr, i);
                            }
                        }
                    });
                } else {
                    output
                        .chunks_mut(par_chunk_len * real_output_words)
                        .enumerate()
                        .par_bridge()
                        .for_each(|(ch_idx, out_chunk)| {
                            let start = ch_idx * par_chunk_len;
                            let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                            for i in start..end {
                                unsafe {
                                    (symbol)(
                                        out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                        i,
                                    );
                                }
                            }
                        });
                }
            }
        } else {
            // non-parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                output[..].as_mut_ptr(),
                                (arg_input & 0xffffffff) as u32,
                                (arg_input >> 32) as u32,
                                i,
                            );
                        }
                    }
                } else {
                    for i in 0..num {
                        unsafe {
                            (symbol)(
                                output[i * real_output_words..].as_mut_ptr(),
                                (arg_input & 0xffffffff) as u32,
                                (arg_input >> 32) as u32,
                                i,
                            );
                        }
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                if self.aggregated_output {
                    for i in 0..num {
                        unsafe {
                            (symbol)(output[..].as_mut_ptr(), i);
                        }
                    }
                } else {
                    for i in 0..num {
                        unsafe {
                            (symbol)(output[i * real_output_words..].as_mut_ptr(), i);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    unsafe fn execute_buffer_internal(
        &mut self,
        input: &CPUDataHolder,
        arg_input: u64,
        buffer: &mut CPUDataHolder,
    ) -> Result<CPUDataHolder, Self::ErrorType> {
        let input_r = input.get();
        let input = input_r.get();
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output = vec![0; num * real_output_words];
        let mut buffer_w = buffer.get_mut();
        let buffer = buffer_w.get_mut();
        self.call_execute_buffer_internal(
            num,
            input,
            &mut output,
            buffer,
            real_input_words,
            real_output_words,
            arg_input,
        )?;
        let out_len = output.len();
        Ok(CPUDataHolder {
            buffer: output.into(),
            range: 0..out_len,
        })
    }

    unsafe fn execute_buffer_reuse_internal(
        &mut self,
        input: &CPUDataHolder,
        arg_input: u64,
        output: &mut CPUDataHolder,
        buffer: &mut CPUDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let input_r = input.get();
        let input = input_r.get();
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            output_len / real_output_words
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        assert!(output_len >= real_output_words * num);
        let mut buffer_w = buffer.get_mut();
        let buffer = buffer_w.get_mut();
        self.call_execute_buffer_internal(
            num,
            input,
            output,
            buffer,
            real_input_words,
            real_output_words,
            arg_input,
        )
    }

    unsafe fn execute_buffer_single_internal(
        &mut self,
        output: &mut CPUDataHolder,
        arg_input: u64,
        buffer: &mut CPUDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if real_input_words != 0 {
            output_len / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        assert!(output_len >= real_output_words * num);
        let mut buffer_w = buffer.get_mut();
        let buffer = buffer_w.get_mut();
        if let Some(par_chunk_len) = self.parallel {
            // parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                output
                    .chunks_mut(par_chunk_len * real_output_words)
                    .enumerate()
                    .par_bridge()
                    .for_each(|(ch_idx, out_chunk)| {
                        let buffer_ptr = buffer[..].as_ptr();
                        let start = ch_idx * par_chunk_len;
                        let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                        for i in start..end {
                            let buffer_ptr = buffer_ptr.cast_mut();
                            unsafe {
                                (symbol)(
                                    out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                    (arg_input & 0xffffffff) as u32,
                                    (arg_input >> 32) as u32,
                                    buffer_ptr,
                                    i,
                                );
                            }
                        }
                    });
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                output
                    .chunks_mut(par_chunk_len * real_output_words)
                    .enumerate()
                    .par_bridge()
                    .for_each(|(ch_idx, out_chunk)| {
                        let buffer_ptr = buffer[..].as_ptr();
                        let start = ch_idx * par_chunk_len;
                        let end = std::cmp::min((ch_idx + 1) * par_chunk_len, num);
                        for i in start..end {
                            let buffer_ptr = buffer_ptr.cast_mut();
                            unsafe {
                                (symbol)(
                                    out_chunk[(i - start) * real_output_words..].as_mut_ptr(),
                                    buffer_ptr,
                                    i,
                                );
                            }
                        }
                    });
            }
        } else {
            // non-parallel code
            if self.arg_input_len.is_some() {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                            buffer.as_mut_ptr(),
                            i,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, *mut u32, usize)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            output[i * real_output_words..].as_mut_ptr(),
                            buffer.as_mut_ptr(),
                            i,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn new_data(&mut self, len: usize) -> CPUDataHolder {
        CPUDataHolder::new(vec![0u32; len])
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> CPUDataHolder {
        CPUDataHolder::new(data)
    }

    fn new_data_from_slice(&mut self, data: &[u32]) -> CPUDataHolder {
        CPUDataHolder::new_from_slice(data)
    }

    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
    }

    #[inline]
    fn is_single_buffer(&self) -> bool {
        self.single_buffer
    }

    #[inline]
    fn word_len(&self) -> u32 {
        u32::try_from(self.words_per_real_word << 5).unwrap()
    }

    #[inline]
    fn output_is_aggregated(&self) -> bool {
        self.aggregated_output
    }

    #[inline]
    fn aggr_output_len(&self) -> Option<usize> {
        self.aggr_output_len
    }

    #[inline]
    fn is_aggregated_to_buffer(&self) -> bool {
        self.aggregated_to_buffer
    }

    #[inline]
    fn input_is_populated(&self) -> bool {
        self.populated_input
    }

    #[inline]
    fn pop_input_len(&self) -> Option<usize> {
        self.pop_input_len
    }

    #[inline]
    fn is_populated_from_buffer(&self) -> bool {
        self.populated_from_buffer
    }

    #[inline]
    fn dont_clear_outputs(&self) -> bool {
        self.dont_clear_outputs
    }

    fn is_sequential_execution(&self) -> bool {
        self.parallel.is_none()
    }

    #[inline]
    fn inner_loop(&self) -> Option<u32> {
        self.inner_loop
    }
}

impl<'a>
    DataTransforms<
        'a,
        CPUDataReader<'a>,
        CPUDataWriter<'a>,
        CPUDataHolder,
        CPUDataInputTransformer,
        CPUDataOutputTransformer,
    > for CPUExecutor
{
    type ErrorType = Infallible;

    fn input_transformer(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<CPUDataInputTransformer, Self::ErrorType> {
        Ok(CPUDataInputTransformer::new(
            u32::try_from(self.words_per_real_word << 5).unwrap(),
            input_elem_len,
            self.real_input_len,
            bit_mapping,
            true,
        ))
    }
    fn output_transformer(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<CPUDataOutputTransformer, Self::ErrorType> {
        Ok(CPUDataOutputTransformer::new(
            u32::try_from(self.words_per_real_word << 5).unwrap(),
            self.real_output_len,
            output_elem_len,
            bit_mapping,
            true,
        ))
    }
}

struct CircuitEntry {
    sym_name: String,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(Vec<usize>, usize)>,
    output_placement: Option<(Vec<usize>, usize)>,
    arg_input_len: Option<usize>,
    elem_input_len: Option<usize>,
    single_buffer: bool,
    aggregated_output: bool,
    aggregated_to_buffer: bool,
    aggr_output_len: Option<usize>,
    populated_input: bool,
    populated_from_buffer: bool,
    pop_input_len: Option<usize>,
    pop_input_len_from_buffer: Option<usize>,
    exclude_outputs_len: Option<usize>,
    dont_clear_outputs: bool,
    inner_loop: Option<u32>,
}

/// Main CPU builder.
///
/// See more in [Builder].
pub struct CPUBuilder<'a> {
    cpu_ext: CPUExtension,
    entries: Vec<CircuitEntry>,
    writer: CLangWriter<'a>,
    optimize_negs: bool,
    parallel: Option<usize>,
    wire_order: bool,
}

impl<'a> CPUBuilder<'a> {
    /// Creates CPU builder with given CPU builder configuration if supplied, otherwise
    /// it creates CPU builder with default configuration and includes given CPU instruction
    /// set extension and custom CLangWriter configuration.
    pub fn new_with_cpu_ext_and_clang_config(
        cpu_ext: CPUExtension,
        clang_config: &'a CLangWriterConfig,
        config: Option<CPUBuilderConfig>,
    ) -> Self {
        let config = config.unwrap_or(CPU_BUILDER_CONFIG_DEFAULT);
        let array_len = if let Some(alen) = config.array_len {
            assert_ne!(alen, 0);
            Some(alen)
        } else {
            None
        };
        if let Some(par_chunk_len) = config.parallel {
            assert_ne!(par_chunk_len, 0);
        }
        let mut writer = clang_config.writer_with_array_len(array_len);
        writer.prolog();
        Self {
            cpu_ext,
            entries: vec![],
            writer,
            optimize_negs: config.optimize_negs,
            parallel: config.parallel,
            wire_order: config.wire_order,
        }
    }

    /// Creates CPU builder with given CPU builder configuration if supplied, otherwise
    /// it creates CPU builder with default configuration and includes given CPU instruction
    /// set extension.
    pub fn new_with_cpu_ext(cpu_ext: CPUExtension, config: Option<CPUBuilderConfig>) -> Self {
        Self::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            get_build_config(cpu_ext).writer_config,
            config,
        )
    }

    /// Creates new builder with given CPU builder configuration if supplied, otherwise
    /// it creates CPU builder with default configuration.
    pub fn new(config: Option<CPUBuilderConfig>) -> Self {
        Self::new_with_cpu_ext_and_clang_config(
            *CPU_EXTENSION,
            get_build_config(*CPU_EXTENSION).writer_config,
            config,
        )
    }

    /// Creates new builder with given parallel CPU builder configuration if supplied, otherwise
    /// it creates CPU builder with default configuration. It CPU builder replaces
    /// parallel property by `par_chunk_len` value in CPU builder configuration before
    /// creation.
    pub fn new_parallel(config: Option<CPUBuilderConfig>, par_chunk_len: Option<usize>) -> Self {
        let mut config = config.unwrap_or(CPU_BUILDER_CONFIG_DEFAULT);
        config.parallel = Some(par_chunk_len.unwrap_or(4096));
        Self::new_with_cpu_ext_and_clang_config(
            *CPU_EXTENSION,
            get_build_config(*CPU_EXTENSION).writer_config,
            Some(config),
        )
    }
}

impl<'b, 'a> Builder<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder, CPUExecutor>
    for CPUBuilder<'b>
{
    type ErrorType = BuildError;

    fn user_defs(&mut self, user_defs: &str) {
        self.writer.user_defs(user_defs);
    }

    fn transform_helpers(&mut self) {
        self.writer.transform_helpers();
    }

    fn add_with_config<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        let name = format!("{}_{}", name, get_timestamp());
        let sym_name = format!("gate_sys_{}", name);
        self.entries.push(CircuitEntry {
            sym_name,
            input_len: usize::try_from(circuit.input_len()).unwrap(),
            output_len: circuit.outputs().len(),
            input_placement: code_config.input_placement.map(|(p, l)| (p.to_vec(), l)),
            output_placement: code_config.output_placement.map(|(p, l)| (p.to_vec(), l)),
            arg_input_len: code_config.arg_inputs.map(|x| x.len()),
            elem_input_len: code_config.elem_inputs.map(|x| x.len()),
            single_buffer: code_config.single_buffer,
            aggregated_output: code_config.aggr_output_code.is_some(),
            aggregated_to_buffer: code_config.aggr_output_code.is_some()
                && code_config.aggr_to_buffer.is_some(),
            aggr_output_len: if code_config.aggr_output_code.is_some() {
                Some(
                    code_config
                        .aggr_output_len
                        .unwrap_or(default_aggr_output_len(self.word_len())),
                )
            } else {
                None
            },
            populated_input: code_config.pop_input_code.is_some(),
            populated_from_buffer: code_config.pop_input_code.is_some()
                && code_config.pop_from_buffer.is_some(),
            pop_input_len_from_buffer: if code_config.pop_input_code.is_some()
                && code_config.pop_from_buffer.is_some()
            {
                code_config.pop_from_buffer.map(|x| x.len())
            } else {
                None
            },
            pop_input_len: if code_config.pop_input_code.is_some() {
                Some(
                    code_config
                        .pop_input_len
                        .unwrap_or(default_pop_input_len(self.word_len())),
                )
            } else {
                None
            },
            exclude_outputs_len: code_config.exclude_outputs.map(|x| x.len()),
            dont_clear_outputs: code_config.dont_clear_outputs,
            inner_loop: code_config.inner_loop,
        });
        generate_code_with_config_and_wire_order(
            &mut self.writer,
            &name,
            circuit,
            self.optimize_negs,
            self.wire_order,
            code_config,
        );
    }

    fn build(mut self) -> Result<Vec<CPUExecutor>, Self::ErrorType> {
        self.writer.epilog();
        let words_per_real_word = usize::try_from(self.writer.word_len() >> 5).unwrap();
        let shlib = SharedLib::new_with_cpu_ext(self.cpu_ext);
        let source = self.writer.out();
        dump_source_code("CPU Functions", &source);
        let lib = Arc::new(shlib.build(&source)?);
        Ok(self
            .entries
            .iter()
            .map(|e| {
                let lib = lib.clone();
                CPUExecutor {
                    input_len: e.input_len,
                    output_len: e.output_len,
                    real_input_len: e.input_placement.as_ref().map(|x| x.1).unwrap_or(
                        e.input_len
                            - e.arg_input_len.unwrap_or(0)
                            - e.elem_input_len.unwrap_or(0)
                            - e.pop_input_len_from_buffer.unwrap_or(0),
                    ),
                    real_output_len: e
                        .output_placement
                        .as_ref()
                        .map(|x| x.1)
                        .unwrap_or(e.output_len - e.exclude_outputs_len.unwrap_or(0)),
                    words_per_real_word,
                    arg_input_len: e.arg_input_len,
                    elem_input_num: e.elem_input_len.unwrap_or(0),
                    library: lib,
                    sym_name: e.sym_name.clone(),
                    single_buffer: e.single_buffer,
                    aggregated_output: e.aggregated_output,
                    aggregated_to_buffer: e.aggregated_to_buffer,
                    aggr_output_len: e.aggr_output_len,
                    populated_input: e.populated_input,
                    populated_from_buffer: e.populated_from_buffer,
                    pop_input_len_from_buffer: e.pop_input_len_from_buffer,
                    pop_input_len: e.pop_input_len,
                    parallel: self.parallel,
                    dont_clear_outputs: e.dont_clear_outputs,
                    inner_loop: e.inner_loop,
                }
            })
            .collect::<Vec<_>>())
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.writer.word_len()
    }

    #[inline]
    fn type_len(&self) -> u32 {
        self.writer.word_len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    fn is_executor_per_thread() -> bool {
        true
    }

    #[inline]
    fn is_data_holder_global() -> bool {
        true
    }

    #[inline]
    fn is_data_holder_in_builder() -> bool {
        true
    }

    #[inline]
    fn preferred_input_count(&self) -> usize {
        64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cpu_from_file() {
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelMMX,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelSSE,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelSSE2,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2 avx"[..]))
                .unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX2,
            detect_cpu_from_file(&mut BufReader::new(
                &b"flags\t\t: fpu mmx sse sse2 avx avx2"[..]
            ))
            .unwrap()
        );
        if *utils::GATE_SYS_UNTESTED {
            assert_eq!(
                CPUExtension::IntelAVX512,
                detect_cpu_from_file(&mut BufReader::new(
                    &b"flags\t\t: fpu mmx sse sse2 avx avx512f"[..]
                ))
                .unwrap()
            );
        }
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp"[..])).unwrap()
        );
        if *utils::GATE_SYS_UNTESTED {
            assert_eq!(
                CPUExtension::ARMNEON,
                detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp neon"[..])).unwrap()
            );
        }
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(
                &b"Features\t: xxx\nCPU architecture: 8\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(
                &b"Features\t: fp\nCPU architecture: 8\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(
                &b"CPU architecture: 8\nFeatures\t: xxx\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(
                &b"CPU architecture: 8\nFeatures\t: fp\n"[..]
            ))
            .unwrap()
        );
    }

    #[test]
    fn test_shared_lib() {
        let shlib = SharedLib::new_with_cpu_ext(CPUExtension::NoExtension);
        let lib = shlib
            .build(
                r##"
#include <stdint.h>
uint32_t myvalue(uint32_t a, uint32_t b, uint32_t c) {
    return a * b + c;
}
        "##
                .as_bytes(),
            )
            .unwrap();
        let value = unsafe {
            let func: Symbol<unsafe extern "C" fn(u32, u32, u32) -> u32> =
                lib.get(b"myvalue").unwrap();
            func(12, 16, 5)
        };
        assert_eq!(12 * 16 + 5, value);
    }
}

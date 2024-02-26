use crate::clang_writer::*;
use crate::cpu_data_transform::*;
use crate::gencode::generate_code_ext;
use crate::utils::get_timestamp;
use crate::*;
use libloading::{Library, Symbol};
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

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("IO error {0}")]
    IOError(#[from] io::Error),
    #[error("Compile error {0}")]
    CompileError(String),
    #[error("LibLoading error {0}")]
    LibLoadingError(#[from] libloading::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUExtension {
    NoExtension,
    IntelMMX,
    IntelSSE,
    IntelAVX,
    IntelAVX512,
    ARMNEON,
}

fn detect_cpu_from_file(file: impl BufRead) -> Result<CPUExtension, DetectCPUError> {
    let mut have_fp = false;
    let mut is_armv8 = false;
    let mut have_flags = false;
    for rl in file.lines() {
        let line = rl?;
        if line.starts_with("flags") || line.starts_with("Features") {
            if line.find(" avx512").is_some() {
                return Ok(CPUExtension::IntelAVX512);
            } else if line.find(" avx").is_some() {
                return Ok(CPUExtension::IntelAVX);
            } else if line.find(" sse").is_some() {
                return Ok(CPUExtension::IntelSSE);
            } else if line.find(" mmx").is_some() {
                return Ok(CPUExtension::IntelMMX);
            } else if line.find(" neon").is_some() {
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

fn detect_cpu() -> Result<CPUExtension, DetectCPUError> {
    detect_cpu_from_file(BufReader::new(File::open("/proc/cpuinfo")?))
}

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

const BUILD_CONFIG_INTEL_AVX: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX,
    extra_flags: &["-mavx"],
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
        CPUExtension::IntelAVX => BUILD_CONFIG_INTEL_AVX,
        CPUExtension::IntelAVX512 => BUILD_CONFIG_INTEL_AVX512,
        CPUExtension::ARMNEON => BUILD_CONFIG_ARM_NEON,
    }
}

// shared library object

#[dynamic]
static GATE_SYS_CC: String = env::var("GATE_SYS_CC").unwrap_or("clang".to_string());

struct SharedLib {
    cpu_ext: CPUExtension,
    source_path: PathBuf,
    shared_library_path: PathBuf,
}

impl SharedLib {
    fn new_with_cpu_ext(cpu_ext: CPUExtension) -> Self {
        let temp_dir_path = temp_dir();
        let unix_time = get_timestamp();
        Self {
            cpu_ext,
            source_path: temp_dir_path.join(format!("gate_x4x_source_{}.c", unix_time)),
            shared_library_path: temp_dir_path.join(format!("gate_x4x_lib_{}.so", unix_time)),
        }
    }

    fn build(self, source: &[u8]) -> Result<Library, BuildError> {
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

pub struct CPUDataReader<'a> {
    buffer: &'a [u32],
}

impl<'a> DataReader for CPUDataReader<'a> {
    #[inline]
    fn get(&self) -> &[u32] {
        self.buffer
    }
}

pub struct CPUDataWriter<'a> {
    buffer: &'a mut [u32],
}

impl<'a> DataWriter for CPUDataWriter<'a> {
    #[inline]
    fn get_mut(&mut self) -> &mut [u32] {
        self.buffer
    }
}

pub struct CPUDataHolder {
    buffer: Vec<u32>,
    range: Range<usize>,
}

impl CPUDataHolder {
    #[inline]
    pub fn new(data: Vec<u32>) -> Self {
        let len = data.len();
        Self {
            buffer: data,
            range: 0..len,
        }
    }
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
    fn release(self) -> Vec<u32> {
        self.buffer[self.range.clone()].to_vec()
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

#[derive(Clone, Debug)]
pub struct CPUBuilderConfig {
    pub optimize_negs: bool,
}

const CPU_BUILDER_CONFIG_DEFAULT: CPUBuilderConfig = CPUBuilderConfig {
    optimize_negs: true,
};

#[derive(Clone)]
pub struct CPUExecutor {
    input_len: usize,
    output_len: usize,
    real_input_len: usize,
    real_output_len: usize,
    words_per_real_word: usize,
    have_arg_inputs: bool,
    elem_input_num: usize,
    library: Arc<Library>,
    sym_name: String,
    single_buffer: bool,
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

    unsafe fn execute_internal(
        &mut self,
        input: &CPUDataHolder,
        arg_input: u64,
    ) -> Result<CPUDataHolder, Self::ErrorType> {
        let input_r = input.get();
        let input = input_r.get();
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output = vec![0; num * real_output_words];
        if self.elem_input_num != 0 {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                            i as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            i as u32,
                        );
                    }
                }
            }
        } else {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                        );
                    }
                }
            }
        }
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
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if real_input_words != 0 {
            input.len() / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            output_len / real_output_words
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        assert!(output_len >= real_output_words * num);
        if self.elem_input_num != 0 {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                            i as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            i as u32,
                        );
                    }
                }
            }
        } else {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*const u32, *mut u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            input[i * real_input_words..].as_ptr(),
                            output[i * real_output_words..].as_mut_ptr(),
                        );
                    }
                }
            }
        }
        Ok(())
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut CPUDataHolder,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.get().get().len();
        let num = if real_input_words != 0 {
            output_len / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        let mut output_w = output.get_mut();
        let output = output_w.get_mut();
        assert!(output_len >= real_output_words * num);
        if self.elem_input_num != 0 {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                            i as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(output[i * real_output_words..].as_mut_ptr(), i as u32);
                    }
                }
            }
        } else {
            if self.have_arg_inputs {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32, u32, u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(
                            output[i * real_output_words..].as_mut_ptr(),
                            (arg_input & 0xffffffff) as u32,
                            (arg_input >> 32) as u32,
                        );
                    }
                }
            } else {
                let symbol: Symbol<unsafe extern "C" fn(*mut u32)> =
                    unsafe { self.library.get(self.sym_name.as_bytes())? };
                for i in 0..num {
                    unsafe {
                        (symbol)(output[i * real_output_words..].as_mut_ptr());
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

    fn input_tx(
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
    fn output_tx(
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
}

pub struct CPUBuilder<'a> {
    cpu_ext: CPUExtension,
    entries: Vec<CircuitEntry>,
    writer: CLangWriter<'a>,
    optimize_negs: bool,
}

impl<'a> CPUBuilder<'a> {
    pub fn new_with_cpu_ext_and_clang_config(
        cpu_ext: CPUExtension,
        clang_config: &'a CLangWriterConfig,
        config: Option<CPUBuilderConfig>,
    ) -> Self {
        let mut writer = clang_config.writer();
        writer.prolog();
        Self {
            cpu_ext,
            entries: vec![],
            writer,
            optimize_negs: config.unwrap_or(CPU_BUILDER_CONFIG_DEFAULT).optimize_negs,
        }
    }

    pub fn new_with_cpu_ext(cpu_ext: CPUExtension, config: Option<CPUBuilderConfig>) -> Self {
        Self::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            get_build_config(cpu_ext).writer_config,
            config,
        )
    }

    pub fn new(config: Option<CPUBuilderConfig>) -> Self {
        Self::new_with_cpu_ext_and_clang_config(
            *CPU_EXTENSION,
            get_build_config(*CPU_EXTENSION).writer_config,
            config,
        )
    }
}

impl<'b, 'a> Builder<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder, CPUExecutor>
    for CPUBuilder<'b>
{
    type ErrorType = BuildError;

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
        <usize as TryFrom<T>>::Error: Debug,
    {
        let name = format!("{}_{}", name, get_timestamp());
        let sym_name = format!("gate_sys_{}", name);
        self.entries.push(CircuitEntry {
            sym_name,
            input_len: usize::try_from(circuit.input_len()).unwrap(),
            output_len: circuit.outputs().len(),
            input_placement: input_placement.map(|(p, l)| (p.to_vec(), l)),
            output_placement: output_placement.map(|(p, l)| (p.to_vec(), l)),
            arg_input_len: arg_inputs.map(|x| x.len()),
            elem_input_len: elem_inputs.map(|x| x.len()),
            single_buffer,
        });
        generate_code_ext(
            &mut self.writer,
            &name,
            circuit,
            self.optimize_negs,
            input_placement,
            output_placement,
            arg_inputs,
            elem_inputs,
            single_buffer,
        );
    }

    fn build(mut self) -> Result<Vec<CPUExecutor>, Self::ErrorType> {
        self.writer.epilog();
        let words_per_real_word = usize::try_from(self.writer.word_len() >> 5).unwrap();
        let shlib = SharedLib::new_with_cpu_ext(self.cpu_ext);
        let lib = Arc::new(shlib.build(&self.writer.out())?);
        Ok(self
            .entries
            .iter()
            .map(|e| {
                let lib = lib.clone();
                CPUExecutor {
                    input_len: e.input_len,
                    output_len: e.output_len,
                    real_input_len: e.input_placement.as_ref().map(|x| x.1).unwrap_or(
                        e.input_len - e.arg_input_len.unwrap_or(0) - e.elem_input_len.unwrap_or(0),
                    ),
                    real_output_len: e
                        .output_placement
                        .as_ref()
                        .map(|x| x.1)
                        .unwrap_or(e.output_len),
                    words_per_real_word,
                    have_arg_inputs: e.arg_input_len.is_some(),
                    elem_input_num: e.elem_input_len.unwrap_or(0),
                    library: lib,
                    sym_name: e.sym_name.clone(),
                    single_buffer: e.single_buffer,
                }
            })
            .collect::<Vec<_>>())
    }

    #[inline]
    fn word_len(&self) -> u32 {
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
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2 avx"[..]))
                .unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX512,
            detect_cpu_from_file(&mut BufReader::new(
                &b"flags\t\t: fpu mmx sse sse2 avx avx512f"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp neon"[..])).unwrap()
        );
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

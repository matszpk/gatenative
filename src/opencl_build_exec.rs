#![cfg_attr(docsrs, feature(doc_cfg))]
//! Simulation execution on GPU using OpenCL standard.
//!
//! The module provides builder and executors to run simulation on GPU using OpenCL standard.
//! It uses CLangWriter to generate OpenCL C code. The code generator and builder
//! builts separate kernel for every circuit. Currently code for OpenCL devices supports
//! 32-bit processor word, because almost GPU is natively uses that length.
//!
//! Every operation (simulation execution, reading and writing data) will be finished with
//! OpenCL finish command to finish all operations.

use crate::clang_writer::*;
use crate::gencode::generate_code_with_config;
use crate::opencl_data_transform::*;
use crate::utils::{dump_source_code, get_timestamp};
use crate::*;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::Device;
use opencl3::error_codes::ClError;
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, ClMem, CL_MAP_READ, CL_MAP_WRITE, CL_MEM_READ_WRITE};
use opencl3::program::Program;
use opencl3::types::{cl_mem, cl_mem_flags, cl_uint, cl_ulong, CL_BLOCKING};

use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

/// OpenCL Data reader.
///
/// See more in [DataReader].
pub struct OpenCLDataReader<'a> {
    holder: &'a OpenCLDataHolder,
    mem: &'a [u32],
}

impl<'a> OpenCLDataReader<'a> {
    fn new(holder: &'a OpenCLDataHolder, range: &Range<usize>) -> Self {
        let xrange = if !range.is_empty() {
            range.start..range.end
        } else {
            0..1
        };
        let mem = unsafe {
            let mut ptr: cl_mem = std::ptr::null_mut();
            holder
                .cmd_queue
                .enqueue_map_buffer(
                    &holder.buffer,
                    CL_BLOCKING,
                    CL_MAP_READ,
                    4 * xrange.start,
                    4 * (xrange.end - xrange.start),
                    &mut ptr,
                    &[],
                )
                .unwrap();
            ptr
        };
        Self {
            holder,
            mem: unsafe { std::slice::from_raw_parts(mem.cast::<u32>(), range.end - range.start) },
        }
    }
}

impl<'a> DataReader for OpenCLDataReader<'a> {
    #[inline]
    fn get(&self) -> &[u32] {
        self.mem
    }
}

impl<'a> Drop for OpenCLDataReader<'a> {
    fn drop(&mut self) {
        unsafe {
            let mem: cl_mem = self.mem.as_ptr().cast_mut().cast();
            self.holder
                .cmd_queue
                .enqueue_unmap_mem_object(self.holder.buffer.get(), mem, &[])
                .unwrap();
            self.holder.cmd_queue.finish().unwrap();
        }
    }
}

/// CPU Data writer.
///
/// See more in [DataWriter].
pub struct OpenCLDataWriter<'a> {
    holder: &'a OpenCLDataHolder,
    mem: &'a mut [u32],
}

impl<'a> OpenCLDataWriter<'a> {
    fn new(holder: &'a OpenCLDataHolder, range: &Range<usize>) -> Self {
        let xrange = if !range.is_empty() {
            range.start..range.end
        } else {
            0..1
        };
        let mem = unsafe {
            let mut ptr: cl_mem = std::ptr::null_mut();
            holder
                .cmd_queue
                .enqueue_map_buffer(
                    &holder.buffer,
                    CL_BLOCKING,
                    CL_MAP_WRITE,
                    4 * xrange.start,
                    4 * (xrange.end - xrange.start),
                    &mut ptr,
                    &[],
                )
                .unwrap();
            ptr
        };
        Self {
            holder,
            mem: unsafe {
                std::slice::from_raw_parts_mut(mem.cast::<u32>(), range.end - range.start)
            },
        }
    }
}

impl<'a> DataWriter for OpenCLDataWriter<'a> {
    #[inline]
    fn get_mut(&mut self) -> &mut [u32] {
        self.mem
    }
}

impl<'a> Drop for OpenCLDataWriter<'a> {
    fn drop(&mut self) {
        unsafe {
            let mem: cl_mem = self.mem.as_mut_ptr().cast();
            self.holder
                .cmd_queue
                .enqueue_unmap_mem_object(self.holder.buffer.get(), mem, &[])
                .unwrap();
            self.holder.cmd_queue.finish().unwrap();
        }
    }
}

/// CPU Data holder.
///
/// See more in [DataHolder].
pub struct OpenCLDataHolder {
    len: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    pub(crate) buffer: Buffer<u32>,
    pub(crate) range: Range<usize>,
}

impl OpenCLDataHolder {
    /// Creates new OpenCL data holder with `len` length. `context` is OpenCL context,
    /// `cmd_queue` is OpenCL command queue tied with context.
    /// `flags` are OpenCL memory flags.
    pub fn new(
        len: usize,
        context: Arc<Context>,
        cmd_queue: Arc<CommandQueue>,
        flags: cl_mem_flags,
    ) -> OpenCLDataHolder {
        let mut buffer =
            unsafe { Buffer::create(context.deref(), flags, len, std::ptr::null_mut()).unwrap() };
        unsafe {
            cmd_queue
                .enqueue_fill_buffer(&mut buffer, &[0u32], 0, 4 * len, &[])
                .unwrap();
            cmd_queue.finish().unwrap();
        }
        Self {
            len,
            context,
            cmd_queue,
            buffer,
            range: 0..len,
        }
    }

    /// Returns reference to buffer that holds data.
    pub unsafe fn buffer(&self) -> &Buffer<u32> {
        &self.buffer
    }
    /// Returns mutable reference to buffer that holds data.
    pub unsafe fn buffer_mut(&mut self) -> &mut Buffer<u32> {
        &mut self.buffer
    }
}

impl<'a> DataHolder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>> for OpenCLDataHolder {
    #[inline]
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }
    fn get(&'a self) -> OpenCLDataReader<'a> {
        OpenCLDataReader::new(self, &self.range)
    }
    fn get_mut(&'a mut self) -> OpenCLDataWriter<'a> {
        OpenCLDataWriter::new(self, &self.range)
    }
    fn process<F, Out>(&self, mut f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        let r = OpenCLDataReader::new(self, &self.range);
        f(r.get())
    }
    fn process_mut<F, Out>(&mut self, mut f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        let mut w = OpenCLDataWriter::new(self, &self.range);
        f(w.get_mut())
    }
    fn copy(&self) -> Self {
        let len = self.len();
        let mut new = Self::new(
            len,
            self.context.clone(),
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        unsafe {
            self.cmd_queue
                .enqueue_copy_buffer(
                    &self.buffer,
                    &mut new.buffer,
                    self.range.start * 4,
                    0,
                    len * 4,
                    &[],
                )
                .unwrap();
        }
        self.cmd_queue.finish().unwrap();
        new
    }
    fn fill(&mut self, value: u32) {
        unsafe {
            self.cmd_queue
                .enqueue_fill_buffer(
                    &mut self.buffer,
                    &[value],
                    self.range.start * 4,
                    self.range.end * 4,
                    &[],
                )
                .unwrap();
        }
        self.cmd_queue.finish().unwrap();
    }
    fn release(self) -> Vec<u32> {
        let mut out = vec![0u32; self.len()];
        unsafe {
            self.cmd_queue
                .enqueue_read_buffer(
                    &self.buffer,
                    CL_BLOCKING,
                    4 * self.range.start,
                    &mut out[..],
                    &[],
                )
                .unwrap();
        }
        out
    }
    fn free(self) {}
}

impl RangedData for OpenCLDataHolder {
    fn set_range(&mut self, range: Range<usize>) {
        self.range = std::cmp::min(self.len, range.start)..std::cmp::min(self.len, range.end);
        if self.range.start >= self.range.end {
            self.range = 0..0;
        }
    }
}

/// Main OpenCL executor.
///
/// This executor provides data transformers by [DataTransforms]. See more in [Executor].
pub struct OpenCLExecutor {
    input_len: usize,
    output_len: usize,
    real_input_len: usize,
    real_output_len: usize,
    words_per_real_word: usize,
    arg_input_len: Option<usize>,
    elem_input_num: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    program: Arc<Program>,
    group_len: usize,
    kernel: Kernel,
    single_buffer: bool,
    group_vec: bool,
    aggregated_output: bool,
    aggregated_to_buffer: bool,
    aggr_output_len: Option<usize>,
    populated_input: bool,
    populated_from_buffer: bool,
    pop_input_len: Option<usize>,
    pop_input_len_from_buffer: Option<usize>,
    dont_clear_outputs: bool,
    inner_loop: Option<u32>,
}

impl OpenCLExecutor {
    /// Returns OpenCL context.
    pub unsafe fn context(&self) -> Arc<Context> {
        self.context.clone()
    }
    /// Returns OpenCL command queue.
    pub unsafe fn command_queue(&self) -> Arc<CommandQueue> {
        self.cmd_queue.clone()
    }
    /// Returns `group_len` (group length).
    pub unsafe fn group_len(&self) -> usize {
        self.group_len
    }
}

impl<'a> Executor<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder>
    for OpenCLExecutor
{
    type ErrorType = ClError;
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
        input: &OpenCLDataHolder,
        arg_input: u64,
    ) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let real_input_words = if !self.populated_input {
            self.real_input_len * self.words_per_real_word
        } else {
            0
        };
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        let output = OpenCLDataHolder::new(
            if self.aggregated_output {
                self.aggr_output_len.unwrap()
            } else {
                real_output_words * num
            },
            self.context.clone(),
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(output)
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &OpenCLDataHolder,
        arg_input: u64,
        output: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = if !self.populated_input {
            self.real_input_len * self.words_per_real_word
        } else {
            0
        };
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.len();
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            (output.range.end - output.range.start) / real_output_words
        };
        if !self.aggregated_output {
            assert!(output_len >= real_output_words * num);
        }
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(())
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut OpenCLDataHolder,
        arg_input: u64,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.len();
        let num = if self.populated_input {
            (1 << (self.input_len - self.arg_input_len.unwrap_or(0) - 5)) / self.words_per_real_word
        } else if real_input_words != 0 {
            (output.range.end - output.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else {
            0
        };
        if !self.aggregated_output {
            assert!(output_len >= real_output_words * num);
        }
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&output.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(())
    }

    // with buffer argument
    unsafe fn execute_buffer_internal(
        &mut self,
        input: &OpenCLDataHolder,
        arg_input: u64,
        buffer: &mut OpenCLDataHolder,
    ) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            0
        };
        let output = OpenCLDataHolder::new(
            real_output_words * num,
            self.context.clone(),
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        let cl_buffer_start = cl_ulong::try_from(buffer.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(output)
    }

    unsafe fn execute_buffer_reuse_internal(
        &mut self,
        input: &OpenCLDataHolder,
        arg_input: u64,
        output: &mut OpenCLDataHolder,
        buffer: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.len();
        let num = if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            (output.range.end - output.range.start) / real_output_words
        };
        assert!(output_len >= real_output_words * num);
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        let cl_buffer_start = cl_ulong::try_from(buffer.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(())
    }

    unsafe fn execute_buffer_single_internal(
        &mut self,
        output: &mut OpenCLDataHolder,
        arg_input: u64,
        buffer: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let output_len = output.len();
        let num = if real_input_words != 0 {
            (output.range.end - output.range.start) / real_input_words
        } else if self.elem_input_num != 0 {
            (1 << (self.elem_input_num - 5)) / self.words_per_real_word
        } else if let Some(pop_from_buffer_len) = self.pop_input_len_from_buffer {
            (1 << (pop_from_buffer_len - 5)) / self.words_per_real_word
        } else {
            0
        };
        assert!(output_len >= real_output_words * num);
        let cl_num = cl_ulong::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        let cl_buffer_start = cl_ulong::try_from(buffer.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let (num, group_len) = if self.group_vec {
            (num * self.group_len, self.group_len)
        } else {
            // optimize group_len to number of work
            (num, std::cmp::min(num, self.group_len))
        };
        unsafe {
            if self.arg_input_len.is_some() {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&cl_buffer_start)
                    .set_arg(&output.buffer)
                    .set_arg(&buffer.buffer)
                    .set_local_work_size(group_len)
                    .set_global_work_size(((num + group_len - 1) / group_len) * group_len)
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
            self.cmd_queue.finish()?;
        }
        Ok(())
    }

    fn new_data(&mut self, len: usize) -> OpenCLDataHolder {
        OpenCLDataHolder::new(
            len,
            self.context.clone(),
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        )
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> OpenCLDataHolder {
        self.new_data_from_slice(&data)
    }

    fn new_data_from_slice(&mut self, data: &[u32]) -> OpenCLDataHolder {
        let mut output = OpenCLDataHolder::new(
            data.len(),
            self.context.clone(),
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        unsafe {
            self.cmd_queue
                .enqueue_write_buffer(&mut output.buffer, CL_BLOCKING, 0, &data, &[])
                .unwrap();
        }
        output
    }

    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        let name = self.kernel.function_name().unwrap();
        let device = self.context.devices()[0];
        Some(Self {
            input_len: self.input_len,
            output_len: self.output_len,
            real_input_len: self.real_input_len,
            real_output_len: self.real_output_len,
            words_per_real_word: self.words_per_real_word,
            arg_input_len: self.arg_input_len,
            context: self.context.clone(),
            #[allow(deprecated)]
            cmd_queue: Arc::new(unsafe { CommandQueue::create(&self.context, device, 0).unwrap() }),
            group_len: self.group_len,
            program: self.program.clone(),
            kernel: Kernel::create(&self.program, &name).unwrap(),
            single_buffer: self.single_buffer,
            group_vec: self.group_vec,
            elem_input_num: self.elem_input_num,
            aggregated_output: self.aggregated_output,
            aggregated_to_buffer: self.aggregated_to_buffer,
            aggr_output_len: self.aggr_output_len,
            populated_input: self.populated_input,
            populated_from_buffer: self.populated_from_buffer,
            pop_input_len: self.pop_input_len,
            pop_input_len_from_buffer: self.pop_input_len_from_buffer,
            dont_clear_outputs: self.dont_clear_outputs,
            inner_loop: self.inner_loop,
        })
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
        false
    }

    #[inline]
    fn inner_loop(&self) -> Option<u32> {
        self.inner_loop
    }
}

impl Clone for OpenCLExecutor {
    fn clone(&self) -> Self {
        self.try_clone().unwrap()
    }
}

impl<'a>
    DataTransforms<
        'a,
        OpenCLDataReader<'a>,
        OpenCLDataWriter<'a>,
        OpenCLDataHolder,
        OpenCLDataInputTransformer,
        OpenCLDataOutputTransformer,
    > for OpenCLExecutor
{
    type ErrorType = OpenCLBuildError;

    fn input_transformer(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<OpenCLDataInputTransformer, Self::ErrorType> {
        OpenCLDataInputTransformer::new(
            self.context.clone(),
            self.cmd_queue.clone(),
            u32::try_from(self.words_per_real_word << 5).unwrap(),
            input_elem_len,
            self.real_input_len,
            bit_mapping,
        )
    }
    fn output_transformer(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<OpenCLDataOutputTransformer, Self::ErrorType> {
        OpenCLDataOutputTransformer::new(
            self.context.clone(),
            self.cmd_queue.clone(),
            u32::try_from(self.words_per_real_word << 5).unwrap(),
            self.real_output_len,
            output_elem_len,
            bit_mapping,
        )
    }
}

/// Error type for OpenCL builder.
#[derive(Clone, Debug)]
pub enum OpenCLBuildError {
    /// OpenCL error.
    OpenCLError(i32),
    /// Build error.
    BuildError(String),
}

impl std::error::Error for OpenCLBuildError {}

impl std::fmt::Display for OpenCLBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OpenCLBuildError::OpenCLError(err) => {
                let err = ClError(*err);
                write!(f, "OpenCL error: {}", err)?;
            }
            OpenCLBuildError::BuildError(err) => {
                write!(f, "Build error: {}", err)?;
            }
        }
        Ok(())
    }
}

impl From<String> for OpenCLBuildError {
    fn from(v: String) -> Self {
        OpenCLBuildError::BuildError(v)
    }
}

impl From<ClError> for OpenCLBuildError {
    fn from(v: ClError) -> Self {
        OpenCLBuildError::OpenCLError(v.0)
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
/// Structure holds OpenCL builder configuration.
#[derive(Clone, Debug)]
pub struct OpenCLBuilderConfig {
    /// If true then code generator optimizes negation while creating code to simulate circuit.
    pub optimize_negs: bool,
    /// If true then use grouping threads making word processor longer.
    pub group_vec: bool,
    /// If set then set group length for groupping. It must be not greater than maximal
    /// group size for OpenCL device.
    pub group_len: Option<usize>,
    /// Experimental and not recommended. Enables NVIDIA LOP3 instruction generation.
    /// Unfortunatelly, it doesn't improve perfomance.
    pub lop3: bool,
}

impl OpenCLBuilderConfig {
    /// Creates empty OpenCL builder configuration.
    pub fn new() -> Self {
        Self {
            optimize_negs: false,
            group_vec: false,
            group_len: None,
            lop3: false,
        }
    }
    /// Sets optimizaton of negations.
    pub fn optimize_negs(mut self, optimize_negs: bool) -> Self {
        self.optimize_negs = optimize_negs;
        self
    }
    /// Sets usage of grouping.
    pub fn group_vec(mut self, group_vec: bool) -> Self {
        self.group_vec = group_vec;
        self
    }
    /// Sets length of groupping.
    pub fn group_len(mut self, group_len: Option<usize>) -> Self {
        self.group_len = group_len;
        self
    }
    /// Sets NVIDIA LOP3 generation.
    pub fn lop3(mut self, lop3: bool) -> Self {
        self.lop3 = lop3;
        self
    }
}

/// Default CPU builder configuration.
pub const OPENCL_BUILDER_CONFIG_DEFAULT: OpenCLBuilderConfig = OpenCLBuilderConfig {
    optimize_negs: true,
    group_vec: false,
    group_len: None,
    lop3: false, // now is disabled in default config
};

/// Main OpenCL builder.
///
/// See more in [Builder].
pub struct OpenCLBuilder<'a> {
    entries: Vec<CircuitEntry>,
    writer: CLangWriter<'a>,
    optimize_negs: bool,
    group_vec: bool,
    group_len: usize,
    context: Arc<Context>,
}

/// Returns preferred work group size for device (includes specifics of NVIDIA GPU's).
pub fn get_preferred_work_group_size(device: &Device) -> usize {
    let group_len = if let Ok(vendor) = device.vendor() {
        if vendor.starts_with("NVIDIA") {
            // preferred for bigger kernels
            256
        } else {
            device.max_work_group_size().unwrap()
        }
    } else {
        device.max_work_group_size().unwrap()
    };
    usize::try_from(group_len).unwrap()
}

/// Detects whether NVIDIA GPU can execute LOP3 instruction.
pub fn detect_nvidia_lop3(device: &Device) -> bool {
    if let Ok(comp_cap_major) = device.compute_capability_major_nv() {
        if let Ok(comp_cap_minor) = device.compute_capability_minor_nv() {
            // lop3 available from ComputeCapability >= 5.0
            (comp_cap_major, comp_cap_minor) >= (5, 0)
        } else {
            false
        }
    } else {
        false
    }
}

impl<'a> OpenCLBuilder<'a> {
    /// Creates new OpenCL builder for given OpenCL device and given OpenCL
    /// builder configuration from `config`.
    pub fn new(device: &Device, config: Option<OpenCLBuilderConfig>) -> Self {
        let config = config.unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT);
        let lop3 = detect_nvidia_lop3(device);
        let mut writer = if config.group_vec {
            if config.lop3 && lop3 {
                CLANG_WRITER_OPENCL_U32_LOP3_GROUP_VEC.writer()
            } else {
                CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer()
            }
        } else {
            if config.lop3 && lop3 {
                CLANG_WRITER_OPENCL_U32_LOP3.writer()
            } else {
                CLANG_WRITER_OPENCL_U32.writer()
            }
        };
        if let Some(group_len) = config.group_len {
            assert_ne!(group_len, 0);
        }
        writer.prolog();
        Self {
            entries: vec![],
            writer,
            optimize_negs: config.optimize_negs,
            group_vec: config.group_vec,
            group_len: config
                .group_len
                .unwrap_or(get_preferred_work_group_size(&device)),
            context: Arc::new(Context::from_device(device).unwrap()),
        }
    }

    /// Creates new OpenCL builder for given OpenCL context and given OpenCL
    /// builder configuration from `config`.
    pub fn new_with_context(context: Arc<Context>, config: Option<OpenCLBuilderConfig>) -> Self {
        let config = config.unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT);
        let lop3 = {
            let device_id = context.devices()[0];
            let device = Device::from(device_id);
            detect_nvidia_lop3(&device)
        };
        let mut writer = if config.group_vec {
            if config.lop3 && lop3 {
                CLANG_WRITER_OPENCL_U32_LOP3_GROUP_VEC.writer()
            } else {
                CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer()
            }
        } else {
            if config.lop3 && lop3 {
                CLANG_WRITER_OPENCL_U32_LOP3.writer()
            } else {
                CLANG_WRITER_OPENCL_U32.writer()
            }
        };
        if let Some(group_len) = config.group_len {
            assert_ne!(group_len, 0);
        }
        writer.prolog();
        let device = Device::new(context.devices()[0]);
        Self {
            entries: vec![],
            writer,
            optimize_negs: config.optimize_negs,
            group_vec: config.group_vec,
            group_len: config
                .group_len
                .unwrap_or(get_preferred_work_group_size(&device)),
            context,
        }
    }
}

impl<'b, 'a>
    Builder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder, OpenCLExecutor>
    for OpenCLBuilder<'b>
{
    type ErrorType = OpenCLBuildError;

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
        generate_code_with_config(
            &mut self.writer,
            &name,
            circuit,
            self.optimize_negs,
            code_config,
        );
    }

    fn build(mut self) -> Result<Vec<OpenCLExecutor>, Self::ErrorType> {
        self.writer.epilog();
        let words_per_real_word = usize::try_from(self.word_len() >> 5).unwrap();
        let device = self.context.devices()[0];
        #[allow(deprecated)]
        let cmd_queue = Arc::new(unsafe { CommandQueue::create(&self.context, device, 0)? });
        let source = self.writer.out();
        dump_source_code("OpenCL Functions", &source);
        let program = Arc::new(Program::create_and_build_from_source(
            &self.context,
            &String::from_utf8(source).unwrap(),
            "",
        )?);
        self.entries
            .iter()
            .map(|e| {
                Ok(OpenCLExecutor {
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
                    context: self.context.clone(),
                    cmd_queue: cmd_queue.clone(),
                    program: program.clone(),
                    group_len: self.group_len,
                    kernel: Kernel::create(&program, &e.sym_name)?,
                    single_buffer: e.single_buffer,
                    group_vec: self.group_vec,
                    aggregated_output: e.aggregated_output,
                    aggregated_to_buffer: e.aggregated_to_buffer,
                    aggr_output_len: e.aggr_output_len,
                    populated_input: e.populated_input,
                    populated_from_buffer: e.populated_from_buffer,
                    pop_input_len: e.pop_input_len,
                    pop_input_len_from_buffer: e.pop_input_len_from_buffer,
                    dont_clear_outputs: e.dont_clear_outputs,
                    inner_loop: e.inner_loop,
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }

    #[inline]
    fn word_len(&self) -> u32 {
        if self.group_vec {
            u32::try_from(usize::try_from(self.writer.word_len()).unwrap() * self.group_len)
                .unwrap()
        } else {
            self.writer.word_len()
        }
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
        false
    }

    #[inline]
    fn is_data_holder_global() -> bool {
        false
    }

    #[inline]
    fn is_data_holder_in_builder() -> bool {
        true
    }

    fn preferred_input_count(&self) -> usize {
        self.context
            .devices()
            .into_iter()
            .map(|device_id| {
                let device = Device::new(device_id.clone());
                let group_len = usize::try_from(get_preferred_work_group_size(&device)).unwrap();
                let compute_units = usize::try_from(device.max_compute_units().unwrap()).unwrap();
                compute_units * group_len
            })
            .max()
            .unwrap()
    }
}

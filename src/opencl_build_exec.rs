use crate::clang_writer::*;
use crate::gencode::generate_code;
use crate::*;

use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::error_codes::ClError;
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{
    Buffer, ClMem, CL_MAP_READ, CL_MAP_WRITE, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE,
    CL_MEM_WRITE_ONLY,
};
use opencl3::program::Program;
use opencl3::types::{
    cl_bool, cl_event, cl_map_flags, cl_mem, cl_mem_flags, cl_uint, CL_BLOCKING, CL_NON_BLOCKING,
};

use std::sync::Arc;

pub struct OpenCLDataReader<'a> {
    holder: &'a OpenCLDataHolder,
    mem: cl_mem,
}

impl<'a> OpenCLDataReader<'a> {
    fn new(holder: &'a OpenCLDataHolder) -> Self {
        let mem = unsafe {
            let mut ptr: cl_mem = std::ptr::null_mut();
            holder
                .cmd_queue
                .enqueue_map_buffer(
                    &holder.buffer,
                    CL_BLOCKING,
                    CL_MAP_READ,
                    0,
                    4 * holder.len,
                    &mut ptr,
                    &[],
                )
                .unwrap();
            ptr
        };
        Self { holder, mem }
    }
}

impl<'a> DataReader for OpenCLDataReader<'a> {
    #[inline]
    fn get(&self) -> &[u32] {
        unsafe { std::slice::from_raw_parts(self.mem.cast::<u32>(), self.holder.len) }
    }
}

impl<'a> Drop for OpenCLDataReader<'a> {
    fn drop(&mut self) {
        unsafe {
            self.holder
                .cmd_queue
                .enqueue_unmap_mem_object(self.holder.buffer.get(), self.mem, &[])
                .unwrap();
            self.holder.cmd_queue.finish().unwrap();
        }
    }
}

pub struct OpenCLDataWriter<'a> {
    holder: &'a OpenCLDataHolder,
    mem: cl_mem,
}

impl<'a> OpenCLDataWriter<'a> {
    fn new(holder: &'a OpenCLDataHolder) -> Self {
        let mem = unsafe {
            let mut ptr: cl_mem = std::ptr::null_mut();
            holder
                .cmd_queue
                .enqueue_map_buffer(
                    &holder.buffer,
                    CL_BLOCKING,
                    CL_MAP_WRITE,
                    0,
                    4 * holder.len,
                    &mut ptr,
                    &[],
                )
                .unwrap();
            ptr
        };
        Self { holder, mem }
    }
}

impl<'a> DataWriter for OpenCLDataWriter<'a> {
    #[inline]
    fn get_mut(&mut self) -> &mut [u32] {
        unsafe { std::slice::from_raw_parts_mut(self.mem.cast::<u32>(), self.holder.len) }
    }
}

impl<'a> Drop for OpenCLDataWriter<'a> {
    fn drop(&mut self) {
        unsafe {
            self.holder
                .cmd_queue
                .enqueue_unmap_mem_object(self.holder.buffer.get(), self.mem, &[])
                .unwrap();
            self.holder.cmd_queue.finish().unwrap();
        }
    }
}

pub struct OpenCLDataHolder {
    len: usize,
    cmd_queue: Arc<CommandQueue>,
    buffer: Buffer<u32>,
}

impl OpenCLDataHolder {
    fn new(
        len: usize,
        context: &Context,
        cmd_queue: Arc<CommandQueue>,
        flags: cl_mem_flags,
    ) -> OpenCLDataHolder {
        let buffer = unsafe { Buffer::create(context, flags, len, std::ptr::null_mut()).unwrap() };
        Self {
            len,
            cmd_queue,
            buffer,
        }
    }
}

impl<'a> DataHolder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>> for OpenCLDataHolder {
    fn get(&'a self) -> OpenCLDataReader<'a> {
        OpenCLDataReader::new(self)
    }
    fn get_mut(&'a mut self) -> OpenCLDataWriter<'a> {
        OpenCLDataWriter::new(self)
    }
    fn release(self) -> Vec<u32> {
        let mut out = vec![0u32; self.len];
        unsafe {
            self.cmd_queue
                .enqueue_read_buffer(&self.buffer, CL_BLOCKING, 0, &mut out[..], &[])
                .unwrap();
        }
        out
    }
    fn free(self) {}
}

pub struct OpenCLExecutor {
    input_len: usize,
    output_len: usize,
    real_input_len: usize,
    real_output_len: usize,
    words_per_real_word: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    group_len: usize,
    kernel: Kernel,
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

    fn execute(&mut self, input: &OpenCLDataHolder) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            input.len / real_input_words
        } else {
            0
        };
        let output = OpenCLDataHolder::new(
            real_output_words * num,
            &self.context,
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        let cl_num = cl_uint::try_from(num).unwrap();
        unsafe {
            ExecuteKernel::new(&self.kernel)
                .set_arg(&cl_num)
                .set_arg(&input.buffer)
                .set_arg(&output.buffer)
                .set_local_work_size(self.group_len)
                .set_global_work_size(
                    ((num + self.group_len - 1) / self.group_len) * self.group_len,
                )
                .enqueue_nd_range(&self.cmd_queue)
                .unwrap();
            self.cmd_queue.finish().unwrap();
        }
        Ok(output)
    }

    fn execute_reuse(
        &mut self,
        input: &OpenCLDataHolder,
        output: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            input.len / real_input_words
        } else {
            output.len / real_output_words
        };
        let cl_num = cl_uint::try_from(num).unwrap();
        unsafe {
            ExecuteKernel::new(&self.kernel)
                .set_arg(&cl_num)
                .set_arg(&input.buffer)
                .set_arg(&output.buffer)
                .set_local_work_size(self.group_len)
                .set_global_work_size(
                    ((num + self.group_len - 1) / self.group_len) * self.group_len,
                )
                .enqueue_nd_range(&self.cmd_queue)
                .unwrap();
            self.cmd_queue.finish().unwrap();
        }
        Ok(())
    }

    fn new_data(&mut self, len: usize) -> OpenCLDataHolder {
        OpenCLDataHolder::new(
            len,
            &self.context,
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        )
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> OpenCLDataHolder {
        let mut output = OpenCLDataHolder::new(
            data.len(),
            &self.context,
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
}

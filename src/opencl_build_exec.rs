use crate::clang_writer::*;
use crate::gencode::generate_code;
use crate::*;
use gatesim::*;

use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{
    Buffer, ClMem, CL_MAP_READ, CL_MAP_WRITE, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE,
    CL_MEM_WRITE_ONLY,
};
use opencl3::program::Program;
use opencl3::types::{
    cl_bool, cl_event, cl_map_flags, cl_mem, cl_uint, CL_BLOCKING, CL_NON_BLOCKING,
};

use libloading::{Library, Symbol};
use static_init::dynamic;
use thiserror::Error;

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

use crate::clang_writer::*;
use crate::gencode::generate_code;
use crate::utils::get_timestamp;
use crate::*;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::Device;
use opencl3::error_codes::ClError;
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, ClMem, CL_MAP_READ, CL_MAP_WRITE, CL_MEM_READ_WRITE};
use opencl3::program::Program;
use opencl3::types::{cl_mem, cl_mem_flags, cl_uint, CL_BLOCKING};

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

    pub unsafe fn buffer(&self) -> &Buffer<u32> {
        &self.buffer
    }
    pub unsafe fn buffer_mut(&mut self) -> &mut Buffer<u32> {
        &mut self.buffer
    }
}

impl<'a> DataHolder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>> for OpenCLDataHolder {
    #[inline]
    fn len(&'a self) -> usize {
        self.len
    }
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
    arg_input_len: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    group_len: usize,
    kernel: Kernel,
}

impl OpenCLExecutor {
    pub unsafe fn context(&self) -> Arc<Context> {
        self.context.clone()
    }
    pub unsafe fn command_queue(&self) -> Arc<CommandQueue> {
        self.cmd_queue.clone()
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
                .enqueue_nd_range(&self.cmd_queue)?;
            self.cmd_queue.finish()?;
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
                .enqueue_nd_range(&self.cmd_queue)?;
            self.cmd_queue.finish()?;
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

#[derive(Clone, Debug)]
pub enum OpenCLBuildError {
    OpenCLError(i32),
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
    arg_input_len: usize,
}

#[derive(Clone, Debug)]
pub struct OpenCLBuilderConfig {
    pub optimize_negs: bool,
}

const OPENCL_BUILDER_CONFIG_DEFAULT: OpenCLBuilderConfig = OpenCLBuilderConfig {
    optimize_negs: true,
};

pub struct OpenCLBuilder<'a> {
    entries: Vec<CircuitEntry>,
    writer: CLangWriter<'a>,
    optimize_negs: bool,
    context: Arc<Context>,
}

impl<'a> OpenCLBuilder<'a> {
    pub fn new(device: &Device, config: Option<OpenCLBuilderConfig>) -> Self {
        let mut writer = CLANG_WRITER_OPENCL_U32.writer();
        writer.prolog();
        Self {
            entries: vec![],
            writer,
            optimize_negs: config
                .unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT)
                .optimize_negs,
            context: Arc::new(Context::from_device(device).unwrap()),
        }
    }

    pub fn new_with_context(context: Arc<Context>, config: Option<OpenCLBuilderConfig>) -> Self {
        let mut writer = CLANG_WRITER_OPENCL_U32.writer();
        writer.prolog();
        Self {
            entries: vec![],
            writer,
            optimize_negs: config
                .unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT)
                .optimize_negs,
            context,
        }
    }
}

impl<'b, 'a>
    Builder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder, OpenCLExecutor>
    for OpenCLBuilder<'b>
{
    type ErrorType = OpenCLBuildError;

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
        let name = format!("{}_{}", name, get_timestamp());
        let sym_name = format!("gate_sys_{}", name);
        self.entries.push(CircuitEntry {
            sym_name,
            input_len: usize::try_from(circuit.input_len()).unwrap(),
            output_len: circuit.outputs().len(),
            input_placement: input_placement.map(|(p, l)| (p.to_vec(), l)),
            output_placement: output_placement.map(|(p, l)| (p.to_vec(), l)),
            arg_input_len: arg_inputs.map(|x| x.len()).unwrap_or(0),
        });
        generate_code(
            &mut self.writer,
            &name,
            circuit,
            self.optimize_negs,
            input_placement,
            output_placement,
            arg_inputs,
        );
    }

    fn build(mut self) -> Result<Vec<OpenCLExecutor>, Self::ErrorType> {
        self.writer.epilog();
        let words_per_real_word = usize::try_from(self.writer.word_len() >> 5).unwrap();
        let device = self.context.devices()[0];
        #[allow(deprecated)]
        let cmd_queue = Arc::new(unsafe { CommandQueue::create(&self.context, device, 0)? });
        let program = Program::create_and_build_from_source(
            &self.context,
            &String::from_utf8(self.writer.out()).unwrap(),
            "",
        )?;
        let device = Device::new(device);
        let group_len = usize::try_from(device.max_work_group_size()?).unwrap();
        self.entries
            .iter()
            .map(|e| {
                Ok(OpenCLExecutor {
                    input_len: e.input_len,
                    output_len: e.output_len,
                    real_input_len: e
                        .input_placement
                        .as_ref()
                        .map(|x| x.1)
                        .unwrap_or(e.input_len - e.arg_input_len),
                    real_output_len: e
                        .output_placement
                        .as_ref()
                        .map(|x| x.1)
                        .unwrap_or(e.output_len),
                    words_per_real_word,
                    arg_input_len: e.arg_input_len,
                    context: self.context.clone(),
                    cmd_queue: cmd_queue.clone(),
                    group_len,
                    kernel: Kernel::create(&program, &e.sym_name)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn word_len(&self) -> u32 {
        self.writer.word_len()
    }
}

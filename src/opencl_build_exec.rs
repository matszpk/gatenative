use crate::clang_writer::*;
use crate::gencode::generate_code_ext;
use crate::utils::get_timestamp;
use crate::*;
use static_init::dynamic;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::Device;
use opencl3::error_codes::ClError;
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, ClMem, CL_MAP_READ, CL_MAP_WRITE, CL_MEM_READ_WRITE};
use opencl3::program::Program;
use opencl3::types::{cl_mem, cl_mem_flags, cl_uint, CL_BLOCKING};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

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

pub struct OpenCLDataHolder {
    len: usize,
    cmd_queue: Arc<CommandQueue>,
    buffer: Buffer<u32>,
    range: Range<usize>,
}

impl OpenCLDataHolder {
    pub fn new(
        len: usize,
        context: &Context,
        cmd_queue: Arc<CommandQueue>,
        flags: cl_mem_flags,
    ) -> OpenCLDataHolder {
        let mut buffer =
            unsafe { Buffer::create(context, flags, len, std::ptr::null_mut()).unwrap() };
        unsafe {
            cmd_queue
                .enqueue_fill_buffer(&mut buffer, &[0u32], 0, 4 * len, &[])
                .unwrap();
            cmd_queue.finish().unwrap();
        }
        Self {
            len,
            cmd_queue,
            buffer,
            range: 0..len,
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

pub struct OpenCLExecutor {
    input_len: usize,
    output_len: usize,
    real_input_len: usize,
    real_output_len: usize,
    words_per_real_word: usize,
    have_arg_inputs: bool,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    program: Arc<Program>,
    group_len: usize,
    kernel: Kernel,
    single_buffer: bool,
    group_vec: bool,
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

    unsafe fn execute_internal(
        &mut self,
        input: &OpenCLDataHolder,
        arg_input: u64,
    ) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
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
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_uint::try_from(input.range.start).unwrap();
        let cl_output_start = cl_uint::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let num = if self.group_vec {
            num * self.group_len
        } else {
            num
        };
        unsafe {
            if self.have_arg_inputs {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
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
        let real_input_words = self.real_input_len * self.words_per_real_word;
        let real_output_words = self.real_output_len * self.words_per_real_word;
        let num = if real_input_words != 0 {
            (input.range.end - input.range.start) / real_input_words
        } else {
            (output.range.end - output.range.start) / real_output_words
        };
        let cl_num = cl_uint::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_input_start = cl_uint::try_from(input.range.start).unwrap();
        let cl_output_start = cl_uint::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let num = if self.group_vec {
            num * self.group_len
        } else {
            num
        };
        unsafe {
            if self.have_arg_inputs {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_input_start)
                    .set_arg(&cl_output_start)
                    .set_arg(&input.buffer)
                    .set_arg(&output.buffer)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
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
        let num = if real_input_words != 0 {
            (output.range.end - output.range.start) / real_input_words
        } else {
            0
        };
        let cl_num = cl_uint::try_from(num).unwrap();
        let cl_arg_input = cl_uint::try_from(arg_input & 0xffffffff).unwrap();
        let cl_arg_input_2 = cl_uint::try_from(arg_input >> 32).unwrap();
        let cl_output_start = cl_uint::try_from(output.range.start).unwrap();
        // kernel worksize: if group_vec: group_len*num
        let num = if self.group_vec {
            num * self.group_len
        } else {
            num
        };
        unsafe {
            if self.have_arg_inputs {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&output.buffer)
                    .set_arg(&cl_arg_input)
                    .set_arg(&cl_arg_input_2)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
                    .enqueue_nd_range(&self.cmd_queue)?;
            } else {
                ExecuteKernel::new(&self.kernel)
                    .set_arg(&cl_num)
                    .set_arg(&cl_output_start)
                    .set_arg(&output.buffer)
                    .set_local_work_size(self.group_len)
                    .set_global_work_size(
                        ((num + self.group_len - 1) / self.group_len) * self.group_len,
                    )
                    .enqueue_nd_range(&self.cmd_queue)?;
            }
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
        self.new_data_from_slice(&data)
    }

    fn new_data_from_slice(&mut self, data: &[u32]) -> OpenCLDataHolder {
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
            have_arg_inputs: self.have_arg_inputs,
            context: self.context.clone(),
            #[allow(deprecated)]
            cmd_queue: Arc::new(unsafe { CommandQueue::create(&self.context, device, 0).unwrap() }),
            group_len: self.group_len,
            program: self.program.clone(),
            kernel: Kernel::create(&self.program, &name).unwrap(),
            single_buffer: self.single_buffer,
            group_vec: self.group_vec,
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

    fn input_tx(
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
    fn output_tx(
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
    arg_input_len: Option<usize>,
    single_buffer: bool,
}

#[derive(Clone, Debug)]
pub struct OpenCLBuilderConfig {
    pub optimize_negs: bool,
    pub group_vec: bool,
    pub group_len: Option<usize>,
}

const OPENCL_BUILDER_CONFIG_DEFAULT: OpenCLBuilderConfig = OpenCLBuilderConfig {
    optimize_negs: true,
    group_vec: false,
    group_len: None,
};

pub struct OpenCLBuilder<'a> {
    entries: Vec<CircuitEntry>,
    writer: CLangWriter<'a>,
    optimize_negs: bool,
    group_vec: bool,
    group_len: usize,
    context: Arc<Context>,
}

impl<'a> OpenCLBuilder<'a> {
    pub fn new(device: &Device, config: Option<OpenCLBuilderConfig>) -> Self {
        let config = config.unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT);
        let mut writer = if config.group_vec {
            CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer()
        } else {
            CLANG_WRITER_OPENCL_U32.writer()
        };
        writer.prolog();
        Self {
            entries: vec![],
            writer,
            optimize_negs: config.optimize_negs,
            group_vec: config.group_vec,
            group_len: config
                .group_len
                .unwrap_or(usize::try_from(device.max_work_group_size().unwrap()).unwrap()),
            context: Arc::new(Context::from_device(device).unwrap()),
        }
    }

    pub fn new_with_context(context: Arc<Context>, config: Option<OpenCLBuilderConfig>) -> Self {
        let config = config.unwrap_or(OPENCL_BUILDER_CONFIG_DEFAULT);
        let mut writer = if config.group_vec {
            CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer()
        } else {
            CLANG_WRITER_OPENCL_U32.writer()
        };
        writer.prolog();
        let device = Device::new(context.devices()[0]);
        Self {
            entries: vec![],
            writer,
            optimize_negs: config.optimize_negs,
            group_vec: config.group_vec,
            group_len: config
                .group_len
                .unwrap_or(usize::try_from(device.max_work_group_size().unwrap()).unwrap()),
            context,
        }
    }
}

impl<'b, 'a>
    Builder<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder, OpenCLExecutor>
    for OpenCLBuilder<'b>
{
    type ErrorType = OpenCLBuildError;

    fn add_ext<T>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
        arg_inputs: Option<&[usize]>,
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
            None,
            single_buffer,
        );
    }

    fn build(mut self) -> Result<Vec<OpenCLExecutor>, Self::ErrorType> {
        self.writer.epilog();
        let words_per_real_word = usize::try_from(self.word_len() >> 5).unwrap();
        let device = self.context.devices()[0];
        #[allow(deprecated)]
        let cmd_queue = Arc::new(unsafe { CommandQueue::create(&self.context, device, 0)? });
        let program = Arc::new(Program::create_and_build_from_source(
            &self.context,
            &String::from_utf8(self.writer.out()).unwrap(),
            "",
        )?);
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
                        .unwrap_or(e.input_len - e.arg_input_len.unwrap_or(0)),
                    real_output_len: e
                        .output_placement
                        .as_ref()
                        .map(|x| x.1)
                        .unwrap_or(e.output_len),
                    words_per_real_word,
                    have_arg_inputs: e.arg_input_len.is_some(),
                    context: self.context.clone(),
                    cmd_queue: cmd_queue.clone(),
                    program: program.clone(),
                    group_len: self.group_len,
                    kernel: Kernel::create(&program, &e.sym_name)?,
                    single_buffer: e.single_buffer,
                    group_vec: self.group_vec,
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
                let group_len = usize::try_from(device.max_work_group_size().unwrap()).unwrap();
                let compute_units = usize::try_from(device.max_compute_units().unwrap()).unwrap();
                compute_units * group_len
            })
            .max()
            .unwrap()
    }
}

struct HashableContext(Arc<Context>);

impl Eq for HashableContext {}

impl PartialEq for HashableContext {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Hash for HashableContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().hash(state);
    }
}

const INPUT_TRANSFORMER_SOURCE: &'static str = r##"
kernel void xxx_gate_input_transform(uint n, uint input_start, uint output_start, uint word_len,
        uint input_elem_len, uint output_elem_len, uint bit_mapping_len,
        const global uint* bit_mapping, const global uint* input, global uint* output) {
    const uint i = get_global_id(0);
    if (i >= n) return;
    uint ibi;
    const uint word_w = word_len >> 5;
    const uint gidx = i / word_len;
    const uint widx = (i>>5) - gidx * word_w;
    const uint sbit = i & 31;
    const uint input_elem_word_num = input_elem_len >> 5;
    const uint output_group_word_num = output_elem_len * word_w;
    const global uint* input_elem = input + i*input_elem_word_num + input_start;
    global uint* output_group = output + widx + gidx*output_group_word_num + output_start;
    for (ibi = 0; ibi < bit_mapping_len; ibi++) {
        const uint inbit = bit_mapping[ibi];
        const uint inbit_val = (input_elem[inbit >> 5] >> (inbit & 31)) & 1;
        atomic_or(output_group + word_w*ibi, (inbit_val << sbit));
    }
}
"##;

#[dynamic]
static mut INPUT_TX_PROGRAMS: HashMap<HashableContext, Arc<Program>> = HashMap::new();

fn get_input_tx_program(context: Arc<Context>) -> Result<Arc<Program>, OpenCLBuildError> {
    let mut tx_programs = INPUT_TX_PROGRAMS.write();
    if let Some(p) = tx_programs.get(&HashableContext(context.clone())) {
        Ok(p.clone())
    } else {
        let p = Arc::new(Program::create_and_build_from_source(
            &context,
            INPUT_TRANSFORMER_SOURCE,
            "",
        )?);
        tx_programs.insert(HashableContext(context.clone()), p.clone());
        Ok(p)
    }
}

/// convert input data into circuit input form.
pub struct OpenCLDataInputTransformer {
    word_len: u32,
    input_elem_len: usize,
    output_elem_len: usize,
    bit_mapping: Buffer<u32>,
    bit_mapping_len: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    kernel: Kernel,
    group_len: usize,
}

impl OpenCLDataInputTransformer {
    /// An bit_mapping - index is bit of output's element, value is bit of input's element.
    pub fn new(
        context: Arc<Context>,
        cmd_queue: Arc<CommandQueue>,
        word_len: u32,
        input_elem_len: usize,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<Self, OpenCLBuildError> {
        assert_eq!((word_len & 31), 0);
        assert_eq!((input_elem_len & 31), 0);
        assert!(input_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(output_elem_len >= bit_mapping.len());
        let device = Device::new(context.devices()[0]);
        let program = get_input_tx_program(context.clone())?;
        let mut buffer = unsafe {
            Buffer::<u32>::create(
                &context,
                CL_MEM_READ_WRITE,
                bit_mapping.len(),
                std::ptr::null_mut(),
            )
            .unwrap()
        };
        let bit_mapping_len = bit_mapping.len();
        unsafe {
            let mut bit_mapping = bit_mapping
                .iter()
                .map(|x| u32::try_from(*x).unwrap())
                .collect::<Vec<_>>();
            cmd_queue.enqueue_write_buffer(
                &mut buffer,
                CL_BLOCKING,
                0,
                &mut bit_mapping[..],
                &[],
            )?;
        }
        Ok(Self {
            word_len,
            input_elem_len: ((input_elem_len + 31) >> 5) << 5,
            output_elem_len,
            bit_mapping: buffer,
            bit_mapping_len,
            context,
            cmd_queue,
            group_len: usize::try_from(device.max_work_group_size().unwrap()).unwrap(),
            kernel: Kernel::create(&program, "xxx_gate_input_transform").unwrap(),
        })
    }

    pub unsafe fn context(&self) -> Arc<Context> {
        self.context.clone()
    }
    pub unsafe fn command_queue(&self) -> Arc<CommandQueue> {
        self.cmd_queue.clone()
    }
}

impl<'a> DataTransformer<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder>
    for OpenCLDataInputTransformer
{
    type ErrorType = ClError;

    fn transform(&mut self, input: &OpenCLDataHolder) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let mut output = OpenCLDataHolder::new(
            self.output_data_len(input.len()),
            &self.context,
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        self.transform_reuse(input, &mut output)?;
        Ok(output)
    }

    fn transform_reuse(
        &mut self,
        input: &OpenCLDataHolder,
        output: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        assert_eq!(
            input.len() % (((self.word_len as usize) * self.input_elem_len) >> 5),
            0
        );
        assert_eq!(
            output.len() % (((self.word_len as usize) * self.output_elem_len) >> 5),
            0
        );
        let input_elem_word_num = self.input_elem_len >> 5;
        let elem_num = input.len() / input_elem_word_num;
        let num = elem_num;
        let cl_num = cl_uint::try_from(num).unwrap();
        // println!("ddebug: {} {} {} {}",
        //          elem_num, num, self.word_len_fac1_pow, self.word_len_fac2);
        let cl_input_start = cl_uint::try_from(input.range.start).unwrap();
        let cl_output_start = cl_uint::try_from(output.range.start).unwrap();
        let cl_word_len = cl_uint::try_from(self.word_len).unwrap();
        let cl_input_elem_len = cl_uint::try_from(self.input_elem_len).unwrap();
        let cl_output_elem_len = cl_uint::try_from(self.output_elem_len).unwrap();
        let cl_bit_mapping_len = cl_uint::try_from(self.bit_mapping_len).unwrap();
        let output_len = output.len();
        unsafe {
            self.cmd_queue.enqueue_fill_buffer(
                &mut output.buffer,
                &[0u32],
                0,
                4 * output_len,
                &[],
            )?;
        }
        self.cmd_queue.finish().unwrap();
        unsafe {
            ExecuteKernel::new(&self.kernel)
                .set_arg(&cl_num)
                .set_arg(&cl_input_start)
                .set_arg(&cl_output_start)
                .set_arg(&cl_word_len)
                .set_arg(&cl_input_elem_len)
                .set_arg(&cl_output_elem_len)
                .set_arg(&cl_bit_mapping_len)
                .set_arg(&self.bit_mapping)
                .set_arg(&input.buffer)
                .set_arg(&output.buffer)
                .set_local_work_size(self.group_len)
                .set_global_work_size(
                    ((num + self.group_len - 1) / self.group_len) * self.group_len,
                )
                .enqueue_nd_range(&self.cmd_queue)?;
        }
        self.cmd_queue.finish()?;
        Ok(())
    }

    fn input_elem_len(&self) -> usize {
        self.input_elem_len
    }
    fn output_elem_len(&self) -> usize {
        self.output_elem_len
    }
}

const OUTPUT_TRANSFORMER_SOURCE: &'static str = r##"
kernel void xxx_gate_output_transform(uint n, uint output_start, uint input_start, uint word_len,
        uint input_elem_len, uint output_elem_len, uint bit_mapping_len,
        const global uint* bit_mapping, const global uint* output, global uint* input) {
    const uint i = get_global_id(0);
    if (i >= n) return;
    uint ibi;
    const uint word_w = word_len >> 5;
    const uint gidx = i / word_len;
    const uint widx = (i>>5) - gidx * word_w;
    const uint sbit = i & 31;
    const uint input_elem_word_num = input_elem_len >> 5;
    const uint output_group_word_num = output_elem_len * word_w;
    global uint* input_elem = input + i*input_elem_word_num + input_start;
    const global uint* output_group = output + widx + gidx*output_group_word_num + output_start;
    for (ibi = 0; ibi < bit_mapping_len; ibi++) {
        const uint outbit_val = (output_group[word_w*ibi] >> sbit) & 1;
        const uint inbit = bit_mapping[ibi];
        input_elem[inbit >> 5] |= outbit_val << (inbit & 31);
    }
}
"##;

#[dynamic]
static mut OUTPUT_TX_PROGRAMS: HashMap<HashableContext, Arc<Program>> = HashMap::new();

fn get_output_tx_program(context: Arc<Context>) -> Result<Arc<Program>, OpenCLBuildError> {
    let mut tx_programs = OUTPUT_TX_PROGRAMS.write();
    if let Some(p) = tx_programs.get(&HashableContext(context.clone())) {
        Ok(p.clone())
    } else {
        let p = Arc::new(Program::create_and_build_from_source(
            &context,
            OUTPUT_TRANSFORMER_SOURCE,
            "",
        )?);
        tx_programs.insert(HashableContext(context.clone()), p.clone());
        Ok(p)
    }
}

/// convert output data from circuit input form into output form.
pub struct OpenCLDataOutputTransformer {
    word_len: u32,
    input_elem_len: usize,
    output_elem_len: usize,
    bit_mapping: Buffer<u32>,
    bit_mapping_len: usize,
    context: Arc<Context>,
    cmd_queue: Arc<CommandQueue>,
    kernel: Kernel,
    group_len: usize,
}

impl OpenCLDataOutputTransformer {
    /// An output_elem_len - number of bits of really single input element.
    /// An input_elem_len - number of bits of really single output element.
    /// An bit_mapping - index is bit of really input's element,
    //  value is bit of reallyoutput's element.
    pub fn new(
        context: Arc<Context>,
        cmd_queue: Arc<CommandQueue>,
        word_len: u32,
        output_elem_len: usize,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<Self, OpenCLBuildError> {
        assert_eq!((word_len & 31), 0);
        assert_eq!((input_elem_len & 31), 0);
        assert!(input_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(output_elem_len >= bit_mapping.len());
        let device = Device::new(context.devices()[0]);
        let program = get_output_tx_program(context.clone())?;
        let mut buffer = unsafe {
            Buffer::<u32>::create(
                &context,
                CL_MEM_READ_WRITE,
                bit_mapping.len(),
                std::ptr::null_mut(),
            )
            .unwrap()
        };
        let bit_mapping_len = bit_mapping.len();
        unsafe {
            let mut bit_mapping = bit_mapping
                .iter()
                .map(|x| u32::try_from(*x).unwrap())
                .collect::<Vec<_>>();
            cmd_queue.enqueue_write_buffer(
                &mut buffer,
                CL_BLOCKING,
                0,
                &mut bit_mapping[..],
                &[],
            )?;
        }
        Ok(Self {
            word_len,
            input_elem_len: ((input_elem_len + 31) >> 5) << 5,
            output_elem_len,
            bit_mapping: buffer,
            bit_mapping_len,
            context,
            cmd_queue,
            group_len: usize::try_from(device.max_work_group_size().unwrap()).unwrap(),
            kernel: Kernel::create(&program, "xxx_gate_output_transform").unwrap(),
        })
    }

    pub unsafe fn context(&self) -> Arc<Context> {
        self.context.clone()
    }
    pub unsafe fn command_queue(&self) -> Arc<CommandQueue> {
        self.cmd_queue.clone()
    }
}

impl<'a> DataTransformer<'a, OpenCLDataReader<'a>, OpenCLDataWriter<'a>, OpenCLDataHolder>
    for OpenCLDataOutputTransformer
{
    type ErrorType = ClError;

    fn transform(&mut self, input: &OpenCLDataHolder) -> Result<OpenCLDataHolder, Self::ErrorType> {
        let mut output = OpenCLDataHolder::new(
            self.output_data_len(input.len()),
            &self.context,
            self.cmd_queue.clone(),
            CL_MEM_READ_WRITE,
        );
        self.transform_reuse(input, &mut output)?;
        Ok(output)
    }

    /// changed names of arguments:
    /// output - really input data, input - really output data
    fn transform_reuse(
        &mut self,
        output: &OpenCLDataHolder,
        input: &mut OpenCLDataHolder,
    ) -> Result<(), Self::ErrorType> {
        assert_eq!(
            input.len() % (((self.word_len as usize) * self.input_elem_len) >> 5),
            0
        );
        assert_eq!(
            output.len() % (((self.word_len as usize) * self.output_elem_len) >> 5),
            0
        );
        let input_elem_word_num = self.input_elem_len >> 5;
        let elem_num = input.len() / input_elem_word_num;
        let num = elem_num;
        let cl_num = cl_uint::try_from(num).unwrap();
        // println!("ddebug: {} {} {} {}",
        //          elem_num, num, self.word_len_fac1_pow, self.word_len_fac2);
        let cl_word_len = cl_uint::try_from(self.word_len).unwrap();
        let cl_output_start = cl_uint::try_from(output.range.start).unwrap();
        let cl_input_start = cl_uint::try_from(input.range.start).unwrap();
        let cl_input_elem_len = cl_uint::try_from(self.input_elem_len).unwrap();
        let cl_output_elem_len = cl_uint::try_from(self.output_elem_len).unwrap();
        let cl_bit_mapping_len = cl_uint::try_from(self.bit_mapping_len).unwrap();
        let input_len = input.len();
        unsafe {
            self.cmd_queue.enqueue_fill_buffer(
                &mut input.buffer,
                &[0u32],
                0,
                4 * input_len,
                &[],
            )?;
        }
        self.cmd_queue.finish().unwrap();
        unsafe {
            ExecuteKernel::new(&self.kernel)
                .set_arg(&cl_num)
                .set_arg(&cl_output_start)
                .set_arg(&cl_input_start)
                .set_arg(&cl_word_len)
                .set_arg(&cl_input_elem_len)
                .set_arg(&cl_output_elem_len)
                .set_arg(&cl_bit_mapping_len)
                .set_arg(&self.bit_mapping)
                .set_arg(&output.buffer)
                .set_arg(&input.buffer)
                .set_local_work_size(self.group_len)
                .set_global_work_size(
                    ((num + self.group_len - 1) / self.group_len) * self.group_len,
                )
                .enqueue_nd_range(&self.cmd_queue)?;
        }
        self.cmd_queue.finish()?;
        Ok(())
    }

    fn input_elem_len(&self) -> usize {
        self.output_elem_len
    }
    fn output_elem_len(&self) -> usize {
        self.input_elem_len
    }
}

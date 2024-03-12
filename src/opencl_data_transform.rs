use crate::opencl_build_exec::*;
use crate::*;
use static_init::dynamic;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::Device;
use opencl3::error_codes::ClError;
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_WRITE};
use opencl3::program::Program;
use opencl3::types::{cl_uint, cl_ulong, CL_BLOCKING};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

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
kernel void xxx_gate_input_transform(ulong n, ulong input_start, ulong output_start,
        uint word_len, uint input_elem_len, uint output_elem_len, uint bit_mapping_len,
        const global uint* bit_mapping, const global uint* input, global uint* output) {
    const size_t i = get_global_id(0);
    if (i >= n) return;
    const size_t li = get_local_id(0);
    uint ibi;
    local uint local_data[64];
    const uint word_w = word_len >> 5;
    const uint lvi = li >> 5;
    const size_t gidx = i / word_len;
    const uint widx = (i>>5) - gidx * word_w;
    const uint sbit = i & 31;
    const uint input_elem_word_num = input_elem_len >> 5;
    const uint output_group_word_num = output_elem_len * word_w;
    const global uint* input_elem = input + i*input_elem_word_num + input_start;
    global uint* output_group = output + widx + gidx*output_group_word_num + output_start;
    if (li < 64) {
        local_data[li] = 0;
    }
    barrier(CLK_LOCAL_MEM_FENCE);
    for (ibi = 0; ibi < bit_mapping_len; ibi++) {
        const uint inbit = bit_mapping[ibi];
        const uint inbit_val = (input_elem[inbit >> 5] >> (inbit & 31)) & 1;
        atomic_or(&local_data[lvi], (inbit_val << sbit));
        barrier(CLK_LOCAL_MEM_FENCE);
        if (sbit == 0) {
            output_group[word_w*ibi] = local_data[lvi];
            local_data[lvi] = 0;
        }
        barrier(CLK_LOCAL_MEM_FENCE);
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
    bit_mapping: Arc<Buffer<u32>>,
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
        let group_len = usize::try_from(device.max_work_group_size().unwrap()).unwrap();
        assert!(group_len >= 32);
        let group_len = std::cmp::min(32 * 64, (group_len >> 5) << 5);
        Ok(Self {
            word_len,
            input_elem_len: ((input_elem_len + 31) >> 5) << 5,
            output_elem_len,
            bit_mapping: Arc::new(buffer),
            bit_mapping_len,
            context,
            cmd_queue,
            group_len,
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
            self.context.clone(),
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
        let cl_num = cl_ulong::try_from(num).unwrap();
        // println!("ddebug: {} {} {} {}",
        //          elem_num, num, self.word_len_fac1_pow, self.word_len_fac2);
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
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
                .set_arg(self.bit_mapping.deref())
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

impl Clone for OpenCLDataInputTransformer {
    fn clone(&self) -> Self {
        let program = get_input_tx_program(self.context.clone()).unwrap();
        Self {
            word_len: self.word_len,
            input_elem_len: self.input_elem_len,
            output_elem_len: self.output_elem_len,
            bit_mapping: self.bit_mapping.clone(),
            bit_mapping_len: self.bit_mapping_len,
            context: self.context.clone(),
            cmd_queue: self.cmd_queue.clone(),
            kernel: Kernel::create(&program, "xxx_gate_input_transform").unwrap(),
            group_len: self.group_len,
        }
    }
}

const OUTPUT_TRANSFORMER_SOURCE: &'static str = r##"
kernel void xxx_gate_output_transform(ulong n, ulong output_start, ulong input_start,
        uint word_len, uint input_elem_len, uint output_elem_len, uint bit_mapping_len,
        const global uint* bit_mapping, const global uint* output, global uint* input) {
    const size_t i = get_global_id(0);
    if (i >= n) return;
    uint ibi;
    const uint word_w = word_len >> 5;
    const size_t gidx = i / word_len;
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
    bit_mapping: Arc<Buffer<u32>>,
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
            bit_mapping: Arc::new(buffer),
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
            self.context.clone(),
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
        let cl_num = cl_ulong::try_from(num).unwrap();
        // println!("ddebug: {} {} {} {}",
        //          elem_num, num, self.word_len_fac1_pow, self.word_len_fac2);
        let cl_output_start = cl_ulong::try_from(output.range.start).unwrap();
        let cl_input_start = cl_ulong::try_from(input.range.start).unwrap();
        let cl_word_len = cl_uint::try_from(self.word_len).unwrap();
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
                .set_arg(self.bit_mapping.deref())
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

impl Clone for OpenCLDataOutputTransformer {
    fn clone(&self) -> Self {
        let program = get_output_tx_program(self.context.clone()).unwrap();
        Self {
            word_len: self.word_len,
            input_elem_len: self.input_elem_len,
            output_elem_len: self.output_elem_len,
            bit_mapping: self.bit_mapping.clone(),
            bit_mapping_len: self.bit_mapping_len,
            context: self.context.clone(),
            cmd_queue: self.cmd_queue.clone(),
            kernel: Kernel::create(&program, "xxx_gate_output_transform").unwrap(),
            group_len: self.group_len,
        }
    }
}

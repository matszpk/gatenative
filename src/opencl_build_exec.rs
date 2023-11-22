use crate::clang_writer::*;
use crate::gencode::generate_code;
use crate::{Builder, CodeWriter, DataHolder, Executor};
use gatesim::*;

use opencl3::command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_READ_WRITE, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{cl_event, cl_uint, CL_BLOCKING, CL_NON_BLOCKING};

use libloading::{Library, Symbol};
use static_init::dynamic;
use thiserror::Error;


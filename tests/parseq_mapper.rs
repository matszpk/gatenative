use gatenative::cpu_build_exec::*;
use gatenative::opencl_build_exec::*;
use gatenative::parseq_mapper::*;
use gatenative::*;
use gatesim::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::types::CL_BLOCKING;

#[test]
fn test_parseq_mapper_data_holder() {
    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );
    let par_builder = CPUBuilder::new(None);
    let seq_builders = get_all_devices(CL_DEVICE_TYPE_GPU)
        .unwrap()
        .into_iter()
        .map(|dev_id| {
            let device = Device::new(dev_id);
            OpenCLBuilder::new(&device, None)
        })
        .collect::<Vec<_>>();

    let parseq_builder = ParSeqMapperBuilder::new(par_builder, seq_builders);
}

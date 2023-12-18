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
    let seq_num = seq_builders.len();

    let mut builder = ParSeqMapperBuilder::new(par_builder, seq_builders);
    let circuit = Circuit::new(4, [], [(0, false), (1, false), (2, false), (3, false)]).unwrap();
    builder.add("mul2x2", circuit.clone(), &[0, 1]);
    let mut execs = builder.build().unwrap();
    let mut data = execs[0].new_data(96);
    let selections = std::iter::once(ParSeqSelection::Par)
        .chain((0..seq_num).map(|i| ParSeqSelection::Seq(i)))
        .collect::<Vec<_>>();
    println!("selections: {:?}", selections);
    data.process_mut(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let mut wr = sel_data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 1100 + 0).unwrap();
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let mut wr = sel_data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 1100 + 1 + si).unwrap();
            }
        }
    });
    // check
    data.process(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let rd = sel_data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(*x, u32::try_from(i * 1100 + 0).unwrap(), "{} {}", 0, i);
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let rd = sel_data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(
                    *x,
                    u32::try_from(i * 1100 + 1 + si).unwrap(),
                    "{} {}",
                    si,
                    i
                );
            }
        }
    });
}

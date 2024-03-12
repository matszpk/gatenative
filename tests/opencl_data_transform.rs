use gatenative::opencl_build_exec::*;
use gatenative::opencl_data_transform::*;
use gatenative::*;
use gatesim::*;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::memory::CL_MEM_READ_WRITE;

use std::sync::Arc;

#[test]
fn test_opencl_input_output_data_transformer() {
    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );
    let context = Arc::new(Context::from_device(&device).unwrap());
    #[allow(deprecated)]
    let cmd_queue =
        unsafe { Arc::new(CommandQueue::create(&context, context.devices()[0], 0).unwrap()) };

    for shift in [0, 123] {
        for word_len_fac in [4, 5, 6] {
            let word_len = 32 * word_len_fac; // 1 32-bit word
            let input_elem_len = 96;
            let output_elem_len = 77;
            let input_elem_word_num = input_elem_len * (word_len >> 5);
            let output_elem_word_num = output_elem_len * (word_len >> 5);
            let group_num = 17541;
            let mut input = OpenCLDataHolder::new(
                shift + input_elem_word_num * group_num,
                context.clone(),
                cmd_queue.clone(),
                CL_MEM_READ_WRITE,
            );
            if shift != 0 {
                input.set_range(shift..shift + input_elem_word_num * group_num);
            }
            let mut input_2 = OpenCLDataHolder::new(
                shift + input_elem_word_num * group_num,
                context.clone(),
                cmd_queue.clone(),
                CL_MEM_READ_WRITE,
            );
            if shift != 0 {
                input_2.set_range(shift..shift + input_elem_word_num * group_num);
            }
            let mut output = OpenCLDataHolder::new(
                shift + output_elem_word_num * group_num,
                context.clone(),
                cmd_queue.clone(),
                CL_MEM_READ_WRITE,
            );
            if shift != 0 {
                output.set_range(shift..shift + output_elem_word_num * group_num);
            }
            let bit_mapping = (0..27)
                .chain(32..32 + 22)
                .chain(64..64 + 28)
                .collect::<Vec<_>>();
            {
                let mut input = input.get_mut();
                let input = input.get_mut();
                for (i, v) in input.iter_mut().enumerate() {
                    *v = (100 + i + 51 * i * i) as u32;
                }
            }
            let mut expected = vec![0u32; output.len()];
            let wl = word_len >> 5;
            {
                let input = input.get();
                let input = input.get();
                for i in 0..32 * wl * group_num {
                    let input_slice = &input[i * 3..(i + 1) * 3];
                    let idx = (i >> 5) as usize;
                    let gidx = idx / wl;
                    let widx = idx % wl;
                    let idx = gidx * wl * output_elem_len + widx;
                    let sbit = i & 31;
                    //println!("X{}: {} {} {} {}", i, idx, gidx, widx, sbit);
                    for bit in 0..output_elem_len {
                        let xbit = bit_mapping[bit];
                        expected[idx + bit * wl] |=
                            u32::from((input_slice[xbit >> 5] >> (xbit & 31)) & 1) << sbit;
                    }
                }
            }
            let mut transformer = OpenCLDataInputTransformer::new(
                context.clone(),
                cmd_queue.clone(),
                u32::try_from(word_len).unwrap(),
                input_elem_len,
                output_elem_len,
                &bit_mapping,
            )
            .unwrap();
            use std::time::SystemTime;
            let start = SystemTime::now();
            transformer.transform_reuse(&input, &mut output).unwrap();
            let elapsed = start.elapsed().unwrap();
            println!("Time: {} s", elapsed.as_secs_f64());
            {
                let output = output.get();
                let output = output.get();
                for i in 0..output.len() {
                    assert_eq!(
                        expected[i], output[i],
                        "{} shift={}: {}",
                        word_len_fac, shift, i
                    );
                }
            }

            let mut transformer = OpenCLDataOutputTransformer::new(
                context.clone(),
                cmd_queue.clone(),
                u32::try_from(word_len).unwrap(),
                output_elem_len,
                input_elem_len,
                &bit_mapping,
            )
            .unwrap();
            let start = SystemTime::now();
            transformer.transform_reuse(&output, &mut input_2).unwrap();
            let elapsed = start.elapsed().unwrap();
            println!("Time: {} s", elapsed.as_secs_f64());
            {
                let input = input.release();
                let input_2 = input_2.release();
                for i in 0..input.len() {
                    let expected = match i % 3 {
                        0 => input[i] & ((1 << 27) - 1),
                        1 => input[i] & ((1 << 22) - 1),
                        2 => input[i] & ((1 << 28) - 1),
                        _ => {
                            panic!("Unexpected!");
                        }
                    };
                    assert_eq!(
                        expected, input_2[i],
                        "{} shift={}: {}",
                        word_len_fac, shift, i
                    );
                }
            }
        }
    }
}

#[test]
fn test_opencl_data_transforms() {
    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );
    let mut builder = OpenCLBuilder::new(&device, None);
    let circuit = Circuit::new(
        40,
        (0..20).map(|i| Gate::new_xor(i, 20 + i)),
        (0..20).map(|i| (40 + i, false)),
    )
    .unwrap();
    builder.add("xor", circuit.clone(), None, None, None);
    let mut execs = builder.build().unwrap();
    let input = execs[0].new_data_from_vec(
        (0..1024u32 * 128u32)
            .map(|i| 124 * i + 21 * i * i + 1342)
            .collect::<Vec<_>>(),
    );
    let mut it = execs[0]
        .input_transformer(64, &(0..20).chain(32..32 + 20).collect::<Vec<_>>())
        .unwrap();
    let mut ot = execs[0]
        .output_transformer(32, &(0..20).collect::<Vec<_>>())
        .unwrap();
    let mut input_circ =
        execs[0].new_data((input.len() * it.output_elem_len()) / it.input_elem_len());
    it.transform_reuse(&input, &mut input_circ).unwrap();
    let output_circ = execs[0].execute(&input_circ, 0).unwrap();
    let mut output =
        execs[0].new_data((output_circ.len() * ot.output_elem_len()) / ot.input_elem_len());
    ot.transform_reuse(&output_circ, &mut output).unwrap();
    {
        let input = input.get();
        let input = input.get();
        let output = output.get();
        let output = output.get();
        for (i, v) in output.iter().enumerate() {
            assert_eq!(
                (input[2 * i] ^ input[2 * i + 1]) & ((1 << 20) - 1),
                *v,
                "{}",
                i
            );
        }
    }
    // use helpers
    let input_circ = it.transform(&input).unwrap();
    let output_circ = execs[0].execute(&input_circ, 0).unwrap();
    let output = ot.transform(&output_circ).unwrap();
    {
        let input = input.release();
        let output = output.release();
        for (i, v) in output.iter().enumerate() {
            assert_eq!(
                (input[2 * i] ^ input[2 * i + 1]) & ((1 << 20) - 1),
                *v,
                "{}",
                i
            );
        }
    }
}

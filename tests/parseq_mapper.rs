use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::opencl_build_exec::*;
use gatenative::parseq_mapper::*;
use gatenative::*;
use gatesim::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};

use std::sync::{
    atomic::{self, AtomicUsize},
    Arc,
};
use std::thread;
use std::time::Duration;

#[test]
fn test_parseq_mapper_all_data_holder() {
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
    assert_eq!(data.len(), 96);

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

    for sel in &selections {
        data.process_single_mut(*sel, |obj| match obj {
            ParSeqObject::Par(sel_data) => {
                sel_data.process_mut(|data| {
                    for (i, x) in data.iter_mut().enumerate() {
                        *x = u32::try_from(i * 1700 + 0).unwrap();
                    }
                });
            }
            ParSeqObject::Seq((si, sel_data)) => {
                sel_data.process_mut(|data| {
                    for (i, x) in data.iter_mut().enumerate() {
                        *x = u32::try_from(i * 1700 + 1 + si).unwrap();
                    }
                });
            }
        });
    }
    // check
    for sel in &selections {
        data.process_single(*sel, |obj| match obj {
            ParSeqObject::Par(sel_data) => {
                sel_data.process(|data| {
                    for (i, x) in data.iter().enumerate() {
                        assert_eq!(*x, u32::try_from(i * 1700 + 0).unwrap(), "{} {}", 0, i);
                    }
                });
            }
            ParSeqObject::Seq((si, sel_data)) => {
                sel_data.process(|data| {
                    for (i, x) in data.iter().enumerate() {
                        assert_eq!(
                            *x,
                            u32::try_from(i * 1700 + 1 + si).unwrap(),
                            "{} {}",
                            si,
                            i
                        );
                    }
                });
            }
        });
    }

    data.process_mut(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            sel_data.process_mut(|data| {
                for (i, x) in data.iter_mut().enumerate() {
                    *x = u32::try_from(i * 2300 + 0).unwrap();
                }
            });
        }
        ParSeqObject::Seq((si, sel_data)) => {
            sel_data.process_mut(|data| {
                for (i, x) in data.iter_mut().enumerate() {
                    *x = u32::try_from(i * 2300 + 1 + si).unwrap();
                }
            });
        }
    });
    // check
    data.process(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            sel_data.process(|data| {
                for (i, x) in data.iter().enumerate() {
                    assert_eq!(*x, u32::try_from(i * 2300 + 0).unwrap(), "{} {}", 0, i);
                }
            });
        }
        ParSeqObject::Seq((si, sel_data)) => {
            sel_data.process(|data| {
                for (i, x) in data.iter().enumerate() {
                    assert_eq!(
                        *x,
                        u32::try_from(i * 2300 + 1 + si).unwrap(),
                        "{} {}",
                        si,
                        i
                    );
                }
            });
        }
    });

    // set range
    data.set_range(32..64);
    assert_eq!(data.len(), 32);
    data.process_mut(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let mut wr = sel_data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 9200 + 0).unwrap();
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let mut wr = sel_data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 9200 + 1 + si).unwrap();
            }
        }
    });
    data.set_range_from(0..);
    assert_eq!(data.len(), 96);
    // check
    data.process(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let rd = sel_data.get();
            for (i, x) in rd.get().iter().enumerate() {
                let (diff, factor) = if (32..64).contains(&i) {
                    (32, 9200)
                } else {
                    (0, 2300)
                };
                assert_eq!(
                    *x,
                    u32::try_from((i - diff) * factor + 0).unwrap(),
                    "{} {}",
                    0,
                    i
                );
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let rd = sel_data.get();
            for (i, x) in rd.get().iter().enumerate() {
                let (diff, factor) = if (32..64).contains(&i) {
                    (32, 9200)
                } else {
                    (0, 2300)
                };
                assert_eq!(
                    *x,
                    u32::try_from((i - diff) * factor + 1 + si).unwrap(),
                    "{} {}",
                    si,
                    i
                );
            }
        }
    });

    // new data from vec
    let data = execs[0].new_data_from_vec(|sel| match sel {
        ParSeqSelection::Par => (0..160u32)
            .map(|x| 24 * x * x + 561 * x + 1223)
            .collect::<Vec<_>>(),
        ParSeqSelection::Seq(i) => (0..160u32)
            .map(|x| 42 * (u32::try_from(i).unwrap() + 1) * x * x + 1561 * x + 7521)
            .collect::<Vec<_>>(),
    });
    // check
    assert_eq!(data.len(), 160);
    data.process(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let rd = sel_data.get();
            for (i, v) in rd.get().iter().enumerate() {
                let x = u32::try_from(i).unwrap();
                assert_eq!(*v, 24 * x * x + 561 * x + 1223, "{} {}", 0, x);
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let rd = sel_data.get();
            for (i, v) in rd.get().iter().enumerate() {
                let x = u32::try_from(i).unwrap();
                assert_eq!(
                    *v,
                    42 * (u32::try_from(si).unwrap() + 1) * x * x + 1561 * x + 7521,
                    "{} {}",
                    si,
                    i
                );
            }
        }
    });

    // new data from slice
    let gen_data = (0..selections.len())
        .map(|i| {
            (0..192u32)
                .map(|x| 14 * (u32::try_from(i).unwrap() + 1) * x * x + 7643 * x + 55515)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let data = execs[0].new_data_from_slice(|sel| match sel {
        ParSeqSelection::Par => &gen_data[0][..],
        ParSeqSelection::Seq(i) => &gen_data[i + 1][..],
    });
    // check
    assert_eq!(data.len(), 192);
    data.process(|obj| match obj {
        ParSeqObject::Par(sel_data) => {
            let rd = sel_data.get();
            for (i, v) in rd.get().iter().enumerate() {
                let x = u32::try_from(i).unwrap();
                assert_eq!(*v, 14 * 1 * x * x + 7643 * x + 55515, "{} {}", 0, x);
            }
        }
        ParSeqObject::Seq((si, sel_data)) => {
            let rd = sel_data.get();
            for (i, v) in rd.get().iter().enumerate() {
                let x = u32::try_from(i).unwrap();
                assert_eq!(
                    *v,
                    14 * (u32::try_from(si).unwrap() + 2) * x * x + 7643 * x + 55515,
                    "{} {}",
                    si,
                    i
                );
            }
        }
    });
}

#[test]
fn test_parseq_mapper_builder_and_exec() {
    let par_builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
        CPUExtension::NoExtension,
        &CLANG_WRITER_U64,
        None,
    );
    let seq_builders = get_all_devices(CL_DEVICE_TYPE_GPU)
        .unwrap()
        .into_iter()
        .map(|dev_id| {
            let device = Device::new(dev_id);
            OpenCLBuilder::new(&device, None)
        })
        .collect::<Vec<_>>();
    let seq_num = seq_builders.len();
    let selections = std::iter::once(ParSeqSelection::Par)
        .chain((0..seq_num).map(|i| ParSeqSelection::Seq(i)))
        .collect::<Vec<_>>();

    let arg_input_indices = [0, 3, 5, 8, 10];
    let rest_input_indices = {
        let mut rest_input_indices = vec![];
        let mut j = 0;
        for i in 0..12 {
            if j < arg_input_indices.len() && arg_input_indices[j] == i {
                j += 1;
                continue;
            } else {
                rest_input_indices.push(i);
            }
        }
        rest_input_indices
    };

    let mut builder = ParSeqMapperBuilder::new(par_builder, seq_builders);
    let word_lens = selections
        .iter()
        .map(|sel| builder.word_len(*sel))
        .collect::<Vec<_>>();
    let circuit = Circuit::new(
        12,
        [
            Gate::new_and(0, 1),   // 12
            Gate::new_and(2, 5),   // 13
            Gate::new_nor(12, 13), // 14
            Gate::new_and(3, 6),   // 15
            Gate::new_and(7, 9),   // 16
            Gate::new_nor(15, 16), // 17
            Gate::new_and(4, 8),   // 18
            Gate::new_and(10, 11), // 19
            Gate::new_nor(18, 19), // 20
            Gate::new_and(5, 7),   // 21
            Gate::new_and(9, 11),  // 22
            Gate::new_nor(21, 22), // 23
            Gate::new_and(2, 6),   // 24
            Gate::new_and(8, 9),   // 25
            Gate::new_nor(24, 25), // 26
            Gate::new_and(0, 4),   // 27
            Gate::new_and(7, 10),  // 28
            Gate::new_nor(27, 28), // 29
            Gate::new_and(1, 6),   // 30
            Gate::new_and(7, 8),   // 31
            Gate::new_nor(30, 31), // 32
            Gate::new_and(1, 2),   // 33
            Gate::new_and(4, 9),   // 34
            Gate::new_nor(33, 34), // 35
            Gate::new_xor(14, 17), // 36
            Gate::new_xor(20, 23), // 37
            Gate::new_xor(26, 29), // 38
            Gate::new_xor(32, 35), // 39
            Gate::new_xor(36, 37), // 40
            Gate::new_xor(38, 39), // 41
            Gate::new_xor(40, 41), // 42
        ],
        [(42, false)],
    )
    .unwrap();
    builder.add("xcircuit", circuit.clone(), &arg_input_indices[..]);
    let mut execs = builder.build().unwrap();
    let xcircuit_inputs = (0..selections.len())
        .map(|seli| {
            let word_len = (word_lens[seli] >> 5) as usize;
            // number of chunks
            let xcircuit_data_num = (((128 >> 5) + word_len - 1) / word_len) * word_len;
            let rest_num = rest_input_indices.len();
            let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
            // prepare input for executor
            for i in 0..128 {
                let idx = (i >> 5) / word_len;
                let widx = (i >> 5) % word_len;
                let bit = i & 31;
                for j in 0..rest_num {
                    xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                        ((u32::try_from(i).unwrap() >> j) & 1) << bit;
                }
            }
            println!("seli: {}, len={}", seli, xcircuit_input.len());
            xcircuit_input
        })
        .collect::<Vec<_>>();
    // set input
    let input = execs[0].new_data_from_slice(|sel| {
        let seli = match sel {
            ParSeqSelection::Par => 0,
            ParSeqSelection::Seq(i) => i + 1,
        };
        &xcircuit_inputs[seli]
    });

    let call_count = Arc::new(AtomicUsize::new(0));
    assert!(execs[0]
        .execute(
            &input,
            true,
            |sel, input, result_out, arg_input| {
                call_count.fetch_add(1, atomic::Ordering::SeqCst);
                thread::sleep(Duration::from_millis(100));
                let seli = match sel {
                    ParSeqSelection::Par => 0,
                    ParSeqSelection::Seq(i) => i + 1,
                };
                if !input.process_single(sel, |d| match d {
                    ParSeqObject::Par(d) => {
                        let res_input = d.get();
                        let res_input = res_input.get();
                        res_input == &xcircuit_inputs[seli][..]
                    }
                    ParSeqObject::Seq((_, d)) => {
                        let res_input = d.get();
                        let res_input = res_input.get();
                        res_input == &xcircuit_inputs[seli][..]
                    }
                }) {
                    return false;
                }
                let mut input = vec![false; 12];
                let word_len = (word_lens[seli] >> 5) as usize;
                // number of chunks
                let xcircuit_data_num = (((128 >> 5) + word_len - 1) / word_len) * word_len;

                let mut xcircuit_out = vec![0u32; xcircuit_data_num];
                // fill inputs by arg_inputs
                for (i, v) in arg_input_indices.iter().enumerate() {
                    input[*v] = ((arg_input >> i) & 1) != 0;
                }
                // prepare expected output
                for rest in 0..128 {
                    // fill input by rest of bits of input
                    for (i, v) in rest_input_indices.iter().enumerate() {
                        input[*v] = ((rest >> i) & 1) != 0;
                    }
                    let value = circuit.eval(input.clone())[0];
                    let idx = (rest >> 5) / word_len;
                    let widx = (rest >> 5) % word_len;
                    let bit = rest & 31;
                    xcircuit_out[word_len * idx + widx] |= (value as u32) << bit;
                }
                println!("Called {} {:?}", arg_input, sel);
                // execute circuit
                match sel {
                    ParSeqSelection::Par => {
                        let result_out = result_out.par().unwrap();
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    }
                    ParSeqSelection::Seq(_) => {
                        let result_out = result_out.seq().unwrap();
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    }
                }
            },
            |out1, out2| out1 && out2
        )
        .unwrap());
    assert_eq!(32, call_count.load(atomic::Ordering::SeqCst));
}

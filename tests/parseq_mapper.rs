use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::opencl_build_exec::*;
use gatenative::parseq_mapper::*;
use gatenative::*;
use gatesim::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};

use std::str::FromStr;
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
    for do_use_seq in [false, true] {
        let par_builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            &CLANG_WRITER_U64,
            None,
        );
        let seq_builders = if do_use_seq {
            get_all_devices(CL_DEVICE_TYPE_GPU)
                .unwrap()
                .into_iter()
                .map(|dev_id| {
                    let device = Device::new(dev_id);
                    OpenCLBuilder::new(&device, None)
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
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
        let circuit2 = Circuit::new(
            12,
            [
                Gate::new_xor(0, 1),     // 12
                Gate::new_and(2, 5),     // 13
                Gate::new_nor(12, 13),   // 14
                Gate::new_and(3, 6),     // 15
                Gate::new_and(7, 9),     // 16
                Gate::new_nor(15, 16),   // 17
                Gate::new_and(4, 8),     // 18
                Gate::new_nimpl(11, 10), // 19
                Gate::new_nor(18, 19),   // 20
                Gate::new_nor(5, 8),     // 21
                Gate::new_nimpl(9, 11),  // 22
                Gate::new_nor(21, 22),   // 23
                Gate::new_xor(2, 6),     // 24
                Gate::new_and(8, 9),     // 25
                Gate::new_nor(24, 25),   // 26
                Gate::new_and(1, 4),     // 27
                Gate::new_nimpl(7, 10),  // 28
                Gate::new_nor(27, 28),   // 29
                Gate::new_and(2, 6),     // 30
                Gate::new_nimpl(7, 8),   // 31
                Gate::new_nor(30, 31),   // 32
                Gate::new_nor(1, 2),     // 33
                Gate::new_and(4, 9),     // 34
                Gate::new_nor(33, 34),   // 35
                Gate::new_xor(14, 17),   // 36
                Gate::new_xor(20, 23),   // 37
                Gate::new_xor(26, 29),   // 38
                Gate::new_xor(32, 35),   // 39
                Gate::new_xor(36, 37),   // 40
                Gate::new_xor(38, 39),   // 41
                Gate::new_xor(40, 41),   // 42
            ],
            [(42, false)],
        )
        .unwrap();

        builder.add("xcircuit", circuit.clone(), &arg_input_indices[..]);
        builder.add("xcircuit2", circuit2.clone(), &arg_input_indices[..]);

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
                    // get result output and compare
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
                |out1, out2| out1 && out2,
                |_| false
            )
            .unwrap());
        assert_eq!(32, call_count.load(atomic::Ordering::SeqCst));

        let call_count = Arc::new(AtomicUsize::new(0));
        assert!(execs[1]
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
                        let value = circuit2.eval(input.clone())[0];
                        let idx = (rest >> 5) / word_len;
                        let widx = (rest >> 5) % word_len;
                        let bit = rest & 31;
                        xcircuit_out[word_len * idx + widx] |= (value as u32) << bit;
                    }
                    // get result output and compare
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
                |out1, out2| out1 && out2,
                |_| false
            )
            .unwrap());
        assert_eq!(32, call_count.load(atomic::Ordering::SeqCst));

        let call_count = Arc::new(AtomicUsize::new(0));
        assert!(execs[0]
            .execute_direct(
                &input,
                true,
                |sel, input, result_out, arg_input| {
                    call_count.fetch_add(1, atomic::Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(100));
                    let seli = match sel {
                        ParSeqSelection::Par => 0,
                        ParSeqSelection::Seq(i) => i + 1,
                    };
                    if input != &xcircuit_inputs[seli][..] {
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
                    // get result output and compare
                    xcircuit_out
                        .into_iter()
                        .enumerate()
                        .all(|(i, exp)| result_out[i] == exp)
                },
                |out1, out2| out1 && out2,
                |_| false
            )
            .unwrap());
        assert_eq!(32, call_count.load(atomic::Ordering::SeqCst));
    }
}

const COMB_CIRCUIT_CODE: &str = concat!(
    "{0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 ",
    "xor(0,7) and(16,5):0 xor(1,8) and(0,7) xor(18,19) and(20,6):1 xor(2,9) ",
    "and(18,19) and(1,8) nor(23,24) xor(22,25) nimpl(7,26):2 xor(3,10) nimpl(22,25) ",
    "and(2,9) nor(29,30) xor(28,31) nimpl(8,32):3 xor(4,0) xor(34,11) nimpl(28,31) ",
    "and(3,10) nor(36,37) xor(35,38) xor(39,5) nimpl(9,40):4 xor(5,1) and(4,0) ",
    "xor(42,43) xor(44,12) nimpl(35,38) and(34,11) nor(46,47) xor(45,48) xor(49,6) ",
    "nimpl(5,39) xor(50,51) nimpl(10,52):5 xor(6,2) and(42,43) and(5,1) nor(55,56) ",
    "xor(54,57) xor(58,13) nimpl(45,48) and(44,12) nor(60,61) xor(59,62) xor(63,7) ",
    "nimpl(51,50) nimpl(6,49) nor(65,66) xor(64,67) nimpl(11,68):6 xor(7,3) ",
    "nimpl(54,57) and(6,2) nor(71,72) xor(70,73) xor(74,14) nor(59,62) nimpl(13,58) ",
    "nor(76,77) xor(75,78) xor(79,8) nimpl(64,67) and(63,7) nor(81,82) xor(80,83) ",
    "nimpl(12,84):7 xor(8,4) nimpl(70,73) and(7,3) nor(87,88) xor(86,89) xor(90,15) ",
    "nor(75,78) nimpl(14,74) nor(92,93) xor(91,94) xor(95,9) nimpl(80,83) and(79,8) ",
    "nor(97,98) xor(96,99) nimpl(13,100):8 xor(9,5) nimpl(86,89) and(8,4) nor(103,104) ",
    "xor(102,105) xor(106,0) nor(91,94) nimpl(15,90) nor(108,109) xor(107,110) ",
    "xor(111,10) nimpl(96,99) and(95,9) nor(113,114) xor(112,115) nimpl(14,116):9 ",
    "xor(10,6) nimpl(102,105) and(9,5) nor(119,120) xor(118,121) xor(122,1) ",
    "nor(107,110) nimpl(0,106) nor(124,125) xor(123,126) xor(127,11) nimpl(112,115) ",
    "and(111,10) nor(129,130) xor(128,131) nimpl(15,132):10 xor(11,7) nimpl(118,121) ",
    "and(10,6) nor(135,136) xor(134,137) xor(138,2) nor(123,126) nimpl(1,122) ",
    "nor(140,141) xor(139,142) xor(143,12) nimpl(128,131) and(127,11) nor(145,146) ",
    "xor(144,147) nimpl(0,148):11}(16)"
);

const COMB_CIRCUIT_AGGR_CODE: &str = r##"{
    unsigned int i;
    uint32_t out[(TYPE_LEN >> 5)*12];
    uint32_t* output_u32 = (uint32_t*)output;
    GET_U32_ALL(out + 0*(TYPE_LEN>>5), o0);
    GET_U32_ALL(out + 1*(TYPE_LEN>>5), o1);
    GET_U32_ALL(out + 2*(TYPE_LEN>>5), o2);
    GET_U32_ALL(out + 3*(TYPE_LEN>>5), o3);
    GET_U32_ALL(out + 4*(TYPE_LEN>>5), o4);
    GET_U32_ALL(out + 5*(TYPE_LEN>>5), o5);
    GET_U32_ALL(out + 6*(TYPE_LEN>>5), o6);
    GET_U32_ALL(out + 7*(TYPE_LEN>>5), o7);
    GET_U32_ALL(out + 8*(TYPE_LEN>>5), o8);
    GET_U32_ALL(out + 9*(TYPE_LEN>>5), o9);
    GET_U32_ALL(out + 10*(TYPE_LEN>>5), o10);
    GET_U32_ALL(out + 11*(TYPE_LEN>>5), o11);
    for (i = 0; i < TYPE_LEN; i++) {
        uint32_t out_idx = ((out[(i>>5) + (TYPE_LEN>>5)*0] >> (i&31)) & 1) |
            (((out[(i>>5) + (TYPE_LEN>>5)*1] >> (i&31)) & 1) << 1) |
            (((out[(i>>5) + (TYPE_LEN>>5)*2] >> (i&31)) & 1) << 2) |
            (((out[(i>>5) + (TYPE_LEN>>5)*3] >> (i&31)) & 1) << 3) |
            (((out[(i>>5) + (TYPE_LEN>>5)*4] >> (i&31)) & 1) << 4) |
            (((out[(i>>5) + (TYPE_LEN>>5)*5] >> (i&31)) & 1) << 5) |
            (((out[(i>>5) + (TYPE_LEN>>5)*6] >> (i&31)) & 1) << 6) |
            (((out[(i>>5) + (TYPE_LEN>>5)*7] >> (i&31)) & 1) << 7) |
            (((out[(i>>5) + (TYPE_LEN>>5)*8] >> (i&31)) & 1) << 8) |
            (((out[(i>>5) + (TYPE_LEN>>5)*9] >> (i&31)) & 1) << 9) |
            (((out[(i>>5) + (TYPE_LEN>>5)*10] >> (i&31)) & 1) << 10) |
            (((out[(i>>5) + (TYPE_LEN>>5)*11] >> (i&31)) & 1) << 11);
        __sync_fetch_and_or(&output_u32[out_idx >> 5], (1 << (out_idx & 31)));
    }
}"##;

const COMB_CIRCUIT_AGGR_CODE_OPENCL: &str = r##"{
    unsigned int i;
    uint out[(TYPE_LEN >> 5)*12];
    global uint* output_u32 = (global uint*)output;
    GET_U32_ALL(out + 0*(TYPE_LEN>>5), o0);
    GET_U32_ALL(out + 1*(TYPE_LEN>>5), o1);
    GET_U32_ALL(out + 2*(TYPE_LEN>>5), o2);
    GET_U32_ALL(out + 3*(TYPE_LEN>>5), o3);
    GET_U32_ALL(out + 4*(TYPE_LEN>>5), o4);
    GET_U32_ALL(out + 5*(TYPE_LEN>>5), o5);
    GET_U32_ALL(out + 6*(TYPE_LEN>>5), o6);
    GET_U32_ALL(out + 7*(TYPE_LEN>>5), o7);
    GET_U32_ALL(out + 8*(TYPE_LEN>>5), o8);
    GET_U32_ALL(out + 9*(TYPE_LEN>>5), o9);
    GET_U32_ALL(out + 10*(TYPE_LEN>>5), o10);
    GET_U32_ALL(out + 11*(TYPE_LEN>>5), o11);
    for (i = 0; i < TYPE_LEN; i++) {
        uint out_idx = ((out[(i>>5) + (TYPE_LEN>>5)*0] >> (i&31)) & 1) |
            (((out[(i>>5) + (TYPE_LEN>>5)*1] >> (i&31)) & 1) << 1) |
            (((out[(i>>5) + (TYPE_LEN>>5)*2] >> (i&31)) & 1) << 2) |
            (((out[(i>>5) + (TYPE_LEN>>5)*3] >> (i&31)) & 1) << 3) |
            (((out[(i>>5) + (TYPE_LEN>>5)*4] >> (i&31)) & 1) << 4) |
            (((out[(i>>5) + (TYPE_LEN>>5)*5] >> (i&31)) & 1) << 5) |
            (((out[(i>>5) + (TYPE_LEN>>5)*6] >> (i&31)) & 1) << 6) |
            (((out[(i>>5) + (TYPE_LEN>>5)*7] >> (i&31)) & 1) << 7) |
            (((out[(i>>5) + (TYPE_LEN>>5)*8] >> (i&31)) & 1) << 8) |
            (((out[(i>>5) + (TYPE_LEN>>5)*9] >> (i&31)) & 1) << 9) |
            (((out[(i>>5) + (TYPE_LEN>>5)*10] >> (i&31)) & 1) << 10) |
            (((out[(i>>5) + (TYPE_LEN>>5)*11] >> (i&31)) & 1) << 11);
        atomic_or(&output_u32[out_idx >> 5], (1 << (out_idx & 31)));
    }
}"##;

const COMB_CIRCUIT_EXPECTED: [u32; 128] = [
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 939491327,
    4294967295, 4294967295, 3221225471, 1543503871, 2147483647, 2004844543, 3086483455, 805142527,
    4294967295, 4294967295, 4294967295, 4160749567, 4294967295, 3489658879, 2147481599, 536565759,
    4294967295, 3087007743, 3082813439, 2077734911, 503316479, 2809659391, 2503417855, 19096951,
    4294967295, 2147483647, 2013265919, 4160749567, 4294967295, 931102719, 1610612735, 85292543,
    4294967295, 2147483647, 1438646271, 1312536575, 4290772991, 291452927, 565411711, 285546271,
    4294967295, 935288831, 930611199, 1744295935, 662700031, 94330879, 1732196351, 289738727,
    2013265919, 1970789887, 1466940927, 118362399, 394256255, 10175999, 69310335, 68157743,
    1600085855, 1600085855, 1600085855, 358571871, 1600085855, 387931999, 1600085855, 375349087,
    1600085855, 1465868127, 1600085855, 522147679, 1465868127, 392109919, 257908575, 320019295,
    1600085855, 526344031, 1600085855, 387931999, 1465868127, 354359135, 1196623711, 18027615,
    1600085855, 358571359, 425156447, 421024607, 1129799519, 270472479, 17651039, 1179993,
    1600085855, 526344031, 1461673823, 392124255, 1600085855, 291446623, 325017439, 88301395,
    1600085855, 526341983, 1432311639, 17506127, 454778719, 18436959, 23007063, 65558, 1600085855,
    118955871, 1398759263, 374806295, 1449088863, 21697055, 117526879, 21299231, 56581983,
    273094487, 123016983, 84085013, 274661723, 67311135, 1140916567, 16781322,
];

#[test]
fn test_parseq_mapper_builder_and_exec_with_aggr_output() {
    let circuit = Circuit::<u32>::from_str(COMB_CIRCUIT_CODE).unwrap();
    for (config_num, do_use_seq) in [false, true].into_iter().enumerate() {
        let par_builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            &CLANG_WRITER_U64,
            None,
        );
        let seq_builders = if do_use_seq {
            get_all_devices(CL_DEVICE_TYPE_GPU)
                .unwrap()
                .into_iter()
                .map(|dev_id| {
                    let device = Device::new(dev_id);
                    OpenCLBuilder::new(&device, None)
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        let mut builder = ParSeqMapperBuilder::new(par_builder, seq_builders);
        let sel_config = |sel| match sel {
            ParSeqSelection::Par => ParSeqDynamicConfig::new()
                .aggr_output_code(Some(COMB_CIRCUIT_AGGR_CODE))
                .aggr_output_len(Some(1 << (12 - 5))),
            ParSeqSelection::Seq(_) => ParSeqDynamicConfig::new()
                .aggr_output_code(Some(COMB_CIRCUIT_AGGR_CODE_OPENCL))
                .aggr_output_len(Some(1 << (12 - 5))),
        };
        builder.add_with_config(
            "comb_aggr_out_arg",
            circuit.clone(),
            &(0..4).collect::<Vec<_>>(),
            None,
            sel_config,
        );
        builder.add_with_config(
            "comb_aggr_out_arg_elem_full",
            circuit.clone(),
            &(0..4).collect::<Vec<_>>(),
            Some(&(4..16).collect::<Vec<_>>()),
            sel_config,
        );
        let mut execs = builder.build().unwrap();
        //let input = execs.new_data_from_vec(|sel| (0..1 << 12).collect::<Vec<_>>());
        let mut input_circ = None;
        {
            let mut ptransforms = ParSeqMapperTransforms::new(&mut execs[0]);
            ptransforms
                .with_input_transforms(
                    |ps_exec, mut par_it, mut seq_it| {
                        input_circ =
                            Some(ps_exec.new_data_with_executor(|execobj| match execobj {
                                ParSeqObject::Par(exec) => {
                                    let input =
                                        exec.new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
                                    ParSeqObject::Par(par_it.transform(&input).unwrap())
                                }
                                ParSeqObject::Seq((i, exec)) => {
                                    let input =
                                        exec.new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
                                    ParSeqObject::Seq(
                                        Arc::get_mut(&mut seq_it[i])
                                            .unwrap()
                                            .transform(&input)
                                            .unwrap(),
                                    )
                                }
                            }));
                    },
                    32,
                    &(0..12).collect::<Vec<_>>(),
                )
                .unwrap();
        }
        let input_circ = input_circ.unwrap();
        let exec_count = Arc::new(AtomicUsize::new(0));
        let output = execs[0]
            .execute_direct(
                &input_circ,
                vec![0u32; COMB_CIRCUIT_EXPECTED.len()],
                |_, _, result_out, _| {
                    exec_count.fetch_add(1, atomic::Ordering::SeqCst);
                    result_out.to_vec()
                },
                |out, result_out| {
                    let mut new_out = vec![0u32; COMB_CIRCUIT_EXPECTED.len()];
                    for (i, v) in new_out.iter_mut().enumerate() {
                        *v = out[i] | result_out[i];
                    }
                    new_out
                },
                |_| false,
            )
            .unwrap();
        assert_eq!(exec_count.load(atomic::Ordering::SeqCst), 16);
        assert_eq!(COMB_CIRCUIT_EXPECTED.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(COMB_CIRCUIT_EXPECTED[i], *out, "{}: {}", config_num, i);
        }

        // with elem input full
        let input_circ = execs[1].new_data(16);
        let exec_count = Arc::new(AtomicUsize::new(0));
        let output = execs[1]
            .execute_direct(
                &input_circ,
                vec![0u32; COMB_CIRCUIT_EXPECTED.len()],
                |_, _, result_out, _| {
                    exec_count.fetch_add(1, atomic::Ordering::SeqCst);
                    result_out.to_vec()
                },
                |out, result_out| {
                    let mut new_out = vec![0u32; COMB_CIRCUIT_EXPECTED.len()];
                    for (i, v) in new_out.iter_mut().enumerate() {
                        *v = out[i] | result_out[i];
                    }
                    new_out
                },
                |_| false,
            )
            .unwrap();
        assert_eq!(exec_count.load(atomic::Ordering::SeqCst), 16);
        assert_eq!(COMB_CIRCUIT_EXPECTED.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(COMB_CIRCUIT_EXPECTED[i], *out, "{}: {}", config_num, i);
        }
    }
}

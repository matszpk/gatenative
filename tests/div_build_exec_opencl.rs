use gatenative::div_build_exec::*;
use gatenative::opencl_build_exec::*;
use gatenative::*;
use gatesim::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};

#[test]
fn test_div_builder_and_exec_opencl() {
    let no_opt_neg_config = OpenCLBuilderConfig {
        optimize_negs: false,
        group_vec: false,
        group_len: None,
    };
    let opt_neg_config = OpenCLBuilderConfig {
        optimize_negs: true,
        group_vec: false,
        group_len: None,
    };

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        for max_gates in [2, 3, 4, 5, 100] {
            let builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
            let word_len = (builder.word_len() >> 5) as usize;
            let mut builder = DivBuilder::new(builder, max_gates);
            let circuit = Circuit::new(
                4,
                [
                    Gate::new_and(0, 2),
                    Gate::new_and(1, 2),
                    Gate::new_and(0, 3),
                    Gate::new_and(1, 3),
                    // add a1*b0 + a0*b1
                    Gate::new_xor(5, 6),
                    Gate::new_and(5, 6),
                    // add c(a1*b0 + a0*b1) + a1*b1
                    Gate::new_xor(7, 9),
                    Gate::new_and(7, 9),
                ],
                [(4, false), (8, false), (10, false), (11, false)],
            )
            .unwrap();
            builder.add("mul2x2", circuit.clone(), None, None, None);
            let input_ps = (&[3, 1, 6, 4][..], 8);
            let output_ps = (&[5, 3, 1, 4][..], 7);
            builder.add(
                "mul2x2p",
                circuit.clone(),
                Some(input_ps.clone()),
                Some(output_ps.clone()),
                None,
            );
            builder.add_ext("mul2x2sb", circuit.clone(), None, None, None, None, true);
            let mut execs = builder.build().unwrap();

            const MUL2X2_INPUT_TEMPLATE: [u32; 4] = [
                0b1010101010101010,
                0b1100110011001100,
                0b1111000011110000,
                0b1111111100000000,
            ];
            let mul2x2_input = MUL2X2_INPUT_TEMPLATE
                .into_iter()
                .map(|x| std::iter::once(x).chain(std::iter::repeat(0).take(word_len - 1)))
                .flatten()
                .collect::<Vec<_>>();
            let input = execs[0].new_data_from_vec(mul2x2_input.clone());
            let out = execs[0].execute(&input, 0).unwrap().release();
            for i in 0..16 {
                let a = i & 3;
                let b = i >> 2;
                let c = ((out[0] >> i) & 1)
                    + (((out[1 * word_len] >> i) & 1) << 1)
                    + (((out[2 * word_len] >> i) & 1) << 2)
                    + (((out[3 * word_len] >> i) & 1) << 3);
                assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
            }
            // more inputs
            let mul2x2_more_input_combs = {
                let mut input = vec![];
                let mut s = 0x34251u32;
                for _ in 0..64 * 24 {
                    input.push(s & 15);
                    s = (s ^ (s * 1895952115 + 159502151)) ^ 0xba001a4;
                    s = s.rotate_right(s & 15);
                }
                input
            };
            let mut more_input_holder =
                execs[0].new_data((mul2x2_more_input_combs.len() >> 6) * 4 * 2);
            {
                let mut more_input_w = more_input_holder.get_mut();
                let more_input = more_input_w.get_mut();
                for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
                    let idx = (i >> 5) / word_len;
                    let half_idx = (i >> 5) % word_len;
                    let shift = i & 31;
                    more_input[idx * 4 * word_len + 0 + half_idx] |= (v & 1) << shift;
                    more_input[idx * 4 * word_len + word_len + half_idx] |= ((v >> 1) & 1) << shift;
                    more_input[idx * 4 * word_len + 2 * word_len + half_idx] |=
                        ((v >> 2) & 1) << shift;
                    more_input[idx * 4 * word_len + 3 * word_len + half_idx] |=
                        ((v >> 3) & 1) << shift;
                }
            }
            let out = execs[0].execute(&more_input_holder, 0).unwrap().release();
            for (i, v) in mul2x2_more_input_combs.into_iter().enumerate() {
                let idx = (i >> 5) / word_len;
                let half_idx = (i >> 5) % word_len;
                let shift = i & 31;
                let a = v & 3;
                let b = v >> 2;
                let c = ((out[idx * 4 * word_len + 0 + half_idx] >> shift) & 1)
                    + (((out[idx * 4 * word_len + word_len + half_idx] >> shift) & 1) << 1)
                    + (((out[idx * 4 * word_len + 2 * word_len + half_idx] >> shift) & 1) << 2)
                    + (((out[idx * 4 * word_len + 3 * word_len + half_idx] >> shift) & 1) << 3);
                assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
            }
            // execute with input and output placements
            let mut mul2x2_input_p = execs[2].new_data(word_len * 8);
            {
                let mut mul2x2_input_p_slice_w = mul2x2_input_p.get_mut();
                let mul2x2_input_p_slice = mul2x2_input_p_slice_w.get_mut();
                for i in 0..4 {
                    for j in 0..word_len {
                        mul2x2_input_p_slice[word_len * input_ps.0[i] + j] =
                            mul2x2_input[word_len * i + j];
                    }
                }
            }
            let out = execs[1].execute(&mul2x2_input_p, 0).unwrap().release();
            for i in 0..16 {
                let a = i & 3;
                let b = i >> 2;
                let c = ((out[word_len * output_ps.0[0]] >> i) & 1)
                    + (((out[word_len * output_ps.0[1]] >> i) & 1) << 1)
                    + (((out[word_len * output_ps.0[2]] >> i) & 1) << 2)
                    + (((out[word_len * output_ps.0[3]] >> i) & 1) << 3);
                assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
            }
            // execute circuit with single buffer
            let mul2x2_input = MUL2X2_INPUT_TEMPLATE
                .into_iter()
                .map(|x| std::iter::once(x).chain(std::iter::repeat(0).take(word_len - 1)))
                .flatten()
                .collect::<Vec<_>>();
            let mut out = execs[2].new_data_from_vec(mul2x2_input.clone());
            execs[2].execute_single(&mut out, 0).unwrap();
            let out = out.release();
            for i in 0..16 {
                let a = i & 3;
                let b = i >> 2;
                let c = ((out[0] >> i) & 1)
                    + (((out[1 * word_len] >> i) & 1) << 1)
                    + (((out[2 * word_len] >> i) & 1) << 2)
                    + (((out[3 * word_len] >> i) & 1) << 3);
                assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
            }
        }

        // arg_input example
        for max_gates in [5, 10, 13, 16, 20, 100] {
            let builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
            let word_len = (builder.word_len() >> 5) as usize;
            let mut builder = DivBuilder::new(builder, max_gates);
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
            let arg_input_indices = [0, 3, 5, 8];
            let (input_ps, input_ps_len) = (&[3, 7, 13, 19, 20, 26, 29, 32][..], 35);
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
            builder.add(
                "xcircuit",
                circuit.clone(),
                None,
                None,
                Some(&arg_input_indices[..]),
            );
            builder.add(
                "xcircuit_input_ps",
                circuit.clone(),
                Some((input_ps, input_ps_len)),
                None,
                Some(&arg_input_indices[..]),
            );
            let mut execs = builder.build().unwrap();
            // number of chunks
            let xcircuit_data_num = (((256 >> 5) + word_len - 1) / word_len) * word_len;
            let rest_num = rest_input_indices.len();
            let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
            let mut xcircuit_input_ps = vec![0u32; xcircuit_data_num * input_ps_len];
            // prepare input for executor
            for i in 0..256 {
                let idx = (i >> 5) / word_len;
                let widx = (i >> 5) % word_len;
                let bit = i & 31;
                for j in 0..rest_num {
                    xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                        ((u32::try_from(i).unwrap() >> j) & 1) << bit;
                    let ps_j = input_ps[j];
                    xcircuit_input_ps[input_ps_len * word_len * idx + word_len * ps_j + widx] |=
                        ((u32::try_from(i).unwrap() >> j) & 1) << bit;
                }
            }
            for arg_input in 0..16 {
                let mut input = vec![false; 12];
                let mut xcircuit_out = vec![0u32; xcircuit_data_num];
                // fill inputs by arg_inputs
                for (i, v) in arg_input_indices.iter().enumerate() {
                    input[*v] = ((arg_input >> i) & 1) != 0;
                }
                // prepare expected output
                for rest in 0..256 {
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
                // execute circuit
                let input = execs[0].new_data_from_vec(xcircuit_input.clone());
                let result_out = execs[0].execute(&input, arg_input).unwrap().release();
                for (i, &exp) in xcircuit_out.iter().enumerate() {
                    assert_eq!(exp, result_out[i], "{}: {} {}", config_num, arg_input, i);
                }
                let input = execs[1].new_data_from_vec(xcircuit_input_ps.clone());
                let result_out = execs[1].execute(&input, arg_input).unwrap().release();
                for (i, exp) in xcircuit_out.into_iter().enumerate() {
                    assert_eq!(exp, result_out[i], "{}: {} {}", config_num, arg_input, i);
                }
            }
        }
    }
}

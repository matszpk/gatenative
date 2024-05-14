use gatenative::opencl_build_exec::*;
use gatenative::*;
use gatesim::*;
use gateutil::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::types::CL_BLOCKING;

use std::str::FromStr;

fn mul_add_circuit() -> Circuit<u32> {
    Circuit::new(
        24,
        [
            Gate::new_and(0, 1),
            Gate::new_xor(24, 2),
            Gate::new_and(0, 3),
            Gate::new_and(4, 1),
            Gate::new_xor(26, 27),
            Gate::new_xor(28, 5),
            Gate::new_and(24, 2),
            Gate::new_xor(29, 30),
            Gate::new_and(0, 6),
            Gate::new_and(4, 3),
            Gate::new_and(7, 1),
            Gate::new_xor(33, 34),
            Gate::new_xor(32, 35),
            Gate::new_and(26, 27),
            Gate::new_xor(36, 37),
            Gate::new_xor(38, 8),
            Gate::new_and(29, 30),
            Gate::new_and(28, 5),
            Gate::new_nor(40, 41),
            Gate::new_xor(39, 42),
            Gate::new_and(0, 9),
            Gate::new_and(4, 6),
            Gate::new_xor(44, 45),
            Gate::new_and(7, 3),
            Gate::new_and(10, 1),
            Gate::new_xor(47, 48),
            Gate::new_xor(46, 49),
            Gate::new_and(33, 34),
            Gate::new_xor(50, 51),
            Gate::new_and(36, 37),
            Gate::new_and(32, 35),
            Gate::new_nor(53, 54),
            Gate::new_xor(52, 55),
            Gate::new_xor(56, 11),
            Gate::new_nimpl(39, 42),
            Gate::new_and(38, 8),
            Gate::new_nor(58, 59),
            Gate::new_xor(57, 60),
            Gate::new_and(0, 12),
            Gate::new_and(4, 9),
            Gate::new_and(7, 6),
            Gate::new_xor(63, 64),
            Gate::new_and(10, 3),
            Gate::new_and(13, 1),
            Gate::new_xor(66, 67),
            Gate::new_xor(65, 68),
            Gate::new_xor(62, 69),
            Gate::new_and(47, 48),
            Gate::new_xor(70, 71),
            Gate::new_and(46, 49),
            Gate::new_and(44, 45),
            Gate::new_nor(73, 74),
            Gate::new_xor(72, 75),
            Gate::new_nimpl(52, 55),
            Gate::new_and(50, 51),
            Gate::new_nor(77, 78),
            Gate::new_xor(76, 79),
            Gate::new_xor(80, 14),
            Gate::new_nor(57, 60),
            Gate::new_nimpl(11, 56),
            Gate::new_nor(82, 83),
            Gate::new_xor(81, 84),
            Gate::new_and(0, 15),
            Gate::new_and(4, 12),
            Gate::new_and(7, 9),
            Gate::new_xor(87, 88),
            Gate::new_and(10, 6),
            Gate::new_xor(89, 90),
            Gate::new_and(13, 3),
            Gate::new_and(16, 1),
            Gate::new_xor(92, 93),
            Gate::new_xor(91, 94),
            Gate::new_and(66, 67),
            Gate::new_xor(95, 96),
            Gate::new_xor(86, 97),
            Gate::new_and(65, 68),
            Gate::new_and(63, 64),
            Gate::new_nor(99, 100),
            Gate::new_xor(98, 101),
            Gate::new_and(70, 71),
            Gate::new_and(62, 69),
            Gate::new_nor(103, 104),
            Gate::new_xor(102, 105),
            Gate::new_nor(76, 79),
            Gate::new_nimpl(72, 75),
            Gate::new_nor(107, 108),
            Gate::new_xor(106, 109),
            Gate::new_xor(110, 17),
            Gate::new_nimpl(81, 84),
            Gate::new_and(80, 14),
            Gate::new_nor(112, 113),
            Gate::new_xor(111, 114),
            Gate::new_and(0, 18),
            Gate::new_and(4, 15),
            Gate::new_xor(116, 117),
            Gate::new_and(7, 12),
            Gate::new_xor(118, 119),
            Gate::new_and(10, 9),
            Gate::new_and(13, 6),
            Gate::new_xor(121, 122),
            Gate::new_and(16, 3),
            Gate::new_and(19, 1),
            Gate::new_xor(124, 125),
            Gate::new_xor(123, 126),
            Gate::new_and(89, 90),
            Gate::new_and(87, 88),
            Gate::new_nor(128, 129),
            Gate::new_xor(127, 130),
            Gate::new_and(92, 93),
            Gate::new_xor(131, 132),
            Gate::new_xor(120, 133),
            Gate::new_and(95, 96),
            Gate::new_and(91, 94),
            Gate::new_nor(135, 136),
            Gate::new_xor(134, 137),
            Gate::new_nimpl(98, 101),
            Gate::new_and(86, 97),
            Gate::new_nor(139, 140),
            Gate::new_xor(138, 141),
            Gate::new_nimpl(106, 109),
            Gate::new_nor(102, 105),
            Gate::new_nor(143, 144),
            Gate::new_xor(142, 145),
            Gate::new_xor(146, 20),
            Gate::new_nor(111, 114),
            Gate::new_nimpl(17, 110),
            Gate::new_nor(148, 149),
            Gate::new_xor(147, 150),
            Gate::new_and(0, 21),
            Gate::new_and(4, 18),
            Gate::new_xor(152, 153),
            Gate::new_and(7, 15),
            Gate::new_xor(154, 155),
            Gate::new_and(10, 12),
            Gate::new_and(13, 9),
            Gate::new_xor(157, 158),
            Gate::new_and(16, 6),
            Gate::new_xor(159, 160),
            Gate::new_and(19, 3),
            Gate::new_and(22, 1),
            Gate::new_xor(162, 163),
            Gate::new_xor(161, 164),
            Gate::new_and(124, 125),
            Gate::new_xor(165, 166),
            Gate::new_and(118, 119),
            Gate::new_and(116, 117),
            Gate::new_nor(168, 169),
            Gate::new_xor(167, 170),
            Gate::new_and(123, 126),
            Gate::new_and(121, 122),
            Gate::new_nor(172, 173),
            Gate::new_xor(171, 174),
            Gate::new_xor(156, 175),
            Gate::new_nimpl(132, 131),
            Gate::new_nimpl(127, 130),
            Gate::new_nor(177, 178),
            Gate::new_xor(176, 179),
            Gate::new_nor(134, 137),
            Gate::new_nimpl(120, 133),
            Gate::new_nor(181, 182),
            Gate::new_xor(180, 183),
            Gate::new_nor(142, 145),
            Gate::new_nimpl(138, 141),
            Gate::new_nor(185, 186),
            Gate::new_xor(184, 187),
            Gate::new_xor(188, 23),
            Gate::new_nimpl(147, 150),
            Gate::new_and(146, 20),
            Gate::new_nor(190, 191),
            Gate::new_xor(189, 192),
        ],
        [
            (25, false),
            (31, false),
            (43, true),
            (61, false),
            (85, true),
            (115, false),
            (151, true),
            (193, false),
        ],
    )
    .unwrap()
}

const MUL_ADD_INPUT_MAP: [usize; 24] = [
    0, 4, 7, 10, 13, 16, 19, 22, 1, 3, 6, 9, 12, 15, 18, 21, 2, 5, 8, 11, 14, 17, 20, 23,
];
fn gen_mul_add_input(word_len: usize) -> (Vec<u32>, Vec<u32>) {
    let mut input = vec![0u32; ((1 << 24) >> 5) * 24];
    let mut exp_output = vec![0u32; ((1 << 24) >> 5) * 8];
    for a in 0u32..256 {
        for b in 0u32..256 {
            for c in 0u32..256 {
                let i = ((a & 0xff) | ((b & 0xff) << 8) | ((c & 0xff) << 16)) as usize;
                let idx = (i >> 5) / word_len;
                let half_idx = (i >> 5) % word_len;
                let shift = i & 31;
                let exp = (a.overflowing_mul(b).0).overflowing_add(c).0;
                for bit in 0..8 {
                    let ix = MUL_ADD_INPUT_MAP[bit];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((a >> bit) & 1) << shift;
                    let ix = MUL_ADD_INPUT_MAP[bit + 8];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((b >> bit) & 1) << shift;
                    let ix = MUL_ADD_INPUT_MAP[bit + 16];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((c >> bit) & 1) << shift;
                    exp_output[idx * 8 * word_len + bit * word_len + half_idx] |=
                        ((exp >> bit) & 1) << shift;
                }
            }
        }
    }
    (input, exp_output)
}

#[test]
fn test_opencl_builder_and_exec() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let word_len =
        (OpenCLBuilder::new(&device, Some(no_opt_neg_config.clone())).word_len() >> 5) as usize;
    let (mul_add_input, mul_add_output) = gen_mul_add_input(word_len);

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
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
        builder.add_with_config(
            "mul2x2sb",
            circuit.clone(),
            CodeConfig::new().single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        // check elem_count
        assert_eq!(execs[0].input_data_len(16 * 1024), 2048);
        assert_eq!(execs[0].output_data_len(16 * 1024), 2048);
        assert_eq!(execs[1].input_data_len(16 * 1024), 4096);
        assert_eq!(execs[1].output_data_len(16 * 1024), 3584);
        assert_eq!(execs[0].elem_count(135 * 32 * 4), 138240);
        for (i, exec) in execs.iter().enumerate() {
            assert!(!exec.output_is_aggregated(), "{}: {}", config_num, i);
            assert_eq!(exec.aggr_output_len(), None, "{}: {}", config_num, i);
            assert!(!exec.input_is_populated(), "{}: {}", config_num, i);
            assert_eq!(exec.pop_input_len(), None, "{}: {}", config_num, i);
        }

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
        let input_holder = execs[0].new_data_from_vec(mul2x2_input.clone());
        let out = execs[0].execute(&input_holder, 0).unwrap().release();
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
        let mut more_input = vec![0; (mul2x2_more_input_combs.len() >> 6) * 4 * 2];
        let more_input_num = more_input.len() / 4;
        for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
            let idx = (i >> 5) / word_len;
            let half_idx = (i >> 5) % word_len;
            let shift = i & 31;
            more_input[idx * 4 * word_len + 0 + half_idx] |= (v & 1) << shift;
            more_input[idx * 4 * word_len + word_len + half_idx] |= ((v >> 1) & 1) << shift;
            more_input[idx * 4 * word_len + 2 * word_len + half_idx] |= ((v >> 2) & 1) << shift;
            more_input[idx * 4 * word_len + 3 * word_len + half_idx] |= ((v >> 3) & 1) << shift;
        }
        let more_input_holder = execs[0].new_data_from_vec(more_input);
        let out = execs[0].execute(&more_input_holder, 0).unwrap().release();
        for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
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
        // test reusing output data holder
        let mut out = execs[0].new_data(more_input_num * 4);
        execs[0]
            .execute_reuse(&more_input_holder, 0, &mut out)
            .unwrap();
        let out = out.release();
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
        let mut mul2x2_input_p_slice = vec![0u32; word_len * 8];
        for i in 0..4 {
            for j in 0..word_len {
                mul2x2_input_p_slice[word_len * input_ps.0[i] + j] = mul2x2_input[word_len * i + j];
            }
        }
        let mul2x2_input_p = execs[1].new_data_from_vec(mul2x2_input_p_slice);
        let out = execs[1].execute(&mul2x2_input_p, 0).unwrap().release();
        let out_len = out.len();
        for i in 0..16 {
            let a = i & 3;
            let b = i >> 2;
            let c = ((out[word_len * output_ps.0[0]] >> i) & 1)
                + (((out[word_len * output_ps.0[1]] >> i) & 1) << 1)
                + (((out[word_len * output_ps.0[2]] >> i) & 1) << 2)
                + (((out[word_len * output_ps.0[3]] >> i) & 1) << 3);
            assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
        }
        let mut out = execs[1].new_data(out_len);
        execs[1]
            .execute_reuse(&mul2x2_input_p, 0, &mut out)
            .unwrap();
        let out = out.release();
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

        // arg_input example
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
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
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

        // more complex circuit
        let circuit = mul_add_circuit();
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        builder.add("mul_add", circuit.clone(), None, None, None);
        let mut execs = builder.build().unwrap();
        let mul_add_input = execs[0].new_data_from_vec(mul_add_input.clone());
        let out = execs[0].execute(&mul_add_input, 0).unwrap().release();
        for (i, v) in mul_add_output.iter().enumerate() {
            assert_eq!(*v, out[i], "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_arg_input() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        // with arg_input
        let circuit = translate_inputs_rev(mul_add_circuit(), MUL_ADD_INPUT_MAP);
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        builder.add_with_config(
            "mul_add_sb",
            circuit.clone(),
            CodeConfig::new()
                .output_placement(Some((&(0..8).collect::<Vec<_>>(), 20)))
                .arg_inputs(Some(&(20..24).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        builder.add_with_config(
            "mul_add_sb_ip",
            circuit.clone(),
            CodeConfig::new()
                .input_placement(Some((
                    &(0..20).map(|i| (19 - i) + 4).collect::<Vec<_>>(),
                    24,
                )))
                .output_placement(Some((&(0..8).collect::<Vec<_>>(), 24)))
                .arg_inputs(Some(&(20..24).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        let mut it = execs[0]
            .input_transformer(32, &(0..20).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        for arg_input in 0..16 {
            let input =
                execs[0].new_data_from_vec((0..1 << 20).map(|i| i ^ 0xff000).collect::<Vec<_>>());
            let mut input_circ = it.transform(&input).unwrap();
            execs[0].execute_single(&mut input_circ, arg_input).unwrap();
            let output = ot.transform(&input_circ).unwrap();
            let output = output.release();
            assert_eq!(output.len(), 1 << 20);
            for (i, v) in output.into_iter().enumerate() {
                let ix = (i ^ 0xff000) | (usize::try_from(arg_input).unwrap() << 20);
                let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
                assert_eq!(out, v, "{}: {}", config_num, i);
            }
        }

        let mut it = execs[1]
            .input_transformer(
                32,
                &(0..24)
                    .map(|i| if i >= 4 { 23 - i } else { 0 })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        let mut ot = execs[1]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        for arg_input in 0..16 {
            let input =
                execs[1].new_data_from_vec((0..1 << 20).map(|i| i ^ 0xff000).collect::<Vec<_>>());
            let mut input_circ = it.transform(&input).unwrap();
            execs[1].execute_single(&mut input_circ, arg_input).unwrap();
            let output = ot.transform(&input_circ).unwrap();
            let output = output.release();
            assert_eq!(output.len(), 1 << 20);
            for (i, v) in output.into_iter().enumerate() {
                let ix = (i ^ 0xff000) | (usize::try_from(arg_input).unwrap() << 20);
                let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
                assert_eq!(out, v, "{}: {}", config_num, i);
            }
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_elem_input() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        // with elem_index
        let circuit = translate_inputs_rev(mul_add_circuit(), MUL_ADD_INPUT_MAP);
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        builder.add_with_config(
            "mul_add_elem",
            circuit.clone(),
            CodeConfig::new().elem_inputs(Some(&(0..12).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "mul_add_elem_full",
            circuit.clone(),
            CodeConfig::new().elem_inputs(Some(&(0..24).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "mul_add_elem_sb",
            circuit.clone(),
            CodeConfig::new()
                .output_placement(Some((&(0..8).collect::<Vec<_>>(), 12)))
                .elem_inputs(Some(&(0..12).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        builder.add_with_config(
            "mul_add_elem_arginput",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(20..24).collect::<Vec<_>>()))
                .elem_inputs(Some(&(0..12).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "mul_add_elem_sb_ip",
            circuit.clone(),
            CodeConfig::new()
                .input_placement(Some((
                    &(0..12).map(|i| (11 - i) + 4).collect::<Vec<_>>(),
                    16,
                )))
                .output_placement(Some((&(0..8).collect::<Vec<_>>(), 16)))
                .elem_inputs(Some(&(0..12).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        assert_eq!(execs[0].input_data_len(16 * 1024), 6144);
        assert_eq!(execs[0].output_data_len(16 * 1024), 4096);
        assert_eq!(execs[1].input_data_len(16 * 1024), 1);
        assert_eq!(execs[1].output_data_len(16 * 1024), 4096);
        assert_eq!(execs[0].elem_count(1024 * 48), 131072);
        assert_eq!(execs[1].elem_count(1024 * 48), 1 << 24);

        let mut it = execs[0]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        let input =
            execs[0].new_data_from_vec((0..1 << 24).map(|i| (i >> 12) ^ 0xfff).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output_circ = execs[0].execute(&input_circ, 0).unwrap();
        let output_circ_len = output_circ.len();
        let output = ot.transform(&output_circ).unwrap();
        let output = output.release();
        assert_eq!(output.len(), 1 << 24);
        for (i, v) in output.into_iter().enumerate() {
            let ix = i ^ 0xfff000;
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, i);
        }

        let mut output_circ = execs[0].new_data(output_circ_len);
        execs[0]
            .execute_reuse(&input_circ, 0, &mut output_circ)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap();
        let output = output.release();
        assert_eq!(output.len(), 1 << 24);
        for (i, v) in output.into_iter().enumerate() {
            let ix = i ^ 0xfff000;
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, i);
        }

        // with elem full
        let mut ot = execs[1]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        let input = execs[1].new_data(1);
        let output_circ = execs[1].execute(&input, 0).unwrap();
        let output_circ_len = output_circ.len();
        let output = ot.transform(&output_circ).unwrap();
        let output = output.release();
        assert!(output.len() != 0);
        assert_eq!(output.len(), 1 << 24);
        for (ix, v) in output.into_iter().enumerate() {
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, ix);
        }

        let mut output_circ = execs[1].new_data(output_circ_len);
        execs[1]
            .execute_reuse(&input_circ, 0, &mut output_circ)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap();
        let output = output.release();
        for (ix, v) in output.into_iter().enumerate() {
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, ix);
        }

        // with single buffer
        let mut it = execs[2]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[2]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        let input =
            execs[2].new_data_from_vec((0..1 << 24).map(|i| (i >> 12) ^ 0xfff).collect::<Vec<_>>());
        let mut input_circ = it.transform(&input).unwrap();
        execs[2].execute_single(&mut input_circ, 0).unwrap();
        let output = ot.transform(&input_circ).unwrap();
        let output = output.release();
        assert_eq!(output.len(), 1 << 24);
        for (i, v) in output.into_iter().enumerate() {
            let ix = i ^ 0xfff000;
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, i);
        }

        // with elem_input and arg_input
        let mut it = execs[3]
            .input_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[3]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        for arg_input in 0..16 {
            let input = execs[3]
                .new_data_from_vec((0..1 << 20).map(|i| (i >> 12) ^ 0xff).collect::<Vec<_>>());
            let input_circ = it.transform(&input).unwrap();
            let output_circ = execs[3].execute(&input_circ, arg_input).unwrap();
            let output_circ_len = output_circ.len();
            let output = ot.transform(&output_circ).unwrap();
            let output = output.release();
            assert_eq!(output.len(), 1 << 20);
            for (i, v) in output.into_iter().enumerate() {
                let ix = (i ^ 0xff000) | (usize::try_from(arg_input).unwrap() << 20);
                let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
                assert_eq!(out, v, "{}: {} {}", config_num, arg_input, i);
            }

            let mut output_circ = execs[3].new_data(output_circ_len);
            execs[3]
                .execute_reuse(&input_circ, arg_input, &mut output_circ)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap();
            let output = output.release();
            for (i, v) in output.into_iter().enumerate() {
                let ix = (i ^ 0xff000) | (usize::try_from(arg_input).unwrap() << 20);
                let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
                assert_eq!(out, v, "x {}: {} {}", config_num, arg_input, i);
            }
        }

        // with single buffer and input_placement
        let mut it = execs[4]
            .input_transformer(
                32,
                &(0..16)
                    .map(|i| if i >= 4 { 15 - i } else { 0 })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        let mut ot = execs[4]
            .output_transformer(32, &(0..8).collect::<Vec<_>>())
            .unwrap();
        let input =
            execs[4].new_data_from_vec((0..1 << 24).map(|i| (i >> 12) ^ 0xfff).collect::<Vec<_>>());
        let mut input_circ = it.transform(&input).unwrap();
        execs[4].execute_single(&mut input_circ, 0).unwrap();
        let output = ot.transform(&input_circ).unwrap();
        let output = output.release();
        for (i, v) in output.into_iter().enumerate() {
            let ix = i ^ 0xfff000;
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_group_vec() {
    let opt_neg_config = OpenCLBuilderConfig::new()
        .optimize_negs(true)
        .group_vec(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let word_len =
        (OpenCLBuilder::new(&device, Some(opt_neg_config.clone())).word_len() >> 5) as usize;
    let (mul_add_input, mul_add_output) = gen_mul_add_input(word_len);
    println!("WordLen: {}", word_len);

    let circuit = mul_add_circuit();
    let mut builder = OpenCLBuilder::new(&device, Some(opt_neg_config));
    builder.add("mul_add", circuit, None, None, None);

    // for single_buffer
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
    builder.add_with_config(
        "mul2x2sb",
        circuit.clone(),
        CodeConfig::new().single_buffer(true),
    );

    let mut execs = builder.build().unwrap();
    let mul_add_input = execs[0].new_data_from_vec(mul_add_input.clone());
    let out = execs[0].execute(&mul_add_input, 0).unwrap().release();
    for (i, v) in mul_add_output.iter().enumerate() {
        assert_eq!(*v, out[i], "{}", i);
    }
    let mut out = execs[0].new_data(mul_add_input.len() / 3);
    execs[0].execute_reuse(&mul_add_input, 0, &mut out).unwrap();
    let out = out.release();
    for (i, v) in mul_add_output.iter().enumerate() {
        assert_eq!(*v, out[i], "{}", i);
    }

    // single buffer
    let mul2x2_more_input_combs = {
        let mut input = vec![];
        let mut s = 0x34251u32;
        for _ in 0..word_len * 4 * 24 {
            input.push(s & 15);
            s = (s ^ (s * 1895952115 + 159502151)) ^ 0xba001a4;
            s = s.rotate_right(s & 15);
        }
        input
    };
    let mut more_input = vec![0; (mul2x2_more_input_combs.len() >> 6) * 4 * 2];
    for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
        let idx = (i >> 5) / word_len;
        let half_idx = (i >> 5) % word_len;
        let shift = i & 31;
        more_input[idx * 4 * word_len + 0 + half_idx] |= (v & 1) << shift;
        more_input[idx * 4 * word_len + word_len + half_idx] |= ((v >> 1) & 1) << shift;
        more_input[idx * 4 * word_len + 2 * word_len + half_idx] |= ((v >> 2) & 1) << shift;
        more_input[idx * 4 * word_len + 3 * word_len + half_idx] |= ((v >> 3) & 1) << shift;
    }
    let mut more_input_holder = execs[1].new_data_from_vec(more_input);
    execs[1].execute_single(&mut more_input_holder, 0).unwrap();
    let out = more_input_holder.release();
    for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
        let idx = (i >> 5) / word_len;
        let half_idx = (i >> 5) % word_len;
        let shift = i & 31;
        let a = v & 3;
        let b = v >> 2;
        let c = ((out[idx * 4 * word_len + 0 + half_idx] >> shift) & 1)
            + (((out[idx * 4 * word_len + word_len + half_idx] >> shift) & 1) << 1)
            + (((out[idx * 4 * word_len + 2 * word_len + half_idx] >> shift) & 1) << 2)
            + (((out[idx * 4 * word_len + 3 * word_len + half_idx] >> shift) & 1) << 3);
        assert_eq!((a * b) & 15, c, "{}", i);
    }
}

const COMB_CIRCUIT: &str = concat!(
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

const COMB_CIRCUIT2: &str = r##"
{0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 xor(4,9) and(20,5):0 xor(5,0) xor(22,10)
nimpl(4,9) nimpl(20,24) xor(23,25) and(26,6):1 xor(6,1) and(5,0) xor(28,29) xor(30,11)
nor(23,25) nimpl(22,10) nor(32,33) xor(31,34) and(35,7):2 xor(7,2) and(28,29) and(6,1)
nor(38,39) xor(37,40) xor(41,12) nor(31,34) nimpl(30,11) nor(43,44) xor(42,45) nimpl(8,46):3
xor(8,3) nimpl(37,40) and(7,2) nor(49,50) xor(48,51) xor(52,13) nimpl(42,45) nor(41,12)
nor(54,55) xor(53,56) xor(57,7) nimpl(9,58):4 xor(9,4) nimpl(48,51) and(8,3) nor(61,62)
xor(60,63) xor(64,14) nimpl(53,56) nor(52,13) nor(66,67) xor(65,68) xor(69,8) nimpl(7,57)
xor(70,71) nimpl(10,72):5 xor(10,5) nimpl(60,63) and(9,4) nor(75,76) xor(74,77) xor(78,15)
nimpl(65,68) nor(64,14) nor(80,81) xor(79,82) xor(83,9) nimpl(71,70) nimpl(8,69) nor(85,86)
xor(84,87) and(88,11):6 xor(11,6) nimpl(74,77) and(10,5) nor(91,92) xor(90,93) xor(94,16)
nimpl(79,82) nor(78,15) nor(96,97) xor(95,98) xor(99,10) nor(84,87) nimpl(9,83) nor(101,102)
xor(100,103) and(104,12):7 xor(0,17) nimpl(95,98) nor(94,16) nor(107,108) xor(106,109)
xor(110,11) nor(100,103) nimpl(10,99) nor(112,113) xor(111,114) nimpl(13,115):8 xor(1,18)
nor(106,109) nimpl(0,17) nor(118,119) xor(117,120) nimpl(111,114) and(110,11) nor(122,123)
xor(121,124) nimpl(14,125):9 xor(2,19) nor(117,120) nimpl(1,18) nor(128,129) xor(127,130)
nimpl(121,124) xor(131,132) and(133,15):10 xor(3,0) nor(127,130) nimpl(2,19) nor(136,137)
xor(135,138) xor(139,14) and(131,132) xor(140,141) and(142,16):11}(20)"##;

const COMB_AGGR_OUTPUT_CODE: &str = r##"{
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

const AGGR_OUTPUT_EXPECTED: [u32; 128] = [
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
fn test_opencl_builder_and_exec_with_aggr_output() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let circuit = Circuit::<u32>::from_str(COMB_CIRCUIT).unwrap();
        let circuit2 = Circuit::<u32>::from_str(COMB_CIRCUIT2).unwrap();

        let aggr_output_code = COMB_AGGR_OUTPUT_CODE;
        // 0
        builder.add_with_config(
            "comb_aggr_out",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 1
        builder.add_with_config(
            "comb_aggr_out_elem_full",
            circuit.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 2
        builder.add_with_config(
            "comb_aggr_out_ip",
            circuit.clone(),
            CodeConfig::new()
                .input_placement(Some((&(4..20).collect::<Vec<_>>(), 24)))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 3
        builder.add_with_config(
            "comb_aggr_out_elem",
            circuit.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 4
        builder.add_with_config(
            "comb_aggr_out_arg",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 5
        builder.add_with_config(
            "comb_aggr_out_arg_elem_full",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .elem_inputs(Some(&(4..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 6
        builder.add_with_config(
            "comb_aggr_out_arg_elem",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..2).collect::<Vec<_>>()))
                .elem_inputs(Some(&(4..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 7
        builder.add_with_config(
            "comb2_aggr_out",
            circuit2.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        // 8
        builder.add_with_config(
            "comb2_aggr_out_elem_full",
            circuit2.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..20).collect::<Vec<_>>()))
                .aggr_output_code(Some(aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        let mut execs = builder.build().unwrap();
        assert_eq!(execs[0].input_data_len(12 * 512), 3072);
        assert_eq!(execs[1].input_data_len(12 * 512), 1);
        for (i, exec) in execs.iter().enumerate() {
            assert_eq!(
                exec.output_data_len(17 * 1024),
                128,
                "{}: {}",
                config_num,
                i
            );
            assert!(exec.output_is_aggregated(), "{}: {}", config_num, i);
            assert!(!exec.is_aggregated_to_buffer(), "{}: {}", config_num, i);
            assert_eq!(
                exec.aggr_output_len(),
                Some(1 << (12 - 5)),
                "{}: {}",
                config_num,
                i
            );
        }

        let expected = AGGR_OUTPUT_EXPECTED;
        let expected2 = vec![
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 2013265919, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 2013265919, 4294967295, 4294967295,
            4294967295, 1442840575, 4294967295, 4294967295, 4294967295, 1442840575, 4294967295,
            4294967295, 4294967295, 1442840575, 4294967295, 4294967295, 4294967295, 1442840575,
            4294967295, 4294967295, 4294967295, 1442840575, 4294967295, 3221225343, 4160716799,
            1442840575, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            2147483647, 4294967295, 3758096383, 4294967295, 4294967295, 4294967295, 4286545919,
            4294967295, 4294934527, 2147483647, 3758063615, 4294967295, 4286578687, 2147483647,
            3212804095, 4294967295, 3221192703, 2147483647, 2684321663, 4294967295, 4294967295,
            4294967295, 4152360959, 4294967295, 4294967295, 4294967295, 1610055679, 4294967295,
            4294967295, 4294967295, 358612991, 2147483647, 3221225471, 2147483647, 1429700599,
            4294967295, 4294967295, 4294967295, 1432354815, 2013265919, 3212828671, 2147483647,
            1429436279, 4294967295, 2139095039, 4286578687, 1432353791, 2013265919, 4018135039,
            503316479, 288584567,
        ];
        let mut it = execs[0]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let input = execs[0].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output = execs[0].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[0].new_data(output.len());
        execs[0].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // with full elem inputs
        let input_circ = execs[1].new_data(1);
        let output = execs[1].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[1].new_data(output.len());
        execs[1].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // with input_placement
        let mut it = execs[2]
            .input_transformer(32, &(0..24).collect::<Vec<_>>())
            .unwrap();
        let input = execs[2].new_data_from_vec((0..1 << 16).map(|i| i << 4).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output = execs[2].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[2].new_data(output.len());
        execs[2].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // elem input
        let mut it = execs[3]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[3].new_data_from_vec((0..1 << 16).map(|i| i >> 4).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output = execs[3].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[3].new_data(output.len());
        execs[3].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // arg_input
        let mut it = execs[4]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[4].new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..16 {
            let output = execs[4].execute(&input_circ, i).unwrap().release();
            assert_eq!(output_comb.len(), output.len());
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..16 {
            let mut output = execs[4].new_data(expected.len());
            execs[4].execute_reuse(&input_circ, i, &mut output).unwrap();
            assert_eq!(output_comb.len(), output.len());
            let output = output.release();
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // arg_input and elem_input
        let input_circ = execs[5].new_data(1);
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..16 {
            let output = execs[5].execute(&input_circ, i).unwrap().release();
            assert_eq!(output_comb.len(), output.len());
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..16 {
            let mut output = execs[5].new_data(expected.len());
            execs[5].execute_reuse(&input_circ, i, &mut output).unwrap();
            assert_eq!(output_comb.len(), output.len());
            let output = output.release();
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // arg_input and elem_input
        let mut it = execs[6]
            .input_transformer(32, &(0..2).collect::<Vec<_>>())
            .unwrap();
        let input =
            execs[6].new_data_from_vec((0..1 << 14).map(|i| (i >> 12) & 3).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..4 {
            let output = execs[6].execute(&input_circ, i).unwrap().release();
            assert_eq!(output_comb.len(), output.len());
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output_comb = vec![0u32; expected.len()];
        for i in 0..4 {
            let mut output = execs[6].new_data(expected.len());
            execs[6].execute_reuse(&input_circ, i, &mut output).unwrap();
            assert_eq!(output_comb.len(), output.len());
            let output = output.release();
            for (i, v) in output_comb.iter_mut().enumerate() {
                *v |= output[i];
            }
        }
        for (i, out) in output_comb.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }

        // circuit 2
        let mut it = execs[7]
            .input_transformer(32, &(0..20).collect::<Vec<_>>())
            .unwrap();
        let input = execs[7].new_data_from_vec((0..1 << 20).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output = execs[7].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected2.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected2[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[7].new_data(output.len());
        execs[7].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected2.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected2[i], *out, "{}: {}", config_num, i);
        }

        // with full elem inputs
        let input_circ = execs[8].new_data(1);
        let output = execs[8].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected2.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected2[i], *out, "{}: {}", config_num, i);
        }
        let mut output = execs[8].new_data(output.len());
        execs[8].execute_reuse(&input_circ, 0, &mut output).unwrap();
        assert_eq!(expected2.len(), output.len());
        let output = output.release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected2[i], *out, "{}: {}", config_num, i);
        }
    }
}

const COMB_AGGR_OUTPUT_CODE_WITH_HELPERS: &str = r##"{
    unsigned int i;
    uint out[TYPE_LEN];
    global uint* output_u32 = (global uint*)output;
    OUTPUT_TRANSFORM_B12(out, o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11);
    for (i = 0; i < TYPE_LEN; i++) {
        const uint out_idx = out[i];
        atomic_or(&output_u32[out_idx >> 5], (1 << (out_idx & 31)));
    }
}"##;

#[test]
fn test_opencl_builder_and_exec_with_aggr_output_with_helpers() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let circuit = Circuit::<u32>::from_str(COMB_CIRCUIT).unwrap();
        // 0
        builder.transform_helpers();
        builder.add_with_config(
            "comb_aggr_out",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(COMB_AGGR_OUTPUT_CODE_WITH_HELPERS))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        let mut execs = builder.build().unwrap();
        let expected = AGGR_OUTPUT_EXPECTED;
        let mut it = execs[0]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let input = execs[0].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let output = execs[0].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected[i], *out, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_aggr_output_to_buffer() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let circuit = Circuit::<u32>::from_str(COMB_CIRCUIT).unwrap();
    let expected_out = (0..1u32 << 16)
        .map(|x| {
            let out = circuit.eval((0..16).map(|i| ((x >> i) & 1) != 0));
            let mut y = 0;
            for (i, outb) in out.into_iter().enumerate() {
                y |= u32::from(outb) << i;
            }
            y
        })
        .collect::<Vec<_>>();
    let expected_out_op = expected_out
        .iter()
        .map(|y| {
            (y & 1)
                | ((y & 2) << 1)
                | ((y & 4) << 2)
                | ((y & 8) << 3)
                | ((y & 16) << 4)
                | ((y & 32) << 5)
                | ((y & 64) << 6)
                | ((y & 128) << 7)
                | ((y & 256) << 8)
                | ((y & 512) << 9)
                | ((y & 1024) << 10)
                | ((y & 2048) << 11)
        })
        .collect::<Vec<_>>();
    let expected_out_op_2 = expected_out
        .iter()
        .enumerate()
        .map(|(x, y)| {
            (u32::try_from(x).unwrap() & 0xaaaa)
                | (y & 1)
                | ((y & 2) << 1)
                | ((y & 4) << 2)
                | ((y & 8) << 3)
                | ((y & 16) << 4)
                | ((y & 32) << 5)
                | ((y & 64) << 6)
                | ((y & 128) << 7)
                | ((y & 256) << 8)
                | ((y & 512) << 9)
                | ((y & 1024) << 10)
                | ((y & 2048) << 11)
        })
        .collect::<Vec<_>>();
    let params = [11, 5, 12];
    let expected_out_aggr_pop = (0..1 << 16)
        .map(|x| {
            let pidx = x >> 12;
            let x = (x & 0xfff) | ((((pidx * params[0] + params[1]) ^ params[2]) & 15) << 12);
            expected_out[x as usize]
        })
        .collect::<Vec<_>>();
    let expected_out_excl = (0..1u32 << 16)
        .map(|x| {
            let out = circuit.eval((0..16).map(|i| ((x >> i) & 1) != 0));
            let mut y = 0;
            for (i, outb) in out.into_iter().enumerate() {
                y |= u32::from(outb) << i;
            }
            ((y >> 1) & 3) | (((y >> 5) & 7) << 2)
        })
        .collect::<Vec<_>>();

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let comb_aggr_output_code = r##"{
    unsigned int i;
    uint out[TYPE_LEN];
    global uint* output_u32 = (global uint*)buffer;
    OUTPUT_TRANSFORM_B12(out, o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11);
    for (i = 0; i < TYPE_LEN; i++) {
        const uint out_idx = out[i];
        atomic_or(&output_u32[out_idx >> 5], (1 << (out_idx & 31)));
    }
}"##;
        let comb_aggr_output_code_2 = r##"{
    unsigned int i;
    uint out[TYPE_LEN];
    global uint* output_u32 = (global uint*)buffer;
    OUTPUT_TRANSFORM_B10(out, o0, o1, o2, o3, o4, o6, o7, o9, o10, o11);
    for (i = 0; i < TYPE_LEN; i++) {
        const uint out_idx = (out[i] & 0x1f) | ((out[i] & 0x60) << 1) |
                ((out[i] & 0x380) << 2);
        atomic_or(&output_u32[out_idx >> 5], (1 << (out_idx & 31)));
    }
}"##;
        let comb_pop_input_code = r##"{
    unsigned int i;
    uint inp[TYPE_LEN];
    global uint* params = ((global uint*)buffer) + 128;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < TYPE_LEN; i++) {
        const uint pidx = (idx*TYPE_LEN + i) >> 12;
        inp[i] = ((pidx*p0 + p1) ^ p2) & 15;
    }
    INPUT_TRANSFORM_B4(i12, i13, i14, i15, inp);
}"##;
        let expected_buffer_2: [u32; 128] = [
            4294967295, 0, 4294967295, 0, 4294967295, 0, 4294967295, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4294967295, 0, 4294967295, 0, 4294967295, 0, 4294967295, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4294967295, 0, 4294967295, 0, 4294967295, 0, 2147483647, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4294967295, 0, 2013265919, 0, 939524095, 0, 2004860927, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1600085855, 0, 1600085855, 0, 1600085855, 0, 1600085855, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1600085855, 0, 1600085855, 0, 1465868127, 0, 1197430623, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1600085855, 0, 1465868127, 0, 1600085855, 0, 392126303, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1600085855, 0, 1465868127, 0, 1465866079, 0, 1195725151, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        builder.transform_helpers();
        // 0
        builder.add_with_config(
            "comb_aggr_out",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "comb_aggr_out_2",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(comb_aggr_output_code_2))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&[0, 1, 2, 3, 4, 6, 7, 9, 10, 11])),
        );
        // 2
        builder.add_with_config(
            "comb_aggr_out_elem_full",
            circuit.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>())),
        );
        // 3
        builder.add_with_config(
            "comb_aggr_out_op",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>()))
                .output_placement(Some((&[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22], 24))),
        );
        // 4
        builder.add_with_config(
            "comb_aggr_out_op_sb",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>()))
                .input_placement(Some((&(0..16).collect::<Vec<_>>(), 24)))
                .output_placement(Some((&[0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22], 24)))
                .single_buffer(true),
        );
        // 5
        builder.add_with_config(
            "comb_aggr_out_arg",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>())),
        );
        // 6 with pop_input
        builder.add_with_config(
            "comb_aggr_out_pop_in",
            circuit.clone(),
            CodeConfig::new()
                .pop_input_code(Some(comb_pop_input_code))
                .pop_input_len(Some(128 + 3))
                .pop_from_buffer(Some(&(12..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(128 + 3))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>())),
        );
        // 7 with pop_input
        builder.add_with_config(
            "comb_aggr_out_pop_in_sb",
            circuit.clone(),
            CodeConfig::new()
                .pop_input_code(Some(comb_pop_input_code))
                .pop_input_len(Some(128 + 3))
                .pop_from_buffer(Some(&(12..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(comb_aggr_output_code))
                .aggr_output_len(Some(128 + 3))
                .aggr_to_buffer(Some(&(0..12).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        // 8: with exclude_outputs
        builder.add_with_config(
            "comb_aggr_out_2_excl",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(comb_aggr_output_code_2))
                .aggr_output_len(Some(1 << (12 - 5)))
                .aggr_to_buffer(Some(&[0, 1, 2, 3, 4, 6, 7, 9, 10, 11]))
                .exclude_outputs(Some(&[0, 3, 4, 8, 9, 10, 11])),
        );
        let mut execs = builder.build().unwrap();
        let expected_counts = [
            (2048, 1536, 2048),
            (2048, 1536, 2048),
            (1, 1536, 65536),
            (2048, 3072, 2048),
            (3072, 3072, 1344),
            (1536, 1536, 2720),
            (1536, 1536, 2720),
            (1536, 1536, 2720),
            (2048, 640, 2048),
        ];
        for (i, exec) in execs.iter().enumerate() {
            assert!(exec.output_is_aggregated(), "{}: {}", config_num, i);
            assert!(exec.is_aggregated_to_buffer(), "{}: {}", config_num, i);
            assert_eq!(expected_counts[i].0, exec.input_data_len(4096));
            assert_eq!(expected_counts[i].1, exec.output_data_len(4096));
            assert_eq!(expected_counts[i].2, exec.elem_count(1024));
        }
        let expected_buffer = AGGR_OUTPUT_EXPECTED;
        let mut it = execs[0]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[0].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer = execs[0].new_data(expected_buffer.len());
        let output_circ = execs[0]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[0].new_data(expected_buffer.len());
        let mut output_circ = execs[0].new_data(output_circ.len());
        execs[0]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // with limited output
        let mut it = execs[1]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[1]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[1].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer = execs[1].new_data(expected_buffer.len());
        let output_circ = execs[1]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer_2[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[1].new_data(expected_buffer.len());
        let mut output_circ = execs[1].new_data(output_circ.len());
        execs[1]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer_2[i], *out, "{}: {}", config_num, i);
        }
        // with elem full
        let mut ot = execs[2]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[2].new_data(1);
        let mut buffer = execs[2].new_data(expected_buffer.len());
        let output_circ = execs[2]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[2].new_data(expected_buffer.len());
        let mut output_circ = execs[2].new_data(output_circ.len());
        execs[2]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // with output placement
        let mut it = execs[3]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[3]
            .output_transformer(32, &(0..24).collect::<Vec<_>>())
            .unwrap();
        let input = execs[3].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer = execs[3].new_data(expected_buffer.len());
        let output_circ = execs[3]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_op.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_op[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[3].new_data(expected_buffer.len());
        let mut output_circ = execs[3].new_data(output_circ.len());
        execs[3]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_op[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // with single_buffer
        let mut it = execs[4]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[4]
            .output_transformer(32, &(0..24).collect::<Vec<_>>())
            .unwrap();
        let input = execs[4].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let mut output_circ = it.transform(&input).unwrap();
        let mut buffer = execs[4].new_data(expected_buffer.len());
        execs[4]
            .execute_buffer_single(&mut output_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_op.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_op_2[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // arg_inputs
        let mut it = execs[5]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[5]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[5].new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer_comb = vec![0u32; expected_buffer.len()];
        for arg in 0..16 {
            let mut buffer = execs[5].new_data(expected_buffer.len());
            let output_circ = execs[5]
                .execute_buffer(&input_circ, arg, &mut buffer)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(expected_out.len() >> 4, output.len());
            let arg = usize::try_from(arg).unwrap();
            for (i, out) in output.iter().enumerate() {
                assert_eq!(expected_out[(i << 4) + arg], *out, "{}: {}", config_num, i);
            }
            let buffer = buffer.release();
            for (i, v) in buffer_comb.iter_mut().enumerate() {
                *v |= buffer[i];
            }
        }
        for (i, out) in buffer_comb.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer_comb = vec![0u32; expected_buffer.len()];
        for arg in 0..16 {
            let mut buffer = execs[5].new_data(expected_buffer.len());
            let mut output_circ = execs[5].new_data_output_elems(1 << 12);
            execs[5]
                .execute_buffer_reuse(&input_circ, arg, &mut output_circ, &mut buffer)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(expected_out.len() >> 4, output.len());
            let arg = usize::try_from(arg).unwrap();
            for (i, out) in output.iter().enumerate() {
                assert_eq!(expected_out[(i << 4) + arg], *out, "{}: {}", config_num, i);
            }
            let buffer = buffer.release();
            for (i, v) in buffer_comb.iter_mut().enumerate() {
                *v |= buffer[i];
            }
        }
        for (i, out) in buffer_comb.iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // with pop_input
        let mut it = execs[6]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[6]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[6].new_data_from_vec((0..1 << 16).map(|x| x & 0xfff).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer = execs[6].new_data_from_vec(
            std::iter::repeat(0u32)
                .take(expected_buffer.len())
                .chain(params.iter().copied())
                .collect::<Vec<_>>(),
        );
        let output_circ = execs[6]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_aggr_pop.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_aggr_pop[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer[0..expected_buffer.len()].iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // with pop_input and single_buffer
        let mut it = execs[7]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[7]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[7].new_data_from_vec((0..1 << 16).map(|x| x & 0xfff).collect::<Vec<_>>());
        let mut output_circ = it.transform(&input).unwrap();
        let mut buffer = execs[7].new_data_from_vec(
            std::iter::repeat(0u32)
                .take(expected_buffer.len())
                .chain(params.iter().copied())
                .collect::<Vec<_>>(),
        );
        execs[7]
            .execute_buffer_single(&mut output_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_aggr_pop.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_aggr_pop[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer[0..expected_buffer.len()].iter().enumerate() {
            assert_eq!(expected_buffer[i], *out, "{}: {}", config_num, i);
        }
        // 8: with limited output and excluded outputs
        // with limited output
        let mut it = execs[8]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[8]
            .output_transformer(32, &(0..5).collect::<Vec<_>>())
            .unwrap();
        let input = execs[8].new_data_from_vec((0..1 << 16).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut buffer = execs[8].new_data(expected_buffer.len());
        let output_circ = execs[8]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_excl.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_excl[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer_2[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[8].new_data(expected_buffer.len());
        let mut output_circ = execs[8].new_data(output_circ.len());
        execs[8]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(expected_out_excl.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_excl[i], *out, "{}: {}", config_num, i);
        }
        let buffer = buffer.release();
        for (i, out) in buffer.iter().enumerate() {
            assert_eq!(expected_buffer_2[i], *out, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_pop_input() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let circuit2 = Circuit::<u32>::from_str(COMB_CIRCUIT2).unwrap();
    let circuit2_out = (0..1u32 << 20)
        .map(|x| {
            let out = circuit2.eval((0..20).map(|i| ((x >> i) & 1) != 0));
            let mut y = 0;
            for (i, outb) in out.into_iter().enumerate() {
                y |= u32::from(outb) << i;
            }
            y
        })
        .collect::<Vec<_>>();
    let params = [7, 3, 19];
    let expected_out = (0..1u32 << 20)
        .map(|x| {
            let x = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xfffff;
            circuit2_out[x as usize]
        })
        .collect::<Vec<_>>();
    let expected_out_2 = (0..1u32 << 20)
        .map(|x| {
            let old_x = x;
            let x = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xffff;
            circuit2_out[(x as usize) | (((old_x & 15) << 16) as usize)]
        })
        .collect::<Vec<_>>();
    let expected_out_3 = (0..1u32 << 20)
        .map(|x| {
            let old_x = x;
            let x = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xffff;
            circuit2_out[(x as usize) | ((old_x & 0xf0000) as usize)]
        })
        .collect::<Vec<_>>();
    let expected_out_4 = (0..1u32 << 20)
        .map(|x| {
            let old_x = x;
            let new_x = x >> 4;
            let x = (new_x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((new_x << params[1]) & !params[2])
                .0)
                & 0xfff;
            circuit2_out[((x << 4) as usize) | ((old_x & 0xf000f) as usize)]
        })
        .collect::<Vec<_>>();

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let pop_input_code = r##"{
    unsigned int i;
    uint inp[(TYPE_LEN >> 5)*20];
    const global uint* params = (const global uint*)input;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < (TYPE_LEN >> 5)*20; i++)
        inp[i] = 0;
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        const uint y = (x*p0 + ((x << p1) & ~p2)) & 0xfffff;
        inp[(i>>5) + (TYPE_LEN>>5)*0] |= (((y >> 0)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*1] |= (((y >> 1)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*2] |= (((y >> 2)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*3] |= (((y >> 3)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*4] |= (((y >> 4)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*5] |= (((y >> 5)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*6] |= (((y >> 6)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*7] |= (((y >> 7)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*8] |= (((y >> 8)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*9] |= (((y >> 9)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*10] |= (((y >> 10)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*11] |= (((y >> 11)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*12] |= (((y >> 12)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*13] |= (((y >> 13)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*14] |= (((y >> 14)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*15] |= (((y >> 15)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*16] |= (((y >> 16)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*17] |= (((y >> 17)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*18] |= (((y >> 18)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*19] |= (((y >> 19)&1) << (i&31));
    }
    SET_U32_ALL(i0, inp + 0*(TYPE_LEN>>5));
    SET_U32_ALL(i1, inp + 1*(TYPE_LEN>>5));
    SET_U32_ALL(i2, inp + 2*(TYPE_LEN>>5));
    SET_U32_ALL(i3, inp + 3*(TYPE_LEN>>5));
    SET_U32_ALL(i4, inp + 4*(TYPE_LEN>>5));
    SET_U32_ALL(i5, inp + 5*(TYPE_LEN>>5));
    SET_U32_ALL(i6, inp + 6*(TYPE_LEN>>5));
    SET_U32_ALL(i7, inp + 7*(TYPE_LEN>>5));
    SET_U32_ALL(i8, inp + 8*(TYPE_LEN>>5));
    SET_U32_ALL(i9, inp + 9*(TYPE_LEN>>5));
    SET_U32_ALL(i10, inp + 10*(TYPE_LEN>>5));
    SET_U32_ALL(i11, inp + 11*(TYPE_LEN>>5));
    SET_U32_ALL(i12, inp + 12*(TYPE_LEN>>5));
    SET_U32_ALL(i13, inp + 13*(TYPE_LEN>>5));
    SET_U32_ALL(i14, inp + 14*(TYPE_LEN>>5));
    SET_U32_ALL(i15, inp + 15*(TYPE_LEN>>5));
    SET_U32_ALL(i16, inp + 16*(TYPE_LEN>>5));
    SET_U32_ALL(i17, inp + 17*(TYPE_LEN>>5));
    SET_U32_ALL(i18, inp + 18*(TYPE_LEN>>5));
    SET_U32_ALL(i19, inp + 19*(TYPE_LEN>>5));
}"##;
        let pop_input_code_2 = r##"{
    unsigned int i;
    uint inp[(TYPE_LEN >> 5)*16];
    const global uint* params = (const global uint*)input;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < (TYPE_LEN >> 5)*16; i++)
        inp[i] = 0;
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        const uint y = (x*p0 + ((x << p1) & ~p2)) & 0xffff;
        inp[(i>>5) + (TYPE_LEN>>5)*0] |= (((y >> 0)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*1] |= (((y >> 1)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*2] |= (((y >> 2)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*3] |= (((y >> 3)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*4] |= (((y >> 4)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*5] |= (((y >> 5)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*6] |= (((y >> 6)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*7] |= (((y >> 7)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*8] |= (((y >> 8)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*9] |= (((y >> 9)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*10] |= (((y >> 10)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*11] |= (((y >> 11)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*12] |= (((y >> 12)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*13] |= (((y >> 13)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*14] |= (((y >> 14)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*15] |= (((y >> 15)&1) << (i&31));
    }
    SET_U32_ALL(i0, inp + 0*(TYPE_LEN>>5));
    SET_U32_ALL(i1, inp + 1*(TYPE_LEN>>5));
    SET_U32_ALL(i2, inp + 2*(TYPE_LEN>>5));
    SET_U32_ALL(i3, inp + 3*(TYPE_LEN>>5));
    SET_U32_ALL(i4, inp + 4*(TYPE_LEN>>5));
    SET_U32_ALL(i5, inp + 5*(TYPE_LEN>>5));
    SET_U32_ALL(i6, inp + 6*(TYPE_LEN>>5));
    SET_U32_ALL(i7, inp + 7*(TYPE_LEN>>5));
    SET_U32_ALL(i8, inp + 8*(TYPE_LEN>>5));
    SET_U32_ALL(i9, inp + 9*(TYPE_LEN>>5));
    SET_U32_ALL(i10, inp + 10*(TYPE_LEN>>5));
    SET_U32_ALL(i11, inp + 11*(TYPE_LEN>>5));
    SET_U32_ALL(i12, inp + 12*(TYPE_LEN>>5));
    SET_U32_ALL(i13, inp + 13*(TYPE_LEN>>5));
    SET_U32_ALL(i14, inp + 14*(TYPE_LEN>>5));
    SET_U32_ALL(i15, inp + 15*(TYPE_LEN>>5));
}"##;
        let pop_input_code_3 = r##"{
    unsigned int i;
    uint inp[(TYPE_LEN >> 5)*12];
    const global uint* params = (const global uint*)input;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < (TYPE_LEN >> 5)*12; i++)
        inp[i] = 0;
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = (idx*TYPE_LEN + i) >> 4;
        const uint y = (x*p0 + ((x << p1) & ~p2)) & 0xfff;
        inp[(i>>5) + (TYPE_LEN>>5)*0] |= (((y >> 0)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*1] |= (((y >> 1)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*2] |= (((y >> 2)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*3] |= (((y >> 3)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*4] |= (((y >> 4)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*5] |= (((y >> 5)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*6] |= (((y >> 6)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*7] |= (((y >> 7)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*8] |= (((y >> 8)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*9] |= (((y >> 9)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*10] |= (((y >> 10)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*11] |= (((y >> 11)&1) << (i&31));
    }
    SET_U32_ALL(i4, inp + 0*(TYPE_LEN>>5));
    SET_U32_ALL(i5, inp + 1*(TYPE_LEN>>5));
    SET_U32_ALL(i6, inp + 2*(TYPE_LEN>>5));
    SET_U32_ALL(i7, inp + 3*(TYPE_LEN>>5));
    SET_U32_ALL(i8, inp + 4*(TYPE_LEN>>5));
    SET_U32_ALL(i9, inp + 5*(TYPE_LEN>>5));
    SET_U32_ALL(i10, inp + 6*(TYPE_LEN>>5));
    SET_U32_ALL(i11, inp + 7*(TYPE_LEN>>5));
    SET_U32_ALL(i12, inp + 8*(TYPE_LEN>>5));
    SET_U32_ALL(i13, inp + 9*(TYPE_LEN>>5));
    SET_U32_ALL(i14, inp + 10*(TYPE_LEN>>5));
    SET_U32_ALL(i15, inp + 11*(TYPE_LEN>>5));
}"##;
        let pop_input_code_aggr = r##"{
    unsigned int i;
    uint inp[(TYPE_LEN >> 5)*20];
    const global uint* params = (const global uint*)input;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < (TYPE_LEN >> 5)*20; i++)
        inp[i] = 0;
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        const uint y = ((x*p0 & 0xea1b) + ((x << p1) & ~p2)) & 0xfffff;
        inp[(i>>5) + (TYPE_LEN>>5)*0] |= (((y >> 0)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*1] |= (((y >> 1)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*2] |= (((y >> 2)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*3] |= (((y >> 3)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*4] |= (((y >> 4)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*5] |= (((y >> 5)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*6] |= (((y >> 6)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*7] |= (((y >> 7)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*8] |= (((y >> 8)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*9] |= (((y >> 9)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*10] |= (((y >> 10)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*11] |= (((y >> 11)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*12] |= (((y >> 12)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*13] |= (((y >> 13)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*14] |= (((y >> 14)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*15] |= (((y >> 15)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*16] |= (((y >> 16)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*17] |= (((y >> 17)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*18] |= (((y >> 18)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*19] |= (((y >> 19)&1) << (i&31));
    }
    SET_U32_ALL(i0, inp + 0*(TYPE_LEN>>5));
    SET_U32_ALL(i1, inp + 1*(TYPE_LEN>>5));
    SET_U32_ALL(i2, inp + 2*(TYPE_LEN>>5));
    SET_U32_ALL(i3, inp + 3*(TYPE_LEN>>5));
    SET_U32_ALL(i4, inp + 4*(TYPE_LEN>>5));
    SET_U32_ALL(i5, inp + 5*(TYPE_LEN>>5));
    SET_U32_ALL(i6, inp + 6*(TYPE_LEN>>5));
    SET_U32_ALL(i7, inp + 7*(TYPE_LEN>>5));
    SET_U32_ALL(i8, inp + 8*(TYPE_LEN>>5));
    SET_U32_ALL(i9, inp + 9*(TYPE_LEN>>5));
    SET_U32_ALL(i10, inp + 10*(TYPE_LEN>>5));
    SET_U32_ALL(i11, inp + 11*(TYPE_LEN>>5));
    SET_U32_ALL(i12, inp + 12*(TYPE_LEN>>5));
    SET_U32_ALL(i13, inp + 13*(TYPE_LEN>>5));
    SET_U32_ALL(i14, inp + 14*(TYPE_LEN>>5));
    SET_U32_ALL(i15, inp + 15*(TYPE_LEN>>5));
    SET_U32_ALL(i16, inp + 16*(TYPE_LEN>>5));
    SET_U32_ALL(i17, inp + 17*(TYPE_LEN>>5));
    SET_U32_ALL(i18, inp + 18*(TYPE_LEN>>5));
    SET_U32_ALL(i19, inp + 19*(TYPE_LEN>>5));
}"##;
        let pop_input_code_aggr_sb = r##"{
    unsigned int i;
    uint inp[(TYPE_LEN >> 5)*20];
    const global uint* params = (const global uint*)output;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < (TYPE_LEN >> 5)*20; i++)
        inp[i] = 0;
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        const uint y = ((x*p0 & 0xea1b) + ((x << p1) & ~p2)) & 0xfffff;
        inp[(i>>5) + (TYPE_LEN>>5)*0] |= (((y >> 0)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*1] |= (((y >> 1)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*2] |= (((y >> 2)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*3] |= (((y >> 3)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*4] |= (((y >> 4)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*5] |= (((y >> 5)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*6] |= (((y >> 6)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*7] |= (((y >> 7)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*8] |= (((y >> 8)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*9] |= (((y >> 9)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*10] |= (((y >> 10)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*11] |= (((y >> 11)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*12] |= (((y >> 12)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*13] |= (((y >> 13)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*14] |= (((y >> 14)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*15] |= (((y >> 15)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*16] |= (((y >> 16)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*17] |= (((y >> 17)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*18] |= (((y >> 18)&1) << (i&31));
        inp[(i>>5) + (TYPE_LEN>>5)*19] |= (((y >> 19)&1) << (i&31));
    }
    SET_U32_ALL(i0, inp + 0*(TYPE_LEN>>5));
    SET_U32_ALL(i1, inp + 1*(TYPE_LEN>>5));
    SET_U32_ALL(i2, inp + 2*(TYPE_LEN>>5));
    SET_U32_ALL(i3, inp + 3*(TYPE_LEN>>5));
    SET_U32_ALL(i4, inp + 4*(TYPE_LEN>>5));
    SET_U32_ALL(i5, inp + 5*(TYPE_LEN>>5));
    SET_U32_ALL(i6, inp + 6*(TYPE_LEN>>5));
    SET_U32_ALL(i7, inp + 7*(TYPE_LEN>>5));
    SET_U32_ALL(i8, inp + 8*(TYPE_LEN>>5));
    SET_U32_ALL(i9, inp + 9*(TYPE_LEN>>5));
    SET_U32_ALL(i10, inp + 10*(TYPE_LEN>>5));
    SET_U32_ALL(i11, inp + 11*(TYPE_LEN>>5));
    SET_U32_ALL(i12, inp + 12*(TYPE_LEN>>5));
    SET_U32_ALL(i13, inp + 13*(TYPE_LEN>>5));
    SET_U32_ALL(i14, inp + 14*(TYPE_LEN>>5));
    SET_U32_ALL(i15, inp + 15*(TYPE_LEN>>5));
    SET_U32_ALL(i16, inp + 16*(TYPE_LEN>>5));
    SET_U32_ALL(i17, inp + 17*(TYPE_LEN>>5));
    SET_U32_ALL(i18, inp + 18*(TYPE_LEN>>5));
    SET_U32_ALL(i19, inp + 19*(TYPE_LEN>>5));
}"##;
        let fixed_aggr_output_code = r##"{
    unsigned int i;
    uint out[(TYPE_LEN >> 5)*12];
    global uint* output_u32 = ((global uint*)output) + 3;
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
        builder.add_with_config(
            "comb_pop_in",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code))
                .pop_input_len(Some(3)),
        );
        builder.add_with_config(
            "comb_pop_in_1",
            circuit2.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(16..20).collect::<Vec<_>>()))
                .pop_input_code(Some(pop_input_code_2))
                .pop_input_len(Some(3)),
        );
        builder.add_with_config(
            "comb_pop_in_2",
            circuit2.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(16..20).collect::<Vec<_>>()))
                .pop_input_code(Some(pop_input_code_2))
                .pop_input_len(Some(3)),
        );
        builder.add_with_config(
            "comb_pop_in_3",
            circuit2.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(16..20).collect::<Vec<_>>()))
                .elem_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .pop_input_code(Some(pop_input_code_3))
                .pop_input_len(Some(3)),
        );
        // 4
        builder.add_with_config(
            "comb_pop_in_aggr_out",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code_aggr))
                .pop_input_len(Some(3))
                .aggr_output_code(Some(COMB_AGGR_OUTPUT_CODE))
                .aggr_output_len(Some(128)),
        );
        // 5
        builder.add_with_config(
            "comb_pop_in_aggr_out_sb",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code_aggr_sb))
                .pop_input_len(Some(128 + 3))
                .aggr_output_code(Some(fixed_aggr_output_code))
                .aggr_output_len(Some(128 + 3))
                .single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        for (i, exec) in execs[0..5].iter().enumerate() {
            assert_eq!(exec.input_data_len(12 * 512), 3, "{}: {}", config_num, i);
            assert!(exec.input_is_populated(), "{}: {}", config_num, i);
            assert_eq!(exec.pop_input_len(), Some(3), "{}: {}", config_num, i);
            assert!(!exec.is_populated_from_buffer(), "{}: {}", config_num, i);
        }
        assert_eq!(execs[5].input_data_len(12 * 512), 131, "{}", config_num);
        assert!(execs[5].input_is_populated(), "{}", config_num);
        assert_eq!(execs[5].pop_input_len(), Some(131), "{}", config_num);
        assert!(!execs[5].is_populated_from_buffer(), "{}", config_num);
        assert_eq!(execs[0].elem_count(111), 1 << 20);
        assert_eq!(execs[1].elem_count(111), 1 << 20);
        assert_eq!(execs[2].elem_count(111), 1 << 16);
        assert_eq!(execs[3].elem_count(111), 1 << 16);
        assert_eq!(execs[4].elem_count(111), 1 << 20);
        assert_eq!(execs[5].elem_count(111), 1 << 20);
        assert_eq!(execs[0].output_data_len(12 * 512), 2304);
        assert_eq!(execs[1].output_data_len(12 * 512), 2304);
        assert_eq!(execs[2].output_data_len(12 * 512), 2304);
        assert_eq!(execs[3].output_data_len(12 * 512), 2304);
        assert_eq!(execs[4].output_data_len(12 * 512), 128);
        assert_eq!(execs[5].output_data_len(12 * 512), 131);

        // tests
        let mut ot = execs[0]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[0].new_data_from_slice(&params[..]);
        let output_circ = execs[0].execute(&input_circ, 0).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output_circ = execs[0].new_data(output_circ.len());
        execs[0]
            .execute_reuse(&input_circ, 0, &mut output_circ)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        // tests
        let mut ot = execs[1]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[1].new_data_from_slice(&params[..]);
        let output_circ = execs[1].execute(&input_circ, 0).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_2[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output_circ = execs[1].new_data(output_circ.len());
        execs[1]
            .execute_reuse(&input_circ, 0, &mut output_circ)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_2[i], *out, "{}: {}", config_num, i);
        }
        // arg_inputs
        let mut ot = execs[2]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[2].new_data_from_slice(&params[..]);
        for arg in 0..16 {
            let output_circ = execs[2].execute(&input_circ, arg).unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out_3[((arg << 16) as usize) + i],
                    *out,
                    "{} {}: {}",
                    config_num,
                    arg,
                    i
                );
            }
            // reuse
            let mut output_circ = execs[2].new_data(output_circ.len());
            execs[2]
                .execute_reuse(&input_circ, arg, &mut output_circ)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out_3[((arg << 16) as usize) + i],
                    *out,
                    "{} {}: {}",
                    config_num,
                    arg,
                    i
                );
            }
        }
        // arg_inputs with elem_inputs
        let mut ot = execs[3]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[3].new_data_from_slice(&params[..]);
        for arg in 0..16 {
            let output_circ = execs[3].execute(&input_circ, arg).unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out_4[((arg << 16) as usize) + i],
                    *out,
                    "{} {}: {}",
                    config_num,
                    arg,
                    i
                );
            }
            // reuse
            let mut output_circ = execs[3].new_data(output_circ.len());
            execs[3]
                .execute_reuse(&input_circ, arg, &mut output_circ)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out_4[((arg << 16) as usize) + i],
                    *out,
                    "{} {}: {}",
                    config_num,
                    arg,
                    i
                );
            }
        }
        // with aggr_output
        let expected_out_aggr = [
            4294967295, 4294967295, 4294967295, 2684354559, 4294967295, 1073692671, 4294934527,
            2612510719, 4294967295, 2139095039, 2147483647, 1073725439, 2147467263, 1040130047,
            2145097599, 1058510719, 4294967295, 3219128191, 1610612735, 3017801599, 1073709055,
            2550112095, 1064779775, 3084098399, 4286578687, 966524767, 2139092991, 563320671,
            796868607, 857446231, 1062671351, 293606751, 4294967295, 4294967295, 4286578687,
            297762815, 1073692671, 3212820351, 2139060223, 299188147, 4253024255, 2000682879,
            2139092991, 1096794047, 1040138239, 1065295743, 2136150911, 21042999, 2147483647,
            3084869503, 1535115263, 295696223, 536821759, 2541723487, 930559999, 294719253,
            4286562303, 698081119, 1870100479, 26448735, 224083903, 589005591, 605229879,
            293601299, 4294967295, 2139095039, 939524095, 1472184319, 1069498367, 2549858303,
            4292870111, 2337774999, 4294967295, 2105540607, 931133439, 2009020407, 1069498367,
            932385791, 2144437215, 257394007, 2684352511, 926908287, 931133439, 860829567,
            2146910207, 856887135, 1331525591, 71505495, 394231807, 658220895, 897398655,
            286463839, 1435962367, 622002975, 253563231, 67244119, 536870911, 528482303, 868204543,
            1360213951, 2012692479, 1058478463, 2199903583, 5308743, 366968831, 225653759,
            1906835263, 286491455, 934478847, 756489083, 190415135, 1118471, 2684354559, 125261695,
            1665094655, 286508831, 1400846335, 38996823, 155802947, 268439623, 91183103, 33625423,
            557139455, 286425361, 1444348799, 621937239, 155783451, 4167,
        ];

        let input_circ = execs[4].new_data_from_slice(&params[..]);
        let output = execs[4].execute(&input_circ, 0).unwrap().release();
        assert_eq!(expected_out_aggr.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_aggr[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut output = execs[4].new_data(output.len());
        execs[4].execute_reuse(&input_circ, 0, &mut output).unwrap();
        let output = output.release();
        assert_eq!(expected_out_aggr.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_aggr[i], *out, "{}: {}", config_num, i);
        }
        // with aggr_output with single buffer
        let mut params_extended = vec![0u32; 128 + 3];
        params_extended[0..3].copy_from_slice(&params[..]);
        let mut output = execs[5].new_data_from_vec(params_extended);
        execs[5].execute_single(&mut output, 0).unwrap();
        let output = output.release();
        assert_eq!(expected_out_aggr.len() + 3, output.len());
        assert_eq!(&output[0..3], &params[..]);
        for (i, out) in output[3..].iter().enumerate() {
            assert_eq!(expected_out_aggr[i], *out, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_pop_input_with_helpers() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let params = [7, 3, 19];
    let circuit2 = Circuit::<u32>::from_str(COMB_CIRCUIT2).unwrap();
    let circuit2_out = (0..1u32 << 20)
        .map(|x| {
            let out = circuit2.eval((0..20).map(|i| ((x >> i) & 1) != 0));
            let mut y = 0;
            for (i, outb) in out.into_iter().enumerate() {
                y |= u32::from(outb) << i;
            }
            y
        })
        .collect::<Vec<_>>();
    let expected_out = (0..1u32 << 20)
        .map(|x| {
            let x = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xfffff;
            circuit2_out[x as usize]
        })
        .collect::<Vec<_>>();
    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        println!("Config: {}", config_num);
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let pop_input_code = r##"{
    unsigned int i;
    uint inp[TYPE_LEN];
    const global uint* params = (const global uint*)input;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        inp[i] = (x*p0 + ((x << p1) & ~p2)) & 0xfffff;
    }
    INPUT_TRANSFORM_B20(i0, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11,
            i12, i13, i14, i15, i16, i17, i18, i19, inp);
}"##;
        builder.transform_helpers();
        builder.add_with_config(
            "comb_pop_in",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code))
                .pop_input_len(Some(3)),
        );
        let mut execs = builder.build().unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[0].new_data_from_slice(&params[..]);
        let output_circ = execs[0].execute(&input_circ, 0).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_opencl_builder_and_exec_with_pop_from_buffer() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    let circuit2 = Circuit::<u32>::from_str(COMB_CIRCUIT2).unwrap();
    let circuit2_out = (0..1u32 << 20)
        .map(|x| {
            let out = circuit2.eval((0..20).map(|i| ((x >> i) & 1) != 0));
            let mut y = 0;
            for (i, outb) in out.into_iter().enumerate() {
                y |= u32::from(outb) << i;
            }
            y
        })
        .collect::<Vec<_>>();
    let params = [7, 3, 19];
    let expected_out = (0..1u32 << 20)
        .map(|x| {
            let newx = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xffff;
            let idx = (newx << 2) | ((x >> 16) & 3) | (x & 0xc0000);
            circuit2_out[idx as usize]
        })
        .collect::<Vec<_>>();
    let expected_out_2 = (0..1u32 << 16)
        .map(|x| {
            let idx = (x & 0xffff) | ((((params[0] + params[1]) ^ params[2]) & 15) << 16);
            circuit2_out[idx as usize]
        })
        .collect::<Vec<_>>();
    let expected_out_3 = (0..1u32 << 20)
        .map(|x| {
            let x = (x
                .overflowing_mul(params[0])
                .0
                .overflowing_add((x << params[1]) & !params[2])
                .0)
                & 0xfffff;
            circuit2_out[x as usize]
        })
        .collect::<Vec<_>>();
    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let pop_input_code = r##"{
    unsigned int i;
    uint inp[TYPE_LEN];
    const global uint* params = (const global uint*)buffer;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        inp[i] = (x*p0 + ((x << p1) & ~p2)) & 0xffff;
    }
    INPUT_TRANSFORM_B16(i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13,
            i14, i15, i16, i17, inp);
}"##;
        let pop_input_code_2 = r##"{
    unsigned int i;
    uint inp[TYPE_LEN];
    const global uint* params = (const global uint*)buffer;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < TYPE_LEN; i++) {
        inp[i] = ((p0 + p1) ^ p2) & 15;
    }
    INPUT_TRANSFORM_B4(i16, i17, i18, i19, inp);
}"##;
        let pop_input_code_3 = r##"{
    unsigned int i;
    uint inp[TYPE_LEN];
    const global uint* params = (const global uint*)buffer;
    const uint p0 = params[0];
    const uint p1 = params[1];
    const uint p2 = params[2];
    for (i = 0; i < TYPE_LEN; i++) {
        const uint x = idx*TYPE_LEN + i;
        inp[i] = (x*p0 + ((x << p1) & ~p2)) & 0xfffff;
    }
    INPUT_TRANSFORM_B20(i0, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11,
            i12, i13, i14, i15, i16, i17, i18, i19, inp);
}"##;
        builder.transform_helpers();
        builder.add_with_config(
            "comb_pop_from_buffer",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(2..18).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "comb_pop_from_buffer_2",
            circuit2.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..16).collect::<Vec<_>>()))
                .pop_input_code(Some(pop_input_code_2))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(16..20).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "comb_pop_from_buffer_3",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code_3))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(0..20).collect::<Vec<_>>())),
        );
        builder.add_with_config(
            "comb_pop_from_buffer_4",
            circuit2.clone(),
            CodeConfig::new()
                .pop_input_code(Some(pop_input_code))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(2..18).collect::<Vec<_>>()))
                .input_placement(Some((&[0, 1, 2, 3], 12)))
                .single_buffer(true),
        );
        // 4: arg_input
        builder.add_with_config(
            "comb_pop_from_buffer_arg_1",
            circuit2.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&[0, 1, 18, 19]))
                .pop_input_code(Some(pop_input_code))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(2..18).collect::<Vec<_>>())),
        );
        // 5: arg input with single buffer (based on comb_pop_from_buffer_2)
        builder.add_with_config(
            "comb_pop_from_buffer_arg_2",
            circuit2.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(12..16).collect::<Vec<_>>()))
                .pop_input_code(Some(pop_input_code_2))
                .pop_input_len(Some(3))
                .pop_from_buffer(Some(&(16..20).collect::<Vec<_>>()))
                .single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        let expected_counts = [
            (512, 1536, 8192),
            (1, 1536, 65536),
            (1, 1536, 1048576),
            (1536, 1536, 2720),
            (1, 1536, 65536),
            (1536, 1536, 2720),
        ];
        for (i, exec) in execs.iter().enumerate() {
            assert!(exec.input_is_populated(), "{}: {}", config_num, i);
            assert!(exec.is_populated_from_buffer(), "{}: {}", config_num, i);
            assert_eq!(expected_counts[i].0, exec.input_data_len(4096));
            assert_eq!(expected_counts[i].1, exec.output_data_len(4096));
            assert_eq!(expected_counts[i].2, exec.elem_count(1024));
        }
        // first exec
        let mut it = execs[0].input_transformer(32, &[0, 1, 2, 3]).unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut buffer = execs[0].new_data_from_slice(&params[..]);
        let input = execs[0].new_data_from_vec(
            (0..1 << 20)
                .map(|i| (i & 0xf0000) >> 16)
                .collect::<Vec<_>>(),
        );
        let input_circ = it.transform(&input).unwrap();
        let output_circ = execs[0]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[0].new_data_from_slice(&params[..]);
        let mut output_circ = execs[0].new_data(output_circ.len());
        execs[0]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        // second exec
        let mut ot = execs[1]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut buffer = execs[1].new_data_from_slice(&params[..]);
        let input_circ = execs[1].new_data(1);
        let output_circ = execs[1]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 16, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_2[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[1].new_data_from_slice(&params[..]);
        let mut output_circ = execs[1].new_data(output_circ.len());
        execs[1]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 16, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_2[i], *out, "{}: {}", config_num, i);
        }
        // third exec
        let mut ot = execs[2]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input_circ = execs[2].new_data(1);
        let mut buffer = execs[2].new_data_from_slice(&params[..]);
        let output_circ = execs[2]
            .execute_buffer(&input_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_3[i], *out, "{}: {}", config_num, i);
        }
        // reuse
        let mut buffer = execs[2].new_data_from_slice(&params[..]);
        let mut output_circ = execs[2].new_data(output_circ.len());
        execs[2]
            .execute_buffer_reuse(&input_circ, 0, &mut output_circ, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out_3[i], *out, "{}: {}", config_num, i);
        }
        // with single buffer
        let mut it = execs[3].input_transformer(32, &[0, 1, 2, 3]).unwrap();
        let mut ot = execs[3]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut buffer = execs[3].new_data_from_slice(&params[..]);
        let input = execs[3].new_data_from_vec(
            (0..1 << 20)
                .map(|i| (i & 0xf0000) >> 16)
                .collect::<Vec<_>>(),
        );
        let mut output_circ = it.transform(&input).unwrap();
        execs[3]
            .execute_buffer_single(&mut output_circ, 0, &mut buffer)
            .unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(1 << 20, output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(expected_out[i], *out, "{}: {}", config_num, i);
        }
        // arg_inputs
        let mut ot = execs[4]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        for arg in 0usize..16 {
            let mut buffer = execs[4].new_data_from_slice(&params[..]);
            let input_circ = execs[4].new_data(1);
            let output_circ = execs[4]
                .execute_buffer(&input_circ, arg as u64, &mut buffer)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out[(arg << 16) + i],
                    *out,
                    "{}: {} {}",
                    config_num,
                    arg,
                    i
                );
            }
            // reuse
            let mut buffer = execs[4].new_data_from_slice(&params[..]);
            let mut output_circ = execs[4].new_data(output_circ.len());
            execs[4]
                .execute_buffer_reuse(&input_circ, arg as u64, &mut output_circ, &mut buffer)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 16, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out[(arg << 16) + i],
                    *out,
                    "{}: {} {}",
                    config_num,
                    arg,
                    i
                );
            }
        }
        // arg_inputs 2
        let mut it = execs[5]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[5]
            .output_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[5].new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
        for arg in 0usize..16 {
            let mut buffer = execs[5].new_data_from_slice(&params[..]);
            let mut output_circ = it.transform(&input).unwrap();
            execs[5]
                .execute_buffer_single(&mut output_circ, arg as u64, &mut buffer)
                .unwrap();
            let output = ot.transform(&output_circ).unwrap().release();
            assert_eq!(1 << 12, output.len());
            for (i, out) in output.iter().enumerate() {
                assert_eq!(
                    expected_out_2[(arg << 12) + i],
                    *out,
                    "{}: {} {}",
                    config_num,
                    arg,
                    i
                );
            }
        }
    }
}

#[test]
fn test_opencl_data_holder() {
    let no_opt_neg_config = OpenCLBuilderConfig::new();
    let opt_neg_config = OpenCLBuilderConfig::new().optimize_negs(true);

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let circuit =
            Circuit::new(4, [], [(0, false), (1, false), (2, false), (3, false)]).unwrap();
        builder.add("mul2x2", circuit.clone(), None, None, None);
        let circuit = Circuit::new(4, [], [(0, true), (1, true), (2, true), (3, true)]).unwrap();
        builder.add_with_config("mul2x2sb", circuit, CodeConfig::new().single_buffer(true));
        let circuit = Circuit::new(
            8,
            [
                Gate::new_xor(0, 4),
                Gate::new_xor(1, 5),
                Gate::new_xor(2, 6),
                Gate::new_xor(3, 7),
            ],
            [(8, false), (9, false), (10, false), (11, false)],
        )
        .unwrap();
        builder.add(
            "mul2x2arg",
            circuit.clone(),
            None,
            None,
            Some(&[4, 5, 6, 7]),
        );
        builder.add_with_config(
            "mul2x2argsb",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&[4, 5, 6, 7]))
                .single_buffer(true),
        );
        let mut execs = builder.build().unwrap();
        let mut data = execs[0].new_data(10);
        {
            let mut wr = data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 111).unwrap();
            }
        }
        {
            let rd = data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(
                    u32::try_from(i * 111).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        {
            let cmd_queue = unsafe { execs[0].command_queue() };
            let mut out = vec![0u32; 10];
            unsafe {
                cmd_queue
                    .enqueue_read_buffer(data.buffer(), CL_BLOCKING, 0, &mut out[..], &[])
                    .unwrap();
            }
            for (i, x) in out.into_iter().enumerate() {
                assert_eq!(
                    u32::try_from(i * 111).unwrap(),
                    x,
                    "2: {} {}",
                    config_num,
                    i
                );
            }
        }
        data.set_range(2..8);
        {
            let rd = data.get();
            assert_eq!(rd.get().len(), 6);
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(
                    u32::try_from((i + 2) * 111).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        {
            let mut rd = data.get_mut();
            assert_eq!(rd.get_mut().len(), 6);
            for (i, x) in rd.get_mut().iter().enumerate() {
                assert_eq!(
                    u32::try_from((i + 2) * 111).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        data.set_range_from(3..);
        {
            let rd = data.get();
            assert_eq!(rd.get().len(), 7);
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(
                    u32::try_from((i + 3) * 111).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        {
            let mut rd = data.get_mut();
            assert_eq!(rd.get_mut().len(), 7);
            for (i, x) in rd.get_mut().iter().enumerate() {
                assert_eq!(
                    u32::try_from((i + 3) * 111).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        data.set_range(7..2);
        {
            let rd = data.get();
            assert_eq!(rd.get().len(), 0);
        }
        {
            let mut rd = data.get_mut();
            assert_eq!(rd.get_mut().len(), 0);
        }

        // test executor
        let mut input = execs[0].new_data_from_vec(vec![0, 0, 0, 0x11, 0x22, 0x4400, 0x660000]);
        input.set_range(3..7);
        {
            let output = execs[0].execute(&input, 0).unwrap();
            let rd = output.get();
            let output = rd.get();
            assert_eq!([17, 34, 17408, 6684672], output);
        };
        data.set_range(4..8);
        execs[0].execute_reuse(&input, 0, &mut data).unwrap();
        {
            data.set_range_from(0..);
            let rd = data.get();
            let output = rd.get();
            assert_eq!([0, 111, 222, 333, 17, 34, 17408, 6684672, 888, 999], output);
        }
        data.set_range(5..9);
        execs[1].execute_single(&mut data, 0).unwrap();
        {
            data.set_range_from(0..);
            let rd = data.get();
            let output = rd.get();
            assert_eq!(
                [0, 111, 222, 333, 17, !34, !17408, !6684672, !888, 999],
                output
            );
        }
        // with arg_input
        let mut input = execs[2].new_data_from_vec(vec![0, 0x11, 0x22, 0x4400, 0x660000, 0, 0, 0]);
        input.set_range(1..5);
        {
            let output = execs[2].execute(&input, 0b1011).unwrap();
            let rd = output.get();
            let output = rd.get();
            assert_eq!([!17, !34, 17408, !6684672], output);
        };
        let mut input = execs[2].new_data_from_vec(vec![4, 56, 23, 212, 11, 55, 77, 11, 542]);
        let mut output = execs[2].new_data_from_vec(vec![9, 7, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0]);
        input.set_range(3..7);
        output.set_range(6..10);
        execs[2].execute_reuse(&input, 0b1101, &mut output).unwrap();
        {
            output.set_range_from(0..);
            let rd = output.get();
            let output = rd.get();
            assert_eq!([9, 7, 0, 0, 0, 0, !212, 11, !55, !77, 0, 0], output);
        }
        let mut data = execs[2].new_data_from_vec(vec![
            77, 33, 11, 923, 13, 456, 951, 21, 11, 561, 103, 34, 833, 221, 895,
        ]);
        data.set_range(9..13);
        execs[3].execute_single(&mut data, 0b1001).unwrap();
        {
            data.set_range_from(0..);
            let rd = data.get();
            let output = rd.get();
            assert_eq!(
                [77, 33, 11, 923, 13, 456, 951, 21, 11, !561, 103, 34, !833, 221, 895],
                output
            );
        }

        // test new from slice
        let array = [3, 5, 2, 11, 581, 521];
        let mut data = execs[0].new_data_from_slice(&array[..]);
        {
            let rd = data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(array[i], *x, "1: {} {}", config_num, i);
            }
        }

        // test copy
        let data2 = data.copy();
        {
            let rd = data2.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(array[i], *x, "1: {} {}", config_num, i);
            }
        }
        // test copy of range
        data.set_range(1..5);
        let data2 = data.copy();
        {
            assert_eq!(data2.len(), 4);
            let rd = data2.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(array[i + 1], *x, "1: {} {}", config_num, i);
            }
        }

        // process tests
        let mut data = execs[0].new_data(10);
        {
            let mut wr = data.get_mut();
            for (i, x) in wr.get_mut().iter_mut().enumerate() {
                *x = u32::try_from(i * 337).unwrap();
            }
        }
        assert!(data.process(|d| d
            .iter()
            .enumerate()
            .all(|(i, v)| *v == u32::try_from(i * 337).unwrap())));
        // process_mut tests
        data.process_mut(|d| {
            for (i, v) in d.iter_mut().enumerate() {
                *v *= 7 * u32::try_from(i).unwrap();
            }
        });
        {
            let rd = data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(
                    u32::try_from(7 * 337 * i * i).unwrap(),
                    *x,
                    "1: {} {}",
                    config_num,
                    i
                );
            }
        }
        data.set_range(1..7);
        assert!(data.process(|d| d
            .iter()
            .enumerate()
            .all(|(i, v)| *v == u32::try_from((i + 1) * (i + 1) * 7 * 337).unwrap())));
        data.set_range(2..8);
        data.process_mut(|d| {
            for (i, v) in d.iter_mut().enumerate() {
                *v += 5 * u32::try_from(i).unwrap()
            }
        });
        data.set_range_from(0..);
        {
            let rd = data.get();
            for (i, x) in rd.get().iter().enumerate() {
                let mut exp = u32::try_from(7 * 337 * i * i).unwrap();
                if i >= 2 && i < 8 {
                    exp += 5 * u32::try_from(i - 2).unwrap()
                }
                assert_eq!(exp, *x, "1: {} {}", config_num, i);
            }
        }
    }
}

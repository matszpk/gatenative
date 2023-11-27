use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::*;
use gatesim::*;

use std::collections::HashMap;

fn gen_mul_add_input(word_len: usize) -> (CPUDataHolder, Vec<u32>) {
    let input_map = [
        0, 4, 7, 10, 13, 16, 19, 22, 1, 3, 6, 9, 12, 15, 18, 21, 2, 5, 8, 11, 14, 17, 20, 23,
    ];
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
                    let ix = input_map[bit];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((a >> bit) & 1) << shift;
                    let ix = input_map[bit + 8];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((b >> bit) & 1) << shift;
                    let ix = input_map[bit + 16];
                    input[idx * 24 * word_len + ix * word_len + half_idx] |=
                        ((c >> bit) & 1) << shift;
                    exp_output[idx * 8 * word_len + bit * word_len + half_idx] |=
                        ((exp >> bit) & 1) << shift;
                }
            }
        }
    }
    (CPUDataHolder::new(input), exp_output)
}

#[test]
fn test_cpu_builder_and_exec() {
    use CPUExtension::*;
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
    };

    let mut mul_add_data_map = HashMap::<usize, (CPUDataHolder, Vec<u32>)>::new();
    mul_add_data_map.insert(2, gen_mul_add_input(2));
    let mut configs = vec![
        (NoExtension, &CLANG_WRITER_U64_TEST_IMPL, None),
        (NoExtension, &CLANG_WRITER_U64_TEST_NIMPL, None),
        (NoExtension, &CLANG_WRITER_U64, Some(no_opt_neg_config)),
        (NoExtension, &CLANG_WRITER_U64, Some(opt_neg_config)),
    ];
    #[cfg(target_pointer_width = "32")]
    {
        configs.push((NoExtension, &CLANG_WRITER_U32, None));
        mul_add_data_map.insert(1, gen_mul_add_input(1));
    }
    #[cfg(target_pointer_width = "64")]
    configs.push((NoExtension, &CLANG_WRITER_U64, None));

    if *CPU_EXTENSION == IntelAVX512
        || *CPU_EXTENSION == IntelAVX
        || *CPU_EXTENSION == IntelSSE
        || *CPU_EXTENSION == IntelMMX
    {
        configs.push((IntelMMX, &CLANG_WRITER_INTEL_MMX, None));
    }
    if *CPU_EXTENSION == IntelAVX512 || *CPU_EXTENSION == IntelAVX || *CPU_EXTENSION == IntelSSE {
        configs.push((IntelSSE, &CLANG_WRITER_INTEL_SSE, None));
        mul_add_data_map.insert(4, gen_mul_add_input(4));
    }
    if *CPU_EXTENSION == IntelAVX512 || *CPU_EXTENSION == IntelAVX {
        configs.push((IntelAVX, &CLANG_WRITER_INTEL_AVX, None));
        mul_add_data_map.insert(8, gen_mul_add_input(8));
    }
    if *CPU_EXTENSION == IntelAVX512 {
        configs.push((IntelAVX512, &CLANG_WRITER_INTEL_AVX512, None));
        mul_add_data_map.insert(16, gen_mul_add_input(16));
    }
    if *CPU_EXTENSION == ARMNEON {
        configs.push((ARMNEON, &CLANG_WRITER_ARM_NEON, None));
        mul_add_data_map.insert(4, gen_mul_add_input(4));
    }

    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            writer_config,
            builder_config.clone(),
        );
        let word_len = (builder.word_len() >> 5) as usize;
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
        let out = execs[0]
            .execute(&CPUDataHolder::new(mul2x2_input.clone()), 0)
            .unwrap()
            .release();
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
            CPUDataHolder::new(vec![0; (mul2x2_more_input_combs.len() >> 6) * 4 * 2]);
        {
            let mut more_input_w = more_input_holder.get_mut();
            let more_input = more_input_w.get_mut();
            for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
                let idx = (i >> 5) / word_len;
                let half_idx = (i >> 5) % word_len;
                let shift = i & 31;
                more_input[idx * 4 * word_len + 0 + half_idx] |= (v & 1) << shift;
                more_input[idx * 4 * word_len + word_len + half_idx] |= ((v >> 1) & 1) << shift;
                more_input[idx * 4 * word_len + 2 * word_len + half_idx] |= ((v >> 2) & 1) << shift;
                more_input[idx * 4 * word_len + 3 * word_len + half_idx] |= ((v >> 3) & 1) << shift;
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
        let mut mul2x2_input_p = CPUDataHolder::new(vec![0u32; word_len * 8]);
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
        let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            writer_config,
            builder_config.clone(),
        );
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
        let circuit = Circuit::new(
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
        .unwrap();
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
        builder.add("mul_add", circuit, None, None, None);
        let mut execs = builder.build().unwrap();
        let exec = &mut execs[0];
        let (mul_add_input, mul_add_output) = mul_add_data_map.get(&word_len).unwrap();
        let out = exec.execute(mul_add_input, 0).unwrap().release();
        for (i, v) in mul_add_output.iter().enumerate() {
            assert_eq!(*v, out[i], "{}: {}", config_num, i);
        }
        let mut output = exec.new_data(((1 << 24) >> 5) * 8);
        exec.execute_reuse(mul_add_input, 0, &mut output).unwrap();
        let out = output.release();
        for (i, v) in mul_add_output.iter().enumerate() {
            assert_eq!(*v, out[i], "{}: {}", config_num, i);
        }
    }
}

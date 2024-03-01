use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::*;
use gatesim::*;
use gateutil::*;

use std::collections::HashMap;
use std::str::FromStr;

const MUL_ADD_INPUT_MAP: [usize; 24] = [
    0, 4, 7, 10, 13, 16, 19, 22, 1, 3, 6, 9, 12, 15, 18, 21, 2, 5, 8, 11, 14, 17, 20, 23,
];
fn gen_mul_add_input(word_len: usize) -> (CPUDataHolder, Vec<u32>) {
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
    (CPUDataHolder::new(input), exp_output)
}

fn get_mul_add_circuit() -> Circuit<u32> {
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

fn get_builder_configs() -> Vec<(
    CPUExtension,
    &'static CLangWriterConfig<'static>,
    Option<CPUBuilderConfig>,
)> {
    use CPUExtension::*;
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
    };

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
    }
    if *CPU_EXTENSION == IntelAVX512 || *CPU_EXTENSION == IntelAVX {
        configs.push((IntelAVX, &CLANG_WRITER_INTEL_AVX, None));
    }
    if *CPU_EXTENSION == IntelAVX512 {
        configs.push((IntelAVX512, &CLANG_WRITER_INTEL_AVX512, None));
    }
    if *CPU_EXTENSION == ARMNEON {
        configs.push((ARMNEON, &CLANG_WRITER_ARM_NEON, None));
    }
    configs
}

#[test]
fn test_cpu_builder_and_exec() {
    let configs = get_builder_configs();
    let mut mul_add_data_map = HashMap::<usize, (CPUDataHolder, Vec<u32>)>::new();
    for (cpu_ext, writer_config, builder_config) in configs.iter() {
        let builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            *cpu_ext,
            writer_config,
            builder_config.clone(),
        );
        let word_num = (builder.word_len() >> 5) as usize;
        if !mul_add_data_map.contains_key(&word_num) {
            mul_add_data_map.insert(word_num, gen_mul_add_input(word_num));
        }
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
        builder.add_with_config(
            "mul2x2sb",
            circuit.clone(),
            CodeConfig::new().single_buffer(true),
        );
        let mut execs = builder.build().unwrap();

        // input and output len
        assert_eq!(execs[0].input_data_len(16 * 1024), 2048);
        assert_eq!(execs[0].output_data_len(16 * 1024), 2048);
        assert_eq!(execs[1].input_data_len(16 * 1024), 4096);
        assert_eq!(execs[1].output_data_len(16 * 1024), 3584);
        //

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
        let circuit = get_mul_add_circuit();
        let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            writer_config,
            builder_config.clone(),
        );
        builder.add("mul_add", circuit.clone(), None, None, None);
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

#[test]
fn test_cpu_builder_and_exec_with_arg_input() {
    let configs = get_builder_configs();
    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        // with arg_input
        let circuit = translate_inputs_rev(get_mul_add_circuit(), MUL_ADD_INPUT_MAP);
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
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
        let mut it = execs[0].input_tx(32, &(0..20).collect::<Vec<_>>()).unwrap();
        let mut ot = execs[0].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
            .input_tx(
                32,
                &(0..24)
                    .map(|i| if i >= 4 { 23 - i } else { 0 })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        let mut ot = execs[1].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
fn test_cpu_builder_and_exec_with_elem_input() {
    let configs = get_builder_configs();
    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        // with elem_index
        let circuit = translate_inputs_rev(get_mul_add_circuit(), MUL_ADD_INPUT_MAP);
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
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

        // input and output len
        assert_eq!(execs[0].input_data_len(16 * 1024), 6144);
        assert_eq!(execs[0].output_data_len(16 * 1024), 4096);
        assert_eq!(execs[1].input_data_len(16 * 1024), 1);
        assert_eq!(execs[1].output_data_len(16 * 1024), 4096);

        let mut it = execs[0].input_tx(32, &(0..12).collect::<Vec<_>>()).unwrap();
        let mut ot = execs[0].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
        for (i, v) in output.into_iter().enumerate() {
            let ix = i ^ 0xfff000;
            let out = u32::try_from(((ix & 0xff) * (ix >> 8) + (ix >> 16)) & 0xff).unwrap();
            assert_eq!(out, v, "{}: {}", config_num, i);
        }

        // with elem full
        let mut ot = execs[1].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
        let input = execs[1].new_data(1);
        let output_circ = execs[1].execute(&input, 0).unwrap();
        let output_circ_len = output_circ.len();
        let output = ot.transform(&output_circ).unwrap();
        let output = output.release();
        assert_eq!(output.len(), 1 << 24);
        assert!(output.len() != 0);
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
        let mut it = execs[2].input_tx(32, &(0..12).collect::<Vec<_>>()).unwrap();
        let mut ot = execs[2].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
        let mut it = execs[3].input_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
        let mut ot = execs[3].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
                assert_eq!(out, v, "{}: {} {}", config_num, arg_input, i);
            }
        }

        // with single buffer and input_placement
        let mut it = execs[4]
            .input_tx(
                32,
                &(0..16)
                    .map(|i| if i >= 4 { 15 - i } else { 0 })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        let mut ot = execs[4].output_tx(32, &(0..8).collect::<Vec<_>>()).unwrap();
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
fn test_cpu_builder_and_exec_with_aggr_output() {
    let configs = get_builder_configs();
    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
        let circuit =
            Circuit::<u32>::from_str(concat!("{0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 ",
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
            "xor(144,147) nimpl(0,148):11}(16)"))
            .unwrap();
        builder.add_with_config(
            "comb_aggr_out",
            circuit.clone(),
            CodeConfig::new()
                .aggr_output_code(Some(
                    r##"{
    unsigned int i;
    uint32_t out[(TYPE_LEN >> 5)*12];
    uint32_t* output_u32 = (uint32_t*)output;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*0)) = o0;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*1)) = o1;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*2)) = o2;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*3)) = o3;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*4)) = o4;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*5)) = o5;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*6)) = o6;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*7)) = o7;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*8)) = o8;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*9)) = o9;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*10)) = o10;
    *((TYPE_NAME*)(out + (TYPE_LEN>>5)*11)) = o11;
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
        output_u32[out_idx >> 5] |= (1 << (out_idx & 31));
    }
}"##,
                ))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        let mut execs = builder.build().unwrap();

        let expected = vec![
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            939491327, 4294967295, 4294967295, 3221225471, 1543503871, 2147483647, 2004844543,
            3086483455, 805142527, 4294967295, 4294967295, 4294967295, 4160749567, 4294967295,
            3489658879, 2147481599, 536565759, 4294967295, 3087007743, 3082813439, 2077734911,
            503316479, 2809659391, 2503417855, 19096951, 4294967295, 2147483647, 2013265919,
            4160749567, 4294967295, 931102719, 1610612735, 85292543, 4294967295, 2147483647,
            1438646271, 1312536575, 4290772991, 291452927, 565411711, 285546271, 4294967295,
            935288831, 930611199, 1744295935, 662700031, 94330879, 1732196351, 289738727,
            2013265919, 1970789887, 1466940927, 118362399, 394256255, 10175999, 69310335, 68157743,
            1600085855, 1600085855, 1600085855, 358571871, 1600085855, 387931999, 1600085855,
            375349087, 1600085855, 1465868127, 1600085855, 522147679, 1465868127, 392109919,
            257908575, 320019295, 1600085855, 526344031, 1600085855, 387931999, 1465868127,
            354359135, 1196623711, 18027615, 1600085855, 358571359, 425156447, 421024607,
            1129799519, 270472479, 17651039, 1179993, 1600085855, 526344031, 1461673823, 392124255,
            1600085855, 291446623, 325017439, 88301395, 1600085855, 526341983, 1432311639,
            17506127, 454778719, 18436959, 23007063, 65558, 1600085855, 118955871, 1398759263,
            374806295, 1449088863, 21697055, 117526879, 21299231, 56581983, 273094487, 123016983,
            84085013, 274661723, 67311135, 1140916567, 16781322,
        ];
        let mut it = execs[0].input_tx(32, &(0..16).collect::<Vec<_>>()).unwrap();
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
fn test_cpu_data_holder() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
    };

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            &CLANG_WRITER_U32,
            Some(builder_config),
        );
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
            assert_eq!(rd.get().len(), 10);
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
        let data = execs[0].new_data_from_slice(&array[..]);
        {
            let rd = data.get();
            for (i, x) in rd.get().iter().enumerate() {
                assert_eq!(array[i], *x, "1: {} {}", config_num, i);
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

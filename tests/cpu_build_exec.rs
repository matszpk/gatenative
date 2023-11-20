use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_cpu_builder_and_exec() {
    use CPUExtension::*;
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
    };

    let (mul_add_input, mul_add_output) = {
        let input_map = [
            0, 4, 7, 10, 13, 16, 19, 22, 1, 3, 6, 9, 12, 15, 18, 21, 2, 5, 8, 11, 14, 17, 20, 23,
        ];
        let mut input = vec![0u32; ((1 << 24) >> 5) * 24];
        let mut exp_output = vec![0u32; ((1 << 24) >> 5) * 8];
        for a in 0u32..256 {
            for b in 0u32..256 {
                for c in 0u32..256 {
                    let i = ((a & 0xff) | ((b & 0xff) << 8) | ((c & 0xff) << 16)) as usize;
                    let idx = i >> 6;
                    let half_idx = (i >> 5) & 1;
                    let shift = i & 31;
                    let exp = (a.overflowing_mul(b).0).overflowing_add(c).0;
                    for bit in 0..8 {
                        let ix = input_map[bit];
                        input[idx * 24 * 2 + ix * 2 + half_idx] |= ((a >> bit) & 1) << shift;
                        let ix = input_map[bit + 8];
                        input[idx * 24 * 2 + ix * 2 + half_idx] |= ((b >> bit) & 1) << shift;
                        let ix = input_map[bit + 16];
                        input[idx * 24 * 2 + ix * 2 + half_idx] |= ((c >> bit) & 1) << shift;
                        exp_output[idx * 8 * 2 + bit * 2 + half_idx] |= ((exp >> bit) & 1) << shift;
                    }
                }
            }
        }
        (input, exp_output)
    };

    for (config_num, (cpu_ext, writer_config, builder_config)) in [
        (NoExtension, &CLANG_WRITER_U64_TEST_IMPL, None),
        (NoExtension, &CLANG_WRITER_U64_TEST_NIMPL, None),
        (NoExtension, &CLANG_WRITER_U64, Some(no_opt_neg_config)),
        (NoExtension, &CLANG_WRITER_U64, Some(opt_neg_config)),
    ]
    .into_iter()
    .enumerate()
    {
        let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            cpu_ext,
            writer_config,
            builder_config.clone(),
        );
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
        builder.add("mul2x2", circuit, None, None);
        let mut execs = builder.build().unwrap();
        let exec = &mut execs[0];
        const MUL2X2_INPUT: [u32; 8] = [
            0b1010101010101010,
            0u32,
            0b1100110011001100,
            0u32,
            0b1111000011110000,
            0u32,
            0b1111111100000000,
            0u32,
        ];
        let out = exec.execute(&MUL2X2_INPUT[..]).unwrap();
        for i in 0..16 {
            let a = i & 3;
            let b = i >> 2;
            let c = ((out[0] >> i) & 1)
                + (((out[2] >> i) & 1) << 1)
                + (((out[4] >> i) & 1) << 2)
                + (((out[6] >> i) & 1) << 3);
            assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
        }
        // more inputs
        let mul2x2_more_input_combs = {
            let mut input = vec![];
            let mut s = 0x34251u32;
            for _ in 0..64 * 9 {
                input.push(s & 15);
                s = (s ^ (s * 1895952115 + 159502151)) ^ 0xba001a4;
                s = s.rotate_right(s & 15);
            }
            input
        };
        let mut more_input = vec![0; (mul2x2_more_input_combs.len() >> 6) * 4 * 2];
        for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
            let idx = i >> 6;
            let half_idx = (i >> 5) & 1;
            let shift = i & 31;
            more_input[idx * 8 + 0 + half_idx] |= (v & 1) << shift;
            more_input[idx * 8 + 2 + half_idx] |= ((v >> 1) & 1) << shift;
            more_input[idx * 8 + 4 + half_idx] |= ((v >> 2) & 1) << shift;
            more_input[idx * 8 + 6 + half_idx] |= ((v >> 3) & 1) << shift;
        }
        let out = exec.execute(&more_input).unwrap();
        for (i, v) in mul2x2_more_input_combs.into_iter().enumerate() {
            let idx = i >> 6;
            let half_idx = (i >> 5) & 1;
            let shift = i & 31;
            let a = v & 3;
            let b = v >> 2;
            let c = ((out[idx * 8 + 0 + half_idx] >> shift) & 1)
                + (((out[idx * 8 + 2 + half_idx] >> shift) & 1) << 1)
                + (((out[idx * 8 + 4 + half_idx] >> shift) & 1) << 2)
                + (((out[idx * 8 + 6 + half_idx] >> shift) & 1) << 3);
            assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
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
        builder.add("mul_add", circuit, None, None);
        let mut execs = builder.build().unwrap();
        let exec = &mut execs[0];
        let out = exec.execute(&mul_add_input).unwrap();
        for (i, v) in mul_add_output.iter().enumerate() {
            assert_eq!(*v, out[i], "{}: {}", config_num, i);
        }
    }
}

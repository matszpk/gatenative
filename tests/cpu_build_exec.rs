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
    for (config_num, (cpu_ext, writer_config, builder_config)) in [
        (NoExtension, &CLANG_WRITER_U64_TEST_IMPL, None),
        (NoExtension, &CLANG_WRITER_U64_TEST_NIMPL, None),
        (NoExtension, &CLANG_WRITER_U64, Some(no_opt_neg_config)),
        (NoExtension, &CLANG_WRITER_U64, Some(opt_neg_config)),
    ]
    .into_iter()
    .enumerate()
    {
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
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
    }
}

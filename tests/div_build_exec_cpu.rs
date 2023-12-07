use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::div_build_exec::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_div_builder_and_exec_cpu() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
    };

    for (config_num, (writer_config, builder_config)) in [
        (&CLANG_WRITER_U32, &no_opt_neg_config),
        (&CLANG_WRITER_U32, &opt_neg_config),
        (&CLANG_WRITER_U64, &no_opt_neg_config),
        (&CLANG_WRITER_U64, &opt_neg_config),
    ]
    .into_iter()
    .enumerate()
    {
        for max_gates in [2, 5, 100] {
            let builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
                CPUExtension::NoExtension,
                writer_config,
                Some(builder_config.clone()),
            );
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
        }
    }
}

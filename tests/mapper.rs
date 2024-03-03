use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::mapper::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;
use std::sync::{
    atomic::{self, AtomicUsize},
    Arc,
};

#[test]
fn test_basic_mapper_builder_and_exec() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
        parallel: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
        parallel: false,
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
        let circuit2 = Circuit::new(
            8,
            [
                Gate::new_and(0, 1),   // 8
                Gate::new_and(2, 5),   // 9
                Gate::new_nor(8, 9),   // 10
                Gate::new_and(3, 6),   // 11
                Gate::new_and(1, 7),   // 12
                Gate::new_nor(11, 12), // 13
                Gate::new_and(4, 6),   // 14
                Gate::new_and(2, 3),   // 15
                Gate::new_nor(14, 15), // 16
                Gate::new_and(5, 7),   // 17
                Gate::new_and(0, 5),   // 18
                Gate::new_nor(17, 18), // 19
                Gate::new_and(2, 7),   // 20
                Gate::new_and(1, 4),   // 21
                Gate::new_nor(20, 21), // 22
                Gate::new_and(0, 4),   // 23
                Gate::new_and(3, 7),   // 24
                Gate::new_nor(23, 24), // 25
                Gate::new_and(1, 6),   // 26
                Gate::new_and(5, 6),   // 27
                Gate::new_nor(26, 27), // 28
                Gate::new_and(1, 2),   // 29
                Gate::new_and(3, 5),   // 30
                Gate::new_nor(29, 30), // 31
                Gate::new_xor(10, 13), // 32
                Gate::new_xor(16, 19), // 33
                Gate::new_xor(22, 25), // 34
                Gate::new_xor(28, 31), // 35
                Gate::new_xor(32, 33), // 36
                Gate::new_xor(34, 35), // 37
                Gate::new_xor(36, 37), // 38
            ],
            [(38, false)],
        )
        .unwrap();
        let builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            writer_config,
            Some(builder_config.clone()),
        );
        let mut builder = BasicMapperBuilder::new(builder);
        let word_len = (builder.word_len() >> 5) as usize;
        let arg_input_indices = [0, 3, 5, 8];
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
        let arg_input_indices_2 = [0, 3, 5];
        let rest_input_indices_2 = {
            let mut rest_input_indices_2 = vec![];
            let mut j = 0;
            for i in 0..8 {
                if j < arg_input_indices_2.len() && arg_input_indices_2[j] == i {
                    j += 1;
                    continue;
                } else {
                    rest_input_indices_2.push(i);
                }
            }
            rest_input_indices_2
        };
        builder.add("xcircuit", circuit.clone(), &arg_input_indices[..]);
        builder.add("xcircuit2", circuit2.clone(), &arg_input_indices_2[..]);
        builder.add_with_config(
            "xcircuit",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&arg_input_indices[..]))
                .elem_inputs(Some(&rest_input_indices[..])),
        );
        let mut execs = builder.build().unwrap();

        // check input and output len
        assert_eq!(execs[0].input_data_len(16 * 1024), 4096);
        assert_eq!(execs[0].output_data_len(16 * 1024), 512);
        assert_eq!(execs[1].input_data_len(16 * 1024), 2560);
        assert_eq!(execs[1].output_data_len(16 * 1024), 512);

        // number of chunks
        let xcircuit_data_num = (((256 >> 5) + word_len - 1) / word_len) * word_len;
        let rest_num = rest_input_indices.len();
        let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
        // prepare input for executor
        for i in 0..256 {
            let idx = (i >> 5) / word_len;
            let widx = (i >> 5) % word_len;
            let bit = i & 31;
            for j in 0..rest_num {
                xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                    ((u32::try_from(i).unwrap() >> j) & 1) << bit;
            }
        }
        let input = execs[0].new_data_from_vec(xcircuit_input.clone());
        let mut exec_count = 0;
        assert!(
            execs[0]
                .execute(
                    &input,
                    true,
                    |out, _, result_out, arg_input| {
                        exec_count += 1;
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
                        // get result output and compare
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        out && xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |_| false
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(exec_count, 16);
        // execute_direct testcase
        let mut exec_count = 0;
        assert!(
            execs[0]
                .execute_direct(
                    &input,
                    true,
                    |out, _, result_out, arg_input| {
                        exec_count += 1;
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
                        // get result output and compare
                        out && xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |_| false
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(exec_count, 16);
        // elem_input
        let mut exec_count = 0;
        assert!(
            execs[2]
                .execute_direct(
                    &input,
                    true,
                    |out, _, result_out, arg_input| {
                        exec_count += 1;
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
                        // get result output and compare
                        out && xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |_| false
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(exec_count, 16);

        if word_len == 1 {
            // number of chunks
            let xcircuit_data_num = (((32 >> 5) + word_len - 1) / word_len) * word_len;
            let rest_num = rest_input_indices_2.len();
            let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
            // prepare input for executor
            for i in 0..32 {
                let idx = (i >> 5) / word_len;
                let widx = (i >> 5) % word_len;
                let bit = i & 31;
                for j in 0..rest_num {
                    xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                        ((u32::try_from(i).unwrap() >> j) & 1) << bit;
                }
            }
            let input = execs[1].new_data_from_vec(xcircuit_input.clone());
            assert!(
                execs[1]
                    .execute(
                        &input,
                        true,
                        |out, _, result_out, arg_input| {
                            let mut input = vec![false; 8];
                            let mut xcircuit_out = vec![0u32; xcircuit_data_num];
                            // fill inputs by arg_inputs
                            for (i, v) in arg_input_indices_2.iter().enumerate() {
                                input[*v] = ((arg_input >> i) & 1) != 0;
                            }
                            // prepare expected output
                            for rest in 0..32 {
                                // fill input by rest of bits of input
                                for (i, v) in rest_input_indices_2.iter().enumerate() {
                                    input[*v] = ((rest >> i) & 1) != 0;
                                }
                                let value = circuit2.eval(input.clone())[0];
                                let idx = (rest >> 5) / word_len;
                                let widx = (rest >> 5) % word_len;
                                let bit = rest & 31;
                                xcircuit_out[word_len * idx + widx] |= (value as u32) << bit;
                            }
                            // get result output and compare
                            let result_out = result_out.get();
                            let result_out = result_out.get();
                            out && xcircuit_out
                                .into_iter()
                                .enumerate()
                                .all(|(i, exp)| result_out[i] == exp)
                        },
                        |_| false
                    )
                    .unwrap(),
                "{}",
                config_num
            );
        }
        // testing stop
        let input = execs[0].new_data_from_vec(xcircuit_input.clone());
        let mut exec_count = 0;
        execs[0]
            .execute(
                &input,
                0,
                |_, _, _, arg_input| {
                    exec_count += 1;
                    arg_input
                },
                |x| *x >= 9,
            )
            .unwrap();
        assert_eq!(exec_count, 10);
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
fn test_basic_mapper_builder_and_exec_with_aggr_output() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
        parallel: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
        parallel: false,
    };

    let circuit = Circuit::<u32>::from_str(COMB_CIRCUIT_CODE).unwrap();
    for (config_num, (writer_config, builder_config)) in [
        (&CLANG_WRITER_U32, &no_opt_neg_config),
        (&CLANG_WRITER_U32, &opt_neg_config),
        (&CLANG_WRITER_U64, &no_opt_neg_config),
        (&CLANG_WRITER_U64, &opt_neg_config),
    ]
    .into_iter()
    .enumerate()
    {
        let builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            writer_config,
            Some(builder_config.clone()),
        );
        let mut builder = BasicMapperBuilder::new(builder);
        builder.add_with_config(
            "comb_aggr_out_arg",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .aggr_output_code(Some(COMB_CIRCUIT_AGGR_CODE))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        builder.add_with_config(
            "comb_aggr_out_arg_elem_full",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..4).collect::<Vec<_>>()))
                .elem_inputs(Some(&(4..16).collect::<Vec<_>>()))
                .aggr_output_code(Some(COMB_CIRCUIT_AGGR_CODE))
                .aggr_output_len(Some(1 << (12 - 5))),
        );
        let mut execs = builder.build().unwrap();
        let mut it = execs[0]
            .input_transformer(32, &(0..12).collect::<Vec<_>>())
            .unwrap();
        let input = execs[0].new_data_from_vec((0..1 << 12).collect::<Vec<_>>());
        let input_circ = it.transform(&input).unwrap();
        let mut exec_count = 0;
        let output = execs[0]
            .execute_direct(
                &input_circ,
                vec![0u32; COMB_CIRCUIT_EXPECTED.len()],
                |out, _, result_out, _| {
                    exec_count += 1;
                    let mut new_out = vec![0u32; COMB_CIRCUIT_EXPECTED.len()];
                    for (i, v) in new_out.iter_mut().enumerate() {
                        *v = out[i] | result_out[i];
                    }
                    new_out
                },
                |_| false,
            )
            .unwrap();
        assert_eq!(exec_count, 16);
        assert_eq!(COMB_CIRCUIT_EXPECTED.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(COMB_CIRCUIT_EXPECTED[i], *out, "{}: {}", config_num, i);
        }

        // with elem input full
        let input_circ = execs[1].new_data(1);
        let mut exec_count = 0;
        let output = execs[1]
            .execute_direct(
                &input_circ,
                vec![0u32; COMB_CIRCUIT_EXPECTED.len()],
                |out, _, result_out, _| {
                    exec_count += 1;
                    let mut new_out = vec![0u32; COMB_CIRCUIT_EXPECTED.len()];
                    for (i, v) in new_out.iter_mut().enumerate() {
                        *v = out[i] | result_out[i];
                    }
                    new_out
                },
                |_| false,
            )
            .unwrap();
        assert_eq!(exec_count, 16);
        assert_eq!(COMB_CIRCUIT_EXPECTED.len(), output.len());
        for (i, out) in output.iter().enumerate() {
            assert_eq!(COMB_CIRCUIT_EXPECTED[i], *out, "{}: {}", config_num, i);
        }
    }
}

#[test]
fn test_par_basic_mapper_builder_and_exec() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
        parallel: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
        parallel: false,
    };

    for (config_num, (writer_config, builder_config)) in [
        (&CLANG_WRITER_U32, &no_opt_neg_config),
        (&CLANG_WRITER_U32, &opt_neg_config),
        // (&CLANG_WRITER_U64, &no_opt_neg_config),
        // (&CLANG_WRITER_U64, &opt_neg_config),
    ]
    .into_iter()
    .enumerate()
    {
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
        let circuit2 = Circuit::new(
            8,
            [
                Gate::new_and(0, 1),   // 8
                Gate::new_and(2, 5),   // 9
                Gate::new_nor(8, 9),   // 10
                Gate::new_and(3, 6),   // 11
                Gate::new_and(1, 7),   // 12
                Gate::new_nor(11, 12), // 13
                Gate::new_and(4, 6),   // 14
                Gate::new_and(2, 3),   // 15
                Gate::new_nor(14, 15), // 16
                Gate::new_and(5, 7),   // 17
                Gate::new_and(0, 5),   // 18
                Gate::new_nor(17, 18), // 19
                Gate::new_and(2, 7),   // 20
                Gate::new_and(1, 4),   // 21
                Gate::new_nor(20, 21), // 22
                Gate::new_and(0, 4),   // 23
                Gate::new_and(3, 7),   // 24
                Gate::new_nor(23, 24), // 25
                Gate::new_and(1, 6),   // 26
                Gate::new_and(5, 6),   // 27
                Gate::new_nor(26, 27), // 28
                Gate::new_and(1, 2),   // 29
                Gate::new_and(3, 5),   // 30
                Gate::new_nor(29, 30), // 31
                Gate::new_xor(10, 13), // 32
                Gate::new_xor(16, 19), // 33
                Gate::new_xor(22, 25), // 34
                Gate::new_xor(28, 31), // 35
                Gate::new_xor(32, 33), // 36
                Gate::new_xor(34, 35), // 37
                Gate::new_xor(36, 37), // 38
            ],
            [(38, false)],
        )
        .unwrap();
        let builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
            CPUExtension::NoExtension,
            writer_config,
            Some(builder_config.clone()),
        );
        let mut builder = ParBasicMapperBuilder::new(builder);
        let word_len = (builder.word_len() >> 5) as usize;
        let arg_input_indices = [0, 3, 5, 8];
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
        let arg_input_indices_2 = [0, 3, 5];
        let rest_input_indices_2 = {
            let mut rest_input_indices_2 = vec![];
            let mut j = 0;
            for i in 0..8 {
                if j < arg_input_indices_2.len() && arg_input_indices_2[j] == i {
                    j += 1;
                    continue;
                } else {
                    rest_input_indices_2.push(i);
                }
            }
            rest_input_indices_2
        };
        builder.add("xcircuit", circuit.clone(), &arg_input_indices[..]);
        builder.add("xcircuit2", circuit2.clone(), &arg_input_indices_2[..]);
        builder.add_with_config(
            "xcircuit",
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&arg_input_indices[..]))
                .elem_inputs(Some(&rest_input_indices[..])),
        );
        let mut execs = builder.build().unwrap();

        // check input and output len
        assert_eq!(execs[0].input_data_len(16 * 1024), 4096);
        assert_eq!(execs[0].output_data_len(16 * 1024), 512);
        assert_eq!(execs[1].input_data_len(16 * 1024), 2560);
        assert_eq!(execs[1].output_data_len(16 * 1024), 512);

        // number of chunks
        let xcircuit_data_num = (((256 >> 5) + word_len - 1) / word_len) * word_len;
        let rest_num = rest_input_indices.len();
        let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
        // prepare input for executor
        for i in 0..256 {
            let idx = (i >> 5) / word_len;
            let widx = (i >> 5) % word_len;
            let bit = i & 31;
            for j in 0..rest_num {
                xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                    ((u32::try_from(i).unwrap() >> j) & 1) << bit;
            }
        }

        let call_count = Arc::new(AtomicUsize::new(0));
        let input = execs[0].new_data_from_vec(xcircuit_input.clone());
        assert!(
            execs[0]
                .execute(
                    &input,
                    true,
                    |_, result_out, arg_input| {
                        call_count.fetch_add(1, atomic::Ordering::SeqCst);
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
                        // get result output and compare
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2,
                    |_| false,
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(16, call_count.load(atomic::Ordering::SeqCst));

        // execute_direct testcase
        let call_count = Arc::new(AtomicUsize::new(0));
        assert!(
            execs[0]
                .execute_direct(
                    &input,
                    true,
                    |_, result_out, arg_input| {
                        call_count.fetch_add(1, atomic::Ordering::SeqCst);
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
                        // get result output and compare
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2,
                    |_| false,
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(16, call_count.load(atomic::Ordering::SeqCst));
        // elem_input
        let call_count = Arc::new(AtomicUsize::new(0));
        assert!(
            execs[2]
                .execute_direct(
                    &input,
                    true,
                    |_, result_out, arg_input| {
                        call_count.fetch_add(1, atomic::Ordering::SeqCst);
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
                        // get result output and compare
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2,
                    |_| false,
                )
                .unwrap(),
            "{}",
            config_num
        );
        assert_eq!(16, call_count.load(atomic::Ordering::SeqCst));

        if word_len == 1 {
            // number of chunks
            let xcircuit_data_num = (((32 >> 5) + word_len - 1) / word_len) * word_len;
            let rest_num = rest_input_indices_2.len();
            let mut xcircuit_input = vec![0u32; xcircuit_data_num * rest_num];
            // prepare input for executor
            for i in 0..32 {
                let idx = (i >> 5) / word_len;
                let widx = (i >> 5) % word_len;
                let bit = i & 31;
                for j in 0..rest_num {
                    xcircuit_input[rest_num * word_len * idx + word_len * j + widx] |=
                        ((u32::try_from(i).unwrap() >> j) & 1) << bit;
                }
            }

            let call_count = Arc::new(AtomicUsize::new(0));
            let input = execs[1].new_data_from_vec(xcircuit_input.clone());
            assert!(
                execs[1]
                    .execute(
                        &input,
                        true,
                        |_, result_out, arg_input| {
                            call_count.fetch_add(1, atomic::Ordering::SeqCst);
                            let mut input = vec![false; 8];
                            let mut xcircuit_out = vec![0u32; xcircuit_data_num];
                            // fill inputs by arg_inputs
                            for (i, v) in arg_input_indices_2.iter().enumerate() {
                                input[*v] = ((arg_input >> i) & 1) != 0;
                            }
                            // prepare expected output
                            for rest in 0..32 {
                                // fill input by rest of bits of input
                                for (i, v) in rest_input_indices_2.iter().enumerate() {
                                    input[*v] = ((rest >> i) & 1) != 0;
                                }
                                let value = circuit2.eval(input.clone())[0];
                                let idx = (rest >> 5) / word_len;
                                let widx = (rest >> 5) % word_len;
                                let bit = rest & 31;
                                xcircuit_out[word_len * idx + widx] |= (value as u32) << bit;
                            }
                            // get result output and compare
                            let result_out = result_out.get();
                            let result_out = result_out.get();
                            xcircuit_out
                                .into_iter()
                                .enumerate()
                                .all(|(i, exp)| result_out[i] == exp)
                        },
                        |out1, out2| out1 && out2,
                        |_| false,
                    )
                    .unwrap(),
                "{}",
                config_num
            );
            assert_eq!(8, call_count.load(atomic::Ordering::SeqCst));
        }

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let call_count = Arc::new(AtomicUsize::new(0));
        pool.install(|| {
            execs[0]
                .execute(
                    &input,
                    0,
                    |_, _, arg_input| {
                        call_count.fetch_add(1, atomic::Ordering::SeqCst);
                        arg_input
                    },
                    |out1, out2| std::cmp::max(out1, out2),
                    |x| *x >= 9,
                )
                .unwrap();
        });
        assert!(call_count.load(atomic::Ordering::SeqCst) < 12);
    }
}

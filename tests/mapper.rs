use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::mapper::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_basic_mapper_builder_and_exec() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
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
        let mut execs = builder.build().unwrap();
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
        assert!(
            execs[0]
                .execute(&input, true, |out, _, result_out, arg_input| {
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
                    let result_out = result_out.get();
                    let result_out = result_out.get();
                    out && xcircuit_out
                        .into_iter()
                        .enumerate()
                        .all(|(i, exp)| result_out[i] == exp)
                })
                .unwrap(),
            "{}",
            config_num
        );
        // execute_direct testcase
        assert!(
            execs[0]
                .execute_direct(&input, true, |out, _, result_out, arg_input| {
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
                    out && xcircuit_out
                        .into_iter()
                        .enumerate()
                        .all(|(i, exp)| result_out[i] == exp)
                })
                .unwrap(),
            "{}",
            config_num
        );
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
                .execute(&input, true, |out, _, result_out, arg_input| {
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
                    // execute circuit
                    let result_out = result_out.get();
                    let result_out = result_out.get();
                    out && xcircuit_out
                        .into_iter()
                        .enumerate()
                        .all(|(i, exp)| result_out[i] == exp)
                })
                .unwrap(),
            "{}",
            config_num
        );
    }
}

#[test]
fn test_par_basic_mapper_builder_and_exec() {
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
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
        let mut execs = builder.build().unwrap();
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
        assert!(
            execs[0]
                .execute(
                    &input,
                    true,
                    |_, result_out, arg_input| {
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
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2
                )
                .unwrap(),
            "{}",
            config_num
        );
        // execute_direct testcase
        assert!(
            execs[0]
                .execute_direct(
                    &input,
                    true,
                    |_, result_out, arg_input| {
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
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2
                )
                .unwrap(),
            "{}",
            config_num
        );
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
                    |_, result_out, arg_input| {
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
                        // execute circuit
                        let result_out = result_out.get();
                        let result_out = result_out.get();
                        xcircuit_out
                            .into_iter()
                            .enumerate()
                            .all(|(i, exp)| result_out[i] == exp)
                    },
                    |out1, out2| out1 && out2
                )
                .unwrap(),
            "{}",
            config_num
        );
    }
}

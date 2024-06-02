use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

const ADDIMMU16_77_CIRCUIT: &str = r##"    {
        0:0n
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        12
        13
        14
        15
        xor(1,0):1
        and(1,0)
        xor(17,2):2n
        and(17,2)
        nor(18,19)
        xor(20,3):3
        nimpl(3,20)
        nimpl(21,22)
        xor(23,4):4n
        nimpl(4,23)
        xor(25,5):5
        and(25,5)
        xor(27,6):6n
        and(27,6)
        nor(28,29)
        xor(30,7):7n
        nimpl(7,30)
        xor(32,8):8
        and(32,8)
        xor(34,9):9
        and(34,9)
        xor(36,10):10
        and(36,10)
        xor(38,11):11
        and(38,11)
        xor(40,12):12
        and(40,12)
        xor(42,13):13
        and(42,13)
        xor(44,14):14
        and(44,14)
        xor(46,15):15
    }(16)
"##;

const ADDU16_CIRCUIT: &str = r##"    {
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        12
        13
        14
        15
        16
        17
        18
        19
        20
        21
        22
        23
        24
        25
        26
        27
        28
        29
        30
        31
        xor(0,16):0
        xor(1,17)
        and(0,16)
        xor(33,34):1
        xor(2,18)
        and(33,34)
        and(1,17)
        nor(37,38)
        xor(36,39):2n
        xor(3,19)
        nimpl(36,39)
        and(2,18)
        nor(42,43)
        xor(41,44):3n
        xor(4,20)
        nimpl(41,44)
        and(3,19)
        nor(47,48)
        xor(46,49):4n
        xor(5,21)
        nimpl(46,49)
        and(4,20)
        nor(52,53)
        xor(51,54):5n
        xor(6,22)
        nimpl(51,54)
        and(5,21)
        nor(57,58)
        xor(56,59):6n
        xor(7,23)
        nimpl(56,59)
        and(6,22)
        nor(62,63)
        xor(61,64):7n
        xor(8,24)
        nimpl(61,64)
        and(7,23)
        nor(67,68)
        xor(66,69):8n
        xor(9,25)
        nimpl(66,69)
        and(8,24)
        nor(72,73)
        xor(71,74):9n
        xor(10,26)
        nimpl(71,74)
        and(9,25)
        nor(77,78)
        xor(76,79):10n
        xor(11,27)
        nimpl(76,79)
        and(10,26)
        nor(82,83)
        xor(81,84):11n
        xor(12,28)
        nimpl(81,84)
        and(11,27)
        nor(87,88)
        xor(86,89):12n
        xor(13,29)
        nimpl(86,89)
        and(12,28)
        nor(92,93)
        xor(91,94):13n
        xor(14,30)
        nimpl(91,94)
        and(13,29)
        nor(97,98)
        xor(96,99):14n
        xor(15,31)
        nimpl(96,99)
        and(14,30)
        nor(102,103)
        xor(101,104):15n
    }(32)
"##;

fn get_builder_configs() -> Vec<(
    CPUExtension,
    &'static CLangWriterConfig<'static>,
    Option<CPUBuilderConfig>,
)> {
    use CPUExtension::*;
    let no_opt_neg_config = CPUBuilderConfig {
        optimize_negs: false,
        parallel: None,
    };
    let opt_neg_config = CPUBuilderConfig {
        optimize_negs: true,
        parallel: None,
    };

    let mut configs = vec![
        (NoExtension, &CLANG_WRITER_U64_TEST_IMPL, None),
        (NoExtension, &CLANG_WRITER_U64_TEST_NIMPL, None),
        (NoExtension, &CLANG_WRITER_U64_TEST_LOP3, None),
        (NoExtension, &CLANG_WRITER_U64, Some(no_opt_neg_config)),
        (NoExtension, &CLANG_WRITER_U64, Some(opt_neg_config)),
    ];
    #[cfg(target_pointer_width = "32")]
    configs.push((NoExtension, &CLANG_WRITER_U32, None));
    #[cfg(target_pointer_width = "64")]
    configs.push((NoExtension, &CLANG_WRITER_U64, None));

    if *CPU_EXTENSION == IntelAVX512
        || *CPU_EXTENSION == IntelAVX2
        || *CPU_EXTENSION == IntelAVX
        || *CPU_EXTENSION == IntelSSE2
        || *CPU_EXTENSION == IntelSSE
        || *CPU_EXTENSION == IntelMMX
    {
        configs.push((IntelMMX, &CLANG_WRITER_INTEL_MMX, None));
    }
    if *CPU_EXTENSION == IntelAVX512
        || *CPU_EXTENSION == IntelAVX2
        || *CPU_EXTENSION == IntelAVX
        || *CPU_EXTENSION == IntelSSE2
        || *CPU_EXTENSION == IntelSSE
    {
        configs.push((IntelSSE, &CLANG_WRITER_INTEL_SSE, None));
    }
    if *CPU_EXTENSION == IntelAVX512
        || *CPU_EXTENSION == IntelAVX2
        || *CPU_EXTENSION == IntelAVX
        || *CPU_EXTENSION == IntelSSE2
    {
        configs.push((IntelSSE2, &CLANG_WRITER_INTEL_SSE2, None));
    }
    if *CPU_EXTENSION == IntelAVX512 || *CPU_EXTENSION == IntelAVX2 || *CPU_EXTENSION == IntelAVX {
        configs.push((IntelAVX, &CLANG_WRITER_INTEL_AVX, None));
    }
    if *CPU_EXTENSION == IntelAVX512 || *CPU_EXTENSION == IntelAVX2 {
        configs.push((IntelAVX2, &CLANG_WRITER_INTEL_AVX2, None));
    }
    if *CPU_EXTENSION == IntelAVX512 {
        configs.push((IntelAVX512, &CLANG_WRITER_INTEL_AVX512, None));
    }
    if *CPU_EXTENSION == ARMNEON {
        configs.push((ARMNEON, &CLANG_WRITER_ARM_NEON, None));
    }
    // double for parallel
    let configs = configs
        .clone()
        .into_iter()
        .chain(
            configs
                .clone()
                .into_iter()
                .map(|(ext, clang_config, cpu_builder_config)| {
                    let mut cpu_builder_config =
                        cpu_builder_config.unwrap_or(CPU_BUILDER_CONFIG_DEFAULT);
                    cpu_builder_config.parallel = Some(64);
                    (ext, clang_config, Some(cpu_builder_config))
                }),
        )
        .collect::<Vec<_>>();
    configs
}

#[test]
fn test_cpu_builder_and_exec_inner_loop() {
    let configs = get_builder_configs();
    let addu16imm77_circuit = Circuit::<u32>::from_str(ADDIMMU16_77_CIRCUIT).unwrap();
    let addu16_circuit = Circuit::<u32>::from_str(ADDU16_CIRCUIT).unwrap();
    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        const ITER_NUM: u32 = 12;
        const ITER2_NUM: u32 = 15;
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
        builder.add_with_config(
            "addu16imm77",
            addu16imm77_circuit.clone(),
            CodeConfig::new().inner_loop(Some(ITER_NUM)),
        );
        builder.add_with_config(
            "addu16",
            addu16_circuit.clone(),
            CodeConfig::new()
                .elem_inputs(Some(&(0..16).collect::<Vec<_>>()))
                .inner_loop(Some(ITER2_NUM)),
        );
        builder.add_with_config(
            "addu16_2",
            addu16_circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(&(0..16).collect::<Vec<_>>()))
                .inner_loop(Some(ITER2_NUM)),
        );
        let mut execs = builder.build().unwrap();

        let mut it = execs[0]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[0]
            .output_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let input = execs[0].new_data_from_vec(
            (0..1 << 16)
                .map(|i| i & ((1 << 16) - 1))
                .collect::<Vec<_>>(),
        );
        let input_circ = it.transform(&input).unwrap();
        let output_circ = execs[0].execute(&input_circ, 0).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(output.len(), 1 << 16);
        // check results
        for i in 0usize..1 << 16 {
            let expv = (u32::try_from(i).unwrap() + (ITER_NUM * 77)) & ((1 << 16) - 1);
            assert_eq!(expv, output[i], "{}: {}", config_num, i);
        }

        // elem inputs
        let mut it = execs[1]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[1]
            .output_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let input = execs[1].new_data_from_vec(
            (0..1u32 << 16)
                .map(|i| ((i.overflowing_mul(689921).0) % 158591) & ((1 << 16) - 1))
                .collect::<Vec<_>>(),
        );
        let input_circ = it.transform(&input).unwrap();
        let output_circ = execs[1].execute(&input_circ, 0).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(output.len(), 1 << 16);
        // check results
        for i in 0usize..1 << 16 {
            let iv = i as u32;
            let iv = ((iv.overflowing_mul(689921).0) % 158591) & ((1 << 16) - 1);
            let expv = (u32::try_from(i).unwrap() * ITER2_NUM + iv) & ((1 << 16) - 1);
            assert_eq!(expv, output[i], "{}: {}", config_num, i);
        }

        // arg inputs
        let arg: u32 = 481;
        let mut it = execs[2]
            .input_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let mut ot = execs[2]
            .output_transformer(32, &(0..16).collect::<Vec<_>>())
            .unwrap();
        let input = execs[2].new_data_from_vec(
            (0..1u32 << 16)
                .map(|i| ((i.overflowing_mul(689921).0) % 158591) & ((1 << 16) - 1))
                .collect::<Vec<_>>(),
        );
        let input_circ = it.transform(&input).unwrap();
        let output_circ = execs[2].execute(&input_circ, arg as u64).unwrap();
        let output = ot.transform(&output_circ).unwrap().release();
        assert_eq!(output.len(), 1 << 16);
        // check results
        for i in 0usize..1 << 16 {
            let iv = i as u32;
            let iv = ((iv.overflowing_mul(689921).0) % 158591) & ((1 << 16) - 1);
            let expv = (arg * ITER2_NUM + iv) & ((1 << 16) - 1);
            assert_eq!(expv, output[i], "{}: {}", config_num, i);
        }
    }
}

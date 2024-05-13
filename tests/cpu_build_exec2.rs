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
    let addu16_circuit = Circuit::<u32>::from_str(ADDIMMU16_77_CIRCUIT).unwrap();
    for (config_num, (cpu_ext, writer_config, builder_config)) in configs.into_iter().enumerate() {
        const ITER_NUM: u32 = 12;
        let mut builder =
            CPUBuilder::new_with_cpu_ext_and_clang_config(cpu_ext, writer_config, builder_config);
        builder.add_with_config(
            "addu16",
            addu16_circuit.clone(),
            CodeConfig::new().inner_loop(Some(ITER_NUM)),
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
    }
}

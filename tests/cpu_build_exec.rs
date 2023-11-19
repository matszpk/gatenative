use gatenative::clang_writer::*;
use gatenative::cpu_build_exec::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_cpu_builder_and_exec() {
    //let no_opt_neg_config = CPUBuilderConfig{ optimize_negs: false };
    let mut builder = CPUBuilder::new_with_cpu_ext_and_clang_config(
        CPUExtension::NoExtension,
        &CLANG_WRITER_U64_TEST_IMPL,
        None,
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
        assert_eq!((a * b) & 15, c, "{}", i);
    }
}

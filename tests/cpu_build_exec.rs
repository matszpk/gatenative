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
        [(4, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();
    builder.add("mul2x2", circuit, None, None);
    let exec = builder.build().unwrap();
}

use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_loop_basic() {
    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(2, 3),
            Gate::new_xor(2, 3),
            Gate::new_nor(0, 3),
            Gate::new_and(4, 5),
            Gate::new_nimpl(4, 6),
            Gate::new_xor(5, 6),
            Gate::new_xor(8, 9),
            Gate::new_nimpl(9, 1),
        ],
        [(7, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new().inner_loop(Some(10)),
    );
    println!("Code: {}", String::from_utf8(writer.out()).unwrap());
    // assert_eq!(
    //     &String::from_utf8(writer.out()).unwrap(),
    //     r##"void gate_sys_xor(uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    //     const uint32_t zero = 0;
    //     const uint32_t one = 0xffffffff;
    //     uint32_t v0;
    //     uint32_t v1;
    //     uint32_t v2;
    //     uint32_t v3;
    //     v0 = ((arg & 2) != 0) ? one : zero;
    //     v1 = output[0];
    //     v2 = (v0 & v1);
    //     v0 = (v0 ^ v1);
    //     v3 = (v2 & v0);
    //     output[3] = v3;
    //     v3 = ((arg & 1) != 0) ? one : zero;
    //     v1 = ~(v3 | v1);
    //     v2 = (v2 & ~v1);
    //     output[2] = ~v2;
    //     v0 = (v0 ^ v1);
    //     v1 = (v2 ^ v0);
    //     v2 = output[1];
    //     output[1] = v1;
    //     v0 = (v0 & ~v2);
    //     output[0] = ~v0;
    // }
    // "##
    // );
}

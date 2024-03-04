use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_populate_input() {
    let circuit = Circuit::new(
        3,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_and(0, 1),
            Gate::new_nor(5, 6),
        ],
        [(4, false), (7, true)],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#define i0
#define i1
#define i2
    v3 = (v0 ^ v1);
    v4 = (v2 ^ v3);
    output[0] = v4;
    v2 = (v2 & v3);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[1] = ~v0;
}
"##
    );
}

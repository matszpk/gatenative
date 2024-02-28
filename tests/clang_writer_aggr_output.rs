use crate::gencode::generate_code_ext;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_aggregate_output() {
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
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output) {
    unsigned int xxx = 1111;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = input[0];
    v1 = input[1];
    v2 = (v0 ^ v1);
    v3 = input[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    v0 = ~v0;
#define o0 (v4)
#define o1 (v0)
    output[0] |= o0 ^ o1;
#undef o0
#undef o1
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output) {
    const __m128 one = *((const __m128*)one_value);
    unsigned int xxx = 1111;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
#define o0 (v4)
#define o1 (v0)
    output[0] |= o0 ^ o1;
#undef o0
#undef o1
}
"##
    );

    let circuit = Circuit::new(
        3,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_and(0, 1),
            Gate::new_nor(5, 6),
        ],
        [(4, false), (7, true), (4, true)],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1 ^ o2;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output) {
    unsigned int xxx = 1111;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = input[0];
    v1 = input[1];
    v2 = (v0 ^ v1);
    v3 = input[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    v0 = ~v0;
    v1 = ~v4;
#define o0 (v4)
#define o1 (v0)
#define o2 (v1)
    output[0] |= o0 ^ o1 ^ o2;
#undef o0
#undef o1
#undef o2
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1 ^ o2;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output) {
    const __m128 one = *((const __m128*)one_value);
    unsigned int xxx = 1111;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v1 = _mm_xor_ps(v4, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v1)
    output[0] |= o0 ^ o1 ^ o2;
#undef o0
#undef o1
#undef o2
}
"##
    );

    let circuit = Circuit::new(
        3,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_and(0, 1),
            Gate::new_nor(5, 6),
        ],
        [
            (4, false),
            (7, true),
            (4, true),
            (7, false),
            (7, true),
            (4, false),
        ],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output) {
    unsigned int xxx = 1111;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = input[0];
    v1 = input[1];
    v2 = (v0 ^ v1);
    v3 = input[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    v0 = ~v0;
    v2 = ~v4;
    v1 = ~v0;
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output) {
    const __m128 one = *((const __m128*)one_value);
    unsigned int xxx = 1111;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v2 = _mm_xor_ps(v4, one);
    v1 = _mm_xor_ps(v0, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );

    // check double negation - shouldn't be generated
    let circuit = Circuit::new(
        3,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_and(0, 1),
            Gate::new_nor(5, 6),
        ],
        [
            (4, false),
            (7, false),
            (4, true),
            (7, true),
            (7, false),
            (4, false),
        ],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        None,
        false,
        Some("    unsigned int xxx = 1111;\n"),
        Some(
            r##"    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
"##,
        ),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output) {
    const __m128 one = *((const __m128*)one_value);
    unsigned int xxx = 1111;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v0 = _mm_xor_ps(v0, one);
    v2 = _mm_xor_ps(v4, one);
    v1 = _mm_xor_ps(v0, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    output[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );
}

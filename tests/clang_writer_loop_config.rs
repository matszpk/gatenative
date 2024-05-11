use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_loop_config() {
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
        CodeConfig::new()
            .input_placement(Some((&[2, 3, 0, 1], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    void* output, size_t idx) {
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[2];
    v1 = input[3];
    v2 = input[0];
    v3 = input[1];
    }
    v4 = (v2 & v3);
    v2 = (v2 ^ v3);
    v5 = (v4 & v2);
    v0 = ~(v0 | v3);
    v3 = (v4 & ~v0);
    v0 = (v2 ^ v0);
    v2 = (v3 ^ v0);
    v0 = (v0 & ~v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v5;
    output[1] = v3;
    output[2] = v2;
    output[3] = v0;
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE2.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[2, 3, 0, 1], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128i* input,
    void* output, size_t idx) {
    const __m128i one = *((const __m128i*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m128i v0;
    __m128i v1;
    __m128i v2;
    __m128i v3;
    __m128i v4;
    __m128i v5;
    __m128i v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm_loadu_si128((const __m128i*)&input[2]);
    v1 = _mm_loadu_si128((const __m128i*)&input[3]);
    v2 = _mm_loadu_si128((const __m128i*)&input[0]);
    v3 = _mm_loadu_si128((const __m128i*)&input[1]);
    }
    v4 = _mm_and_si128(v2, v3);
    v2 = _mm_xor_si128(v2, v3);
    v5 = _mm_and_si128(v4, v2);
    v0 = _mm_or_si128(v0, v3);
    v3 = _mm_and_si128(v4, v0);
    v0 = _mm_xor_si128(v2, v0);
    v2 = _mm_xor_si128(v3, v0);
    v0 = _mm_or_si128(v0, v1);
    v3 = _mm_xor_si128(v3, one);
    v2 = _mm_xor_si128(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_si128((__m128i*)&output[0], v5);
    _mm_storeu_si128((__m128i*)&output[1], v3);
    _mm_storeu_si128((__m128i*)&output[2], v2);
    _mm_storeu_si128((__m128i*)&output[3], v0);
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    void* output, size_t idx) {
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    }
    v4 = (v2 & v3);
    v2 = (v2 ^ v3);
    v5 = (v4 & v2);
    v0 = ~(v0 | v3);
    v3 = (v4 & ~v0);
    v0 = (v2 ^ v0);
    v2 = (v3 ^ v0);
    v0 = (v0 & ~v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v5;
    output[2] = v3;
    output[3] = v2;
    output[0] = v0;
    } else {
    v6 = v2;
    v2 = v3;
    v3 = v6;
    v1 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm_loadu_ps((const float*)&input[0]);
    v1 = _mm_loadu_ps((const float*)&input[1]);
    v2 = _mm_loadu_ps((const float*)&input[2]);
    v3 = _mm_loadu_ps((const float*)&input[3]);
    }
    v4 = _mm_and_ps(v2, v3);
    v2 = _mm_xor_ps(v2, v3);
    v5 = _mm_and_ps(v4, v2);
    v0 = _mm_or_ps(v0, v3);
    v3 = _mm_and_ps(v4, v0);
    v0 = _mm_xor_ps(v2, v0);
    v2 = _mm_xor_ps(v3, v0);
    v0 = _mm_or_ps(v0, v1);
    v3 = _mm_xor_ps(v3, one);
    v2 = _mm_xor_ps(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_ps((float*)&output[1], v5);
    _mm_storeu_ps((float*)&output[2], v3);
    _mm_storeu_ps((float*)&output[3], v2);
    _mm_storeu_ps((float*)&output[0], v0);
    } else {
    v6 = v2;
    v2 = v3;
    v3 = v6;
    v1 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[3, 0, 1, 2], 4)))
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    void* output, size_t idx) {
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[3];
    v1 = input[0];
    v2 = input[1];
    v3 = input[2];
    }
    v4 = (v2 & v3);
    v2 = (v2 ^ v3);
    v5 = (v4 & v2);
    v0 = ~(v0 | v3);
    v3 = (v4 & ~v0);
    v0 = (v2 ^ v0);
    v2 = (v3 ^ v0);
    v0 = (v0 & ~v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v5;
    output[2] = v3;
    output[3] = v2;
    output[0] = v0;
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[3, 0, 1, 2], 4)))
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m64* input,
    void* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    __m64 v5;
    __m64 v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[3];
    v1 = input[0];
    v2 = input[1];
    v3 = input[2];
    }
    v4 = _m_pand(v2, v3);
    v2 = _m_pxor(v2, v3);
    v5 = _m_pand(v4, v2);
    v0 = _m_por(v0, v3);
    v3 = _m_pand(v4, v0);
    v0 = _m_pxor(v2, v0);
    v2 = _m_pxor(v3, v0);
    v0 = _m_por(v0, v1);
    v3 = _m_pxor(v3, one);
    v2 = _m_pxor(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v5;
    output[2] = v3;
    output[3] = v2;
    output[0] = v0;
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    _m_empty();
    } // loop
}
"##
    );
}

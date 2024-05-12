use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

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
    // single buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new().inner_loop(Some(10)).single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(void* output, size_t idx) {
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
    v0 = output[0];
    v1 = output[1];
    v2 = output[2];
    v3 = output[3];
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
    v1 = v3;
    v3 = v0;
    v0 = v5;
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
        CodeConfig::new().inner_loop(Some(10)).single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(void* output, size_t idx) {
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
    v0 = _mm_loadu_si128((const __m128i*)&output[0]);
    v1 = _mm_loadu_si128((const __m128i*)&output[1]);
    v2 = _mm_loadu_si128((const __m128i*)&output[2]);
    v3 = _mm_loadu_si128((const __m128i*)&output[3]);
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
    v1 = v3;
    v3 = v0;
    v0 = v5;
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
            .single_buffer(true)
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(void* output, size_t idx) {
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
    v0 = output[3];
    v1 = output[0];
    v2 = output[1];
    v3 = output[2];
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
    let mut writer = CLANG_WRITER_INTEL_AVX2.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[3, 0, 1, 2], 4)))
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .single_buffer(true)
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(void* output, size_t idx) {
    const __m256i one = *((const __m256i*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m256i v0;
    __m256i v1;
    __m256i v2;
    __m256i v3;
    __m256i v4;
    __m256i v5;
    __m256i v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm256_loadu_si256((const float*)&output[3]);
    v1 = _mm256_loadu_si256((const float*)&output[0]);
    v2 = _mm256_loadu_si256((const float*)&output[1]);
    v3 = _mm256_loadu_si256((const float*)&output[2]);
    }
    v4 = _mm256_and_si256(v2, v3);
    v2 = _mm256_xor_si256(v2, v3);
    v5 = _mm256_and_si256(v4, v2);
    v0 = _mm256_or_si256(v0, v3);
    v3 = _mm256_and_si256(v4, v0);
    v0 = _mm256_xor_si256(v2, v0);
    v2 = _mm256_xor_si256(v3, v0);
    v0 = _mm256_or_si256(v0, v1);
    v3 = _mm256_xor_si256(v3, one);
    v2 = _mm256_xor_si256(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_si256((float*)&output[1], v5);
    _mm256_storeu_si256((float*)&output[2], v3);
    _mm256_storeu_si256((float*)&output[3], v2);
    _mm256_storeu_si256((float*)&output[0], v0);
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    } // loop
}
"##
    );
    // pop_input
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i1 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const void* input,
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i1 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
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
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // pop_input
    let mut writer = CLANG_WRITER_INTEL_SSE2.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i1 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const void* input,
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i1 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
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
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );

    // with excluded output
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
            Gate::new_nor(8, 10),
        ],
        [(7, false), (8, true), (12, false), (10, false), (11, true)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .exclude_outputs(Some(&[1]))
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
    v4 = ~(v3 | v2);
    v0 = (v0 & ~v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v5;
    output[1] = v4;
    output[2] = v2;
    output[3] = v0;
    } else {
    v3 = v0;
    v1 = v4;
    v0 = v5;
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
            .exclude_outputs(Some(&[1]))
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
    v0 = _mm_loadu_si128((const __m128i*)&input[0]);
    v1 = _mm_loadu_si128((const __m128i*)&input[1]);
    v2 = _mm_loadu_si128((const __m128i*)&input[2]);
    v3 = _mm_loadu_si128((const __m128i*)&input[3]);
    }
    v4 = _mm_and_si128(v2, v3);
    v2 = _mm_xor_si128(v2, v3);
    v5 = _mm_and_si128(v4, v2);
    v0 = _mm_or_si128(v0, v3);
    v3 = _mm_and_si128(v4, v0);
    v0 = _mm_xor_si128(v2, v0);
    v2 = _mm_xor_si128(v3, v0);
    v4 = _mm_andnot_si128(v3, v2);
    v0 = _mm_or_si128(v0, v1);
    v3 = _mm_xor_si128(v3, one);
    v2 = _mm_xor_si128(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_si128((__m128i*)&output[0], v5);
    _mm_storeu_si128((__m128i*)&output[1], v4);
    _mm_storeu_si128((__m128i*)&output[2], v2);
    _mm_storeu_si128((__m128i*)&output[3], v0);
    } else {
    v3 = v0;
    v1 = v4;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // with excluded output and placements
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[3, 2, 1, 0], 4)))
            .output_placement(Some((&[2, 3, 0, 1], 4)))
            .exclude_outputs(Some(&[2]))
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
    v1 = input[2];
    v2 = input[1];
    v3 = input[0];
    }
    v4 = (v2 & v3);
    v2 = (v2 ^ v3);
    v5 = (v4 & v2);
    v0 = ~(v0 | v3);
    v3 = (v4 & ~v0);
    v0 = (v2 ^ v0);
    v2 = (v3 ^ v0);
    v4 = ~(v3 | v2);
    v0 = (v0 & ~v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[2] = v5;
    output[3] = v3;
    output[0] = v2;
    output[1] = v0;
    } else {
    v6 = v0;
    v0 = v3;
    v3 = v2;
    v2 = v6;
    v1 = v5;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[3, 2, 1, 0], 4)))
            .output_placement(Some((&[2, 3, 0, 1], 4)))
            .exclude_outputs(Some(&[2]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256* input,
    void* output, size_t idx) {
    const __m256 one = *((const __m256*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    __m256 v5;
    __m256 v6;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm256_loadu_ps((const float*)&input[3]);
    v1 = _mm256_loadu_ps((const float*)&input[2]);
    v2 = _mm256_loadu_ps((const float*)&input[1]);
    v3 = _mm256_loadu_ps((const float*)&input[0]);
    }
    v4 = _mm256_and_ps(v2, v3);
    v2 = _mm256_xor_ps(v2, v3);
    v5 = _mm256_and_ps(v4, v2);
    v0 = _mm256_or_ps(v0, v3);
    v3 = _mm256_and_ps(v4, v0);
    v0 = _mm256_xor_ps(v2, v0);
    v2 = _mm256_xor_ps(v3, v0);
    v4 = _mm256_andnot_ps(v3, v2);
    v0 = _mm256_or_ps(v0, v1);
    v3 = _mm256_xor_ps(v3, one);
    v2 = _mm256_xor_ps(v2, one);
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_ps((float*)&output[2], v5);
    _mm256_storeu_ps((float*)&output[3], v3);
    _mm256_storeu_ps((float*)&output[0], v2);
    _mm256_storeu_ps((float*)&output[1], v0);
    } else {
    v6 = v0;
    v0 = v3;
    v3 = v2;
    v2 = v6;
    v1 = v5;
    }
    } // loop
}
"##
    );

    // arg input and elem_input
    let circuit = Circuit::<u32>::from_str(
        r##"{
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
        xor(0,4):0
        xor(1,5)
        and(0,4)
        xor(17,18):1
        xor(2,6)
        and(17,18)
        and(1,5)
        nor(21,22)
        xor(20,23):2n
        xor(3,7)
        nimpl(20,23)
        and(2,6)
        nor(26,27)
        xor(25,28):3n
        xor(8,12):4
        xor(9,13)
        nimpl(8,12)
        nimpl(30,32)
        xor(31,33):5
        xor(10,14)
        nor(31,33)
        nimpl(9,13)
        nor(36,37)
        xor(35,38):6
        xor(11,15)
        nor(35,38)
        nimpl(10,14)
        nor(41,42)
        xor(40,43):7
    }(16)
"##,
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const uint32_t* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    v4 = input[4];
    v5 = input[5];
    v6 = input[6];
    v7 = input[7];
    }
    v8 = ((arg & 1) != 0) ? one : zero;
    v9 = (v8 ^ v2);
    v10 = ((arg & 2) != 0) ? one : zero;
    v11 = (v10 ^ v3);
    v2 = (v8 & v2);
    v8 = (v11 ^ v2);
    v12 = ((arg & 4) != 0) ? one : zero;
    v13 = (v0 ^ v12);
    v2 = (v11 & v2);
    v3 = (v10 & v3);
    v2 = ~(v2 | v3);
    v3 = (v13 ^ v2);
    v10 = ((arg & 8) != 0) ? one : zero;
    v1 = (v1 ^ v10);
    v2 = (v13 & ~v2);
    v0 = (v0 & v12);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    v1 = elem_low_bit2;
    v2 = (v4 ^ v1);
    v10 = elem_low_bit3;
    v11 = (v5 ^ v10);
    v1 = (v4 & ~v1);
    v1 = (v2 & ~v1);
    v4 = (v11 ^ v1);
    v12 = elem_low_bit0;
    v13 = (v12 ^ v6);
    v1 = ~(v11 | v1);
    v5 = (v5 & ~v10);
    v1 = ~(v1 | v5);
    v5 = (v13 ^ v1);
    v10 = elem_low_bit1;
    v7 = (v10 ^ v7);
    v1 = ~(v13 | v1);
    v6 = (v12 & ~v6);
    v1 = ~(v1 | v6);
    v1 = (v7 ^ v1);
    v3 = ~v3;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v9;
    output[1] = v8;
    output[2] = v3;
    output[3] = v0;
    output[4] = v2;
    output[5] = v4;
    output[6] = v5;
    output[7] = v1;
    } else {
    v6 = v5;
    v5 = v4;
    v4 = v2;
    v2 = v3;
    v3 = v0;
    v7 = v1;
    v1 = v8;
    v0 = v9;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE2.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const __m128i* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m128i zero = *((const __m128i*)zero_value);
    const __m128i one = *((const __m128i*)one_value);
    const __m128i elem_low_bit0 = *((const __m128i*)elem_index_low_tbl);
    const __m128i elem_low_bit1 = *((const __m128i*)(elem_index_low_tbl + 4));
    const __m128i elem_low_bit2 = *((const __m128i*)(elem_index_low_tbl + 8));
    const __m128i elem_low_bit3 = *((const __m128i*)(elem_index_low_tbl + 12));
    const __m128i elem_low_bit4 = *((const __m128i*)(elem_index_low_tbl + 16));
    const __m128i elem_low_bit5 = *((const __m128i*)(elem_index_low_tbl + 20));
    const __m128i elem_low_bit6 = *((const __m128i*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    __m128i v7;
    __m128i v8;
    __m128i v9;
    __m128i v10;
    __m128i v11;
    __m128i v12;
    __m128i v13;
    __m128i v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm_loadu_si128((const __m128i*)&input[0]);
    v1 = _mm_loadu_si128((const __m128i*)&input[1]);
    v2 = _mm_loadu_si128((const __m128i*)&input[2]);
    v3 = _mm_loadu_si128((const __m128i*)&input[3]);
    v4 = _mm_loadu_si128((const __m128i*)&input[4]);
    v5 = _mm_loadu_si128((const __m128i*)&input[5]);
    v6 = _mm_loadu_si128((const __m128i*)&input[6]);
    v7 = _mm_loadu_si128((const __m128i*)&input[7]);
    }
    v8 = ((arg & 1) != 0) ? one : zero;
    v9 = _mm_xor_si128(v8, v2);
    v10 = ((arg & 2) != 0) ? one : zero;
    v11 = _mm_xor_si128(v10, v3);
    v2 = _mm_and_si128(v8, v2);
    v8 = _mm_xor_si128(v11, v2);
    v12 = ((arg & 4) != 0) ? one : zero;
    v13 = _mm_xor_si128(v0, v12);
    v2 = _mm_and_si128(v11, v2);
    v3 = _mm_and_si128(v10, v3);
    v2 = _mm_or_si128(v2, v3);
    v3 = _mm_xor_si128(v13, v2);
    v10 = ((arg & 8) != 0) ? one : zero;
    v1 = _mm_xor_si128(v1, v10);
    v2 = _mm_and_si128(v13, v2);
    v0 = _mm_and_si128(v0, v12);
    v0 = _mm_or_si128(v2, v0);
    v0 = _mm_xor_si128(v1, v0);
    v1 = elem_low_bit2;
    v2 = _mm_xor_si128(v4, v1);
    v10 = elem_low_bit3;
    v11 = _mm_xor_si128(v5, v10);
    v1 = _mm_andnot_si128(v1, v4);
    v1 = _mm_andnot_si128(v1, v2);
    v4 = _mm_xor_si128(v11, v1);
    v12 = elem_low_bit0;
    v13 = _mm_xor_si128(v12, v6);
    v1 = _mm_or_si128(v11, v1);
    v5 = _mm_andnot_si128(v10, v5);
    v1 = _mm_andnot_si128(v5, v1);
    v5 = _mm_xor_si128(v13, v1);
    v10 = elem_low_bit1;
    v7 = _mm_xor_si128(v10, v7);
    v1 = _mm_or_si128(v13, v1);
    v6 = _mm_andnot_si128(v6, v12);
    v1 = _mm_andnot_si128(v6, v1);
    v1 = _mm_xor_si128(v7, v1);
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_si128((__m128i*)&output[0], v9);
    _mm_storeu_si128((__m128i*)&output[1], v8);
    _mm_storeu_si128((__m128i*)&output[2], v3);
    _mm_storeu_si128((__m128i*)&output[3], v0);
    _mm_storeu_si128((__m128i*)&output[4], v2);
    _mm_storeu_si128((__m128i*)&output[5], v4);
    _mm_storeu_si128((__m128i*)&output[6], v5);
    _mm_storeu_si128((__m128i*)&output[7], v1);
    } else {
    v6 = v5;
    v5 = v4;
    v4 = v2;
    v2 = v3;
    v3 = v0;
    v7 = v1;
    v1 = v8;
    v0 = v9;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13, 14]))
            .exclude_outputs(Some(&[6, 7]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const uint32_t* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    v4 = input[4];
    v5 = input[5];
    }
    v6 = ((arg & 1) != 0) ? one : zero;
    v7 = (v6 ^ v2);
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = (v8 ^ v9);
    v2 = (v6 & v2);
    v6 = (v10 ^ v2);
    v11 = ((arg & 8) != 0) ? one : zero;
    v12 = (v0 ^ v11);
    v2 = (v10 & v2);
    v8 = (v8 & v9);
    v2 = ~(v2 | v8);
    v8 = (v12 ^ v2);
    v9 = ((arg & 16) != 0) ? one : zero;
    v1 = (v1 ^ v9);
    v2 = (v12 & ~v2);
    v0 = (v0 & v11);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    v1 = elem_low_bit2;
    v2 = (v3 ^ v1);
    v9 = elem_low_bit3;
    v10 = (v4 ^ v9);
    v1 = (v3 & ~v1);
    v1 = (v2 & ~v1);
    v3 = (v10 ^ v1);
    v11 = elem_low_bit0;
    v12 = elem_low_bit4;
    v13 = (v11 ^ v12);
    v1 = ~(v10 | v1);
    v4 = (v4 & ~v9);
    v1 = ~(v1 | v4);
    v4 = (v13 ^ v1);
    v9 = elem_low_bit1;
    v5 = (v9 ^ v5);
    v1 = ~(v13 | v1);
    v9 = (v11 & ~v12);
    v1 = ~(v1 | v9);
    v1 = (v5 ^ v1);
    v8 = ~v8;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v7;
    output[1] = v6;
    output[2] = v8;
    output[3] = v0;
    output[4] = v2;
    output[5] = v3;
    } else {
    v5 = v3;
    v3 = v0;
    v4 = v2;
    v1 = v6;
    v0 = v7;
    v2 = v8;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13, 14]))
            .exclude_outputs(Some(&[6, 7]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const __m64* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    const __m64 elem_low_bit0 = *((const __m64*)elem_index_low_tbl);
    const __m64 elem_low_bit1 = *((const __m64*)(elem_index_low_tbl + 2));
    const __m64 elem_low_bit2 = *((const __m64*)(elem_index_low_tbl + 4));
    const __m64 elem_low_bit3 = *((const __m64*)(elem_index_low_tbl + 6));
    const __m64 elem_low_bit4 = *((const __m64*)(elem_index_low_tbl + 8));
    const __m64 elem_low_bit5 = *((const __m64*)(elem_index_low_tbl + 10));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    __m64 v7;
    __m64 v8;
    __m64 v9;
    __m64 v10;
    __m64 v11;
    __m64 v12;
    __m64 v13;
    __m64 v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    v4 = input[4];
    v5 = input[5];
    }
    v6 = ((arg & 1) != 0) ? one : zero;
    v7 = _m_pxor(v6, v2);
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = _m_pxor(v8, v9);
    v2 = _m_pand(v6, v2);
    v6 = _m_pxor(v10, v2);
    v11 = ((arg & 8) != 0) ? one : zero;
    v12 = _m_pxor(v0, v11);
    v2 = _m_pand(v10, v2);
    v8 = _m_pand(v8, v9);
    v2 = _m_por(v2, v8);
    v8 = _m_pxor(v12, v2);
    v9 = ((arg & 16) != 0) ? one : zero;
    v1 = _m_pxor(v1, v9);
    v2 = _m_pand(v12, v2);
    v0 = _m_pand(v0, v11);
    v0 = _m_por(v2, v0);
    v0 = _m_pxor(v1, v0);
    v1 = elem_low_bit2;
    v2 = _m_pxor(v3, v1);
    v9 = elem_low_bit3;
    v10 = _m_pxor(v4, v9);
    v1 = _m_pandn(v1, v3);
    v1 = _m_pandn(v1, v2);
    v3 = _m_pxor(v10, v1);
    v11 = elem_low_bit0;
    v12 = elem_low_bit4;
    v13 = _m_pxor(v11, v12);
    v1 = _m_por(v10, v1);
    v4 = _m_pandn(v9, v4);
    v1 = _m_pandn(v4, v1);
    v4 = _m_pxor(v13, v1);
    v9 = elem_low_bit1;
    v5 = _m_pxor(v9, v5);
    v1 = _m_por(v13, v1);
    v9 = _m_pandn(v12, v11);
    v1 = _m_pandn(v9, v1);
    v1 = _m_pxor(v5, v1);
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v7;
    output[1] = v6;
    output[2] = v8;
    output[3] = v0;
    output[4] = v2;
    output[5] = v3;
    } else {
    v5 = v3;
    v3 = v0;
    v4 = v2;
    v1 = v6;
    v0 = v7;
    v2 = v8;
    }
    _m_empty();
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13, 14]))
            .exclude_outputs(Some(&[6, 7]))
            .input_placement(Some((&[1, 3, 0, 4, 2, 5], 6)))
            .output_placement(Some((&[4, 0, 3, 5, 1, 2], 6)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const uint32_t* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[1];
    v1 = input[3];
    v2 = input[0];
    v3 = input[4];
    v4 = input[2];
    v5 = input[5];
    }
    v6 = ((arg & 1) != 0) ? one : zero;
    v7 = (v6 ^ v2);
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = (v8 ^ v9);
    v2 = (v6 & v2);
    v6 = (v10 ^ v2);
    v11 = ((arg & 8) != 0) ? one : zero;
    v12 = (v0 ^ v11);
    v2 = (v10 & v2);
    v8 = (v8 & v9);
    v2 = ~(v2 | v8);
    v8 = (v12 ^ v2);
    v9 = ((arg & 16) != 0) ? one : zero;
    v1 = (v1 ^ v9);
    v2 = (v12 & ~v2);
    v0 = (v0 & v11);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    v1 = elem_low_bit2;
    v2 = (v3 ^ v1);
    v9 = elem_low_bit3;
    v10 = (v4 ^ v9);
    v1 = (v3 & ~v1);
    v1 = (v2 & ~v1);
    v3 = (v10 ^ v1);
    v11 = elem_low_bit0;
    v12 = elem_low_bit4;
    v13 = (v11 ^ v12);
    v1 = ~(v10 | v1);
    v4 = (v4 & ~v9);
    v1 = ~(v1 | v4);
    v4 = (v13 ^ v1);
    v9 = elem_low_bit1;
    v5 = (v9 ^ v5);
    v1 = ~(v13 | v1);
    v9 = (v11 & ~v12);
    v1 = ~(v1 | v9);
    v1 = (v5 ^ v1);
    v8 = ~v8;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[4] = v7;
    output[0] = v6;
    output[3] = v8;
    output[5] = v0;
    output[1] = v2;
    output[2] = v3;
    } else {
    v5 = v0;
    v0 = v2;
    v4 = v3;
    v2 = v6;
    v3 = v7;
    v1 = v8;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6, 7]))
            .elem_inputs(Some(&[10, 11, 12, 13, 14]))
            .exclude_outputs(Some(&[6, 7]))
            .input_placement(Some((&[1, 3, 0, 4, 2, 5], 6)))
            .output_placement(Some((&[4, 0, 3, 5, 1, 2], 6)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const __m64* input,
    void* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    const __m64 elem_low_bit0 = *((const __m64*)elem_index_low_tbl);
    const __m64 elem_low_bit1 = *((const __m64*)(elem_index_low_tbl + 2));
    const __m64 elem_low_bit2 = *((const __m64*)(elem_index_low_tbl + 4));
    const __m64 elem_low_bit3 = *((const __m64*)(elem_index_low_tbl + 6));
    const __m64 elem_low_bit4 = *((const __m64*)(elem_index_low_tbl + 8));
    const __m64 elem_low_bit5 = *((const __m64*)(elem_index_low_tbl + 10));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    __m64 v7;
    __m64 v8;
    __m64 v9;
    __m64 v10;
    __m64 v11;
    __m64 v12;
    __m64 v13;
    __m64 v14;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[1];
    v1 = input[3];
    v2 = input[0];
    v3 = input[4];
    v4 = input[2];
    v5 = input[5];
    }
    v6 = ((arg & 1) != 0) ? one : zero;
    v7 = _m_pxor(v6, v2);
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = _m_pxor(v8, v9);
    v2 = _m_pand(v6, v2);
    v6 = _m_pxor(v10, v2);
    v11 = ((arg & 8) != 0) ? one : zero;
    v12 = _m_pxor(v0, v11);
    v2 = _m_pand(v10, v2);
    v8 = _m_pand(v8, v9);
    v2 = _m_por(v2, v8);
    v8 = _m_pxor(v12, v2);
    v9 = ((arg & 16) != 0) ? one : zero;
    v1 = _m_pxor(v1, v9);
    v2 = _m_pand(v12, v2);
    v0 = _m_pand(v0, v11);
    v0 = _m_por(v2, v0);
    v0 = _m_pxor(v1, v0);
    v1 = elem_low_bit2;
    v2 = _m_pxor(v3, v1);
    v9 = elem_low_bit3;
    v10 = _m_pxor(v4, v9);
    v1 = _m_pandn(v1, v3);
    v1 = _m_pandn(v1, v2);
    v3 = _m_pxor(v10, v1);
    v11 = elem_low_bit0;
    v12 = elem_low_bit4;
    v13 = _m_pxor(v11, v12);
    v1 = _m_por(v10, v1);
    v4 = _m_pandn(v9, v4);
    v1 = _m_pandn(v4, v1);
    v4 = _m_pxor(v13, v1);
    v9 = elem_low_bit1;
    v5 = _m_pxor(v9, v5);
    v1 = _m_por(v13, v1);
    v9 = _m_pandn(v12, v11);
    v1 = _m_pandn(v9, v1);
    v1 = _m_pxor(v5, v1);
    if (iter == iter_max - 1 || stop != 0) {
    output[4] = v7;
    output[0] = v6;
    output[3] = v8;
    output[5] = v0;
    output[1] = v2;
    output[2] = v3;
    } else {
    v5 = v0;
    v0 = v2;
    v4 = v3;
    v2 = v6;
    v3 = v7;
    v1 = v8;
    }
    _m_empty();
    } // loop
}
"##
    );
    // pop_input from buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6]))
            .pop_input_code(Some("    i10 = ((TYPE_NAME*)buffer)[0];"))
            .pop_from_buffer(Some(&[10, 11, 12, 13]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const uint32_t* input,
    void* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i10 (v6)
#define i11 (v7)
#define i12 (v8)
#define i13 (v9)
    i10 = ((TYPE_NAME*)buffer)[0];
#undef i10
#undef i11
#undef i12
#undef i13
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    v4 = input[4];
    v5 = input[5];
    v10 = input[6];
    v11 = input[7];
    }
    v12 = ((arg & 1) != 0) ? one : zero;
    v13 = (v12 ^ v2);
    v14 = ((arg & 2) != 0) ? one : zero;
    v15 = ((arg & 4) != 0) ? one : zero;
    v16 = (v14 ^ v15);
    v2 = (v12 & v2);
    v12 = (v16 ^ v2);
    v17 = ((arg & 8) != 0) ? one : zero;
    v18 = (v0 ^ v17);
    v2 = (v16 & v2);
    v14 = (v14 & v15);
    v2 = ~(v2 | v14);
    v14 = (v18 ^ v2);
    v1 = (v1 ^ v3);
    v2 = (v18 & ~v2);
    v0 = (v0 & v17);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    v1 = (v4 ^ v8);
    v2 = (v5 ^ v9);
    v3 = (v4 & ~v8);
    v3 = (v1 & ~v3);
    v4 = (v2 ^ v3);
    v8 = (v6 ^ v10);
    v2 = ~(v2 | v3);
    v3 = (v5 & ~v9);
    v2 = ~(v2 | v3);
    v3 = (v8 ^ v2);
    v5 = (v7 ^ v11);
    v2 = ~(v8 | v2);
    v6 = (v6 & ~v10);
    v2 = ~(v2 | v6);
    v2 = (v5 ^ v2);
    v14 = ~v14;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v13;
    output[1] = v12;
    output[2] = v14;
    output[3] = v0;
    output[4] = v1;
    output[5] = v4;
    output[6] = v3;
    output[7] = v2;
    } else {
    v10 = v3;
    v3 = v0;
    v5 = v4;
    v4 = v1;
    v11 = v2;
    v1 = v12;
    v0 = v13;
    v2 = v14;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[0, 1, 5, 6]))
            .pop_input_code(Some("    i10 = ((TYPE_NAME*)buffer)[0];"))
            .pop_from_buffer(Some(&[10, 11, 12, 13]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const __m256* input,
    void* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m256 zero = *((const __m256*)zero_value);
    const __m256 one = *((const __m256*)one_value);
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    __m256 v5;
    __m256 v6;
    __m256 v7;
    __m256 v8;
    __m256 v9;
    __m256 v10;
    __m256 v11;
    __m256 v12;
    __m256 v13;
    __m256 v14;
    __m256 v15;
    __m256 v16;
    __m256 v17;
    __m256 v18;
    __m256 v19;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i10 (v6)
#define i11 (v7)
#define i12 (v8)
#define i13 (v9)
    i10 = ((TYPE_NAME*)buffer)[0];
#undef i10
#undef i11
#undef i12
#undef i13
    if (iter == 0) {
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[1]);
    v2 = _mm256_loadu_ps((const float*)&input[2]);
    v3 = _mm256_loadu_ps((const float*)&input[3]);
    v4 = _mm256_loadu_ps((const float*)&input[4]);
    v5 = _mm256_loadu_ps((const float*)&input[5]);
    v10 = _mm256_loadu_ps((const float*)&input[6]);
    v11 = _mm256_loadu_ps((const float*)&input[7]);
    }
    v12 = ((arg & 1) != 0) ? one : zero;
    v13 = _mm256_xor_ps(v12, v2);
    v14 = ((arg & 2) != 0) ? one : zero;
    v15 = ((arg & 4) != 0) ? one : zero;
    v16 = _mm256_xor_ps(v14, v15);
    v2 = _mm256_and_ps(v12, v2);
    v12 = _mm256_xor_ps(v16, v2);
    v17 = ((arg & 8) != 0) ? one : zero;
    v18 = _mm256_xor_ps(v0, v17);
    v2 = _mm256_and_ps(v16, v2);
    v14 = _mm256_and_ps(v14, v15);
    v2 = _mm256_or_ps(v2, v14);
    v14 = _mm256_xor_ps(v18, v2);
    v1 = _mm256_xor_ps(v1, v3);
    v2 = _mm256_and_ps(v18, v2);
    v0 = _mm256_and_ps(v0, v17);
    v0 = _mm256_or_ps(v2, v0);
    v0 = _mm256_xor_ps(v1, v0);
    v1 = _mm256_xor_ps(v4, v8);
    v2 = _mm256_xor_ps(v5, v9);
    v3 = _mm256_andnot_ps(v8, v4);
    v3 = _mm256_andnot_ps(v3, v1);
    v4 = _mm256_xor_ps(v2, v3);
    v8 = _mm256_xor_ps(v6, v10);
    v2 = _mm256_or_ps(v2, v3);
    v3 = _mm256_andnot_ps(v9, v5);
    v2 = _mm256_andnot_ps(v3, v2);
    v3 = _mm256_xor_ps(v8, v2);
    v5 = _mm256_xor_ps(v7, v11);
    v2 = _mm256_or_ps(v8, v2);
    v6 = _mm256_andnot_ps(v10, v6);
    v2 = _mm256_andnot_ps(v6, v2);
    v2 = _mm256_xor_ps(v5, v2);
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_ps((float*)&output[0], v13);
    _mm256_storeu_ps((float*)&output[1], v12);
    _mm256_storeu_ps((float*)&output[2], v14);
    _mm256_storeu_ps((float*)&output[3], v0);
    _mm256_storeu_ps((float*)&output[4], v1);
    _mm256_storeu_ps((float*)&output[5], v4);
    _mm256_storeu_ps((float*)&output[6], v3);
    _mm256_storeu_ps((float*)&output[7], v2);
    } else {
    v10 = v3;
    v3 = v0;
    v5 = v4;
    v4 = v1;
    v11 = v2;
    v1 = v12;
    v0 = v13;
    v2 = v14;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "addsub",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 1, 5, 6]))
            .pop_input_code(Some("    i10 = ((TYPE_NAME*)buffer)[0];"))
            .pop_from_buffer(Some(&[10, 11, 12, 13]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_addsub(const uint32_t* input,
    void* output, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i10 (v6)
#define i11 (v7)
#define i12 (v8)
#define i13 (v9)
    i10 = ((TYPE_NAME*)buffer)[0];
#undef i10
#undef i11
#undef i12
#undef i13
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v2 = input[2];
    v3 = input[3];
    v4 = input[4];
    v5 = input[5];
    v10 = input[6];
    v11 = input[7];
    }
    v12 = elem_low_bit0;
    v13 = (v12 ^ v2);
    v14 = elem_low_bit1;
    v15 = elem_low_bit2;
    v16 = (v14 ^ v15);
    v2 = (v12 & v2);
    v12 = (v16 ^ v2);
    v17 = elem_low_bit3;
    v18 = (v0 ^ v17);
    v2 = (v16 & v2);
    v14 = (v14 & v15);
    v2 = ~(v2 | v14);
    v14 = (v18 ^ v2);
    v1 = (v1 ^ v3);
    v2 = (v18 & ~v2);
    v0 = (v0 & v17);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    v1 = (v4 ^ v8);
    v2 = (v5 ^ v9);
    v3 = (v4 & ~v8);
    v3 = (v1 & ~v3);
    v4 = (v2 ^ v3);
    v8 = (v6 ^ v10);
    v2 = ~(v2 | v3);
    v3 = (v5 & ~v9);
    v2 = ~(v2 | v3);
    v3 = (v8 ^ v2);
    v5 = (v7 ^ v11);
    v2 = ~(v8 | v2);
    v6 = (v6 & ~v10);
    v2 = ~(v2 | v6);
    v2 = (v5 ^ v2);
    v14 = ~v14;
    v0 = ~v0;
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v13;
    output[1] = v12;
    output[2] = v14;
    output[3] = v0;
    output[4] = v1;
    output[5] = v4;
    output[6] = v3;
    output[7] = v2;
    } else {
    v10 = v3;
    v3 = v0;
    v5 = v4;
    v4 = v1;
    v11 = v2;
    v1 = v12;
    v0 = v13;
    v2 = v14;
    }
    } // loop
}
"##
    );
}

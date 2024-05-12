use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

// use std::str::FromStr;

#[test]
fn test_clang_writer_loop_config_2() {
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

    // aggr_output
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;")),
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
    } else {
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // aggr_output
    let mut writer = CLANG_WRITER_INTEL_AVX2.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256i* input,
    void* output, size_t idx) {
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
    v0 = _mm256_loadu_si256((const float*)&input[0]);
    v1 = _mm256_loadu_si256((const float*)&input[1]);
    v2 = _mm256_loadu_si256((const float*)&input[2]);
    v3 = _mm256_loadu_si256((const float*)&input[3]);
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
    } else {
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // pop_input and aggr_output
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)input)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100)),
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
    i3 = ((TYPE_NAME*)input)[0];
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
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
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)input)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100)),
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
    i3 = ((TYPE_NAME*)input)[0];
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
    } else {
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // pop_input and aggr_output with single_buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100))
            .single_buffer(true),
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i3 = ((TYPE_NAME*)output)[0];
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
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
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100))
            .single_buffer(true),
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i3 = ((TYPE_NAME*)output)[0];
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
    } else {
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
    // pop_input and aggr_output with single_buffer, placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100))
            .input_placement(Some((&[3, 0, 1, 2], 4)))
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .single_buffer(true),
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i3 = ((TYPE_NAME*)output)[0];
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
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
            .inner_loop(Some(10))
            .pop_input_code(Some("    i3 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(100))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_output_len(Some(100))
            .input_placement(Some((&[3, 0, 1, 2], 4)))
            .output_placement(Some((&[1, 2, 3, 0], 4)))
            .single_buffer(true),
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
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i3 = ((TYPE_NAME*)output)[0];
#undef i0
#undef i1
#undef i2
#undef i3
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
#define o0 (v5)
#define o1 (v3)
#define o2 (v2)
#define o3 (v0)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
    if (iter == iter_max - 1 || stop != 0) {
    } else {
    v1 = v0;
    v0 = v2;
    v2 = v5;
    }
    } // loop
}
"##
    );
}

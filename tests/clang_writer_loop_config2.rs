use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

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
    // aggr_output to buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
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
    v8 = (v0 ^ v2);
    output[0] = v8;
    v9 = (v1 ^ v3);
    v0 = (v0 & v2);
    v2 = (v9 ^ v0);
    output[1] = v2;
    v10 = ((arg & 1) != 0) ? one : zero;
    v11 = ((arg & 4) != 0) ? one : zero;
    v12 = (v10 ^ v11);
    v0 = (v9 & v0);
    v1 = (v1 & v3);
    v0 = ~(v0 | v1);
    v1 = (v12 ^ v0);
    output[2] = ~v1;
    v3 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 8) != 0) ? one : zero;
    v3 = (v3 ^ v9);
    v0 = (v12 & ~v0);
    v9 = (v10 & v11);
    v0 = ~(v0 | v9);
    v0 = (v3 ^ v0);
    output[3] = ~v0;
    v3 = ((arg & 16) != 0) ? one : zero;
    v9 = ((arg & 64) != 0) ? one : zero;
    v10 = (v3 ^ v9);
    output[4] = v10;
    v11 = ((arg & 32) != 0) ? one : zero;
    v12 = ((arg & 128) != 0) ? one : zero;
    v13 = (v11 ^ v12);
    v3 = (v3 & ~v9);
    v3 = (v10 & ~v3);
    v9 = (v13 ^ v3);
    output[5] = v9;
    v14 = (v4 ^ v6);
    v3 = ~(v13 | v3);
    v11 = (v11 & ~v12);
    v3 = ~(v3 | v11);
    v11 = (v14 ^ v3);
    output[6] = v11;
    v5 = (v5 ^ v7);
    v3 = ~(v14 | v3);
    v4 = (v4 & ~v6);
    v3 = ~(v3 | v4);
    v3 = (v5 ^ v3);
    output[7] = v3;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v8)
#define o2 (v1)
#define o3 (v0)
#define o5 (v9)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v8;
    output[1] = v2;
    output[2] = v1;
    output[3] = v0;
    output[4] = v10;
    output[5] = v9;
    output[6] = v11;
    output[7] = v3;
    } else {
    v7 = v3;
    v3 = v0;
    v15 = v1;
    v1 = v2;
    v2 = v15;
    v0 = v8;
    v5 = v9;
    v4 = v10;
    v6 = v11;
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
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    __m128* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
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
    __m128 v7;
    __m128 v8;
    __m128 v9;
    __m128 v10;
    __m128 v11;
    __m128 v12;
    __m128 v13;
    __m128 v14;
    __m128 v15;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm_loadu_ps((const float*)&input[0]);
    v1 = _mm_loadu_ps((const float*)&input[1]);
    v2 = _mm_loadu_ps((const float*)&input[2]);
    v3 = _mm_loadu_ps((const float*)&input[3]);
    v4 = _mm_loadu_ps((const float*)&input[4]);
    v5 = _mm_loadu_ps((const float*)&input[5]);
    v6 = _mm_loadu_ps((const float*)&input[6]);
    v7 = _mm_loadu_ps((const float*)&input[7]);
    }
    v8 = _mm_xor_ps(v0, v2);
    _mm_storeu_ps((float*)&output[0], v8);
    v9 = _mm_xor_ps(v1, v3);
    v0 = _mm_and_ps(v0, v2);
    v2 = _mm_xor_ps(v9, v0);
    _mm_storeu_ps((float*)&output[1], v2);
    v10 = ((arg & 1) != 0) ? one : zero;
    v11 = ((arg & 4) != 0) ? one : zero;
    v12 = _mm_xor_ps(v10, v11);
    v0 = _mm_and_ps(v9, v0);
    v1 = _mm_and_ps(v1, v3);
    v0 = _mm_or_ps(v0, v1);
    v1 = _mm_xor_ps(v12, v0);
    _mm_storeu_ps((float*)&output[2], v1);
    v3 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 8) != 0) ? one : zero;
    v3 = _mm_xor_ps(v3, v9);
    v0 = _mm_and_ps(v12, v0);
    v9 = _mm_and_ps(v10, v11);
    v0 = _mm_or_ps(v0, v9);
    v0 = _mm_xor_ps(v3, v0);
    _mm_storeu_ps((float*)&output[3], v0);
    v3 = ((arg & 16) != 0) ? one : zero;
    v9 = ((arg & 64) != 0) ? one : zero;
    v10 = _mm_xor_ps(v3, v9);
    _mm_storeu_ps((float*)&output[4], v10);
    v11 = ((arg & 32) != 0) ? one : zero;
    v12 = ((arg & 128) != 0) ? one : zero;
    v13 = _mm_xor_ps(v11, v12);
    v3 = _mm_andnot_ps(v9, v3);
    v3 = _mm_andnot_ps(v3, v10);
    v9 = _mm_xor_ps(v13, v3);
    _mm_storeu_ps((float*)&output[5], v9);
    v14 = _mm_xor_ps(v4, v6);
    v3 = _mm_or_ps(v13, v3);
    v11 = _mm_andnot_ps(v12, v11);
    v3 = _mm_andnot_ps(v11, v3);
    v11 = _mm_xor_ps(v14, v3);
    _mm_storeu_ps((float*)&output[6], v11);
    v5 = _mm_xor_ps(v5, v7);
    v3 = _mm_or_ps(v14, v3);
    v4 = _mm_andnot_ps(v6, v4);
    v3 = _mm_andnot_ps(v4, v3);
    v3 = _mm_xor_ps(v5, v3);
    _mm_storeu_ps((float*)&output[7], v3);
#define o0 (v8)
#define o2 (v1)
#define o3 (v0)
#define o5 (v9)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_ps((float*)&output[0], v8);
    _mm_storeu_ps((float*)&output[1], v2);
    _mm_storeu_ps((float*)&output[2], v1);
    _mm_storeu_ps((float*)&output[3], v0);
    _mm_storeu_ps((float*)&output[4], v10);
    _mm_storeu_ps((float*)&output[5], v9);
    _mm_storeu_ps((float*)&output[6], v11);
    _mm_storeu_ps((float*)&output[7], v3);
    } else {
    v7 = v3;
    v3 = v0;
    v15 = v1;
    v1 = v2;
    v2 = v15;
    v0 = v8;
    v5 = v9;
    v4 = v10;
    v6 = v11;
    }
    } // loop
}
"##
    );
    // aggr_output to buffer with output exclusion
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .elem_inputs(Some(&[0, 1, 14, 15]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
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
    }
    v4 = elem_low_bit0;
    v5 = (v4 ^ v0);
    v6 = elem_low_bit1;
    v7 = (v6 ^ v1);
    v0 = (v4 & v0);
    v4 = (v7 ^ v0);
    output[0] = v4;
    v8 = ((arg & 1) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = (v8 ^ v9);
    v0 = (v7 & v0);
    v1 = (v6 & v1);
    v0 = ~(v0 | v1);
    v1 = (v10 ^ v0);
    v6 = ((arg & 2) != 0) ? one : zero;
    v7 = ((arg & 8) != 0) ? one : zero;
    v6 = (v6 ^ v7);
    v0 = (v10 & ~v0);
    v7 = (v8 & v9);
    v0 = ~(v0 | v7);
    v0 = (v6 ^ v0);
    v6 = ((arg & 16) != 0) ? one : zero;
    v7 = ((arg & 64) != 0) ? one : zero;
    v8 = (v6 ^ v7);
    output[1] = v8;
    v9 = ((arg & 32) != 0) ? one : zero;
    v10 = ((arg & 128) != 0) ? one : zero;
    v11 = (v9 ^ v10);
    v6 = (v6 & ~v7);
    v6 = (v8 & ~v6);
    v7 = (v11 ^ v6);
    v12 = elem_low_bit2;
    v13 = (v2 ^ v12);
    v6 = ~(v11 | v6);
    v9 = (v9 & ~v10);
    v6 = ~(v6 | v9);
    v9 = (v13 ^ v6);
    output[2] = v9;
    v10 = elem_low_bit3;
    v3 = (v3 ^ v10);
    v6 = ~(v13 | v6);
    v2 = (v2 & ~v12);
    v2 = ~(v6 | v2);
    v2 = (v3 ^ v2);
    output[3] = v2;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v5)
#define o2 (v1)
#define o3 (v0)
#define o5 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v4;
    output[1] = v8;
    output[2] = v9;
    output[3] = v2;
    } else {
    v3 = v2;
    v0 = v4;
    v1 = v8;
    v2 = v9;
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
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .elem_inputs(Some(&[0, 1, 14, 15]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256* input,
    __m256* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m256 zero = *((const __m256*)zero_value);
    const __m256 one = *((const __m256*)one_value);
    const __m256 elem_low_bit0 = *((const __m256*)elem_index_low_tbl);
    const __m256 elem_low_bit1 = *((const __m256*)(elem_index_low_tbl + 8));
    const __m256 elem_low_bit2 = *((const __m256*)(elem_index_low_tbl + 16));
    const __m256 elem_low_bit3 = *((const __m256*)(elem_index_low_tbl + 24));
    const __m256 elem_low_bit4 = *((const __m256*)(elem_index_low_tbl + 32));
    const __m256 elem_low_bit5 = *((const __m256*)(elem_index_low_tbl + 40));
    const __m256 elem_low_bit6 = *((const __m256*)(elem_index_low_tbl + 48));
    const __m256 elem_low_bit7 = *((const __m256*)(elem_index_low_tbl + 56));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[1]);
    v2 = _mm256_loadu_ps((const float*)&input[2]);
    v3 = _mm256_loadu_ps((const float*)&input[3]);
    }
    v4 = elem_low_bit0;
    v5 = _mm256_xor_ps(v4, v0);
    v6 = elem_low_bit1;
    v7 = _mm256_xor_ps(v6, v1);
    v0 = _mm256_and_ps(v4, v0);
    v4 = _mm256_xor_ps(v7, v0);
    _mm256_storeu_ps((float*)&output[0], v4);
    v8 = ((arg & 1) != 0) ? one : zero;
    v9 = ((arg & 4) != 0) ? one : zero;
    v10 = _mm256_xor_ps(v8, v9);
    v0 = _mm256_and_ps(v7, v0);
    v1 = _mm256_and_ps(v6, v1);
    v0 = _mm256_or_ps(v0, v1);
    v1 = _mm256_xor_ps(v10, v0);
    v6 = ((arg & 2) != 0) ? one : zero;
    v7 = ((arg & 8) != 0) ? one : zero;
    v6 = _mm256_xor_ps(v6, v7);
    v0 = _mm256_and_ps(v10, v0);
    v7 = _mm256_and_ps(v8, v9);
    v0 = _mm256_or_ps(v0, v7);
    v0 = _mm256_xor_ps(v6, v0);
    v6 = ((arg & 16) != 0) ? one : zero;
    v7 = ((arg & 64) != 0) ? one : zero;
    v8 = _mm256_xor_ps(v6, v7);
    _mm256_storeu_ps((float*)&output[1], v8);
    v9 = ((arg & 32) != 0) ? one : zero;
    v10 = ((arg & 128) != 0) ? one : zero;
    v11 = _mm256_xor_ps(v9, v10);
    v6 = _mm256_andnot_ps(v7, v6);
    v6 = _mm256_andnot_ps(v6, v8);
    v7 = _mm256_xor_ps(v11, v6);
    v12 = elem_low_bit2;
    v13 = _mm256_xor_ps(v2, v12);
    v6 = _mm256_or_ps(v11, v6);
    v9 = _mm256_andnot_ps(v10, v9);
    v6 = _mm256_andnot_ps(v9, v6);
    v9 = _mm256_xor_ps(v13, v6);
    _mm256_storeu_ps((float*)&output[2], v9);
    v10 = elem_low_bit3;
    v3 = _mm256_xor_ps(v3, v10);
    v6 = _mm256_or_ps(v13, v6);
    v2 = _mm256_andnot_ps(v12, v2);
    v2 = _mm256_andnot_ps(v2, v6);
    v2 = _mm256_xor_ps(v3, v2);
    _mm256_storeu_ps((float*)&output[3], v2);
#define o0 (v5)
#define o2 (v1)
#define o3 (v0)
#define o5 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_ps((float*)&output[0], v4);
    _mm256_storeu_ps((float*)&output[1], v8);
    _mm256_storeu_ps((float*)&output[2], v9);
    _mm256_storeu_ps((float*)&output[3], v2);
    } else {
    v3 = v2;
    v0 = v4;
    v1 = v8;
    v2 = v9;
    }
    } // loop
}
"##
    );
    // aggr_output to buffer with placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .input_placement(Some((&[3, 4, 0, 6, 7, 2, 5, 1], 8)))
            .output_placement(Some((&[7, 5, 2, 6, 3, 0, 1, 4], 8)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
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
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[3];
    v1 = input[4];
    v2 = input[0];
    v3 = input[6];
    v4 = input[7];
    v5 = input[2];
    v6 = input[5];
    v7 = input[1];
    }
    v8 = (v0 ^ v2);
    output[7] = v8;
    v9 = (v1 ^ v3);
    v0 = (v0 & v2);
    v2 = (v9 ^ v0);
    output[5] = v2;
    v10 = ((arg & 1) != 0) ? one : zero;
    v11 = ((arg & 4) != 0) ? one : zero;
    v12 = (v10 ^ v11);
    v0 = (v9 & v0);
    v1 = (v1 & v3);
    v0 = ~(v0 | v1);
    v1 = (v12 ^ v0);
    output[2] = ~v1;
    v3 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 8) != 0) ? one : zero;
    v3 = (v3 ^ v9);
    v0 = (v12 & ~v0);
    v9 = (v10 & v11);
    v0 = ~(v0 | v9);
    v0 = (v3 ^ v0);
    output[6] = ~v0;
    v3 = ((arg & 16) != 0) ? one : zero;
    v9 = ((arg & 64) != 0) ? one : zero;
    v10 = (v3 ^ v9);
    output[3] = v10;
    v11 = ((arg & 32) != 0) ? one : zero;
    v12 = ((arg & 128) != 0) ? one : zero;
    v13 = (v11 ^ v12);
    v3 = (v3 & ~v9);
    v3 = (v10 & ~v3);
    v9 = (v13 ^ v3);
    output[0] = v9;
    v14 = (v4 ^ v6);
    v3 = ~(v13 | v3);
    v11 = (v11 & ~v12);
    v3 = ~(v3 | v11);
    v11 = (v14 ^ v3);
    output[1] = v11;
    v5 = (v5 ^ v7);
    v3 = ~(v14 | v3);
    v4 = (v4 & ~v6);
    v3 = ~(v3 | v4);
    v3 = (v5 ^ v3);
    output[4] = v3;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v8)
#define o2 (v1)
#define o3 (v0)
#define o5 (v9)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[7] = v8;
    output[5] = v2;
    output[2] = v1;
    output[6] = v0;
    output[3] = v10;
    output[0] = v9;
    output[1] = v11;
    output[4] = v3;
    } else {
    v5 = v1;
    v1 = v3;
    v3 = v0;
    v6 = v2;
    v4 = v8;
    v2 = v9;
    v0 = v10;
    v7 = v11;
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
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .input_placement(Some((&[3, 4, 0, 6, 7, 2, 5, 1], 8)))
            .output_placement(Some((&[7, 5, 2, 6, 3, 0, 1, 4], 8)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    __m128* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
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
    __m128 v7;
    __m128 v8;
    __m128 v9;
    __m128 v10;
    __m128 v11;
    __m128 v12;
    __m128 v13;
    __m128 v14;
    __m128 v15;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = _mm_loadu_ps((const float*)&input[3]);
    v1 = _mm_loadu_ps((const float*)&input[4]);
    v2 = _mm_loadu_ps((const float*)&input[0]);
    v3 = _mm_loadu_ps((const float*)&input[6]);
    v4 = _mm_loadu_ps((const float*)&input[7]);
    v5 = _mm_loadu_ps((const float*)&input[2]);
    v6 = _mm_loadu_ps((const float*)&input[5]);
    v7 = _mm_loadu_ps((const float*)&input[1]);
    }
    v8 = _mm_xor_ps(v0, v2);
    _mm_storeu_ps((float*)&output[7], v8);
    v9 = _mm_xor_ps(v1, v3);
    v0 = _mm_and_ps(v0, v2);
    v2 = _mm_xor_ps(v9, v0);
    _mm_storeu_ps((float*)&output[5], v2);
    v10 = ((arg & 1) != 0) ? one : zero;
    v11 = ((arg & 4) != 0) ? one : zero;
    v12 = _mm_xor_ps(v10, v11);
    v0 = _mm_and_ps(v9, v0);
    v1 = _mm_and_ps(v1, v3);
    v0 = _mm_or_ps(v0, v1);
    v1 = _mm_xor_ps(v12, v0);
    _mm_storeu_ps((float*)&output[2], v1);
    v3 = ((arg & 2) != 0) ? one : zero;
    v9 = ((arg & 8) != 0) ? one : zero;
    v3 = _mm_xor_ps(v3, v9);
    v0 = _mm_and_ps(v12, v0);
    v9 = _mm_and_ps(v10, v11);
    v0 = _mm_or_ps(v0, v9);
    v0 = _mm_xor_ps(v3, v0);
    _mm_storeu_ps((float*)&output[6], v0);
    v3 = ((arg & 16) != 0) ? one : zero;
    v9 = ((arg & 64) != 0) ? one : zero;
    v10 = _mm_xor_ps(v3, v9);
    _mm_storeu_ps((float*)&output[3], v10);
    v11 = ((arg & 32) != 0) ? one : zero;
    v12 = ((arg & 128) != 0) ? one : zero;
    v13 = _mm_xor_ps(v11, v12);
    v3 = _mm_andnot_ps(v9, v3);
    v3 = _mm_andnot_ps(v3, v10);
    v9 = _mm_xor_ps(v13, v3);
    _mm_storeu_ps((float*)&output[0], v9);
    v14 = _mm_xor_ps(v4, v6);
    v3 = _mm_or_ps(v13, v3);
    v11 = _mm_andnot_ps(v12, v11);
    v3 = _mm_andnot_ps(v11, v3);
    v11 = _mm_xor_ps(v14, v3);
    _mm_storeu_ps((float*)&output[1], v11);
    v5 = _mm_xor_ps(v5, v7);
    v3 = _mm_or_ps(v14, v3);
    v4 = _mm_andnot_ps(v6, v4);
    v3 = _mm_andnot_ps(v4, v3);
    v3 = _mm_xor_ps(v5, v3);
    _mm_storeu_ps((float*)&output[4], v3);
#define o0 (v8)
#define o2 (v1)
#define o3 (v0)
#define o5 (v9)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_ps((float*)&output[7], v8);
    _mm_storeu_ps((float*)&output[5], v2);
    _mm_storeu_ps((float*)&output[2], v1);
    _mm_storeu_ps((float*)&output[6], v0);
    _mm_storeu_ps((float*)&output[3], v10);
    _mm_storeu_ps((float*)&output[0], v9);
    _mm_storeu_ps((float*)&output[1], v11);
    _mm_storeu_ps((float*)&output[4], v3);
    } else {
    v5 = v1;
    v1 = v3;
    v3 = v0;
    v6 = v2;
    v4 = v8;
    v2 = v9;
    v0 = v10;
    v7 = v11;
    }
    } // loop
}
"##
    );
    // pop_from_buffer, aggr_to_buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
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
#define i2 (v2)
#define i3 (v3)
#define i5 (v5)
#define i7 (v7)
#define i8 (v8)
#define i10 (v10)
#define i12 (v12)
#define i13 (v13)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    if (iter == 0) {
    v0 = input[0];
    v1 = input[1];
    v4 = input[2];
    v6 = input[3];
    v9 = input[4];
    v11 = input[5];
    v14 = input[6];
    v15 = input[7];
    }
    v16 = (v0 ^ v4);
    output[0] = v16;
    v17 = (v1 ^ v5);
    v0 = (v0 & v4);
    v4 = (v17 ^ v0);
    output[1] = v4;
    v18 = (v2 ^ v6);
    v0 = (v17 & v0);
    v1 = (v1 & v5);
    v0 = ~(v0 | v1);
    v1 = (v18 ^ v0);
    output[2] = ~v1;
    v3 = (v3 ^ v7);
    v0 = (v18 & ~v0);
    v2 = (v2 & v6);
    v0 = ~(v0 | v2);
    v0 = (v3 ^ v0);
    output[3] = ~v0;
    v2 = (v8 ^ v12);
    output[4] = v2;
    v3 = (v9 ^ v13);
    v5 = (v8 & ~v12);
    v5 = (v2 & ~v5);
    v6 = (v3 ^ v5);
    output[5] = v6;
    v7 = (v10 ^ v14);
    v3 = ~(v3 | v5);
    v5 = (v9 & ~v13);
    v3 = ~(v3 | v5);
    v5 = (v7 ^ v3);
    output[6] = v5;
    v8 = (v11 ^ v15);
    v3 = ~(v7 | v3);
    v7 = (v10 & ~v14);
    v3 = ~(v3 | v7);
    v3 = (v8 ^ v3);
    output[7] = v3;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v16)
#define o2 (v1)
#define o3 (v0)
#define o5 (v6)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v16;
    output[1] = v4;
    output[2] = v1;
    output[3] = v0;
    output[4] = v2;
    output[5] = v6;
    output[6] = v5;
    output[7] = v3;
    } else {
    v11 = v6;
    v6 = v0;
    v19 = v1;
    v1 = v4;
    v4 = v19;
    v9 = v2;
    v15 = v3;
    v14 = v5;
    v0 = v16;
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
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256i* input,
    __m256i* output, void* buffer, size_t idx) {
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
    __m256i v7;
    __m256i v8;
    __m256i v9;
    __m256i v10;
    __m256i v11;
    __m256i v12;
    __m256i v13;
    __m256i v14;
    __m256i v15;
    __m256i v16;
    __m256i v17;
    __m256i v18;
    __m256i v19;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i2 (v2)
#define i3 (v3)
#define i5 (v5)
#define i7 (v7)
#define i8 (v8)
#define i10 (v10)
#define i12 (v12)
#define i13 (v13)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    if (iter == 0) {
    v0 = _mm256_loadu_si256((const float*)&input[0]);
    v1 = _mm256_loadu_si256((const float*)&input[1]);
    v4 = _mm256_loadu_si256((const float*)&input[2]);
    v6 = _mm256_loadu_si256((const float*)&input[3]);
    v9 = _mm256_loadu_si256((const float*)&input[4]);
    v11 = _mm256_loadu_si256((const float*)&input[5]);
    v14 = _mm256_loadu_si256((const float*)&input[6]);
    v15 = _mm256_loadu_si256((const float*)&input[7]);
    }
    v16 = _mm256_xor_si256(v0, v4);
    _mm256_storeu_si256((float*)&output[0], v16);
    v17 = _mm256_xor_si256(v1, v5);
    v0 = _mm256_and_si256(v0, v4);
    v4 = _mm256_xor_si256(v17, v0);
    _mm256_storeu_si256((float*)&output[1], v4);
    v18 = _mm256_xor_si256(v2, v6);
    v0 = _mm256_and_si256(v17, v0);
    v1 = _mm256_and_si256(v1, v5);
    v0 = _mm256_or_si256(v0, v1);
    v1 = _mm256_xor_si256(v18, v0);
    _mm256_storeu_si256((float*)&output[2], v1);
    v3 = _mm256_xor_si256(v3, v7);
    v0 = _mm256_and_si256(v18, v0);
    v2 = _mm256_and_si256(v2, v6);
    v0 = _mm256_or_si256(v0, v2);
    v0 = _mm256_xor_si256(v3, v0);
    _mm256_storeu_si256((float*)&output[3], v0);
    v2 = _mm256_xor_si256(v8, v12);
    _mm256_storeu_si256((float*)&output[4], v2);
    v3 = _mm256_xor_si256(v9, v13);
    v5 = _mm256_andnot_si256(v12, v8);
    v5 = _mm256_andnot_si256(v5, v2);
    v6 = _mm256_xor_si256(v3, v5);
    _mm256_storeu_si256((float*)&output[5], v6);
    v7 = _mm256_xor_si256(v10, v14);
    v3 = _mm256_or_si256(v3, v5);
    v5 = _mm256_andnot_si256(v13, v9);
    v3 = _mm256_andnot_si256(v5, v3);
    v5 = _mm256_xor_si256(v7, v3);
    _mm256_storeu_si256((float*)&output[6], v5);
    v8 = _mm256_xor_si256(v11, v15);
    v3 = _mm256_or_si256(v7, v3);
    v7 = _mm256_andnot_si256(v14, v10);
    v3 = _mm256_andnot_si256(v7, v3);
    v3 = _mm256_xor_si256(v8, v3);
    _mm256_storeu_si256((float*)&output[7], v3);
#define o0 (v16)
#define o2 (v1)
#define o3 (v0)
#define o5 (v6)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_si256((float*)&output[0], v16);
    _mm256_storeu_si256((float*)&output[1], v4);
    _mm256_storeu_si256((float*)&output[2], v1);
    _mm256_storeu_si256((float*)&output[3], v0);
    _mm256_storeu_si256((float*)&output[4], v2);
    _mm256_storeu_si256((float*)&output[5], v6);
    _mm256_storeu_si256((float*)&output[6], v5);
    _mm256_storeu_si256((float*)&output[7], v3);
    } else {
    v11 = v6;
    v6 = v0;
    v19 = v1;
    v1 = v4;
    v4 = v19;
    v9 = v2;
    v15 = v3;
    v14 = v5;
    v0 = v16;
    }
    } // loop
}
"##
    );
    // pop_from_buffer, aggr_to_buffer with single_buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)output)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .single_buffer(true)
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(uint32_t* output, void* buffer, size_t idx) {
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
#define i2 (v2)
#define i3 (v3)
#define i5 (v5)
#define i7 (v7)
#define i8 (v8)
#define i10 (v10)
#define i12 (v12)
#define i13 (v13)
    i2 = ((TYPE_NAME*)output)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    if (iter == 0) {
    v0 = output[0];
    v1 = output[1];
    v4 = output[2];
    v6 = output[3];
    v9 = output[4];
    v11 = output[5];
    v14 = output[6];
    v15 = output[7];
    }
    v16 = (v0 ^ v4);
    output[0] = v16;
    v17 = (v1 ^ v5);
    v0 = (v0 & v4);
    v4 = (v17 ^ v0);
    output[1] = v4;
    v18 = (v2 ^ v6);
    v0 = (v17 & v0);
    v1 = (v1 & v5);
    v0 = ~(v0 | v1);
    v1 = (v18 ^ v0);
    output[2] = ~v1;
    v3 = (v3 ^ v7);
    v0 = (v18 & ~v0);
    v2 = (v2 & v6);
    v0 = ~(v0 | v2);
    v0 = (v3 ^ v0);
    output[3] = ~v0;
    v2 = (v8 ^ v12);
    output[4] = v2;
    v3 = (v9 ^ v13);
    v5 = (v8 & ~v12);
    v5 = (v2 & ~v5);
    v6 = (v3 ^ v5);
    output[5] = v6;
    v7 = (v10 ^ v14);
    v3 = ~(v3 | v5);
    v5 = (v9 & ~v13);
    v3 = ~(v3 | v5);
    v5 = (v7 ^ v3);
    output[6] = v5;
    v8 = (v11 ^ v15);
    v3 = ~(v7 | v3);
    v7 = (v10 & ~v14);
    v3 = ~(v3 | v7);
    v3 = (v8 ^ v3);
    output[7] = v3;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v16)
#define o2 (v1)
#define o3 (v0)
#define o5 (v6)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v16;
    output[1] = v4;
    output[2] = v1;
    output[3] = v0;
    output[4] = v2;
    output[5] = v6;
    output[6] = v5;
    output[7] = v3;
    } else {
    v11 = v6;
    v6 = v0;
    v19 = v1;
    v1 = v4;
    v4 = v19;
    v9 = v2;
    v15 = v3;
    v14 = v5;
    v0 = v16;
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
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)output)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .single_buffer(true)
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(__m256* output, void* buffer, size_t idx) {
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
#define i2 (v2)
#define i3 (v3)
#define i5 (v5)
#define i7 (v7)
#define i8 (v8)
#define i10 (v10)
#define i12 (v12)
#define i13 (v13)
    i2 = ((TYPE_NAME*)output)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    if (iter == 0) {
    v0 = _mm256_loadu_ps((const float*)&output[0]);
    v1 = _mm256_loadu_ps((const float*)&output[1]);
    v4 = _mm256_loadu_ps((const float*)&output[2]);
    v6 = _mm256_loadu_ps((const float*)&output[3]);
    v9 = _mm256_loadu_ps((const float*)&output[4]);
    v11 = _mm256_loadu_ps((const float*)&output[5]);
    v14 = _mm256_loadu_ps((const float*)&output[6]);
    v15 = _mm256_loadu_ps((const float*)&output[7]);
    }
    v16 = _mm256_xor_ps(v0, v4);
    _mm256_storeu_ps((float*)&output[0], v16);
    v17 = _mm256_xor_ps(v1, v5);
    v0 = _mm256_and_ps(v0, v4);
    v4 = _mm256_xor_ps(v17, v0);
    _mm256_storeu_ps((float*)&output[1], v4);
    v18 = _mm256_xor_ps(v2, v6);
    v0 = _mm256_and_ps(v17, v0);
    v1 = _mm256_and_ps(v1, v5);
    v0 = _mm256_or_ps(v0, v1);
    v1 = _mm256_xor_ps(v18, v0);
    _mm256_storeu_ps((float*)&output[2], v1);
    v3 = _mm256_xor_ps(v3, v7);
    v0 = _mm256_and_ps(v18, v0);
    v2 = _mm256_and_ps(v2, v6);
    v0 = _mm256_or_ps(v0, v2);
    v0 = _mm256_xor_ps(v3, v0);
    _mm256_storeu_ps((float*)&output[3], v0);
    v2 = _mm256_xor_ps(v8, v12);
    _mm256_storeu_ps((float*)&output[4], v2);
    v3 = _mm256_xor_ps(v9, v13);
    v5 = _mm256_andnot_ps(v12, v8);
    v5 = _mm256_andnot_ps(v5, v2);
    v6 = _mm256_xor_ps(v3, v5);
    _mm256_storeu_ps((float*)&output[5], v6);
    v7 = _mm256_xor_ps(v10, v14);
    v3 = _mm256_or_ps(v3, v5);
    v5 = _mm256_andnot_ps(v13, v9);
    v3 = _mm256_andnot_ps(v5, v3);
    v5 = _mm256_xor_ps(v7, v3);
    _mm256_storeu_ps((float*)&output[6], v5);
    v8 = _mm256_xor_ps(v11, v15);
    v3 = _mm256_or_ps(v7, v3);
    v7 = _mm256_andnot_ps(v14, v10);
    v3 = _mm256_andnot_ps(v7, v3);
    v3 = _mm256_xor_ps(v8, v3);
    _mm256_storeu_ps((float*)&output[7], v3);
#define o0 (v16)
#define o2 (v1)
#define o3 (v0)
#define o5 (v6)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm256_storeu_ps((float*)&output[0], v16);
    _mm256_storeu_ps((float*)&output[1], v4);
    _mm256_storeu_ps((float*)&output[2], v1);
    _mm256_storeu_ps((float*)&output[3], v0);
    _mm256_storeu_ps((float*)&output[4], v2);
    _mm256_storeu_ps((float*)&output[5], v6);
    _mm256_storeu_ps((float*)&output[6], v5);
    _mm256_storeu_ps((float*)&output[7], v3);
    } else {
    v11 = v6;
    v6 = v0;
    v19 = v1;
    v1 = v4;
    v4 = v19;
    v9 = v2;
    v15 = v3;
    v14 = v5;
    v0 = v16;
    }
    } // loop
}
"##
    );
}

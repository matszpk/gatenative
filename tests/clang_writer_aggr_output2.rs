use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

#[test]
fn test_clang_writer_aggregate_output_to_buffer_2() {
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
            .aggr_to_buffer(Some(&[0, 2, 3, 5])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
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
    v0 = input[0];
    v1 = input[2];
    v2 = (v0 ^ v1);
    output[0] = v2;
    v3 = input[1];
    v4 = input[3];
    v5 = (v3 ^ v4);
    v0 = (v0 & v1);
    v1 = (v5 ^ v0);
    output[1] = v1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v6 = ((arg & 4) != 0) ? one : zero;
    v7 = (v1 ^ v6);
    v0 = (v5 & v0);
    v3 = (v3 & v4);
    v0 = ~(v0 | v3);
    v3 = (v7 ^ v0);
    output[2] = ~v3;
    v4 = ((arg & 2) != 0) ? one : zero;
    v5 = ((arg & 8) != 0) ? one : zero;
    v4 = (v4 ^ v5);
    v0 = (v7 & ~v0);
    v1 = (v1 & v6);
    v0 = ~(v0 | v1);
    v0 = (v4 ^ v0);
    output[3] = ~v0;
    v1 = ((arg & 16) != 0) ? one : zero;
    v4 = ((arg & 64) != 0) ? one : zero;
    v5 = (v1 ^ v4);
    output[4] = v5;
    v6 = ((arg & 32) != 0) ? one : zero;
    v7 = ((arg & 128) != 0) ? one : zero;
    v8 = (v6 ^ v7);
    v1 = (v1 & ~v4);
    v1 = (v5 & ~v1);
    v4 = (v8 ^ v1);
    output[5] = v4;
    v5 = input[4];
    v9 = input[6];
    v10 = (v5 ^ v9);
    v1 = ~(v8 | v1);
    v6 = (v6 & ~v7);
    v1 = ~(v1 | v6);
    v6 = (v10 ^ v1);
    output[6] = v6;
    v6 = input[5];
    v7 = input[7];
    v6 = (v6 ^ v7);
    v1 = ~(v10 | v1);
    v5 = (v5 & ~v9);
    v1 = ~(v1 | v5);
    v1 = (v6 ^ v1);
    output[7] = v1;
    v3 = ~v3;
    v0 = ~v0;
#define o0 (v2)
#define o2 (v3)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .aggr_to_buffer(Some(&[0, 2, 3, 5])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    __m128* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
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
    v0 = _mm_loadu_ps((const float*)&input[0]);
    v1 = _mm_loadu_ps((const float*)&input[2]);
    v2 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[0], v2);
    v3 = _mm_loadu_ps((const float*)&input[1]);
    v4 = _mm_loadu_ps((const float*)&input[3]);
    v5 = _mm_xor_ps(v3, v4);
    v0 = _mm_and_ps(v0, v1);
    v1 = _mm_xor_ps(v5, v0);
    _mm_storeu_ps((float*)&output[1], v1);
    v1 = ((arg & 1) != 0) ? one : zero;
    v6 = ((arg & 4) != 0) ? one : zero;
    v7 = _mm_xor_ps(v1, v6);
    v0 = _mm_and_ps(v5, v0);
    v3 = _mm_and_ps(v3, v4);
    v0 = _mm_or_ps(v0, v3);
    v3 = _mm_xor_ps(v7, v0);
    _mm_storeu_ps((float*)&output[2], v3);
    v4 = ((arg & 2) != 0) ? one : zero;
    v5 = ((arg & 8) != 0) ? one : zero;
    v4 = _mm_xor_ps(v4, v5);
    v0 = _mm_and_ps(v7, v0);
    v1 = _mm_and_ps(v1, v6);
    v0 = _mm_or_ps(v0, v1);
    v0 = _mm_xor_ps(v4, v0);
    _mm_storeu_ps((float*)&output[3], v0);
    v1 = ((arg & 16) != 0) ? one : zero;
    v4 = ((arg & 64) != 0) ? one : zero;
    v5 = _mm_xor_ps(v1, v4);
    _mm_storeu_ps((float*)&output[4], v5);
    v6 = ((arg & 32) != 0) ? one : zero;
    v7 = ((arg & 128) != 0) ? one : zero;
    v8 = _mm_xor_ps(v6, v7);
    v1 = _mm_andnot_ps(v4, v1);
    v1 = _mm_andnot_ps(v1, v5);
    v4 = _mm_xor_ps(v8, v1);
    _mm_storeu_ps((float*)&output[5], v4);
    v5 = _mm_loadu_ps((const float*)&input[4]);
    v9 = _mm_loadu_ps((const float*)&input[6]);
    v10 = _mm_xor_ps(v5, v9);
    v1 = _mm_or_ps(v8, v1);
    v6 = _mm_andnot_ps(v7, v6);
    v1 = _mm_andnot_ps(v6, v1);
    v6 = _mm_xor_ps(v10, v1);
    _mm_storeu_ps((float*)&output[6], v6);
    v6 = _mm_loadu_ps((const float*)&input[5]);
    v7 = _mm_loadu_ps((const float*)&input[7]);
    v6 = _mm_xor_ps(v6, v7);
    v1 = _mm_or_ps(v10, v1);
    v5 = _mm_andnot_ps(v9, v5);
    v1 = _mm_andnot_ps(v5, v1);
    v1 = _mm_xor_ps(v6, v1);
    _mm_storeu_ps((float*)&output[7], v1);
#define o0 (v2)
#define o2 (v3)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .arg_inputs(Some(&[2, 3, 6, 7, 8, 9, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .input_placement(Some((&[3, 4, 0, 6, 7, 2, 5, 1], 8)))
            .output_placement(Some((&[7, 5, 2, 6, 3, 0, 1, 4], 8))),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
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
    v0 = input[3];
    v1 = input[0];
    v2 = (v0 ^ v1);
    output[7] = v2;
    v3 = input[4];
    v4 = input[6];
    v5 = (v3 ^ v4);
    v0 = (v0 & v1);
    v1 = (v5 ^ v0);
    output[5] = v1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v6 = ((arg & 4) != 0) ? one : zero;
    v7 = (v1 ^ v6);
    v0 = (v5 & v0);
    v3 = (v3 & v4);
    v0 = ~(v0 | v3);
    v3 = (v7 ^ v0);
    output[2] = ~v3;
    v4 = ((arg & 2) != 0) ? one : zero;
    v5 = ((arg & 8) != 0) ? one : zero;
    v4 = (v4 ^ v5);
    v0 = (v7 & ~v0);
    v1 = (v1 & v6);
    v0 = ~(v0 | v1);
    v0 = (v4 ^ v0);
    output[6] = ~v0;
    v1 = ((arg & 16) != 0) ? one : zero;
    v4 = ((arg & 64) != 0) ? one : zero;
    v5 = (v1 ^ v4);
    output[3] = v5;
    v6 = ((arg & 32) != 0) ? one : zero;
    v7 = ((arg & 128) != 0) ? one : zero;
    v8 = (v6 ^ v7);
    v1 = (v1 & ~v4);
    v1 = (v5 & ~v1);
    v4 = (v8 ^ v1);
    output[0] = v4;
    v5 = input[7];
    v9 = input[5];
    v10 = (v5 ^ v9);
    v1 = ~(v8 | v1);
    v6 = (v6 & ~v7);
    v1 = ~(v1 | v6);
    v6 = (v10 ^ v1);
    output[1] = v6;
    v6 = input[2];
    v7 = input[1];
    v6 = (v6 ^ v7);
    v1 = ~(v10 | v1);
    v5 = (v5 & ~v9);
    v1 = ~(v1 | v5);
    v1 = (v6 ^ v1);
    output[4] = v1;
    v3 = ~v3;
    v0 = ~v0;
#define o0 (v2)
#define o2 (v3)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .input_placement(Some((&[3, 4, 0, 6, 7, 2, 5, 1], 8)))
            .output_placement(Some((&[7, 5, 2, 6, 3, 0, 1, 4], 8))),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256* input,
    __m256* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m256 zero = *((const __m256*)zero_value);
    const __m256 one = *((const __m256*)one_value);
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
    v0 = _mm256_loadu_ps((const float*)&input[3]);
    v1 = _mm256_loadu_ps((const float*)&input[0]);
    v2 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[7], v2);
    v3 = _mm256_loadu_ps((const float*)&input[4]);
    v4 = _mm256_loadu_ps((const float*)&input[6]);
    v5 = _mm256_xor_ps(v3, v4);
    v0 = _mm256_and_ps(v0, v1);
    v1 = _mm256_xor_ps(v5, v0);
    _mm256_storeu_ps((float*)&output[5], v1);
    v1 = ((arg & 1) != 0) ? one : zero;
    v6 = ((arg & 4) != 0) ? one : zero;
    v7 = _mm256_xor_ps(v1, v6);
    v0 = _mm256_and_ps(v5, v0);
    v3 = _mm256_and_ps(v3, v4);
    v0 = _mm256_or_ps(v0, v3);
    v3 = _mm256_xor_ps(v7, v0);
    _mm256_storeu_ps((float*)&output[2], v3);
    v4 = ((arg & 2) != 0) ? one : zero;
    v5 = ((arg & 8) != 0) ? one : zero;
    v4 = _mm256_xor_ps(v4, v5);
    v0 = _mm256_and_ps(v7, v0);
    v1 = _mm256_and_ps(v1, v6);
    v0 = _mm256_or_ps(v0, v1);
    v0 = _mm256_xor_ps(v4, v0);
    _mm256_storeu_ps((float*)&output[6], v0);
    v1 = ((arg & 16) != 0) ? one : zero;
    v4 = ((arg & 64) != 0) ? one : zero;
    v5 = _mm256_xor_ps(v1, v4);
    _mm256_storeu_ps((float*)&output[3], v5);
    v6 = ((arg & 32) != 0) ? one : zero;
    v7 = ((arg & 128) != 0) ? one : zero;
    v8 = _mm256_xor_ps(v6, v7);
    v1 = _mm256_andnot_ps(v4, v1);
    v1 = _mm256_andnot_ps(v1, v5);
    v4 = _mm256_xor_ps(v8, v1);
    _mm256_storeu_ps((float*)&output[0], v4);
    v5 = _mm256_loadu_ps((const float*)&input[7]);
    v9 = _mm256_loadu_ps((const float*)&input[5]);
    v10 = _mm256_xor_ps(v5, v9);
    v1 = _mm256_or_ps(v8, v1);
    v6 = _mm256_andnot_ps(v7, v6);
    v1 = _mm256_andnot_ps(v6, v1);
    v6 = _mm256_xor_ps(v10, v1);
    _mm256_storeu_ps((float*)&output[1], v6);
    v6 = _mm256_loadu_ps((const float*)&input[2]);
    v7 = _mm256_loadu_ps((const float*)&input[1]);
    v6 = _mm256_xor_ps(v6, v7);
    v1 = _mm256_or_ps(v10, v1);
    v5 = _mm256_andnot_ps(v9, v5);
    v1 = _mm256_andnot_ps(v5, v1);
    v1 = _mm256_xor_ps(v6, v1);
    _mm256_storeu_ps((float*)&output[4], v1);
#define o0 (v2)
#define o2 (v3)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
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
#define i2 (v0)
#define i3 (v1)
#define i5 (v2)
#define i7 (v3)
#define i8 (v4)
#define i10 (v5)
#define i12 (v6)
#define i13 (v7)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    v8 = input[0];
    v9 = input[2];
    v10 = (v8 ^ v9);
    output[0] = v10;
    v11 = input[1];
    v12 = (v11 ^ v2);
    v8 = (v8 & v9);
    v9 = (v12 ^ v8);
    output[1] = v9;
    v9 = input[3];
    v13 = (v0 ^ v9);
    v8 = (v12 & v8);
    v2 = (v11 & v2);
    v2 = ~(v8 | v2);
    v8 = (v13 ^ v2);
    output[2] = ~v8;
    v1 = (v1 ^ v3);
    v2 = (v13 & ~v2);
    v0 = (v0 & v9);
    v0 = ~(v2 | v0);
    v0 = (v1 ^ v0);
    output[3] = ~v0;
    v1 = (v4 ^ v6);
    output[4] = v1;
    v2 = input[4];
    v3 = (v2 ^ v7);
    v4 = (v4 & ~v6);
    v1 = (v1 & ~v4);
    v4 = (v3 ^ v1);
    output[5] = v4;
    v6 = input[6];
    v9 = (v5 ^ v6);
    v1 = ~(v3 | v1);
    v2 = (v2 & ~v7);
    v1 = ~(v1 | v2);
    v2 = (v9 ^ v1);
    output[6] = v2;
    v2 = input[5];
    v3 = input[7];
    v2 = (v2 ^ v3);
    v1 = ~(v9 | v1);
    v3 = (v5 & ~v6);
    v1 = ~(v1 | v3);
    v1 = (v2 ^ v1);
    output[7] = v1;
    v8 = ~v8;
    v0 = ~v0;
#define o0 (v10)
#define o2 (v8)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 5, 7, 8, 10, 12, 13]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    __m128* output, void* buffer, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
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
#define i2 (v0)
#define i3 (v1)
#define i5 (v2)
#define i7 (v3)
#define i8 (v4)
#define i10 (v5)
#define i12 (v6)
#define i13 (v7)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i5
#undef i7
#undef i8
#undef i10
#undef i12
#undef i13
    v8 = _mm_loadu_ps((const float*)&input[0]);
    v9 = _mm_loadu_ps((const float*)&input[2]);
    v10 = _mm_xor_ps(v8, v9);
    _mm_storeu_ps((float*)&output[0], v10);
    v11 = _mm_loadu_ps((const float*)&input[1]);
    v12 = _mm_xor_ps(v11, v2);
    v8 = _mm_and_ps(v8, v9);
    v9 = _mm_xor_ps(v12, v8);
    _mm_storeu_ps((float*)&output[1], v9);
    v9 = _mm_loadu_ps((const float*)&input[3]);
    v13 = _mm_xor_ps(v0, v9);
    v8 = _mm_and_ps(v12, v8);
    v2 = _mm_and_ps(v11, v2);
    v2 = _mm_or_ps(v8, v2);
    v8 = _mm_xor_ps(v13, v2);
    _mm_storeu_ps((float*)&output[2], v8);
    v1 = _mm_xor_ps(v1, v3);
    v2 = _mm_and_ps(v13, v2);
    v0 = _mm_and_ps(v0, v9);
    v0 = _mm_or_ps(v2, v0);
    v0 = _mm_xor_ps(v1, v0);
    _mm_storeu_ps((float*)&output[3], v0);
    v1 = _mm_xor_ps(v4, v6);
    _mm_storeu_ps((float*)&output[4], v1);
    v2 = _mm_loadu_ps((const float*)&input[4]);
    v3 = _mm_xor_ps(v2, v7);
    v4 = _mm_andnot_ps(v6, v4);
    v1 = _mm_andnot_ps(v4, v1);
    v4 = _mm_xor_ps(v3, v1);
    _mm_storeu_ps((float*)&output[5], v4);
    v6 = _mm_loadu_ps((const float*)&input[6]);
    v9 = _mm_xor_ps(v5, v6);
    v1 = _mm_or_ps(v3, v1);
    v2 = _mm_andnot_ps(v7, v2);
    v1 = _mm_andnot_ps(v2, v1);
    v2 = _mm_xor_ps(v9, v1);
    _mm_storeu_ps((float*)&output[6], v2);
    v2 = _mm_loadu_ps((const float*)&input[5]);
    v3 = _mm_loadu_ps((const float*)&input[7]);
    v2 = _mm_xor_ps(v2, v3);
    v1 = _mm_or_ps(v9, v1);
    v3 = _mm_andnot_ps(v6, v5);
    v1 = _mm_andnot_ps(v3, v1);
    v1 = _mm_xor_ps(v2, v1);
    _mm_storeu_ps((float*)&output[7], v1);
#define o0 (v10)
#define o2 (v8)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
}
"##
    );
    // pop_from_buffer, aggr_to_buffer with output exclusion, arg_inputs and elem_inputs
    // with placements
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8))),
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
#define i2 (v0)
#define i3 (v1)
#define i6 (v2)
#define i10 (v3)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    v4 = input[0];
    v5 = elem_low_bit0;
    v6 = (v4 ^ v5);
    v7 = input[2];
    v8 = elem_low_bit1;
    v9 = (v7 ^ v8);
    v4 = (v4 & v5);
    v5 = (v9 ^ v4);
    output[5] = v5;
    v5 = (v0 ^ v2);
    v4 = (v9 & v4);
    v7 = (v7 & v8);
    v4 = ~(v4 | v7);
    v7 = (v5 ^ v4);
    output[7] = ~v7;
    v8 = elem_low_bit2;
    v1 = (v1 ^ v8);
    v4 = (v5 & ~v4);
    v0 = (v0 & v2);
    v0 = ~(v4 | v0);
    v0 = (v1 ^ v0);
    output[3] = ~v0;
    v1 = elem_low_bit3;
    v2 = ((arg & 1) != 0) ? one : zero;
    v4 = (v1 ^ v2);
    output[2] = v4;
    v5 = input[1];
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = (v5 ^ v8);
    v1 = (v1 & ~v2);
    v1 = (v4 & ~v1);
    v2 = (v9 ^ v1);
    v4 = input[5];
    v10 = (v3 ^ v4);
    v1 = ~(v9 | v1);
    v5 = (v5 & ~v8);
    v1 = ~(v1 | v5);
    v5 = (v10 ^ v1);
    output[0] = v5;
    v5 = input[3];
    v8 = input[7];
    v5 = (v5 ^ v8);
    v1 = ~(v10 | v1);
    v3 = (v3 & ~v4);
    v1 = ~(v1 | v3);
    v1 = (v5 ^ v1);
    output[1] = v1;
    v7 = ~v7;
    v0 = ~v0;
#define o0 (v6)
#define o2 (v7)
#define o3 (v0)
#define o5 (v2)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8))),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128i* input,
    __m128i* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
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
#define i2 (v0)
#define i3 (v1)
#define i6 (v2)
#define i10 (v3)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    v4 = _mm_loadu_si128((const __m128i*)&input[0]);
    v5 = elem_low_bit0;
    v6 = _mm_xor_si128(v4, v5);
    v7 = _mm_loadu_si128((const __m128i*)&input[2]);
    v8 = elem_low_bit1;
    v9 = _mm_xor_si128(v7, v8);
    v4 = _mm_and_si128(v4, v5);
    v5 = _mm_xor_si128(v9, v4);
    _mm_storeu_si128((__m128i*)&output[5], v5);
    v5 = _mm_xor_si128(v0, v2);
    v4 = _mm_and_si128(v9, v4);
    v7 = _mm_and_si128(v7, v8);
    v4 = _mm_or_si128(v4, v7);
    v7 = _mm_xor_si128(v5, v4);
    _mm_storeu_si128((__m128i*)&output[7], v7);
    v8 = elem_low_bit2;
    v1 = _mm_xor_si128(v1, v8);
    v4 = _mm_and_si128(v5, v4);
    v0 = _mm_and_si128(v0, v2);
    v0 = _mm_or_si128(v4, v0);
    v0 = _mm_xor_si128(v1, v0);
    _mm_storeu_si128((__m128i*)&output[3], v0);
    v1 = elem_low_bit3;
    v2 = ((arg & 1) != 0) ? one : zero;
    v4 = _mm_xor_si128(v1, v2);
    _mm_storeu_si128((__m128i*)&output[2], v4);
    v5 = _mm_loadu_si128((const __m128i*)&input[1]);
    v8 = ((arg & 2) != 0) ? one : zero;
    v9 = _mm_xor_si128(v5, v8);
    v1 = _mm_andnot_si128(v2, v1);
    v1 = _mm_andnot_si128(v1, v4);
    v2 = _mm_xor_si128(v9, v1);
    v4 = _mm_loadu_si128((const __m128i*)&input[5]);
    v10 = _mm_xor_si128(v3, v4);
    v1 = _mm_or_si128(v9, v1);
    v5 = _mm_andnot_si128(v8, v5);
    v1 = _mm_andnot_si128(v5, v1);
    v5 = _mm_xor_si128(v10, v1);
    _mm_storeu_si128((__m128i*)&output[0], v5);
    v5 = _mm_loadu_si128((const __m128i*)&input[3]);
    v8 = _mm_loadu_si128((const __m128i*)&input[7]);
    v5 = _mm_xor_si128(v5, v8);
    v1 = _mm_or_si128(v10, v1);
    v3 = _mm_andnot_si128(v4, v3);
    v1 = _mm_andnot_si128(v3, v1);
    v1 = _mm_xor_si128(v5, v1);
    _mm_storeu_si128((__m128i*)&output[1], v1);
#define o0 (v6)
#define o2 (v7)
#define o3 (v0)
#define o5 (v2)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
}
"##
    );
    // pop_from_buffer, aggr_to_buffer with output exclusion, arg_inputs and elem_inputs
    // with placements with single_buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)buffer)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)buffer)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
#define i2 (v0)
#define i3 (v1)
#define i6 (v2)
#define i10 (v3)
    i2 = ((TYPE_NAME*)buffer)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    v4 = output[0];
    v5 = elem_low_bit0;
    v6 = (v4 ^ v5);
    v7 = output[2];
    v8 = elem_low_bit1;
    v9 = (v7 ^ v8);
    v4 = (v4 & v5);
    v5 = (v9 ^ v4);
    v10 = output[5];
    output[5] = v5;
    v5 = (v0 ^ v2);
    v4 = (v9 & v4);
    v7 = (v7 & v8);
    v4 = ~(v4 | v7);
    v7 = (v5 ^ v4);
    v8 = output[7];
    output[7] = ~v7;
    v9 = elem_low_bit2;
    v1 = (v1 ^ v9);
    v4 = (v5 & ~v4);
    v0 = (v0 & v2);
    v0 = ~(v4 | v0);
    v0 = (v1 ^ v0);
    v1 = output[3];
    output[3] = ~v0;
    v2 = elem_low_bit3;
    v4 = ((arg & 1) != 0) ? one : zero;
    v5 = (v2 ^ v4);
    output[2] = v5;
    v9 = output[1];
    v11 = ((arg & 2) != 0) ? one : zero;
    v12 = (v9 ^ v11);
    v2 = (v2 & ~v4);
    v2 = (v5 & ~v2);
    v4 = (v12 ^ v2);
    v5 = (v3 ^ v10);
    v2 = ~(v12 | v2);
    v9 = (v9 & ~v11);
    v2 = ~(v2 | v9);
    v9 = (v5 ^ v2);
    output[0] = v9;
    v1 = (v1 ^ v8);
    v2 = ~(v5 | v2);
    v3 = (v3 & ~v10);
    v2 = ~(v2 | v3);
    v1 = (v1 ^ v2);
    output[1] = v1;
    v7 = ~v7;
    v0 = ~v0;
#define o0 (v6)
#define o2 (v7)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)buffer)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
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
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)buffer)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)buffer)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(__m256i* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m256i zero = *((const __m256i*)zero_value);
    const __m256i one = *((const __m256i*)one_value);
    const __m256i elem_low_bit0 = *((const __m256i*)elem_index_low_tbl);
    const __m256i elem_low_bit1 = *((const __m256i*)(elem_index_low_tbl + 8));
    const __m256i elem_low_bit2 = *((const __m256i*)(elem_index_low_tbl + 16));
    const __m256i elem_low_bit3 = *((const __m256i*)(elem_index_low_tbl + 24));
    const __m256i elem_low_bit4 = *((const __m256i*)(elem_index_low_tbl + 32));
    const __m256i elem_low_bit5 = *((const __m256i*)(elem_index_low_tbl + 40));
    const __m256i elem_low_bit6 = *((const __m256i*)(elem_index_low_tbl + 48));
    const __m256i elem_low_bit7 = *((const __m256i*)(elem_index_low_tbl + 56));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
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
#define i2 (v0)
#define i3 (v1)
#define i6 (v2)
#define i10 (v3)
    i2 = ((TYPE_NAME*)buffer)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    v4 = _mm256_loadu_si256((const float*)&output[0]);
    v5 = elem_low_bit0;
    v6 = _mm256_xor_si256(v4, v5);
    v7 = _mm256_loadu_si256((const float*)&output[2]);
    v8 = elem_low_bit1;
    v9 = _mm256_xor_si256(v7, v8);
    v4 = _mm256_and_si256(v4, v5);
    v5 = _mm256_xor_si256(v9, v4);
    v10 = _mm256_loadu_si256((const float*)&output[5]);
    _mm256_storeu_si256((float*)&output[5], v5);
    v5 = _mm256_xor_si256(v0, v2);
    v4 = _mm256_and_si256(v9, v4);
    v7 = _mm256_and_si256(v7, v8);
    v4 = _mm256_or_si256(v4, v7);
    v7 = _mm256_xor_si256(v5, v4);
    v8 = _mm256_loadu_si256((const float*)&output[7]);
    _mm256_storeu_si256((float*)&output[7], v7);
    v9 = elem_low_bit2;
    v1 = _mm256_xor_si256(v1, v9);
    v4 = _mm256_and_si256(v5, v4);
    v0 = _mm256_and_si256(v0, v2);
    v0 = _mm256_or_si256(v4, v0);
    v0 = _mm256_xor_si256(v1, v0);
    v1 = _mm256_loadu_si256((const float*)&output[3]);
    _mm256_storeu_si256((float*)&output[3], v0);
    v2 = elem_low_bit3;
    v4 = ((arg & 1) != 0) ? one : zero;
    v5 = _mm256_xor_si256(v2, v4);
    _mm256_storeu_si256((float*)&output[2], v5);
    v9 = _mm256_loadu_si256((const float*)&output[1]);
    v11 = ((arg & 2) != 0) ? one : zero;
    v12 = _mm256_xor_si256(v9, v11);
    v2 = _mm256_andnot_si256(v4, v2);
    v2 = _mm256_andnot_si256(v2, v5);
    v4 = _mm256_xor_si256(v12, v2);
    v5 = _mm256_xor_si256(v3, v10);
    v2 = _mm256_or_si256(v12, v2);
    v9 = _mm256_andnot_si256(v11, v9);
    v2 = _mm256_andnot_si256(v9, v2);
    v9 = _mm256_xor_si256(v5, v2);
    _mm256_storeu_si256((float*)&output[0], v9);
    v1 = _mm256_xor_si256(v1, v8);
    v2 = _mm256_or_si256(v5, v2);
    v3 = _mm256_andnot_si256(v10, v3);
    v2 = _mm256_andnot_si256(v3, v2);
    v1 = _mm256_xor_si256(v1, v2);
    _mm256_storeu_si256((float*)&output[1], v1);
#define o0 (v6)
#define o2 (v7)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)buffer)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
}
"##
    );
}

#[test]
fn test_clang_writer_aggregate_output_outneg() {
    let circuit = Circuit::new(
        8,
        [
            Gate::new_and(0, 1),
            Gate::new_nimpl(2, 3),
            Gate::new_xor(4, 5),
            Gate::new_nor(6, 7),
        ],
        [
            (8, false),
            (9, true),
            (10, true),
            (11, true),
            (8, true),
            (9, false),
            (10, false),
            (11, false),
            (8, false),
            (9, true),
            (10, true),
            (11, true),
            (8, true),
            (9, false),
            (10, false),
            (11, false),
            (8, false),
            (9, true),
            (10, true),
            (11, true),
            (8, true),
            (9, false),
            (10, false),
            (11, false),
        ],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new().aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    void* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    uint32_t v7;
    v0 = input[0];
    v1 = input[1];
    v0 = (v0 & v1);
    v1 = input[2];
    v2 = input[3];
    v1 = (v1 & ~v2);
    v2 = input[4];
    v3 = input[5];
    v2 = (v2 ^ v3);
    v3 = input[6];
    v4 = input[7];
    v3 = ~(v3 | v4);
    v1 = ~v1;
    v2 = ~v2;
    v3 = ~v3;
    v4 = ~v0;
    v5 = ~v1;
    v6 = ~v2;
    v7 = ~v3;
#define o0 (v0)
#define o1 (v1)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v5)
#define o6 (v6)
#define o7 (v7)
#define o8 (v0)
#define o9 (v1)
#define o10 (v2)
#define o11 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v6)
#define o15 (v7)
#define o16 (v0)
#define o17 (v1)
#define o18 (v2)
#define o19 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v6)
#define o23 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
#undef o6
#undef o7
#undef o8
#undef o9
#undef o10
#undef o11
#undef o12
#undef o13
#undef o14
#undef o15
#undef o16
#undef o17
#undef o18
#undef o19
#undef o20
#undef o21
#undef o22
#undef o23
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new().aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
    __m128 v7;
    v0 = _mm_loadu_ps((const float*)&input[0]);
    v1 = _mm_loadu_ps((const float*)&input[1]);
    v0 = _mm_and_ps(v0, v1);
    v1 = _mm_loadu_ps((const float*)&input[2]);
    v2 = _mm_loadu_ps((const float*)&input[3]);
    v1 = _mm_andnot_ps(v2, v1);
    v2 = _mm_loadu_ps((const float*)&input[4]);
    v3 = _mm_loadu_ps((const float*)&input[5]);
    v2 = _mm_xor_ps(v2, v3);
    v3 = _mm_loadu_ps((const float*)&input[6]);
    v4 = _mm_loadu_ps((const float*)&input[7]);
    v3 = _mm_or_ps(v3, v4);
    v1 = _mm_xor_ps(v1, one);
    v2 = _mm_xor_ps(v2, one);
    v4 = _mm_xor_ps(v0, one);
    v5 = _mm_xor_ps(v1, one);
    v6 = _mm_xor_ps(v2, one);
    v7 = _mm_xor_ps(v3, one);
#define o0 (v0)
#define o1 (v1)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v5)
#define o6 (v6)
#define o7 (v7)
#define o8 (v0)
#define o9 (v1)
#define o10 (v2)
#define o11 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v6)
#define o15 (v7)
#define o16 (v0)
#define o17 (v1)
#define o18 (v2)
#define o19 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v6)
#define o23 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
#undef o6
#undef o7
#undef o8
#undef o9
#undef o10
#undef o11
#undef o12
#undef o13
#undef o14
#undef o15
#undef o16
#undef o17
#undef o18
#undef o19
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&(0..24).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    uint32_t v7;
    v0 = input[0];
    v1 = input[1];
    v0 = (v0 & v1);
    output[0] = v0;
    output[4] = ~v0;
    output[8] = v0;
    output[12] = ~v0;
    output[16] = v0;
    output[20] = ~v0;
    v1 = input[2];
    v2 = input[3];
    v1 = (v1 & ~v2);
    output[1] = ~v1;
    output[5] = v1;
    output[9] = ~v1;
    output[13] = v1;
    output[17] = ~v1;
    output[21] = v1;
    v2 = input[4];
    v3 = input[5];
    v2 = (v2 ^ v3);
    output[2] = ~v2;
    output[6] = v2;
    output[10] = ~v2;
    output[14] = v2;
    output[18] = ~v2;
    output[22] = v2;
    v3 = input[6];
    v4 = input[7];
    v3 = ~(v3 | v4);
    output[3] = ~v3;
    output[7] = v3;
    output[11] = ~v3;
    output[15] = v3;
    output[19] = ~v3;
    output[23] = v3;
    v1 = ~v1;
    v2 = ~v2;
    v3 = ~v3;
    v4 = ~v0;
    v5 = ~v1;
    v6 = ~v2;
    v7 = ~v3;
#define o0 (v0)
#define o1 (v1)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v5)
#define o6 (v6)
#define o7 (v7)
#define o8 (v0)
#define o9 (v1)
#define o10 (v2)
#define o11 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v6)
#define o15 (v7)
#define o16 (v0)
#define o17 (v1)
#define o18 (v2)
#define o19 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v6)
#define o23 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
#undef o6
#undef o7
#undef o8
#undef o9
#undef o10
#undef o11
#undef o12
#undef o13
#undef o14
#undef o15
#undef o16
#undef o17
#undef o18
#undef o19
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&(0..24).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256* input,
    __m256* output, void* buffer, size_t idx) {
    const __m256 one = *((const __m256*)one_value);
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    __m256 v5;
    __m256 v6;
    __m256 v7;
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[1]);
    v0 = _mm256_and_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[0], v0);
    _mm256_storeu_ps((float*)&output[4], _mm256_xor_ps(v0, one));
    _mm256_storeu_ps((float*)&output[8], v0);
    _mm256_storeu_ps((float*)&output[12], _mm256_xor_ps(v0, one));
    _mm256_storeu_ps((float*)&output[16], v0);
    _mm256_storeu_ps((float*)&output[20], _mm256_xor_ps(v0, one));
    v1 = _mm256_loadu_ps((const float*)&input[2]);
    v2 = _mm256_loadu_ps((const float*)&input[3]);
    v1 = _mm256_andnot_ps(v2, v1);
    _mm256_storeu_ps((float*)&output[1], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[5], v1);
    _mm256_storeu_ps((float*)&output[9], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[13], v1);
    _mm256_storeu_ps((float*)&output[17], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[21], v1);
    v2 = _mm256_loadu_ps((const float*)&input[4]);
    v3 = _mm256_loadu_ps((const float*)&input[5]);
    v2 = _mm256_xor_ps(v2, v3);
    _mm256_storeu_ps((float*)&output[2], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[6], v2);
    _mm256_storeu_ps((float*)&output[10], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[14], v2);
    _mm256_storeu_ps((float*)&output[18], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[22], v2);
    v3 = _mm256_loadu_ps((const float*)&input[6]);
    v4 = _mm256_loadu_ps((const float*)&input[7]);
    v3 = _mm256_or_ps(v3, v4);
    _mm256_storeu_ps((float*)&output[3], v3);
    _mm256_storeu_ps((float*)&output[7], _mm256_xor_ps(v3, one));
    _mm256_storeu_ps((float*)&output[11], v3);
    _mm256_storeu_ps((float*)&output[15], _mm256_xor_ps(v3, one));
    _mm256_storeu_ps((float*)&output[19], v3);
    _mm256_storeu_ps((float*)&output[23], _mm256_xor_ps(v3, one));
    v1 = _mm256_xor_ps(v1, one);
    v2 = _mm256_xor_ps(v2, one);
    v4 = _mm256_xor_ps(v0, one);
    v5 = _mm256_xor_ps(v1, one);
    v6 = _mm256_xor_ps(v2, one);
    v7 = _mm256_xor_ps(v3, one);
#define o0 (v0)
#define o1 (v1)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v5)
#define o6 (v6)
#define o7 (v7)
#define o8 (v0)
#define o9 (v1)
#define o10 (v2)
#define o11 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v6)
#define o15 (v7)
#define o16 (v0)
#define o17 (v1)
#define o18 (v2)
#define o19 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v6)
#define o23 (v7)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
#undef o6
#undef o7
#undef o8
#undef o9
#undef o10
#undef o11
#undef o12
#undef o13
#undef o14
#undef o15
#undef o16
#undef o17
#undef o18
#undef o19
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[4, 12, 20, 5, 13, 21, 6, 14, 22, 7, 15, 23])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = input[0];
    v1 = input[1];
    v0 = (v0 & v1);
    output[0] = v0;
    output[4] = ~v0;
    output[8] = v0;
    output[12] = ~v0;
    output[16] = v0;
    output[20] = ~v0;
    v1 = input[2];
    v2 = input[3];
    v1 = (v1 & ~v2);
    output[1] = ~v1;
    output[5] = v1;
    output[9] = ~v1;
    output[13] = v1;
    output[17] = ~v1;
    output[21] = v1;
    v2 = input[4];
    v3 = input[5];
    v2 = (v2 ^ v3);
    output[2] = ~v2;
    output[6] = v2;
    output[10] = ~v2;
    output[14] = v2;
    output[18] = ~v2;
    output[22] = v2;
    v3 = input[6];
    v4 = input[7];
    v3 = ~(v3 | v4);
    output[3] = ~v3;
    output[7] = v3;
    output[11] = ~v3;
    output[15] = v3;
    output[19] = ~v3;
    output[23] = v3;
    v0 = ~v0;
#define o4 (v0)
#define o5 (v1)
#define o6 (v2)
#define o7 (v3)
#define o12 (v0)
#define o13 (v1)
#define o14 (v2)
#define o15 (v3)
#define o20 (v0)
#define o21 (v1)
#define o22 (v2)
#define o23 (v3)
    ((TYPE_NAME*)output)[1] = o2;
#undef o4
#undef o5
#undef o6
#undef o7
#undef o12
#undef o13
#undef o14
#undef o15
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[4, 12, 20, 5, 13, 21, 6, 14, 22, 7, 15, 23])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m128i* input,
    __m128i* output, void* buffer, size_t idx) {
    const __m128i one = *((const __m128i*)one_value);
    __m128i v0;
    __m128i v1;
    __m128i v2;
    __m128i v3;
    __m128i v4;
    v0 = _mm_loadu_si128((const __m128i*)&input[0]);
    v1 = _mm_loadu_si128((const __m128i*)&input[1]);
    v0 = _mm_and_si128(v0, v1);
    _mm_storeu_si128((__m128i*)&output[0], v0);
    _mm_storeu_si128((__m128i*)&output[4], _mm_xor_si128(v0, one));
    _mm_storeu_si128((__m128i*)&output[8], v0);
    _mm_storeu_si128((__m128i*)&output[12], _mm_xor_si128(v0, one));
    _mm_storeu_si128((__m128i*)&output[16], v0);
    _mm_storeu_si128((__m128i*)&output[20], _mm_xor_si128(v0, one));
    v1 = _mm_loadu_si128((const __m128i*)&input[2]);
    v2 = _mm_loadu_si128((const __m128i*)&input[3]);
    v1 = _mm_andnot_si128(v2, v1);
    _mm_storeu_si128((__m128i*)&output[1], _mm_xor_si128(v1, one));
    _mm_storeu_si128((__m128i*)&output[5], v1);
    _mm_storeu_si128((__m128i*)&output[9], _mm_xor_si128(v1, one));
    _mm_storeu_si128((__m128i*)&output[13], v1);
    _mm_storeu_si128((__m128i*)&output[17], _mm_xor_si128(v1, one));
    _mm_storeu_si128((__m128i*)&output[21], v1);
    v2 = _mm_loadu_si128((const __m128i*)&input[4]);
    v3 = _mm_loadu_si128((const __m128i*)&input[5]);
    v2 = _mm_xor_si128(v2, v3);
    _mm_storeu_si128((__m128i*)&output[2], _mm_xor_si128(v2, one));
    _mm_storeu_si128((__m128i*)&output[6], v2);
    _mm_storeu_si128((__m128i*)&output[10], _mm_xor_si128(v2, one));
    _mm_storeu_si128((__m128i*)&output[14], v2);
    _mm_storeu_si128((__m128i*)&output[18], _mm_xor_si128(v2, one));
    _mm_storeu_si128((__m128i*)&output[22], v2);
    v3 = _mm_loadu_si128((const __m128i*)&input[6]);
    v4 = _mm_loadu_si128((const __m128i*)&input[7]);
    v3 = _mm_or_si128(v3, v4);
    _mm_storeu_si128((__m128i*)&output[3], v3);
    _mm_storeu_si128((__m128i*)&output[7], _mm_xor_si128(v3, one));
    _mm_storeu_si128((__m128i*)&output[11], v3);
    _mm_storeu_si128((__m128i*)&output[15], _mm_xor_si128(v3, one));
    _mm_storeu_si128((__m128i*)&output[19], v3);
    _mm_storeu_si128((__m128i*)&output[23], _mm_xor_si128(v3, one));
    v0 = _mm_xor_si128(v0, one);
    v3 = _mm_xor_si128(v3, one);
#define o4 (v0)
#define o5 (v1)
#define o6 (v2)
#define o7 (v3)
#define o12 (v0)
#define o13 (v1)
#define o14 (v2)
#define o15 (v3)
#define o20 (v0)
#define o21 (v1)
#define o22 (v2)
#define o23 (v3)
    ((TYPE_NAME*)output)[1] = o2;
#undef o4
#undef o5
#undef o6
#undef o7
#undef o12
#undef o13
#undef o14
#undef o15
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 4, 12, 20, 1, 5, 13, 21, 6, 14, 22, 7, 15, 23])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    v0 = input[0];
    v1 = input[1];
    v0 = (v0 & v1);
    output[0] = v0;
    output[4] = ~v0;
    output[8] = v0;
    output[12] = ~v0;
    output[16] = v0;
    output[20] = ~v0;
    v1 = input[2];
    v2 = input[3];
    v1 = (v1 & ~v2);
    output[1] = ~v1;
    output[5] = v1;
    output[9] = ~v1;
    output[13] = v1;
    output[17] = ~v1;
    output[21] = v1;
    v2 = input[4];
    v3 = input[5];
    v2 = (v2 ^ v3);
    output[2] = ~v2;
    output[6] = v2;
    output[10] = ~v2;
    output[14] = v2;
    output[18] = ~v2;
    output[22] = v2;
    v3 = input[6];
    v4 = input[7];
    v3 = ~(v3 | v4);
    output[3] = ~v3;
    output[7] = v3;
    output[11] = ~v3;
    output[15] = v3;
    output[19] = ~v3;
    output[23] = v3;
    v1 = ~v1;
    v4 = ~v0;
    v5 = ~v1;
#define o0 (v0)
#define o1 (v1)
#define o4 (v4)
#define o5 (v5)
#define o6 (v2)
#define o7 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v2)
#define o15 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v2)
#define o23 (v3)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o4
#undef o5
#undef o6
#undef o7
#undef o12
#undef o13
#undef o14
#undef o15
#undef o20
#undef o21
#undef o22
#undef o23
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 4, 12, 20, 1, 5, 13, 21, 6, 14, 22, 7, 15, 23])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_mulxx(const __m256* input,
    __m256* output, void* buffer, size_t idx) {
    const __m256 one = *((const __m256*)one_value);
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    __m256 v5;
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[1]);
    v0 = _mm256_and_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[0], v0);
    _mm256_storeu_ps((float*)&output[4], _mm256_xor_ps(v0, one));
    _mm256_storeu_ps((float*)&output[8], v0);
    _mm256_storeu_ps((float*)&output[12], _mm256_xor_ps(v0, one));
    _mm256_storeu_ps((float*)&output[16], v0);
    _mm256_storeu_ps((float*)&output[20], _mm256_xor_ps(v0, one));
    v1 = _mm256_loadu_ps((const float*)&input[2]);
    v2 = _mm256_loadu_ps((const float*)&input[3]);
    v1 = _mm256_andnot_ps(v2, v1);
    _mm256_storeu_ps((float*)&output[1], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[5], v1);
    _mm256_storeu_ps((float*)&output[9], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[13], v1);
    _mm256_storeu_ps((float*)&output[17], _mm256_xor_ps(v1, one));
    _mm256_storeu_ps((float*)&output[21], v1);
    v2 = _mm256_loadu_ps((const float*)&input[4]);
    v3 = _mm256_loadu_ps((const float*)&input[5]);
    v2 = _mm256_xor_ps(v2, v3);
    _mm256_storeu_ps((float*)&output[2], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[6], v2);
    _mm256_storeu_ps((float*)&output[10], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[14], v2);
    _mm256_storeu_ps((float*)&output[18], _mm256_xor_ps(v2, one));
    _mm256_storeu_ps((float*)&output[22], v2);
    v3 = _mm256_loadu_ps((const float*)&input[6]);
    v4 = _mm256_loadu_ps((const float*)&input[7]);
    v3 = _mm256_or_ps(v3, v4);
    _mm256_storeu_ps((float*)&output[3], v3);
    _mm256_storeu_ps((float*)&output[7], _mm256_xor_ps(v3, one));
    _mm256_storeu_ps((float*)&output[11], v3);
    _mm256_storeu_ps((float*)&output[15], _mm256_xor_ps(v3, one));
    _mm256_storeu_ps((float*)&output[19], v3);
    _mm256_storeu_ps((float*)&output[23], _mm256_xor_ps(v3, one));
    v1 = _mm256_xor_ps(v1, one);
    v4 = _mm256_xor_ps(v0, one);
    v5 = _mm256_xor_ps(v1, one);
    v3 = _mm256_xor_ps(v3, one);
#define o0 (v0)
#define o1 (v1)
#define o4 (v4)
#define o5 (v5)
#define o6 (v2)
#define o7 (v3)
#define o12 (v4)
#define o13 (v5)
#define o14 (v2)
#define o15 (v3)
#define o20 (v4)
#define o21 (v5)
#define o22 (v2)
#define o23 (v3)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o1
#undef o4
#undef o5
#undef o6
#undef o7
#undef o12
#undef o13
#undef o14
#undef o15
#undef o20
#undef o21
#undef o22
#undef o23
}
"##
    );
}

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
        CodeConfig::new().inner_loop(Some(10)),
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
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new().inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_mulxx(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global void* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 4 * idx + input_shift;
    const size_t ovn = 4 * idx + output_shift;
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    uint v6;
    if (idx >= n) return;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
    if (iter == 0) {
    v0 = input[ivn + 0];
    v1 = input[ivn + 1];
    v2 = input[ivn + 2];
    v3 = input[ivn + 3];
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
    output[ovn + 0] = v5;
    output[ovn + 1] = v3;
    output[ovn + 2] = v2;
    output[ovn + 3] = v0;
    } else {
    v1 = v3;
    v3 = v0;
    v0 = v5;
    }
    } // loop
}
"##
    );
}

#[test]
fn test_clang_writer_loop_copy_to_input() {
    // testing copy to input
    let circuit = Circuit::new(
        10,
        (0..5).map(|i| Gate::new_xor(2 * i, 2 * i + 1)),
        (0..10).map(|i| {
            if i < 5 {
                (10 + i, false)
            } else {
                (10 + i - 5, false)
            }
        }),
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx",
        circuit.clone(),
        false,
        CodeConfig::new().inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx(const uint32_t* input,
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
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
    v8 = input[8];
    v9 = input[9];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v2 = (v4 ^ v5);
    v3 = (v6 ^ v7);
    v4 = (v8 ^ v9);
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v0;
    output[1] = v1;
    output[2] = v2;
    output[3] = v3;
    output[4] = v4;
    output[5] = v0;
    output[6] = v1;
    output[7] = v2;
    output[8] = v3;
    output[9] = v4;
    } else {
    v5 = v0;
    v6 = v1;
    v7 = v2;
    v8 = v3;
    v9 = v4;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[0, 2, 4, 6, 8, 1, 3, 5, 7, 9], 10)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx(const uint32_t* input,
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
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
    v8 = input[8];
    v9 = input[9];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v2 = (v4 ^ v5);
    v3 = (v6 ^ v7);
    v4 = (v8 ^ v9);
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v0;
    output[2] = v1;
    output[4] = v2;
    output[6] = v3;
    output[8] = v4;
    output[1] = v0;
    output[3] = v1;
    output[5] = v2;
    output[7] = v3;
    output[9] = v4;
    } else {
    v8 = v4;
    v9 = v4;
    v4 = v2;
    v5 = v2;
    v2 = v1;
    v6 = v3;
    v7 = v3;
    v3 = v1;
    v1 = v0;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[1, 2, 3, 4, 0, 5, 6, 7, 8, 9], 10)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx(const uint32_t* input,
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
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
    v8 = input[8];
    v9 = input[9];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v2 = (v4 ^ v5);
    v3 = (v6 ^ v7);
    v4 = (v8 ^ v9);
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v0;
    output[2] = v1;
    output[3] = v2;
    output[4] = v3;
    output[0] = v4;
    output[5] = v0;
    output[6] = v1;
    output[7] = v2;
    output[8] = v3;
    output[9] = v4;
    } else {
    v5 = v0;
    v6 = v1;
    v7 = v2;
    v8 = v3;
    v9 = v4;
    v10 = v0;
    v0 = v4;
    v4 = v3;
    v3 = v2;
    v2 = v1;
    v1 = v10;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[3, 4, 0, 1, 2, 5, 6, 7, 8, 9], 10)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx(const uint32_t* input,
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
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
    v8 = input[8];
    v9 = input[9];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v2 = (v4 ^ v5);
    v3 = (v6 ^ v7);
    v4 = (v8 ^ v9);
    if (iter == iter_max - 1 || stop != 0) {
    output[3] = v0;
    output[4] = v1;
    output[0] = v2;
    output[1] = v3;
    output[2] = v4;
    output[5] = v0;
    output[6] = v1;
    output[7] = v2;
    output[8] = v3;
    output[9] = v4;
    } else {
    v5 = v0;
    v8 = v3;
    v6 = v1;
    v9 = v4;
    v7 = v2;
    v10 = v0;
    v0 = v2;
    v2 = v4;
    v4 = v1;
    v1 = v3;
    v3 = v10;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[1, 5, 3, 6, 9, 0, 7, 8, 2, 4], 10)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx(const uint32_t* input,
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
    uint32_t v7;
    uint32_t v8;
    uint32_t v9;
    uint32_t v10;
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
    v8 = input[8];
    v9 = input[9];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v2 = (v4 ^ v5);
    v3 = (v6 ^ v7);
    v4 = (v8 ^ v9);
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v0;
    output[5] = v1;
    output[3] = v2;
    output[6] = v3;
    output[9] = v4;
    output[0] = v0;
    output[7] = v1;
    output[8] = v2;
    output[2] = v3;
    output[4] = v4;
    } else {
    v5 = v1;
    v7 = v1;
    v1 = v0;
    v8 = v2;
    v6 = v3;
    v10 = v2;
    v2 = v3;
    v3 = v10;
    v9 = v4;
    }
    } // loop
}
"##
    );
}

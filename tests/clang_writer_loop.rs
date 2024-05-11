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
    // more outputs
    let outputs = [20, 22, 25, 29, 34];
    let circuit = Circuit::new(
        20,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_xor(21, 4),
            Gate::new_xor(5, 6),
            Gate::new_xor(23, 7),
            Gate::new_xor(24, 8),
            Gate::new_xor(9, 10),
            Gate::new_xor(26, 11),
            Gate::new_xor(27, 12),
            Gate::new_xor(28, 13),
            Gate::new_xor(14, 15),
            Gate::new_xor(30, 16),
            Gate::new_xor(31, 17),
            Gate::new_xor(32, 18),
            Gate::new_xor(33, 19),
        ],
        (0..5)
            .map(|i| (0..i + 2).map(move |_| (outputs[i], false)))
            .flatten(),
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v2 = (v2 ^ v8);
    v3 = (v9 ^ v10);
    v3 = (v3 ^ v11);
    v3 = (v3 ^ v12);
    v3 = (v3 ^ v13);
    v4 = (v14 ^ v15);
    v4 = (v4 ^ v16);
    v4 = (v4 ^ v17);
    v4 = (v4 ^ v18);
    v4 = (v4 ^ v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v0;
    output[1] = v0;
    output[2] = v1;
    output[3] = v1;
    output[4] = v1;
    output[5] = v2;
    output[6] = v2;
    output[7] = v2;
    output[8] = v2;
    output[9] = v3;
    output[10] = v3;
    output[11] = v3;
    output[12] = v3;
    output[13] = v3;
    output[14] = v4;
    output[15] = v4;
    output[16] = v4;
    output[17] = v4;
    output[18] = v4;
    output[19] = v4;
    } else {
    v5 = v2;
    v6 = v2;
    v7 = v2;
    v8 = v2;
    v2 = v1;
    v9 = v3;
    v10 = v3;
    v11 = v3;
    v12 = v3;
    v13 = v3;
    v3 = v1;
    v14 = v4;
    v15 = v4;
    v16 = v4;
    v17 = v4;
    v18 = v4;
    v19 = v4;
    v4 = v1;
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
            .output_placement(Some((
                &[
                    2,  // 0
                    5,  // 1
                    9,  // 2
                    1,  // 3
                    10, // 4
                    11, // 5
                    12, // 6
                    13, // 7
                    14, // 8
                    15, // 9
                    16, // 10
                    17, // 11
                    18, // 12
                    19, // 13
                    3,  // 14
                    4,  // 15
                    6,  // 16
                    7,  // 17
                    8,  // 18
                    0,  // 19
                ],
                20,
            )))
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v2 = (v2 ^ v8);
    v3 = (v9 ^ v10);
    v3 = (v3 ^ v11);
    v3 = (v3 ^ v12);
    v3 = (v3 ^ v13);
    v4 = (v14 ^ v15);
    v4 = (v4 ^ v16);
    v4 = (v4 ^ v17);
    v4 = (v4 ^ v18);
    v4 = (v4 ^ v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[2] = v0;
    output[5] = v0;
    output[9] = v1;
    output[1] = v1;
    output[10] = v1;
    output[11] = v2;
    output[12] = v2;
    output[13] = v2;
    output[14] = v2;
    output[15] = v3;
    output[16] = v3;
    output[17] = v3;
    output[18] = v3;
    output[19] = v3;
    output[3] = v4;
    output[4] = v4;
    output[6] = v4;
    output[7] = v4;
    output[8] = v4;
    output[0] = v4;
    } else {
    v11 = v2;
    v12 = v2;
    v13 = v2;
    v14 = v2;
    v2 = v0;
    v5 = v0;
    v9 = v1;
    v10 = v1;
    v15 = v3;
    v16 = v3;
    v17 = v3;
    v18 = v3;
    v19 = v3;
    v3 = v4;
    v6 = v4;
    v7 = v4;
    v8 = v4;
    v0 = v4;
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
            .output_placement(Some((
                &[
                    1,  // 0(0)
                    2,  // 1(0)
                    3,  // 2(1)
                    0,  // 3(1)
                    4,  // 4(1)
                    6,  // 5(2)
                    7,  // 6(2)
                    5,  // 7(2)
                    8,  // 8(2)
                    10, // 9(3)
                    12, // 10(3)
                    11, // 11(3)
                    9,  // 12(3)
                    14, // 13(3)
                    15, // 14(4)
                    16, // 15(4)
                    13, // 16(4)
                    17, // 17(4)
                    19, // 18(4)
                    18, // 19(4)
                ],
                20,
            )))
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v2 = (v2 ^ v8);
    v3 = (v9 ^ v10);
    v3 = (v3 ^ v11);
    v3 = (v3 ^ v12);
    v3 = (v3 ^ v13);
    v4 = (v14 ^ v15);
    v4 = (v4 ^ v16);
    v4 = (v4 ^ v17);
    v4 = (v4 ^ v18);
    v4 = (v4 ^ v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v0;
    output[2] = v0;
    output[3] = v1;
    output[0] = v1;
    output[4] = v1;
    output[6] = v2;
    output[7] = v2;
    output[5] = v2;
    output[8] = v2;
    output[10] = v3;
    output[12] = v3;
    output[11] = v3;
    output[9] = v3;
    output[14] = v3;
    output[15] = v4;
    output[16] = v4;
    output[13] = v4;
    output[17] = v4;
    output[19] = v4;
    output[18] = v4;
    } else {
    v6 = v2;
    v7 = v2;
    v5 = v2;
    v8 = v2;
    v2 = v0;
    v10 = v3;
    v12 = v3;
    v11 = v3;
    v9 = v3;
    v14 = v3;
    v3 = v1;
    v15 = v4;
    v16 = v4;
    v13 = v4;
    v17 = v4;
    v19 = v4;
    v18 = v4;
    v4 = v1;
    v20 = v0;
    v0 = v1;
    v1 = v20;
    }
    } // loop
}
"##
    );
    // more outputs
    let outputs = [20, 22, 24, 27, 29, 32, 33];
    let ranges = [2, 3, 3, 4, 3, 4, 1];
    let circuit = Circuit::new(
        20,
        [
            Gate::new_xor(0, 1),     // 20
            Gate::new_xor(2, 3),     // 21
            Gate::new_xor(21, 4),    // 22
            Gate::new_xor(5, 6),     // 23
            Gate::new_xor(23, 7),    // 24
            Gate::new_xor(8, 9),     // 25
            Gate::new_xor(25, 10),   // 26
            Gate::new_xor(26, 11),   // 27
            Gate::new_xor(12, 13),   // 28
            Gate::new_xor(28, 14),   // 29
            Gate::new_xor(15, 16),   // 30
            Gate::new_xor(30, 17),   // 31
            Gate::new_xor(31, 18),   // 32
            Gate::new_nimpl(19, 19), // 33
        ],
        (0..7)
            .map(|i| (0..ranges[i]).map(move |_| (outputs[i], false)))
            .flatten(),
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx2",
        circuit.clone(),
        false,
        CodeConfig::new().inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx2(const uint32_t* input,
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v3 = (v8 ^ v9);
    v3 = (v3 ^ v10);
    v3 = (v3 ^ v11);
    v4 = (v12 ^ v13);
    v4 = (v4 ^ v14);
    v5 = (v15 ^ v16);
    v5 = (v5 ^ v17);
    v5 = (v5 ^ v18);
    v6 = (v19 & ~v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[0] = v0;
    output[1] = v0;
    output[2] = v1;
    output[3] = v1;
    output[4] = v1;
    output[5] = v2;
    output[6] = v2;
    output[7] = v2;
    output[8] = v3;
    output[9] = v3;
    output[10] = v3;
    output[11] = v3;
    output[12] = v4;
    output[13] = v4;
    output[14] = v4;
    output[15] = v5;
    output[16] = v5;
    output[17] = v5;
    output[18] = v5;
    output[19] = v6;
    } else {
    v15 = v5;
    v16 = v5;
    v17 = v5;
    v18 = v5;
    v5 = v2;
    v19 = v6;
    v6 = v2;
    v7 = v2;
    v2 = v1;
    v8 = v3;
    v9 = v3;
    v10 = v3;
    v11 = v3;
    v3 = v1;
    v12 = v4;
    v13 = v4;
    v14 = v4;
    v4 = v1;
    v1 = v0;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx2",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((
                &[
                    1,  // 0(0)
                    9,  // 1(0)
                    2,  // 2(1)
                    3,  // 3(1)
                    4,  // 4(1)
                    5,  // 5(2)
                    0,  // 6(2)
                    6,  // 7(2)
                    8,  // 8(3)
                    10, // 9(3)
                    12, // 10(3)
                    11, // 11(3)
                    7,  // 12(4)
                    14, // 13(4)
                    15, // 14(4)
                    16, // 15(5)
                    13, // 16(5)
                    17, // 17(5)
                    19, // 18(5)
                    18, // 19(4)
                ],
                20,
            )))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx2(const uint32_t* input,
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v3 = (v8 ^ v9);
    v3 = (v3 ^ v10);
    v3 = (v3 ^ v11);
    v4 = (v12 ^ v13);
    v4 = (v4 ^ v14);
    v5 = (v15 ^ v16);
    v5 = (v5 ^ v17);
    v5 = (v5 ^ v18);
    v6 = (v19 & ~v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[1] = v0;
    output[9] = v0;
    output[2] = v1;
    output[3] = v1;
    output[4] = v1;
    output[5] = v2;
    output[0] = v2;
    output[6] = v2;
    output[8] = v3;
    output[10] = v3;
    output[12] = v3;
    output[11] = v3;
    output[7] = v4;
    output[14] = v4;
    output[15] = v4;
    output[16] = v5;
    output[13] = v5;
    output[17] = v5;
    output[19] = v5;
    output[18] = v6;
    } else {
    v9 = v0;
    v7 = v4;
    v14 = v4;
    v15 = v4;
    v4 = v1;
    v8 = v3;
    v10 = v3;
    v12 = v3;
    v11 = v3;
    v3 = v1;
    v16 = v5;
    v13 = v5;
    v17 = v5;
    v19 = v5;
    v5 = v2;
    v18 = v6;
    v6 = v2;
    v20 = v0;
    v0 = v2;
    v2 = v1;
    v1 = v20;
    }
    } // loop
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "cpx2",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((
                &[
                    9,  // 0(0)
                    1,  // 1(0)
                    3,  // 2(1)
                    2,  // 3(1)
                    4,  // 4(1)
                    5,  // 5(2)
                    0,  // 6(2)
                    6,  // 7(2)
                    8,  // 8(3)
                    10, // 9(3)
                    12, // 10(3)
                    11, // 11(3)
                    7,  // 12(4)
                    14, // 13(4)
                    15, // 14(4)
                    16, // 15(5)
                    13, // 16(5)
                    17, // 17(5)
                    19, // 18(5)
                    18, // 19(4)
                ],
                20,
            )))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_cpx2(const uint32_t* input,
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
    uint32_t v11;
    uint32_t v12;
    uint32_t v13;
    uint32_t v14;
    uint32_t v15;
    uint32_t v16;
    uint32_t v17;
    uint32_t v18;
    uint32_t v19;
    uint32_t v20;
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
    v10 = input[10];
    v11 = input[11];
    v12 = input[12];
    v13 = input[13];
    v14 = input[14];
    v15 = input[15];
    v16 = input[16];
    v17 = input[17];
    v18 = input[18];
    v19 = input[19];
    }
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v3);
    v1 = (v1 ^ v4);
    v2 = (v5 ^ v6);
    v2 = (v2 ^ v7);
    v3 = (v8 ^ v9);
    v3 = (v3 ^ v10);
    v3 = (v3 ^ v11);
    v4 = (v12 ^ v13);
    v4 = (v4 ^ v14);
    v5 = (v15 ^ v16);
    v5 = (v5 ^ v17);
    v5 = (v5 ^ v18);
    v6 = (v19 & ~v19);
    if (iter == iter_max - 1 || stop != 0) {
    output[9] = v0;
    output[1] = v0;
    output[3] = v1;
    output[2] = v1;
    output[4] = v1;
    output[5] = v2;
    output[0] = v2;
    output[6] = v2;
    output[8] = v3;
    output[10] = v3;
    output[12] = v3;
    output[11] = v3;
    output[7] = v4;
    output[14] = v4;
    output[15] = v4;
    output[16] = v5;
    output[13] = v5;
    output[17] = v5;
    output[19] = v5;
    output[18] = v6;
    } else {
    v9 = v0;
    v8 = v3;
    v10 = v3;
    v12 = v3;
    v11 = v3;
    v3 = v1;
    v7 = v4;
    v14 = v4;
    v15 = v4;
    v4 = v1;
    v16 = v5;
    v13 = v5;
    v17 = v5;
    v19 = v5;
    v5 = v2;
    v18 = v6;
    v6 = v2;
    v20 = v0;
    v0 = v2;
    v2 = v1;
    v1 = v20;
    }
    } // loop
}
"##
    );
}

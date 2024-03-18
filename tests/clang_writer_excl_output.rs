use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_exclude_output() {
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
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[0])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, size_t idx) {
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
    output[0] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[0])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + input_shift;
    const size_t ovn = 1 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = input[ivn + 1];
    v2 = (v0 ^ v1);
    v3 = input[ivn + 2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + 0] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[0])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (3 * idx) + input_shift;
    const size_t ovn = llen * (1 * idx) + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*1 + lidx];
    v2 = (v0 ^ v1);
    v3 = input[ivn + llen*2 + lidx];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + llen*0 + lidx] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[0])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m64* input,
    __m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _m_pxor(v0, v1);
    v3 = input[2];
    v4 = _m_pxor(v3, v2);
    v2 = _m_pand(v3, v2);
    v0 = _m_pand(v0, v1);
    v0 = _m_por(v2, v0);
    output[0] = v0;
    _m_empty();
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[1])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, size_t idx) {
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
    output[0] = v4;
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().exclude_outputs(Some(&[1])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m64* input,
    __m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v0 = input[0];
    v1 = input[1];
    v2 = _m_pxor(v0, v1);
    v3 = input[2];
    v4 = _m_pxor(v3, v2);
    output[0] = v4;
    v2 = _m_pand(v3, v2);
    v0 = _m_pand(v0, v1);
    v0 = _m_por(v2, v0);
    _m_empty();
}
"##
    );

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
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 1], 4)))
            .exclude_outputs(Some(&[1, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    v0 = output[3];
    v1 = output[0];
    v2 = (v0 & v1);
    v0 = (v0 ^ v1);
    v3 = (v2 & v0);
    output[3] = v3;
    v3 = output[1];
    v1 = ~(v3 | v1);
    v2 = (v2 & ~v1);
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v0);
    v1 = output[2];
    v0 = (v0 & ~v1);
    output[1] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 1], 4)))
            .exclude_outputs(Some(&[1, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(__m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    v0 = output[3];
    v1 = output[0];
    v2 = _m_pand(v0, v1);
    v0 = _m_pxor(v0, v1);
    v3 = _m_pand(v2, v0);
    output[3] = v3;
    v3 = output[1];
    v1 = _m_por(v3, v1);
    v2 = _m_pand(v2, v1);
    v0 = _m_pxor(v0, v1);
    v1 = _m_pxor(v2, v0);
    v1 = output[2];
    v0 = _m_por(v0, v1);
    output[1] = v0;
    _m_empty();
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 1], 4)))
            .exclude_outputs(Some(&[1, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long output_shift,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 4 * idx + output_shift;
    const size_t ovn = 4 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    if (idx >= n) return;
    v0 = output[ivn + 3];
    v1 = output[ivn + 0];
    v2 = (v0 & v1);
    v0 = (v0 ^ v1);
    v3 = (v2 & v0);
    output[ovn + 3] = v3;
    v3 = output[ivn + 1];
    v1 = ~(v3 | v1);
    v2 = (v2 & ~v1);
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v0);
    v1 = output[ivn + 2];
    v0 = (v0 & ~v1);
    output[ovn + 1] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[1, 3], 4)))
            .exclude_outputs(Some(&[1, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = output[3];
    v1 = output[0];
    v2 = (v0 & v1);
    v0 = (v0 ^ v1);
    v3 = (v2 & v0);
    v4 = output[1];
    output[1] = v3;
    v1 = ~(v4 | v1);
    v2 = (v2 & ~v1);
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v0);
    v1 = output[2];
    v0 = (v0 & ~v1);
    output[3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[1, 3], 4)))
            .exclude_outputs(Some(&[1, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(__m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v0 = output[3];
    v1 = output[0];
    v2 = _m_pand(v0, v1);
    v0 = _m_pxor(v0, v1);
    v3 = _m_pand(v2, v0);
    v4 = output[1];
    output[1] = v3;
    v1 = _m_por(v4, v1);
    v2 = _m_pand(v2, v1);
    v0 = _m_pxor(v0, v1);
    v1 = _m_pxor(v2, v0);
    v1 = output[2];
    v0 = _m_por(v0, v1);
    output[3] = v0;
    _m_empty();
}
"##
    );

    // aggr_output with aggr_to_buffer
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
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2]))
            .exclude_outputs(Some(&[0, 2])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = (v0 ^ v1);
    v3 = input[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[0] = ~v0;
    v0 = ~v4;
#define o0 (v4)
#define o2 (v0)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
#undef o2
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE2.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2]))
            .exclude_outputs(Some(&[0, 2])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128i* input,
    __m128i* output, void* buffer, size_t idx) {
    const __m128i one = *((const __m128i*)one_value);
    __m128i v0;
    __m128i v1;
    __m128i v2;
    __m128i v3;
    __m128i v4;
    unsigned int xxx = 1111;
    v0 = _mm_loadu_si128((const __m128i*)&input[0]);
    v1 = _mm_loadu_si128((const __m128i*)&input[1]);
    v2 = _mm_xor_si128(v0, v1);
    v3 = _mm_loadu_si128((const __m128i*)&input[2]);
    v4 = _mm_xor_si128(v3, v2);
    v2 = _mm_and_si128(v3, v2);
    v0 = _mm_and_si128(v0, v1);
    v0 = _mm_or_si128(v2, v0);
    _mm_storeu_si128((__m128i*)&output[0], v0);
    v0 = _mm_xor_si128(v4, one);
#define o0 (v4)
#define o2 (v0)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
#undef o2
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2]))
            .exclude_outputs(Some(&[0, 2])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    unsigned long buffer_shift, const global uint* input,
    global uint* output, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + input_shift;
    const size_t ovn = 1 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    buffer = (const global void*)(((const global char*)buffer) + 4*buffer_shift);
    unsigned int xxx = 1111;
    v0 = input[ivn + 0];
    v1 = input[ivn + 1];
    v2 = (v0 ^ v1);
    v3 = input[ivn + 2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + 0] = ~v0;
    v0 = ~v4;
#define o0 (v4)
#define o2 (v0)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
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
        [(4, false), (5, false), (6, true), (7, true), (3, false)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .exclude_outputs(Some(&[1, 3]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v0 = output[0];
    v1 = output[1];
    v2 = (v0 ^ v1);
    v3 = output[2];
    output[2] = v2;
    v4 = (v3 ^ v2);
    output[0] = v4;
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    output[1] = ~v0;
    v0 = ~(v2 | v0);
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .exclude_outputs(Some(&[1, 3]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(__m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v0 = output[0];
    v1 = output[1];
    v2 = _m_pxor(v0, v1);
    v3 = output[2];
    output[2] = v2;
    v4 = _m_pxor(v3, v2);
    output[0] = v4;
    v2 = _m_pand(v3, v2);
    v0 = _m_pand(v0, v1);
    output[1] = _m_pxor(v0, one);
    v0 = _m_por(v2, v0);
    _m_empty();
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
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 2, 5]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    unsigned int xxx = 1111;
    v0 = output[0];
    v1 = output[1];
    v2 = (v0 ^ v1);
    v3 = output[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[0] = ~v0;
    output[1] = v0;
    output[2] = ~v0;
    v1 = ~v4;
#define o0 (v4)
#define o2 (v1)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
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
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 2, 5]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(__m128* output, void* buffer, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    unsigned int xxx = 1111;
    v0 = _mm_loadu_ps((const float*)&output[0]);
    v1 = _mm_loadu_ps((const float*)&output[1]);
    v2 = _mm_xor_ps(v0, v1);
    v3 = _mm_loadu_ps((const float*)&output[2]);
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    _mm_storeu_ps((float*)&output[0], v0);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v0, one));
    _mm_storeu_ps((float*)&output[2], v0);
    v1 = _mm_xor_ps(v4, one);
    v0 = _mm_xor_ps(v0, one);
#define o0 (v4)
#define o2 (v1)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
#undef o2
#undef o3
#undef o5
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 2, 5]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long output_shift,
    unsigned long buffer_shift, global uint* output, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + output_shift;
    const size_t ovn = 3 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    buffer = (const global void*)(((const global char*)buffer) + 4*buffer_shift);
    unsigned int xxx = 1111;
    v0 = output[ivn + 0];
    v1 = output[ivn + 1];
    v2 = (v0 ^ v1);
    v3 = output[ivn + 2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + 0] = ~v0;
    output[ovn + 1] = v0;
    output[ovn + 2] = ~v0;
    v1 = ~v4;
#define o0 (v4)
#define o2 (v1)
#define o3 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
#undef o2
#undef o3
#undef o5
}
"##
    );

    let circuit = Circuit::new(
        6,
        [
            Gate::new_and(2, 3),
            Gate::new_xor(2, 3),
            Gate::new_nor(0, 3),
            Gate::new_and(6, 7),
            Gate::new_nimpl(6, 8),
            Gate::new_xor(7, 9),
            Gate::new_xor(10, 11),
            Gate::new_nimpl(11, 1),
        ],
        [
            (4, false),
            (5, true),
            (10, true),
            (11, false),
            (12, false),
            (13, true),
        ],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .pop_input_code(Some("    ((TYPE_NAME*)input)[0] |= i0 ^ i2;"))
            .pop_from_buffer(Some(&[1, 4]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o2;"))
            .aggr_to_buffer(Some(&[0, 3]))
            .exclude_outputs(Some(&[0, 3]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    unsigned int xxx = 1111;
#define i1 (v0)
#define i4 (v1)
    ((TYPE_NAME*)input)[0] |= i0 ^ i2;
#define i1
#define i4
    v2 = output[1];
    v3 = output[2];
    v4 = (v2 & v3);
    v5 = output[0];
    v5 = ~(v5 | v3);
    v5 = (v4 & ~v5);
    output[1] = ~v5;
    v2 = (v2 ^ v3);
    v3 = (v4 & v2);
    v2 = (v2 ^ v3);
    v3 = (v5 ^ v2);
    output[2] = v3;
    v0 = (v2 & ~v0);
    v3 = output[3];
    output[3] = ~v0;
    output[0] = ~v3;
#define o0 (v1)
#define o3 (v2)
    ((TYPE_NAME*)output)[0] |= o0 ^ o2;
#undef o0
#undef o3
}
"##
    );
}

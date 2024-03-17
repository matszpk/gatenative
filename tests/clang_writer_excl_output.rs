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
}

use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_populate_input() {
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
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_testcirc(const void* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
    v3 = (v0 ^ v1);
    v4 = (v2 ^ v3);
    output[0] = v4;
    v2 = (v2 & v3);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[1] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#include <xmmintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[4] __attribute__((aligned(16))) =
    { 0, 0, 0, 0 };
static const unsigned int one_value[4] __attribute__((aligned(16))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
static const unsigned int elem_index_low_tbl[7*4]
__attribute__((aligned(16))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff
};
#define TYPE_LEN (128)
#define TYPE_NAME __m128
#define GET_U32(D,X,I) { uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { _mm_storeu_ps((float*)(D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_ps((float*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm_loadu_ps((float*)(S)); }
void gate_sys_testcirc(const void* input,
    __m128* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
    v3 = _mm_xor_ps(v0, v1);
    v4 = _mm_xor_ps(v2, v3);
    _mm_storeu_ps((float*)&output[0], v4);
    v2 = _mm_and_ps(v2, v3);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    _mm_storeu_ps((float*)&output[1], v0);
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global void* input,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + input_shift;
    const size_t ovn = 2 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    input = (const global void*)(((const global char*)input) + 4*input_shift);
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
    v3 = (v0 ^ v1);
    v4 = (v2 ^ v3);
    output[ovn + 0] = v4;
    v2 = (v2 & v3);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + 1] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global void* input,
    global uint* output) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (3 * idx) + input_shift;
    const size_t ovn = llen * (2 * idx) + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    input = (const global void*)(((const global char*)input) + 4*input_shift);
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
    v3 = (v0 ^ v1);
    v4 = (v2 ^ v3);
    output[ovn + llen*0 + lidx] = v4;
    v2 = (v2 & v3);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    output[ovn + llen*1 + lidx] = ~v0;
}
"##
    );

    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(0, 2),
            Gate::new_and(1, 2),
            Gate::new_and(0, 3),
            Gate::new_and(1, 3),
            // add a1*b0 + a0*b1
            Gate::new_xor(5, 6),
            Gate::new_and(5, 6),
            // add c(a1*b0 + a0*b1) + a1*b1
            Gate::new_xor(7, 9),
            Gate::new_and(7, 9),
        ],
        [(4, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
    v4 = (v0 & v2);
    output[0] = v4;
    v2 = (v1 & v2);
    v0 = (v0 & v3);
    v4 = (v2 ^ v0);
    output[1] = ~v4;
    v1 = (v1 & v3);
    v0 = (v2 & v0);
    v2 = (v1 ^ v0);
    output[2] = v2;
    v0 = (v1 & v0);
    output[3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    __m128* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
    v4 = _mm_and_ps(v0, v2);
    _mm_storeu_ps((float*)&output[0], v4);
    v2 = _mm_and_ps(v1, v2);
    v0 = _mm_and_ps(v0, v3);
    v4 = _mm_xor_ps(v2, v0);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v4, one));
    v1 = _mm_and_ps(v1, v3);
    v0 = _mm_and_ps(v2, v0);
    v2 = _mm_xor_ps(v1, v0);
    _mm_storeu_ps((float*)&output[2], v2);
    v0 = _mm_and_ps(v1, v0);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global void* input,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 4 * idx + input_shift;
    const size_t ovn = 4 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    input = (const global void*)(((const global char*)input) + 4*input_shift);
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
    v4 = (v0 & v2);
    output[ovn + 0] = v4;
    v2 = (v1 & v2);
    v0 = (v0 & v3);
    v4 = (v2 ^ v0);
    output[ovn + 1] = ~v4;
    v1 = (v1 & v3);
    v0 = (v2 & v0);
    v2 = (v1 ^ v0);
    output[ovn + 2] = v2;
    v0 = (v1 & v0);
    output[ovn + 3] = ~v0;
}
"##
    );

    // non-empty input_map
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[1, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i3 (v1)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i3
    v2 = ((arg & 2) != 0) ? one : zero;
    v3 = (v0 & v2);
    output[0] = v3;
    v3 = ((arg & 1) != 0) ? one : zero;
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v4 = (v2 ^ v0);
    output[1] = ~v4;
    v1 = (v3 & v1);
    v0 = (v2 & v0);
    v2 = (v1 ^ v0);
    output[2] = v2;
    v0 = (v1 & v0);
    output[3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[1, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    __m128* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
#define i0 (v0)
#define i3 (v1)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i3
    v2 = ((arg & 2) != 0) ? one : zero;
    v3 = _mm_and_ps(v0, v2);
    _mm_storeu_ps((float*)&output[0], v3);
    v3 = ((arg & 1) != 0) ? one : zero;
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v4 = _mm_xor_ps(v2, v0);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v4, one));
    v1 = _mm_and_ps(v3, v1);
    v0 = _mm_and_ps(v2, v0);
    v2 = _mm_xor_ps(v1, v0);
    _mm_storeu_ps((float*)&output[2], v2);
    v0 = _mm_and_ps(v1, v0);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
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
            .elem_inputs(Some(&[1, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    uint32_t* output, size_t idx) {
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
#define i0 (v0)
#define i3 (v1)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i3
    v2 = elem_low_bit1;
    v3 = (v0 & v2);
    output[0] = v3;
    v3 = elem_low_bit0;
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v4 = (v2 ^ v0);
    output[1] = ~v4;
    v1 = (v3 & v1);
    v0 = (v2 & v0);
    v2 = (v1 ^ v0);
    output[2] = v2;
    v0 = (v1 & v0);
    output[3] = ~v0;
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
        [(4, false), (5, true), (12, false), (13, true)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
#define i4 (v4)
#define i5 (v5)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
#undef i4
#undef i5
    v6 = (v2 & v3);
    v0 = ~(v0 | v3);
    v0 = (v6 & ~v0);
    v2 = (v2 ^ v3);
    v3 = (v6 & v2);
    v2 = (v2 ^ v3);
    v0 = (v0 ^ v2);
    output[2] = v0;
    v0 = (v2 & ~v1);
    output[3] = ~v0;
    output[0] = v4;
    output[1] = ~v5;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new().pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    __m128* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
#define i4 (v4)
#define i5 (v5)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i2
#undef i3
#undef i4
#undef i5
    v6 = _mm_and_ps(v2, v3);
    v0 = _mm_or_ps(v0, v3);
    v0 = _mm_and_ps(v6, v0);
    v2 = _mm_xor_ps(v2, v3);
    v3 = _mm_and_ps(v6, v2);
    v2 = _mm_xor_ps(v2, v3);
    v0 = _mm_xor_ps(v0, v2);
    _mm_storeu_ps((float*)&output[2], v0);
    v0 = _mm_andnot_ps(v1, v2);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
    _mm_storeu_ps((float*)&output[0], v4);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v5, one));
}
"##
    );
    // with input_map
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    uint32_t* output, size_t idx) {
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
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = (v4 & v1);
    v6 = elem_low_bit0;
    v6 = ~(v6 | v1);
    v6 = (v5 & ~v6);
    v1 = (v4 ^ v1);
    v4 = (v5 & v1);
    v1 = (v1 ^ v4);
    v4 = (v6 ^ v1);
    output[2] = v4;
    v0 = (v1 & ~v0);
    output[3] = ~v0;
    output[0] = v2;
    output[1] = ~v3;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    __m128* output, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = _mm_and_ps(v4, v1);
    v6 = elem_low_bit0;
    v6 = _mm_or_ps(v6, v1);
    v6 = _mm_and_ps(v5, v6);
    v1 = _mm_xor_ps(v4, v1);
    v4 = _mm_and_ps(v5, v1);
    v1 = _mm_xor_ps(v1, v4);
    v4 = _mm_xor_ps(v6, v1);
    _mm_storeu_ps((float*)&output[2], v4);
    v0 = _mm_andnot_ps(v0, v1);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
    _mm_storeu_ps((float*)&output[0], v2);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v3, one));
}
"##
    );
    // with aggr_output and input_map
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    void* output, size_t idx) {
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
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = (v4 & v1);
    v6 = elem_low_bit0;
    v6 = ~(v6 | v1);
    v6 = (v5 & ~v6);
    v1 = (v4 ^ v1);
    v4 = (v5 & v1);
    v1 = (v1 ^ v4);
    v4 = (v6 ^ v1);
    v0 = (v1 & ~v0);
    v3 = ~v3;
    v0 = ~v0;
#define o0 (v2)
#define o1 (v3)
#define o2 (v4)
#define o3 (v0)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    void* output, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = _mm_and_ps(v4, v1);
    v6 = elem_low_bit0;
    v6 = _mm_or_ps(v6, v1);
    v6 = _mm_and_ps(v5, v6);
    v1 = _mm_xor_ps(v4, v1);
    v4 = _mm_and_ps(v5, v1);
    v1 = _mm_xor_ps(v1, v4);
    v4 = _mm_xor_ps(v6, v1);
    v0 = _mm_andnot_ps(v0, v1);
    v3 = _mm_xor_ps(v3, one);
    v0 = _mm_xor_ps(v0, one);
#define o0 (v2)
#define o1 (v3)
#define o2 (v4)
#define o3 (v0)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
}
"##
    );
    // with aggr_output and input_map and single_buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_input_len(Some(17))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;"))
            .aggr_output_len(Some(17))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(void* output, size_t idx) {
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
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = (v4 & v1);
    v6 = elem_low_bit0;
    v6 = ~(v6 | v1);
    v6 = (v5 & ~v6);
    v1 = (v4 ^ v1);
    v4 = (v5 & v1);
    v1 = (v1 ^ v4);
    v4 = (v6 ^ v1);
    v0 = (v1 & ~v0);
    v3 = ~v3;
    v0 = ~v0;
#define o0 (v2)
#define o1 (v3)
#define o2 (v4)
#define o3 (v0)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
}
"##
    );
    // with aggr_output and input_map and single_buffer
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_input_len(Some(17))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;"))
            .aggr_output_len(Some(17))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(void* output, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = _mm_and_ps(v4, v1);
    v6 = elem_low_bit0;
    v6 = _mm_or_ps(v6, v1);
    v6 = _mm_and_ps(v5, v6);
    v1 = _mm_xor_ps(v4, v1);
    v4 = _mm_and_ps(v5, v1);
    v1 = _mm_xor_ps(v1, v4);
    v4 = _mm_xor_ps(v6, v1);
    v0 = _mm_andnot_ps(v0, v1);
    v3 = _mm_xor_ps(v3, one);
    v0 = _mm_xor_ps(v0, one);
#define o0 (v2)
#define o1 (v3)
#define o2 (v4)
#define o3 (v0)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
}
"##
    );

    // other circuit
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
            (0, false),
            (1, true),
            (4, false),
            (5, true),
            (12, false),
            (13, true),
        ],
    )
    .unwrap();
    // with aggr_output and input_map
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    void* output, size_t idx) {
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
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = (v4 & v1);
    v6 = elem_low_bit0;
    v7 = ~(v6 | v1);
    v7 = (v5 & ~v7);
    v1 = (v4 ^ v1);
    v4 = (v5 & v1);
    v1 = (v1 ^ v4);
    v4 = (v7 ^ v1);
    v1 = (v1 & ~v0);
    v0 = ~v0;
    v3 = ~v3;
    v1 = ~v1;
#define o0 (v6)
#define o1 (v0)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v1)
    output[0] = o0 ^ o1;
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
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .elem_inputs(Some(&[0, 2]))
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const void* input,
    void* output, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
    __m128 v7;
#define i1 (v0)
#define i3 (v1)
#define i4 (v2)
#define i5 (v3)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
#undef i4
#undef i5
    v4 = elem_low_bit1;
    v5 = _mm_and_ps(v4, v1);
    v6 = elem_low_bit0;
    v7 = _mm_or_ps(v6, v1);
    v7 = _mm_and_ps(v5, v7);
    v1 = _mm_xor_ps(v4, v1);
    v4 = _mm_and_ps(v5, v1);
    v1 = _mm_xor_ps(v1, v4);
    v4 = _mm_xor_ps(v7, v1);
    v1 = _mm_andnot_ps(v0, v1);
    v0 = _mm_xor_ps(v0, one);
    v3 = _mm_xor_ps(v3, one);
    v1 = _mm_xor_ps(v1, one);
#define o0 (v6)
#define o1 (v0)
#define o2 (v2)
#define o3 (v3)
#define o4 (v4)
#define o5 (v1)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );
    // with aggr_output and input_map and single buffer
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(11))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;"))
            .aggr_output_len(Some(11))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(void* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    uint32_t v6;
    uint32_t v7;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
#define i4 (v4)
#define i5 (v5)
    i0 = ((TYPE_NAME*)output)[0];
#undef i0
#undef i1
#undef i2
#undef i3
#undef i4
#undef i5
    v6 = (v2 & v3);
    v7 = ~(v0 | v3);
    v7 = (v6 & ~v7);
    v2 = (v2 ^ v3);
    v3 = (v6 & v2);
    v2 = (v2 ^ v3);
    v3 = (v7 ^ v2);
    v2 = (v2 & ~v1);
    v1 = ~v1;
    v5 = ~v5;
    v2 = ~v2;
#define o0 (v0)
#define o1 (v1)
#define o2 (v4)
#define o3 (v5)
#define o4 (v3)
#define o5 (v2)
    output[0] = o0 ^ o1;
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
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(11))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;"))
            .aggr_output_len(Some(11))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    __m128 v5;
    __m128 v6;
    __m128 v7;
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
#define i4 (v4)
#define i5 (v5)
    i0 = ((TYPE_NAME*)output)[0];
#undef i0
#undef i1
#undef i2
#undef i3
#undef i4
#undef i5
    v6 = _mm_and_ps(v2, v3);
    v7 = _mm_or_ps(v0, v3);
    v7 = _mm_and_ps(v6, v7);
    v2 = _mm_xor_ps(v2, v3);
    v3 = _mm_and_ps(v6, v2);
    v2 = _mm_xor_ps(v2, v3);
    v3 = _mm_xor_ps(v7, v2);
    v2 = _mm_andnot_ps(v1, v2);
    v1 = _mm_xor_ps(v1, one);
    v5 = _mm_xor_ps(v5, one);
    v2 = _mm_xor_ps(v2, one);
#define o0 (v0)
#define o1 (v1)
#define o2 (v4)
#define o3 (v5)
#define o4 (v3)
#define o5 (v2)
    output[0] = o0 ^ o1;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)output)[0];"))
            .pop_input_len(Some(11))
            .aggr_output_code(Some("    output[0] = o0 ^ o1;"))
            .aggr_output_len(Some(11))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long output_shift,
    global void* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 6 * idx + output_shift;
    const size_t ovn = 6 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    uint v6;
    uint v7;
    if (idx >= n) return;
    output = (global void*)(((global char*)output) + 4*output_shift);
#define i0 (v0)
#define i1 (v1)
#define i2 (v2)
#define i3 (v3)
#define i4 (v4)
#define i5 (v5)
    i0 = ((TYPE_NAME*)output)[0];
#undef i0
#undef i1
#undef i2
#undef i3
#undef i4
#undef i5
    v6 = (v2 & v3);
    v7 = ~(v0 | v3);
    v7 = (v6 & ~v7);
    v2 = (v2 ^ v3);
    v3 = (v6 & v2);
    v2 = (v2 ^ v3);
    v3 = (v7 ^ v2);
    v2 = (v2 & ~v1);
    v1 = ~v1;
    v5 = ~v5;
    v2 = ~v2;
#define o0 (v0)
#define o1 (v1)
#define o2 (v4)
#define o3 (v5)
#define o4 (v3)
#define o5 (v2)
    output[0] = o0 ^ o1;
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

#[test]
fn test_clang_writer_populate_input_from_buffer() {
    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(0, 2),
            Gate::new_and(1, 2),
            Gate::new_and(0, 3),
            Gate::new_and(1, 3),
            // add a1*b0 + a0*b1
            Gate::new_xor(5, 6),
            Gate::new_and(5, 6),
            // add c(a1*b0 + a0*b1) + a1*b1
            Gate::new_xor(7, 9),
            Gate::new_and(7, 9),
        ],
        [(4, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = input[0];
    v4 = (v0 & v3);
    output[0] = v4;
    v3 = (v1 & v3);
    v0 = (v0 & v2);
    v4 = (v3 ^ v0);
    output[1] = ~v4;
    v1 = (v1 & v2);
    v0 = (v3 & v0);
    v2 = (v1 ^ v0);
    output[2] = v2;
    v0 = (v1 & v0);
    output[3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m128* input,
    __m128* output, void* buffer, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = _mm_loadu_ps((const float*)&input[0]);
    v4 = _mm_and_ps(v0, v3);
    _mm_storeu_ps((float*)&output[0], v4);
    v3 = _mm_and_ps(v1, v3);
    v0 = _mm_and_ps(v0, v2);
    v4 = _mm_xor_ps(v3, v0);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v4, one));
    v1 = _mm_and_ps(v1, v2);
    v0 = _mm_and_ps(v3, v0);
    v2 = _mm_xor_ps(v1, v0);
    _mm_storeu_ps((float*)&output[2], v2);
    v0 = _mm_and_ps(v1, v0);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    unsigned long buffer_shift, const global uint* input,
    global uint* output, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 1 * idx + input_shift;
    const size_t ovn = 4 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    buffer = (global void*)(((global char*)buffer) + 4*buffer_shift);
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = input[ivn + 0];
    v4 = (v0 & v3);
    output[ovn + 0] = v4;
    v3 = (v1 & v3);
    v0 = (v0 & v2);
    v4 = (v3 ^ v0);
    output[ovn + 1] = ~v4;
    v1 = (v1 & v2);
    v0 = (v3 & v0);
    v2 = (v1 ^ v0);
    output[ovn + 2] = v2;
    v0 = (v1 & v0);
    output[ovn + 3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    unsigned long buffer_shift, const global uint* input,
    global uint* output, global void* buffer) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (1 * idx) + input_shift;
    const size_t ovn = llen * (4 * idx) + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    buffer = (global void*)(((global char*)buffer) + 4*buffer_shift);
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = input[ivn + llen*0 + lidx];
    v4 = (v0 & v3);
    output[ovn + llen*0 + lidx] = v4;
    v3 = (v1 & v3);
    v0 = (v0 & v2);
    v4 = (v3 ^ v0);
    output[ovn + llen*1 + lidx] = ~v4;
    v1 = (v1 & v2);
    v0 = (v3 & v0);
    v2 = (v1 ^ v0);
    output[ovn + llen*2 + lidx] = v2;
    v0 = (v1 & v0);
    output[ovn + llen*3 + lidx] = ~v0;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[3, 0, 1])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i3 (v0)
#define i0 (v1)
#define i1 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i3
#undef i0
#undef i1
    v3 = input[0];
    v4 = (v1 & v3);
    output[0] = v4;
    v3 = (v2 & v3);
    v1 = (v1 & v0);
    v4 = (v3 ^ v1);
    output[1] = ~v4;
    v0 = (v2 & v0);
    v1 = (v3 & v1);
    v2 = (v0 ^ v1);
    output[2] = v2;
    v0 = (v0 & v1);
    output[3] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[3, 0, 1])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m128* input,
    __m128* output, void* buffer, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
#define i3 (v0)
#define i0 (v1)
#define i1 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i3
#undef i0
#undef i1
    v3 = _mm_loadu_ps((const float*)&input[0]);
    v4 = _mm_and_ps(v1, v3);
    _mm_storeu_ps((float*)&output[0], v4);
    v3 = _mm_and_ps(v2, v3);
    v1 = _mm_and_ps(v1, v0);
    v4 = _mm_xor_ps(v3, v1);
    _mm_storeu_ps((float*)&output[1], _mm_xor_ps(v4, one));
    v0 = _mm_and_ps(v2, v0);
    v1 = _mm_and_ps(v3, v1);
    v2 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[2], v2);
    v0 = _mm_and_ps(v0, v1);
    _mm_storeu_ps((float*)&output[3], _mm_xor_ps(v0, one));
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = input[0];
    v4 = (v3 & v2);
    v3 = (v3 ^ v2);
    v5 = (v4 & v3);
    output[0] = v5;
    v0 = ~(v0 | v2);
    v2 = (v4 & ~v0);
    output[1] = ~v2;
    v0 = (v3 ^ v0);
    v2 = (v2 ^ v0);
    output[2] = v2;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m64* input,
    __m64* output, void* buffer, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    __m64 v5;
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = input[0];
    v4 = _m_pand(v3, v2);
    v3 = _m_pxor(v3, v2);
    v5 = _m_pand(v4, v3);
    output[0] = v5;
    v0 = _m_por(v0, v2);
    v2 = _m_pand(v4, v0);
    output[1] = _m_pxor(v2, one);
    v0 = _m_pxor(v3, v0);
    v2 = _m_pxor(v2, v0);
    output[2] = _m_pxor(v2, one);
    v0 = _m_por(v0, v1);
    output[3] = v0;
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
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i1 (v0)
#define i3 (v1)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
    v2 = input[1];
    v3 = (v2 & v1);
    v2 = (v2 ^ v1);
    v4 = (v3 & v2);
    output[0] = v4;
    v4 = input[0];
    v1 = ~(v4 | v1);
    v3 = (v3 & ~v1);
    output[1] = ~v3;
    v1 = (v2 ^ v1);
    v2 = (v3 ^ v1);
    output[2] = v2;
    v0 = (v1 & ~v0);
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 3])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m64* input,
    __m64* output, void* buffer, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
#define i1 (v0)
#define i3 (v1)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i3
    v2 = input[1];
    v3 = _m_pand(v2, v1);
    v2 = _m_pxor(v2, v1);
    v4 = _m_pand(v3, v2);
    output[0] = v4;
    v4 = input[0];
    v1 = _m_por(v4, v1);
    v3 = _m_pand(v3, v1);
    output[1] = _m_pxor(v3, one);
    v1 = _m_pxor(v2, v1);
    v2 = _m_pxor(v3, v1);
    output[2] = _m_pxor(v2, one);
    v0 = _m_por(v1, v0);
    output[3] = v0;
    _m_empty();
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
        [(4, false), (5, true), (12, false), (13, true)],
    )
    .unwrap();
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = input[1];
    v4 = (v1 & v3);
    v5 = input[0];
    v5 = ~(v5 | v3);
    v5 = (v4 & ~v5);
    v1 = (v1 ^ v3);
    v3 = (v4 & v1);
    v1 = (v1 ^ v3);
    v3 = (v5 ^ v1);
    output[2] = v3;
    v0 = (v1 & ~v0);
    output[3] = ~v0;
    output[0] = v2;
    v0 = input[2];
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const __m64* input,
    __m64* output, void* buffer, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    __m64 v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = input[1];
    v4 = _m_pand(v1, v3);
    v5 = input[0];
    v5 = _m_por(v5, v3);
    v5 = _m_pand(v4, v5);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pand(v4, v1);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pxor(v5, v1);
    output[2] = v3;
    v0 = _m_pandn(v0, v1);
    output[3] = _m_pxor(v0, one);
    output[0] = v2;
    v0 = input[2];
    output[1] = _m_pxor(v0, one);
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
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4]))
            .input_placement(Some((&[0, 1, 2], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = output[1];
    v4 = (v1 & v3);
    v5 = output[0];
    v5 = ~(v5 | v3);
    v5 = (v4 & ~v5);
    v1 = (v1 ^ v3);
    v3 = (v4 & v1);
    v1 = (v1 ^ v3);
    v3 = (v5 ^ v1);
    v4 = output[2];
    output[2] = v3;
    v0 = (v1 & ~v0);
    output[3] = ~v0;
    output[0] = v2;
    output[1] = ~v4;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4]))
            .input_placement(Some((&[0, 1, 2], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(__m64* output, void* buffer, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    __m64 v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = output[1];
    v4 = _m_pand(v1, v3);
    v5 = output[0];
    v5 = _m_por(v5, v3);
    v5 = _m_pand(v4, v5);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pand(v4, v1);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pxor(v5, v1);
    v4 = output[2];
    output[2] = v3;
    v0 = _m_pandn(v0, v1);
    output[3] = _m_pxor(v0, one);
    output[0] = v2;
    output[1] = _m_pxor(v4, one);
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4]))
            .input_placement(Some((&[0, 1, 2], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long output_shift,
    unsigned long buffer_shift, global uint* output, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 4 * idx + output_shift;
    const size_t ovn = 4 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    if (idx >= n) return;
    buffer = (global void*)(((global char*)buffer) + 4*buffer_shift);
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = output[ivn + 1];
    v4 = (v1 & v3);
    v5 = output[ivn + 0];
    v5 = ~(v5 | v3);
    v5 = (v4 & ~v5);
    v1 = (v1 ^ v3);
    v3 = (v4 & v1);
    v1 = (v1 ^ v3);
    v3 = (v5 ^ v1);
    v4 = output[ivn + 2];
    output[ovn + 2] = v3;
    v0 = (v1 & ~v0);
    output[ovn + 3] = ~v0;
    output[ovn + 0] = v2;
    output[ovn + 1] = ~v4;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4]))
            .input_placement(Some((&[3, 1, 0], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(uint32_t* output, void* buffer, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = output[1];
    v4 = (v1 & v3);
    v5 = output[3];
    v5 = ~(v5 | v3);
    v5 = (v4 & ~v5);
    v1 = (v1 ^ v3);
    v3 = (v4 & v1);
    v1 = (v1 ^ v3);
    v3 = (v5 ^ v1);
    output[2] = v3;
    v0 = (v1 & ~v0);
    output[3] = ~v0;
    v0 = output[0];
    output[0] = v2;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[1, 2, 4]))
            .input_placement(Some((&[3, 1, 0], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(__m64* output, void* buffer, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    __m64 v5;
#define i1 (v0)
#define i2 (v1)
#define i4 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i1
#undef i2
#undef i4
    v3 = output[1];
    v4 = _m_pand(v1, v3);
    v5 = output[3];
    v5 = _m_por(v5, v3);
    v5 = _m_pand(v4, v5);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pand(v4, v1);
    v1 = _m_pxor(v1, v3);
    v3 = _m_pxor(v5, v1);
    output[2] = v3;
    v0 = _m_pandn(v0, v1);
    output[3] = _m_pxor(v0, one);
    v0 = output[0];
    output[0] = v2;
    output[1] = _m_pxor(v0, one);
    _m_empty();
}
"##
    );
    // with arg input
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "testcirc",
        circuit.clone(),
        false,
        CodeConfig::new()
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3]))
            .arg_inputs(Some(&[2])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_testcirc(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = ((arg & 1) != 0) ? one : zero;
    v4 = (v3 & v2);
    v0 = ~(v0 | v2);
    v0 = (v4 & ~v0);
    v2 = (v3 ^ v2);
    v3 = (v4 & v2);
    v2 = (v2 ^ v3);
    v0 = (v0 ^ v2);
    output[2] = v0;
    v0 = (v2 & ~v1);
    output[3] = ~v0;
    v0 = input[0];
    output[0] = v0;
    v0 = input[1];
    output[1] = ~v0;
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
            .pop_input_code(Some("    i0 = ((TYPE_NAME*)input)[0];"))
            .pop_from_buffer(Some(&[0, 1, 3]))
            .arg_inputs(Some(&[2])),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_testcirc(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    unsigned long buffer_shift, const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 2 * idx + input_shift;
    const size_t ovn = 4 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    buffer = (global void*)(((global char*)buffer) + 4*buffer_shift);
#define i0 (v0)
#define i1 (v1)
#define i3 (v2)
    i0 = ((TYPE_NAME*)input)[0];
#undef i0
#undef i1
#undef i3
    v3 = ((arg & 1) != 0) ? one : zero;
    v4 = (v3 & v2);
    v0 = ~(v0 | v2);
    v0 = (v4 & ~v0);
    v2 = (v3 ^ v2);
    v3 = (v4 & v2);
    v2 = (v2 ^ v3);
    v0 = (v0 ^ v2);
    output[ovn + 2] = v0;
    v0 = (v2 & ~v1);
    output[ovn + 3] = ~v0;
    v0 = input[ivn + 0];
    output[ovn + 0] = v0;
    v0 = input[ivn + 1];
    output[ovn + 1] = ~v0;
}
"##
    );
}

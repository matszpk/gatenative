use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_aggregate_output() {
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
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o1;")),
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
void gate_sys_xor(const uint32_t* input,
    void* output, size_t idx) {
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
    v0 = ~v0;
#define o0 (v4)
#define o1 (v0)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1;
#undef o0
#undef o1
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o1;")),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#include <xmmintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[4] = { 0, 0, 0, 0 };
static const unsigned int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
static const unsigned int elem_index_low_tbl[7*4] = {
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
void gate_sys_xor(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
#define o0 (v4)
#define o1 (v0)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1;
#undef o0
#undef o1
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    void* output, size_t idx) {
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
    v0 = ~v0;
    v1 = ~v4;
#define o0 (v4)
#define o1 (v0)
#define o2 (v1)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2;
#undef o0
#undef o1
#undef o2
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
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2;")),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v1 = _mm_xor_ps(v4, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v1)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2;
#undef o0
#undef o1
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
            .aggr_output_code(Some(
                "    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    void* output, size_t idx) {
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
    v0 = ~v0;
    v2 = ~v4;
    v1 = ~v0;
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
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
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some(
                "    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v2 = _mm_xor_ps(v4, one);
    v1 = _mm_xor_ps(v0, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );

    // check double negation - shouldn't be generated
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
            (7, false),
            (4, true),
            (7, true),
            (7, false),
            (4, false),
        ],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some(
                "    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    void* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v0 = _mm_and_ps(v0, v1);
    v0 = _mm_or_ps(v2, v0);
    v0 = _mm_xor_ps(v0, one);
    v2 = _mm_xor_ps(v4, one);
    v1 = _mm_xor_ps(v0, one);
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
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
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some(
                "    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global void* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + input_shift;
    const size_t ovn = 6 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    output = (global void*)(((global char*)output) + 4*output_shift);
    unsigned int xxx = 1111;
    v0 = input[ivn + 0];
    v1 = input[ivn + 1];
    v2 = (v0 ^ v1);
    v3 = input[ivn + 2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    v2 = ~v4;
    v1 = ~v0;
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some(
                "    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global void* output) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (3 * idx) + input_shift;
    const size_t ovn = llen * (6 * idx) + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    output = (global void*)(((global char*)output) + 4*output_shift);
    unsigned int xxx = 1111;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*1 + lidx];
    v2 = (v0 ^ v1);
    v3 = input[ivn + llen*2 + lidx];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v0 = (v0 & v1);
    v0 = ~(v2 | v0);
    v2 = ~v4;
    v1 = ~v0;
#define o0 (v4)
#define o1 (v0)
#define o2 (v2)
#define o3 (v1)
#define o4 (v0)
#define o5 (v4)
    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
#undef o0
#undef o1
#undef o2
#undef o3
#undef o4
#undef o5
}
"##
    );

    // check double negation - shouldn't be generated
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
            (7, false),
            (4, true),
            (7, true),
            (7, false),
            (4, false),
            (0, true),
            (1, false),
            (2, true),
            (0, false),
            (2, false),
            (1, true),
            (2, true),
            (1, false),
            (0, true),
            (0, true),
            (1, false),
            (2, true),
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
            .aggr_output_code(Some(
                "    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    void* output, size_t idx) {
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
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = (v0 ^ v1);
    v3 = input[2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v5 = (v0 & v1);
    v2 = ~(v2 | v5);
    v9 = ~v4;
    v7 = ~v2;
    v5 = ~v0;
    v8 = ~v3;
    v6 = ~v1;
#define o0 (v4)
#define o1 (v2)
#define o2 (v9)
#define o3 (v7)
#define o4 (v2)
#define o5 (v4)
#define o6 (v0)
#define o7 (v1)
#define o8 (v3)
#define o9 (v5)
#define o10 (v8)
#define o11 (v6)
#define o12 (v3)
#define o13 (v1)
#define o14 (v0)
#define o15 (v0)
#define o16 (v1)
#define o17 (v3)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
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
            .aggr_output_code(Some(
                "    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
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
    __m128 v8;
    __m128 v9;
    unsigned int xxx = 1111;
    v0 = input[0];
    v1 = input[1];
    v2 = _mm_xor_ps(v0, v1);
    v3 = input[2];
    v4 = _mm_xor_ps(v3, v2);
    v2 = _mm_and_ps(v3, v2);
    v5 = _mm_and_ps(v0, v1);
    v2 = _mm_or_ps(v2, v5);
    v2 = _mm_xor_ps(v2, one);
    v9 = _mm_xor_ps(v4, one);
    v7 = _mm_xor_ps(v2, one);
    v5 = _mm_xor_ps(v0, one);
    v8 = _mm_xor_ps(v3, one);
    v6 = _mm_xor_ps(v1, one);
#define o0 (v4)
#define o1 (v2)
#define o2 (v9)
#define o3 (v7)
#define o4 (v2)
#define o5 (v4)
#define o6 (v0)
#define o7 (v1)
#define o8 (v3)
#define o9 (v5)
#define o10 (v8)
#define o11 (v6)
#define o12 (v3)
#define o13 (v1)
#define o14 (v0)
#define o15 (v0)
#define o16 (v1)
#define o17 (v3)
    ((TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
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
}
"##
    );
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .init_code(Some("    unsigned int xxx = 1111;"))
            .aggr_output_code(Some(
                "    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;",
            )),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global void* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + input_shift;
    const size_t ovn = 18 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    uint v6;
    uint v7;
    uint v8;
    uint v9;
    if (idx >= n) return;
    output = (global void*)(((global char*)output) + 4*output_shift);
    unsigned int xxx = 1111;
    v0 = input[ivn + 0];
    v1 = input[ivn + 1];
    v2 = (v0 ^ v1);
    v3 = input[ivn + 2];
    v4 = (v3 ^ v2);
    v2 = (v3 & v2);
    v5 = (v0 & v1);
    v2 = ~(v2 | v5);
    v9 = ~v4;
    v7 = ~v2;
    v5 = ~v0;
    v8 = ~v3;
    v6 = ~v1;
#define o0 (v4)
#define o1 (v2)
#define o2 (v9)
#define o3 (v7)
#define o4 (v2)
#define o5 (v4)
#define o6 (v0)
#define o7 (v1)
#define o8 (v3)
#define o9 (v5)
#define o10 (v8)
#define o11 (v6)
#define o12 (v3)
#define o13 (v1)
#define o14 (v0)
#define o15 (v0)
#define o16 (v1)
#define o17 (v3)
    ((global TYPE_NAME*)output)[0] |= o0 ^ o1 ^ o2 & o3 ^ o4 ^ o5;
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
}
"##
    );
}

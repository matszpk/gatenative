use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

fn write_test_code(
    cw_config: &CLangWriterConfig,
    inout_placement: bool,
    arg_input: bool,
) -> String {
    let mut cw = cw_config.writer();
    let supported_ops = cw.supported_ops();
    cw.prolog();
    let mut fw = cw.func_writer(
        "func1",
        3,
        2,
        if inout_placement {
            if arg_input {
                Some((&[11], 68))
            } else {
                Some((&[6, 11, 44], 68))
            }
        } else {
            None
        },
        if inout_placement {
            Some((&[48, 72], 88))
        } else {
            None
        },
        if arg_input { Some(&[0, 2]) } else { None },
    );
    fw.func_start();
    fw.alloc_vars(5);
    fw.gen_load(2, 0);
    fw.gen_load(1, 1);
    fw.gen_load(0, 2);
    fw.gen_op(InstrOp::And, VNegs::NoNegs, 2, 0, 1);
    fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 1);
    fw.gen_op(InstrOp::Xor, VNegs::NoNegs, 3, 0, 1);
    fw.gen_op(InstrOp::And, VNegs::NegOutput, 3, 0, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_IMPL)) != 0 {
        fw.gen_op(InstrOp::Impl, VNegs::NoNegs, 3, 2, 1);
    }
    fw.gen_store(true, 1, 3);
    fw.gen_op(InstrOp::Or, VNegs::NegOutput, 2, 2, 3);
    fw.gen_op(InstrOp::Xor, VNegs::NegOutput, 4, 1, 3);
    fw.gen_op(InstrOp::And, VNegs::NegInput1, 4, 4, 1);
    fw.gen_op(InstrOp::Xor, VNegs::NegInput1, 4, 4, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_NIMPL)) != 0 {
        fw.gen_op(InstrOp::Nimpl, VNegs::NoNegs, 4, 2, 4);
    }
    fw.gen_store(false, 0, 4);
    fw.func_end();
    cw.epilog();
    String::from_utf8(cw.out()).unwrap()
}

fn write_test_code_2(
    cw_config: &CLangWriterConfig,
    inout_placement: bool,
    arg_input: bool,
) -> String {
    let mut cw = cw_config.writer();
    cw.prolog();
    let mut fw = cw.func_writer(
        "func1",
        5,
        2,
        if inout_placement {
            if arg_input {
                Some((&[11, 27], 68))
            } else {
                Some((&[6, 11, 44, 50, 27], 68))
            }
        } else {
            None
        },
        if inout_placement {
            Some((&[48, 72], 88))
        } else {
            None
        },
        if arg_input { Some(&[0, 3, 2]) } else { None },
    );
    fw.func_start();
    fw.alloc_vars(5);
    fw.gen_load(4, 0);
    fw.gen_load(3, 1);
    fw.gen_load(2, 2);
    fw.gen_load(1, 3);
    fw.gen_load(0, 4);
    fw.gen_op(InstrOp::And, VNegs::NoNegs, 0, 0, 1);
    fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 3);
    fw.gen_op(InstrOp::Xor, VNegs::NoNegs, 0, 0, 1);
    fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 4);
    fw.gen_store(true, 0, 0);
    fw.gen_store(false, 1, 1);
    fw.func_end();
    cw.epilog();
    String::from_utf8(cw.out()).unwrap()
}

fn write_test_code_single_buffer(
    cw_config: &CLangWriterConfig,
    inout_placement: bool,
    arg_input: bool,
) -> String {
    let mut cw = cw_config.writer();
    let supported_ops = cw.supported_ops();
    cw.prolog();
    let mut fw = cw.func_writer_with_config(
        "func1",
        3,
        3,
        CodeConfig::new()
            .input_placement(if inout_placement {
                if arg_input {
                    Some((&[11], 99))
                } else {
                    Some((&[6, 11, 44], 99))
                }
            } else {
                None
            })
            .output_placement(if inout_placement {
                Some((&[48, 72, 25], 99))
            } else {
                None
            })
            .arg_inputs(if arg_input { Some(&[0, 2]) } else { None })
            .single_buffer(true),
        None,
    );
    fw.func_start();
    fw.alloc_vars(5);
    fw.gen_load(2, 0);
    fw.gen_load(1, 1);
    fw.gen_load(0, 2);
    fw.gen_op(InstrOp::And, VNegs::NoNegs, 2, 0, 1);
    fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 1);
    fw.gen_op(InstrOp::Xor, VNegs::NoNegs, 3, 0, 1);
    fw.gen_store(false, 2, 3);
    fw.gen_op(InstrOp::And, VNegs::NegOutput, 3, 0, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_IMPL)) != 0 {
        fw.gen_op(InstrOp::Impl, VNegs::NoNegs, 3, 2, 1);
    }
    fw.gen_store(true, 1, 3);
    fw.gen_op(InstrOp::Or, VNegs::NegOutput, 2, 2, 3);
    fw.gen_op(InstrOp::Xor, VNegs::NegOutput, 4, 1, 3);
    fw.gen_op(InstrOp::And, VNegs::NegInput1, 4, 4, 1);
    fw.gen_op(InstrOp::Xor, VNegs::NegInput1, 4, 4, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_NIMPL)) != 0 {
        fw.gen_op(InstrOp::Nimpl, VNegs::NoNegs, 4, 2, 4);
    }
    fw.gen_store(false, 0, 4);
    fw.func_end();
    cw.epilog();
    String::from_utf8(cw.out()).unwrap()
}

fn write_test_code_with_not(
    cw_config: &CLangWriterConfig,
    inout_placement: bool,
    arg_input: bool,
) -> String {
    let mut cw = cw_config.writer();
    let supported_ops = cw.supported_ops();
    cw.prolog();
    let mut fw = cw.func_writer(
        "func1",
        3,
        2,
        if inout_placement {
            if arg_input {
                Some((&[11], 68))
            } else {
                Some((&[6, 11, 44], 68))
            }
        } else {
            None
        },
        if inout_placement {
            Some((&[48, 72], 88))
        } else {
            None
        },
        if arg_input { Some(&[0, 2]) } else { None },
    );
    fw.func_start();
    fw.alloc_vars(5);
    fw.gen_load(2, 0);
    fw.gen_load(1, 1);
    fw.gen_load(0, 2);
    fw.gen_op(InstrOp::And, VNegs::NoNegs, 2, 0, 1);
    fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 1);
    fw.gen_op(InstrOp::Xor, VNegs::NoNegs, 3, 0, 1);
    fw.gen_op(InstrOp::And, VNegs::NegOutput, 3, 0, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_IMPL)) != 0 {
        fw.gen_op(InstrOp::Impl, VNegs::NoNegs, 3, 2, 1);
    }
    fw.gen_store(true, 1, 3);
    fw.gen_op(InstrOp::Or, VNegs::NegOutput, 2, 2, 3);
    fw.gen_op(InstrOp::Xor, VNegs::NegOutput, 4, 1, 3);
    fw.gen_op(InstrOp::And, VNegs::NegInput1, 4, 4, 1);
    fw.gen_not(4, 1);
    if (supported_ops & (1u64 << INSTR_OP_VALUE_NIMPL)) != 0 {
        fw.gen_op(InstrOp::Nimpl, VNegs::NoNegs, 4, 2, 4);
    }
    fw.gen_store(false, 0, 4);
    fw.func_end();
    cw.epilog();
    String::from_utf8(cw.out()).unwrap()
}

#[test]
fn test_clang_writer() {
    {
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U32, false, false)
        );

        // with not
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = ~v1;
    output[0] = v4;
}
"##,
            write_test_code_with_not(&CLANG_WRITER_U32, false, false)
        );

        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = input[6];
    v1 = input[11];
    v0 = input[44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U32, true, false)
        );
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[0];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U32, false, true)
        );
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[11];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U32, true, true)
        );
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v4 = ((arg & 1) != 0) ? one : zero;
    v3 = input[11];
    v2 = ((arg & 4) != 0) ? one : zero;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = input[27];
    v0 = (v0 & v1);
    v1 = (v2 | v3);
    v0 = (v0 ^ v1);
    v1 = (v2 | v4);
    output[48] = ~v0;
    output[72] = v1;
}
"##,
            write_test_code_2(&CLANG_WRITER_U32, true, true)
        );
    }
    {
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (64)
#define TYPE_NAME uint64_t
#define GET_U32(D,X,I) { (D) = ((X) >> ((I)<<5)); }
#define GET_U32_ALL(D,X) { (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }
#define SET_U32(X,S,I) { uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }
#define SET_U32_ALL(X,S) { (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }
void gate_sys_func1(const uint64_t* input,
    uint64_t* output, size_t idx) {
    uint64_t v0;
    uint64_t v1;
    uint64_t v2;
    uint64_t v3;
    uint64_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U64, false, false)
        );
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (64)
#define TYPE_NAME uint64_t
#define GET_U32(D,X,I) { (D) = ((X) >> ((I)<<5)); }
#define GET_U32_ALL(D,X) { (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }
#define SET_U32(X,S,I) { uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }
#define SET_U32_ALL(X,S) { (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }
void gate_sys_func1(const uint64_t* input,
    uint64_t* output, size_t idx) {
    uint64_t v0;
    uint64_t v1;
    uint64_t v2;
    uint64_t v3;
    uint64_t v4;
    v2 = input[6];
    v1 = input[11];
    v0 = input[44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U64, true, false)
        );
        assert_eq!(
            r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (64)
#define TYPE_NAME uint64_t
#define GET_U32(D,X,I) { (D) = ((X) >> ((I)<<5)); }
#define GET_U32_ALL(D,X) { (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }
#define SET_U32(X,S,I) { uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }
#define SET_U32_ALL(X,S) { (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }
void gate_sys_func1(const uint64_t* input,
    uint64_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint64_t zero = 0ULL;
    const uint64_t one = 0xffffffffffffffffULL;
    uint64_t v0;
    uint64_t v1;
    uint64_t v2;
    uint64_t v3;
    uint64_t v4;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[0];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_U64, false, true)
        );
    }
    assert_eq!(
        r##"#include <mmintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[2] = { 0, 0 };
static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };
static const unsigned int elem_index_low_tbl[6*2] = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xcccccccc, 0xcccccccc, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xffff0000, 0xffff0000, 0x00000000, 0xffffffff
};
#define TYPE_LEN (64)
#define TYPE_NAME __m64
#define GET_U32(D,X,I) { (D) = (uint32_t)(_m_to_int(_mm_srli_si64((X), ((I) << 5)))); }
#define GET_U32_ALL(D,X) { \
    (D)[0] = (uint32_t)(_m_to_int((X))); \
    (D)[1] = (uint32_t)(_m_to_int(_mm_srli_si64((X), 32))); \
}
#define SET_U32(X,S,I) { uint32_t temp[2]; \
    *(__m64*)temp = (X); \
    temp[(I)] = (S); \
    (X) = *(__m64*)temp; \
}
#define SET_U32_ALL(X,S) { (X) = *(__m64*)(S); }
void gate_sys_func1(const __m64* input,
    __m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _m_pand(v0, v1);
    v1 = _m_por(v2, v1);
    v3 = _m_pxor(v0, v1);
    v3 = _m_pxor(_m_pand(v0, v1), one);
    output[1] = _m_pxor(v3, one);
    v2 = _m_pxor(_m_por(v2, v3), one);
    v4 = _m_pxor(_m_pxor(v1, v3), one);
    v4 = _m_pand(v4, _m_pxor(v1, one));
    v4 = _m_pxor(v4, _m_pxor(v1, one));
    v4 = _m_pandn(v4, v2);
    output[0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_MMX, false, false)
    );

    // with not
    assert_eq!(
        r##"#include <mmintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[2] = { 0, 0 };
static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };
static const unsigned int elem_index_low_tbl[6*2] = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xcccccccc, 0xcccccccc, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xffff0000, 0xffff0000, 0x00000000, 0xffffffff
};
#define TYPE_LEN (64)
#define TYPE_NAME __m64
#define GET_U32(D,X,I) { (D) = (uint32_t)(_m_to_int(_mm_srli_si64((X), ((I) << 5)))); }
#define GET_U32_ALL(D,X) { \
    (D)[0] = (uint32_t)(_m_to_int((X))); \
    (D)[1] = (uint32_t)(_m_to_int(_mm_srli_si64((X), 32))); \
}
#define SET_U32(X,S,I) { uint32_t temp[2]; \
    *(__m64*)temp = (X); \
    temp[(I)] = (S); \
    (X) = *(__m64*)temp; \
}
#define SET_U32_ALL(X,S) { (X) = *(__m64*)(S); }
void gate_sys_func1(const __m64* input,
    __m64* output, size_t idx) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _m_pand(v0, v1);
    v1 = _m_por(v2, v1);
    v3 = _m_pxor(v0, v1);
    v3 = _m_pxor(_m_pand(v0, v1), one);
    output[1] = _m_pxor(v3, one);
    v2 = _m_pxor(_m_por(v2, v3), one);
    v4 = _m_pxor(_m_pxor(v1, v3), one);
    v4 = _m_pand(v4, _m_pxor(v1, one));
    v4 = _m_pxor(v1, one);
    v4 = _m_pandn(v4, v2);
    output[0] = v4;
}
"##,
        write_test_code_with_not(&CLANG_WRITER_INTEL_MMX, false, false)
    );

    assert_eq!(
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
#define SET_U32(X,S,I) { uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_ps((float*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm_loadu_ps((float*)(S)); }
void gate_sys_func1(const __m128* input,
    __m128* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm_and_ps(v0, v1);
    v1 = _mm_or_ps(v2, v1);
    v3 = _mm_xor_ps(v0, v1);
    v3 = _mm_xor_ps(_mm_and_ps(v0, v1), one);
    output[1] = _mm_xor_ps(v3, one);
    v2 = _mm_xor_ps(_mm_or_ps(v2, v3), one);
    v4 = _mm_xor_ps(_mm_xor_ps(v1, v3), one);
    v4 = _mm_and_ps(v4, _mm_xor_ps(v1, one));
    v4 = _mm_xor_ps(v4, _mm_xor_ps(v1, one));
    v4 = _mm_andnot_ps(v4, v2);
    output[0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_SSE, false, false)
    );
    assert_eq!(
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
#define TYPE_NAME __m128i
#define GET_U32(D,X,I) { uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { _mm_storeu_si128((__m128i*)(D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_si128((__m128i*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm_loadu_si128((__m128i*)(S)); }
void gate_sys_func1(const __m128i* input,
    __m128i* output, size_t idx) {
    const __m128i one = *((const __m128i*)one_value);
    __m128i v0;
    __m128i v1;
    __m128i v2;
    __m128i v3;
    __m128i v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm_and_si128(v0, v1);
    v1 = _mm_or_si128(v2, v1);
    v3 = _mm_xor_si128(v0, v1);
    v3 = _mm_xor_si128(_mm_and_si128(v0, v1), one);
    output[1] = _mm_xor_si128(v3, one);
    v2 = _mm_xor_si128(_mm_or_si128(v2, v3), one);
    v4 = _mm_xor_si128(_mm_xor_si128(v1, v3), one);
    v4 = _mm_and_si128(v4, _mm_xor_si128(v1, one));
    v4 = _mm_xor_si128(v4, _mm_xor_si128(v1, one));
    v4 = _mm_andnot_si128(v4, v2);
    output[0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_SSE2, false, false)
    );
    // with not
    assert_eq!(
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
#define SET_U32(X,S,I) { uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_ps((float*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm_loadu_ps((float*)(S)); }
void gate_sys_func1(const __m128* input,
    __m128* output, size_t idx) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm_and_ps(v0, v1);
    v1 = _mm_or_ps(v2, v1);
    v3 = _mm_xor_ps(v0, v1);
    v3 = _mm_xor_ps(_mm_and_ps(v0, v1), one);
    output[1] = _mm_xor_ps(v3, one);
    v2 = _mm_xor_ps(_mm_or_ps(v2, v3), one);
    v4 = _mm_xor_ps(_mm_xor_ps(v1, v3), one);
    v4 = _mm_and_ps(v4, _mm_xor_ps(v1, one));
    v4 = _mm_xor_ps(v1, one);
    v4 = _mm_andnot_ps(v4, v2);
    output[0] = v4;
}
"##,
        write_test_code_with_not(&CLANG_WRITER_INTEL_SSE, false, false)
    );
    assert_eq!(
        r##"#include <immintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};
static const unsigned int one_value[8] __attribute__((aligned(32))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
static const unsigned int elem_index_low_tbl[8*8]
__attribute__((aligned(32))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
#define TYPE_LEN (256)
#define TYPE_NAME __m256
#define GET_U32(D,X,I) { uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { _mm256_storeu_ps((float*)(D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_ps((float*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm256_loadu_ps((float*)(S)); }
void gate_sys_func1(const __m256* input,
    __m256* output, size_t idx) {
    const __m256 one = *((const __m256*)one_value);
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    v2 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[1]);
    v0 = _mm256_loadu_ps((const float*)&input[2]);
    v2 = _mm256_and_ps(v0, v1);
    v1 = _mm256_or_ps(v2, v1);
    v3 = _mm256_xor_ps(v0, v1);
    v3 = _mm256_xor_ps(_mm256_and_ps(v0, v1), one);
    _mm256_storeu_ps((float*)&output[1], _mm256_xor_ps(v3, one));
    v2 = _mm256_xor_ps(_mm256_or_ps(v2, v3), one);
    v4 = _mm256_xor_ps(_mm256_xor_ps(v1, v3), one);
    v4 = _mm256_and_ps(v4, _mm256_xor_ps(v1, one));
    v4 = _mm256_xor_ps(v4, _mm256_xor_ps(v1, one));
    v4 = _mm256_andnot_ps(v4, v2);
    _mm256_storeu_ps((float*)&output[0], v4);
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_AVX, false, false)
    );
    assert_eq!(
        r##"#include <immintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};
static const unsigned int one_value[8] __attribute__((aligned(32))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
static const unsigned int elem_index_low_tbl[8*8]
__attribute__((aligned(32))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
#define TYPE_LEN (256)
#define TYPE_NAME __m256i
#define GET_U32(D,X,I) { uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { _mm256_storeu_si256((__m256i*)(D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_si256((__m256i*)temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm256_loadu_si256((__m256i*)(S)); }
void gate_sys_func1(const __m256i* input,
    __m256i* output, size_t idx) {
    const __m256i one = *((const __m256i*)one_value);
    __m256i v0;
    __m256i v1;
    __m256i v2;
    __m256i v3;
    __m256i v4;
    v2 = _mm256_loadu_si256((const float*)&input[0]);
    v1 = _mm256_loadu_si256((const float*)&input[1]);
    v0 = _mm256_loadu_si256((const float*)&input[2]);
    v2 = _mm256_and_si256(v0, v1);
    v1 = _mm256_or_si256(v2, v1);
    v3 = _mm256_xor_si256(v0, v1);
    v3 = _mm256_xor_si256(_mm256_and_si256(v0, v1), one);
    _mm256_storeu_si256((float*)&output[1], _mm256_xor_si256(v3, one));
    v2 = _mm256_xor_si256(_mm256_or_si256(v2, v3), one);
    v4 = _mm256_xor_si256(_mm256_xor_si256(v1, v3), one);
    v4 = _mm256_and_si256(v4, _mm256_xor_si256(v1, one));
    v4 = _mm256_xor_si256(v4, _mm256_xor_si256(v1, one));
    v4 = _mm256_andnot_si256(v4, v2);
    _mm256_storeu_si256((float*)&output[0], v4);
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_AVX2, false, false)
    );
    assert_eq!(
        r##"#include <immintrin.h>
#include <stddef.h>
#include <stdint.h>
static const unsigned int zero_value[16] __attribute__((aligned(64))) = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};
static const unsigned int one_value[16] __attribute__((aligned(64))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
static const unsigned int elem_index_low_tbl[9*16]
__attribute__((aligned(64))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
#define TYPE_LEN (512)
#define TYPE_NAME __m512i
#define GET_U32(D,X,I) { uint32_t temp[16]; \
    _mm512_storeu_si512(temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { _mm512_storeu_si512((float*)(D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[16]; \
    _mm512_storeu_si512(temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm512_loadu_si512(temp); \
}
#define SET_U32_ALL(X,S) { (X) = _mm512_loadu_si512((S)); }
void gate_sys_func1(const __m512i* input,
    __m512i* output, size_t idx) {
    const __m512i one = *((const __m512i*)one_value);
    __m512i v0;
    __m512i v1;
    __m512i v2;
    __m512i v3;
    __m512i v4;
    v2 = _mm512_loadu_epi64(&input[0]);
    v1 = _mm512_loadu_epi64(&input[1]);
    v0 = _mm512_loadu_epi64(&input[2]);
    v2 = _mm512_and_epi64(v0, v1);
    v1 = _mm512_or_epi64(v2, v1);
    v3 = _mm512_xor_epi64(v0, v1);
    v3 = _mm512_xor_epi64(_mm512_and_epi64(v0, v1), one);
    _mm512_storeu_epi64(&output[1], _mm512_xor_epi64(v3, one));
    v2 = _mm512_xor_epi64(_mm512_or_epi64(v2, v3), one);
    v4 = _mm512_xor_epi64(_mm512_xor_epi64(v1, v3), one);
    v4 = _mm512_and_epi64(v4, _mm512_xor_epi64(v1, one));
    v4 = _mm512_xor_epi64(v4, _mm512_xor_epi64(v1, one));
    v4 = _mm512_andnot_epi64(v4, v2);
    _mm512_storeu_epi64(&output[0], v4);
}
"##,
        write_test_code(&CLANG_WRITER_INTEL_AVX512, false, false)
    );
    assert_eq!(
        r##"#include <arm_neon.h>
#include <stddef.h>
#include <stdint.h>
#define TYPE_LEN (128)
#define TYPE_NAME uint32x4_t
#define GET_U32(D,X,I) { uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { vst4q_u32((D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
    (X) = vld4q_u32(temp); \
}
#define SET_U32_ALL(X,S) { (X) = vld4q_u32((S)); }
void gate_sys_func1(const uint32x4_t* input,
    uint32x4_t* output, size_t idx) {
    uint32x4_t v0;
    uint32x4_t v1;
    uint32x4_t v2;
    uint32x4_t v3;
    uint32x4_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = vandq_u32(v0, v1);
    v1 = vorrq_u32(v2, v1);
    v3 = veorq_u32(v0, v1);
    v3 = vmvnq_u32(vandq_u32(v0, v1));
    v3 = vornq_u32(v1, v2);
    output[1] = vmvnq_u32(v3);
    v2 = vmvnq_u32(vorrq_u32(v2, v3));
    v4 = vmvnq_u32(veorq_u32(v1, v3));
    v4 = vandq_u32(v4, vmvnq_u32(v1));
    v4 = veorq_u32(v4, vmvnq_u32(v1));
    output[0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_ARM_NEON, false, false)
    );
    // with not
    assert_eq!(
        r##"#include <arm_neon.h>
#include <stddef.h>
#include <stdint.h>
#define TYPE_LEN (128)
#define TYPE_NAME uint32x4_t
#define GET_U32(D,X,I) { uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
}
#define GET_U32_ALL(D,X) { vst4q_u32((D), (X)); }
#define SET_U32(X,S,I) { uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
    (X) = vld4q_u32(temp); \
}
#define SET_U32_ALL(X,S) { (X) = vld4q_u32((S)); }
void gate_sys_func1(const uint32x4_t* input,
    uint32x4_t* output, size_t idx) {
    uint32x4_t v0;
    uint32x4_t v1;
    uint32x4_t v2;
    uint32x4_t v3;
    uint32x4_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = vandq_u32(v0, v1);
    v1 = vorrq_u32(v2, v1);
    v3 = veorq_u32(v0, v1);
    v3 = vmvnq_u32(vandq_u32(v0, v1));
    v3 = vornq_u32(v1, v2);
    output[1] = vmvnq_u32(v3);
    v2 = vmvnq_u32(vorrq_u32(v2, v3));
    v4 = vmvnq_u32(veorq_u32(v1, v3));
    v4 = vandq_u32(v4, vmvnq_u32(v1));
    v4 = vmvnq_u32(v1);
    output[0] = v4;
}
"##,
        write_test_code_with_not(&CLANG_WRITER_ARM_NEON, false, false)
    );

    // opencl
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
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
    v2 = input[ivn + 0];
    v1 = input[ivn + 1];
    v0 = input[ivn + 2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32, false, false)
    );
    // with not
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
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
    v2 = input[ivn + 0];
    v1 = input[ivn + 1];
    v0 = input[ivn + 2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = ~v1;
    output[ovn + 0] = v4;
}
"##,
        write_test_code_with_not(&CLANG_WRITER_OPENCL_U32, false, false)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 68 * idx + input_shift;
    const size_t ovn = 88 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = input[ivn + 6];
    v1 = input[ivn + 11];
    v0 = input[ivn + 44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 48] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32, true, false)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 1 * idx + input_shift;
    const size_t ovn = 2 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[ivn + 0];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 0] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32, false, true)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 68 * idx + input_shift;
    const size_t ovn = 88 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[ivn + 11];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 48] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32, true, true)
    );

    // single buffer
    assert_eq!(
        r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = output[0];
    v1 = output[1];
    v0 = output[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[2] = v3;
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_U32, false, false)
    );
    assert_eq!(
        r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(uint32_t* output, size_t idx) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = output[6];
    v1 = output[11];
    v0 = output[44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[25] = v3;
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_U32, true, false)
    );
    assert_eq!(
        r##"#include <stdint.h>
#include <stddef.h>
#define TYPE_LEN (32)
#define TYPE_NAME uint32_t
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
void gate_sys_func1(uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = output[11];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[25] = v3;
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_U32, true, true)
    );

    // single buffer - opencl
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long output_shift,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 3 * idx + output_shift;
    const size_t ovn = 3 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = output[ivn + 0];
    v1 = output[ivn + 1];
    v0 = output[ivn + 2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[ovn + 2] = v3;
    v3 = ~(v0 & v1);
    output[ovn + 1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 0] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_OPENCL_U32, false, false)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long output_shift,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 99 * idx + output_shift;
    const size_t ovn = 99 * idx + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = output[ivn + 6];
    v1 = output[ivn + 11];
    v0 = output[ivn + 44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[ovn + 25] = v3;
    v3 = ~(v0 & v1);
    output[ovn + 72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 48] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_OPENCL_U32, true, false)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long output_shift,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 99 * idx + output_shift;
    const size_t ovn = 99 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = output[ivn + 11];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[ovn + 25] = v3;
    v3 = ~(v0 & v1);
    output[ovn + 72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 48] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_OPENCL_U32, true, true)
    );

    // opencl group_vec
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
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
    v2 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*1 + lidx];
    v0 = input[ivn + llen*2 + lidx];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + llen*1 + lidx] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + llen*0 + lidx] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32_GROUP_VEC, false, false)
    );
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (1 * idx) + input_shift;
    const size_t ovn = llen * (2 * idx) + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = ((arg & 1) != 0) ? one : zero;
    v1 = input[ivn + llen*0 + lidx];
    v0 = ((arg & 2) != 0) ? one : zero;
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + llen*1 + lidx] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + llen*0 + lidx] = v4;
}
"##,
        write_test_code(&CLANG_WRITER_OPENCL_U32_GROUP_VEC, false, true)
    );
    // single buffer - opencl group_vec
    assert_eq!(
        r##"#define TYPE_LEN (32)
#define TYPE_NAME uint
#define GET_U32(D,X,I) { (D) = (X); }
#define GET_U32_ALL(D,X) { (D)[0] = (X); }
#define SET_U32(X,S,I) { (X) = (S); }
#define SET_U32_ALL(X,S) { (X) = (S)[0]; }
kernel void gate_sys_func1(unsigned long n, 
    unsigned long output_shift,
    global uint* output) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (3 * idx) + output_shift;
    const size_t ovn = llen * (3 * idx) + output_shift;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = output[ivn + llen*0 + lidx];
    v1 = output[ivn + llen*1 + lidx];
    v0 = output[ivn + llen*2 + lidx];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    output[ovn + llen*2 + lidx] = v3;
    v3 = ~(v0 & v1);
    output[ovn + llen*1 + lidx] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + llen*0 + lidx] = v4;
}
"##,
        write_test_code_single_buffer(&CLANG_WRITER_OPENCL_U32_GROUP_VEC, false, false)
    );
}

#[test]
fn test_clang_writer_arginput_64bit() {
    let circuit = Circuit::new(
        120,
        (0..60).map(|i| Gate::new_xor(i, 60 + i)),
        (0..60).map(|i| (120 + i, false)),
    )
    .unwrap();
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit,
        false,
        CodeConfig::new().arg_inputs(Some(&(120 - 64..120).collect::<Vec<_>>())),
    );
    let out = String::from_utf8(writer.out()).unwrap();
    assert_eq!(
        &out,
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 56 * idx + input_shift;
    const size_t ovn = 60 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = input[ivn + 1];
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = input[ivn + 2];
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = input[ivn + 3];
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = input[ivn + 4];
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = input[ivn + 5];
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = input[ivn + 6];
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = input[ivn + 7];
    v1 = ((arg & 2048) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = input[ivn + 8];
    v1 = ((arg & 4096) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = input[ivn + 9];
    v1 = ((arg & 8192) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = input[ivn + 10];
    v1 = ((arg & 16384) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = input[ivn + 11];
    v1 = ((arg & 32768) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = input[ivn + 12];
    v1 = ((arg & 65536) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = input[ivn + 13];
    v1 = ((arg & 131072) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 14];
    v1 = ((arg & 262144) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
    v0 = input[ivn + 15];
    v1 = ((arg & 524288) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 15] = v0;
    v0 = input[ivn + 16];
    v1 = ((arg & 1048576) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 16] = v0;
    v0 = input[ivn + 17];
    v1 = ((arg & 2097152) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 17] = v0;
    v0 = input[ivn + 18];
    v1 = ((arg & 4194304) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 18] = v0;
    v0 = input[ivn + 19];
    v1 = ((arg & 8388608) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 19] = v0;
    v0 = input[ivn + 20];
    v1 = ((arg & 16777216) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 20] = v0;
    v0 = input[ivn + 21];
    v1 = ((arg & 33554432) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 21] = v0;
    v0 = input[ivn + 22];
    v1 = ((arg & 67108864) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 22] = v0;
    v0 = input[ivn + 23];
    v1 = ((arg & 134217728) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 23] = v0;
    v0 = input[ivn + 24];
    v1 = ((arg & 268435456) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 24] = v0;
    v0 = input[ivn + 25];
    v1 = ((arg & 536870912) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 25] = v0;
    v0 = input[ivn + 26];
    v1 = ((arg & 1073741824) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 26] = v0;
    v0 = input[ivn + 27];
    v1 = ((arg & 2147483648) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 27] = v0;
    v0 = input[ivn + 28];
    v1 = ((arg2 & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 28] = v0;
    v0 = input[ivn + 29];
    v1 = ((arg2 & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 29] = v0;
    v0 = input[ivn + 30];
    v1 = ((arg2 & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 30] = v0;
    v0 = input[ivn + 31];
    v1 = ((arg2 & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 31] = v0;
    v0 = input[ivn + 32];
    v1 = ((arg2 & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 32] = v0;
    v0 = input[ivn + 33];
    v1 = ((arg2 & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 33] = v0;
    v0 = input[ivn + 34];
    v1 = ((arg2 & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 34] = v0;
    v0 = input[ivn + 35];
    v1 = ((arg2 & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 35] = v0;
    v0 = input[ivn + 36];
    v1 = ((arg2 & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 36] = v0;
    v0 = input[ivn + 37];
    v1 = ((arg2 & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 37] = v0;
    v0 = input[ivn + 38];
    v1 = ((arg2 & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 38] = v0;
    v0 = input[ivn + 39];
    v1 = ((arg2 & 2048) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 39] = v0;
    v0 = input[ivn + 40];
    v1 = ((arg2 & 4096) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 40] = v0;
    v0 = input[ivn + 41];
    v1 = ((arg2 & 8192) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 41] = v0;
    v0 = input[ivn + 42];
    v1 = ((arg2 & 16384) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 42] = v0;
    v0 = input[ivn + 43];
    v1 = ((arg2 & 32768) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 43] = v0;
    v0 = input[ivn + 44];
    v1 = ((arg2 & 65536) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 44] = v0;
    v0 = input[ivn + 45];
    v1 = ((arg2 & 131072) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 45] = v0;
    v0 = input[ivn + 46];
    v1 = ((arg2 & 262144) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 46] = v0;
    v0 = input[ivn + 47];
    v1 = ((arg2 & 524288) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 47] = v0;
    v0 = input[ivn + 48];
    v1 = ((arg2 & 1048576) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 48] = v0;
    v0 = input[ivn + 49];
    v1 = ((arg2 & 2097152) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 49] = v0;
    v0 = input[ivn + 50];
    v1 = ((arg2 & 4194304) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 50] = v0;
    v0 = input[ivn + 51];
    v1 = ((arg2 & 8388608) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 51] = v0;
    v0 = input[ivn + 52];
    v1 = ((arg2 & 16777216) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 52] = v0;
    v0 = input[ivn + 53];
    v1 = ((arg2 & 33554432) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 53] = v0;
    v0 = input[ivn + 54];
    v1 = ((arg2 & 67108864) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 54] = v0;
    v0 = input[ivn + 55];
    v1 = ((arg2 & 134217728) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 55] = v0;
    v0 = ((arg & 1) != 0) ? one : zero;
    v1 = ((arg2 & 268435456) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 56] = v0;
    v0 = ((arg & 2) != 0) ? one : zero;
    v1 = ((arg2 & 536870912) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 57] = v0;
    v0 = ((arg & 4) != 0) ? one : zero;
    v1 = ((arg2 & 1073741824) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 58] = v0;
    v0 = ((arg & 8) != 0) ? one : zero;
    v1 = ((arg2 & 2147483648) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 59] = v0;
}
"##
    );
}

#[test]
fn test_clang_writer_extra() {
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
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 0], 4)))
            .output_placement(Some((&[3, 2, 1, 0], 4)))
            .arg_inputs(Some(&[0, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    v0 = ((arg & 2) != 0) ? one : zero;
    v1 = output[0];
    v2 = (v0 & v1);
    v0 = (v0 ^ v1);
    v3 = (v2 & v0);
    output[3] = v3;
    v3 = ((arg & 1) != 0) ? one : zero;
    v1 = ~(v3 | v1);
    v2 = (v2 & ~v1);
    output[2] = ~v2;
    v0 = (v0 ^ v1);
    v1 = (v2 ^ v0);
    v2 = output[1];
    output[1] = v1;
    v0 = (v0 & ~v2);
    output[0] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 0], 4)))
            .output_placement(Some((&[3, 2, 1, 0], 4)))
            .arg_inputs(Some(&[0, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(__m64* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    v0 = ((arg & 2) != 0) ? one : zero;
    v1 = output[0];
    v2 = _m_pand(v0, v1);
    v0 = _m_pxor(v0, v1);
    v3 = _m_pand(v2, v0);
    output[3] = v3;
    v3 = ((arg & 1) != 0) ? one : zero;
    v1 = _m_por(v3, v1);
    v2 = _m_pand(v2, v1);
    output[2] = _m_pxor(v2, one);
    v0 = _m_pxor(v0, v1);
    v1 = _m_pxor(v2, v0);
    v2 = output[1];
    output[1] = _m_pxor(v1, one);
    v0 = _m_por(v0, v2);
    output[0] = v0;
}
"##
    );

    // with arg_inputs and single_buffer
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
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .arg_inputs(Some(&[0, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    v0 = ((arg & 2) != 0) ? one : zero;
    v1 = output[1];
    v2 = (v0 & v1);
    v3 = ((arg & 1) != 0) ? one : zero;
    v3 = ~(v3 | v1);
    v3 = (v2 & ~v3);
    v0 = (v0 ^ v1);
    v1 = (v2 & v0);
    v0 = (v0 ^ v1);
    v1 = (v3 ^ v0);
    v2 = output[0];
    output[0] = v1;
    v0 = (v0 & ~v2);
    output[1] = ~v0;
    v0 = output[2];
    output[3] = v0;
    v0 = output[3];
    output[2] = ~v0;
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .arg_inputs(Some(&[0, 2]))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(__m64* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    v0 = ((arg & 2) != 0) ? one : zero;
    v1 = output[1];
    v2 = _m_pand(v0, v1);
    v3 = ((arg & 1) != 0) ? one : zero;
    v3 = _m_por(v3, v1);
    v3 = _m_pand(v2, v3);
    v0 = _m_pxor(v0, v1);
    v1 = _m_pand(v2, v0);
    v0 = _m_pxor(v0, v1);
    v1 = _m_pxor(v3, v0);
    v2 = output[0];
    output[0] = v1;
    v0 = _m_pandn(v2, v0);
    output[1] = _m_pxor(v0, one);
    v0 = output[2];
    output[3] = v0;
    v0 = output[3];
    output[2] = _m_pxor(v0, one);
}
"##
    );
}

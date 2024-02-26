use crate::gencode::generate_code_ext;
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
    let mut fw = cw.func_writer_ext(
        "func1",
        3,
        3,
        if inout_placement {
            if arg_input {
                Some((&[11], 99))
            } else {
                Some((&[6, 11, 44], 99))
            }
        } else {
            None
        },
        if inout_placement {
            Some((&[48, 72, 25], 99))
        } else {
            None
        },
        if arg_input { Some(&[0, 2]) } else { None },
        None,
        true,
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

#[test]
fn test_clang_writer() {
    {
        assert_eq!(
            r##"#include <stdint.h>
void gate_sys_func1(const uint32_t* input,
    uint32_t* output) {
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
        assert_eq!(
            r##"#include <stdint.h>
void gate_sys_func1(const uint32_t* input,
    uint32_t* output) {
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
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2) {
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
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2) {
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
void gate_sys_func1(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2) {
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
void gate_sys_func1(const uint64_t* input,
    uint64_t* output) {
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
void gate_sys_func1(const uint64_t* input,
    uint64_t* output) {
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
void gate_sys_func1(const uint64_t* input,
    uint64_t* output, unsigned int arg, unsigned int arg2) {
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
static const unsigned int zero_value[2] = { 0, 0 };
static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };
static const unsigned int elem_index_low_tbl[6*2] = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xcccccccc, 0xcccccccc, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xffff0000, 0xffff0000, 0x00000000, 0xffffffff
};
void gate_sys_func1(const __m64* input,
    __m64* output) {
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
    assert_eq!(
        r##"#include <xmmintrin.h>
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
void gate_sys_func1(const __m128* input,
    __m128* output) {
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
        r##"#include <immintrin.h>
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
void gate_sys_func1(const __m256* input,
    __m256* output) {
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
void gate_sys_func1(const __m512i* input,
    __m512i* output) {
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
void gate_sys_func1(const uint32x4_t* input,
    uint32x4_t* output) {
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

    // opencl
    assert_eq!(
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 3 * idx + input_shift;
    const unsigned int ovn = 2 * idx + output_shift;
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
    assert_eq!(
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 68 * idx + input_shift;
    const unsigned int ovn = 88 * idx + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 1 * idx + input_shift;
    const unsigned int ovn = 2 * idx + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 68 * idx + input_shift;
    const unsigned int ovn = 88 * idx + output_shift;
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
void gate_sys_func1(uint32_t* output) {
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
void gate_sys_func1(uint32_t* output) {
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
void gate_sys_func1(uint32_t* output, unsigned int arg, unsigned int arg2) {
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int output_shift,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 3 * idx + output_shift;
    const unsigned int ovn = 3 * idx + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int output_shift,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 99 * idx + output_shift;
    const unsigned int ovn = 99 * idx + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int output_shift,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 99 * idx + output_shift;
    const unsigned int ovn = 99 * idx + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output) {
    const uint idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const unsigned int ivn = llen * (3 * idx) + input_shift;
    const unsigned int ovn = llen * (2 * idx) + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const unsigned int ivn = llen * (1 * idx) + input_shift;
    const unsigned int ovn = llen * (2 * idx) + output_shift;
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
        r##"kernel void gate_sys_func1(unsigned int n, 
    unsigned int output_shift,
    global uint* output) {
    const uint idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const unsigned int ivn = llen * (3 * idx) + output_shift;
    const unsigned int ovn = llen * (3 * idx) + output_shift;
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
    generate_code_ext(
        &mut writer,
        "xor",
        circuit,
        false,
        None,
        None,
        Some(&(120 - 64..120).collect::<Vec<_>>()),
        None,
        false,
    );
    let out = String::from_utf8(writer.out()).unwrap();
    assert_eq!(
        &out,
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 56 * idx + input_shift;
    const unsigned int ovn = 60 * idx + output_shift;
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
fn test_clang_writer_elem_index() {
    let circuit = Circuit::new(
        30,
        (0..15).map(|i| Gate::new_xor(i, 15 + i)),
        (0..15).map(|i| (30 + i, false)),
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int task_id) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    uint32_t v0;
    uint32_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[9];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[10];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[12];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[13];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((task_id & 128) != 0) ? one : zero;
    v1 = input[15];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_U64.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint64_t* input,
    uint64_t* output, unsigned int task_id) {
    const uint64_t zero = 0ULL;
    const uint64_t one = 0xffffffffffffffffULL;
    const uint64_t elem_low_bit0 = 0xaaaaaaaaaaaaaaaaULL;
    const uint64_t elem_low_bit1 = 0xccccccccccccccccULL;
    const uint64_t elem_low_bit2 = 0xf0f0f0f0f0f0f0f0ULL;
    const uint64_t elem_low_bit3 = 0xff00ff00ff00ff00ULL;
    const uint64_t elem_low_bit4 = 0xffff0000ffff0000ULL;
    const uint64_t elem_low_bit5 = 0xffffffff00000000ULL;
    uint64_t v0;
    uint64_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[9];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[10];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[12];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[13];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = input[15];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m64* input,
    __m64* output, unsigned int task_id) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    const __m64 elem_low_bit0 = *((const __m64*)elem_index_low_tbl);
    const __m64 elem_low_bit1 = *((const __m64*)(elem_index_low_tbl + 2));
    const __m64 elem_low_bit2 = *((const __m64*)(elem_index_low_tbl + 4));
    const __m64 elem_low_bit3 = *((const __m64*)(elem_index_low_tbl + 6));
    const __m64 elem_low_bit4 = *((const __m64*)(elem_index_low_tbl + 8));
    const __m64 elem_low_bit5 = *((const __m64*)(elem_index_low_tbl + 10));
    __m64 v0;
    __m64 v1;
    v0 = input[0];
    v1 = input[2];
    v0 = _m_pxor(v0, v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = _m_pxor(v0, v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = _m_pxor(v0, v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = _m_pxor(v0, v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = _m_pxor(v0, v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = _m_pxor(v0, v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = _m_pxor(v0, v1);
    output[6] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[9];
    v0 = _m_pxor(v0, v1);
    output[7] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[10];
    v0 = _m_pxor(v0, v1);
    output[8] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[11];
    v0 = _m_pxor(v0, v1);
    output[9] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[12];
    v0 = _m_pxor(v0, v1);
    output[10] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[13];
    v0 = _m_pxor(v0, v1);
    output[11] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[14];
    v0 = _m_pxor(v0, v1);
    output[12] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = input[15];
    v0 = _m_pxor(v0, v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = _m_pxor(v0, v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output, unsigned int task_id) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    __m128 v0;
    __m128 v1;
    v0 = input[0];
    v1 = input[2];
    v0 = _mm_xor_ps(v0, v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = _mm_xor_ps(v0, v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = _mm_xor_ps(v0, v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = _mm_xor_ps(v0, v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = _mm_xor_ps(v0, v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = _mm_xor_ps(v0, v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = _mm_xor_ps(v0, v1);
    output[6] = v0;
    v0 = elem_low_bit6;
    v1 = input[9];
    v0 = _mm_xor_ps(v0, v1);
    output[7] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[10];
    v0 = _mm_xor_ps(v0, v1);
    output[8] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[11];
    v0 = _mm_xor_ps(v0, v1);
    output[9] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[12];
    v0 = _mm_xor_ps(v0, v1);
    output[10] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[13];
    v0 = _mm_xor_ps(v0, v1);
    output[11] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[14];
    v0 = _mm_xor_ps(v0, v1);
    output[12] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[15];
    v0 = _mm_xor_ps(v0, v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = _mm_xor_ps(v0, v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_AVX.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m256* input,
    __m256* output, unsigned int task_id) {
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
    __m256 v0;
    __m256 v1;
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[2]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[0], v0);
    v0 = elem_low_bit0;
    v1 = _mm256_loadu_ps((const float*)&input[3]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[1], v0);
    v0 = elem_low_bit1;
    v1 = _mm256_loadu_ps((const float*)&input[4]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[2], v0);
    v0 = elem_low_bit2;
    v1 = _mm256_loadu_ps((const float*)&input[5]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[3], v0);
    v0 = elem_low_bit3;
    v1 = _mm256_loadu_ps((const float*)&input[6]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[4], v0);
    v0 = elem_low_bit4;
    v1 = _mm256_loadu_ps((const float*)&input[7]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[5], v0);
    v0 = elem_low_bit5;
    v1 = _mm256_loadu_ps((const float*)&input[8]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[6], v0);
    v0 = elem_low_bit6;
    v1 = _mm256_loadu_ps((const float*)&input[9]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[7], v0);
    v0 = elem_low_bit7;
    v1 = _mm256_loadu_ps((const float*)&input[10]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[8], v0);
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[11]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[9], v0);
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[12]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[10], v0);
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[13]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[11], v0);
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[14]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[12], v0);
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[15]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[13], v0);
    v0 = _mm256_loadu_ps((const float*)&input[1]);
    v1 = _mm256_loadu_ps((const float*)&input[16]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[14], v0);
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX512.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m512i* input,
    __m512i* output, unsigned int task_id) {
    const __m512i zero = *((const __m512i*)zero_value);
    const __m512i one = *((const __m512i*)one_value);
    const __m512i elem_low_bit0 = *((const __m512i*)elem_index_low_tbl);
    const __m512i elem_low_bit1 = *((const __m512i*)(elem_index_low_tbl + 16));
    const __m512i elem_low_bit2 = *((const __m512i*)(elem_index_low_tbl + 32));
    const __m512i elem_low_bit3 = *((const __m512i*)(elem_index_low_tbl + 48));
    const __m512i elem_low_bit4 = *((const __m512i*)(elem_index_low_tbl + 64));
    const __m512i elem_low_bit5 = *((const __m512i*)(elem_index_low_tbl + 80));
    const __m512i elem_low_bit6 = *((const __m512i*)(elem_index_low_tbl + 96));
    const __m512i elem_low_bit7 = *((const __m512i*)(elem_index_low_tbl + 112));
    const __m512i elem_low_bit8 = *((const __m512i*)(elem_index_low_tbl + 128));
    __m512i v0;
    __m512i v1;
    v0 = _mm512_loadu_epi64(&input[0]);
    v1 = _mm512_loadu_epi64(&input[2]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[0], v0);
    v0 = elem_low_bit0;
    v1 = _mm512_loadu_epi64(&input[3]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[1], v0);
    v0 = elem_low_bit1;
    v1 = _mm512_loadu_epi64(&input[4]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[2], v0);
    v0 = elem_low_bit2;
    v1 = _mm512_loadu_epi64(&input[5]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[3], v0);
    v0 = elem_low_bit3;
    v1 = _mm512_loadu_epi64(&input[6]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[4], v0);
    v0 = elem_low_bit4;
    v1 = _mm512_loadu_epi64(&input[7]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[5], v0);
    v0 = elem_low_bit5;
    v1 = _mm512_loadu_epi64(&input[8]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[6], v0);
    v0 = elem_low_bit6;
    v1 = _mm512_loadu_epi64(&input[9]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[7], v0);
    v0 = elem_low_bit7;
    v1 = _mm512_loadu_epi64(&input[10]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[8], v0);
    v0 = elem_low_bit8;
    v1 = _mm512_loadu_epi64(&input[11]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[9], v0);
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[12]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[10], v0);
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[13]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[11], v0);
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[14]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[12], v0);
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[15]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[13], v0);
    v0 = _mm512_loadu_epi64(&input[1]);
    v1 = _mm512_loadu_epi64(&input[16]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[14], v0);
}
"##
    );

    let mut writer = CLANG_WRITER_ARM_NEON.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32x4_t* input,
    uint32x4_t* output, unsigned int task_id) {
    const uint32x4_t zero = { 0, 0, 0, 0 };
    const uint32x4_t one = { 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
    const uint32x4_t elem_low_bit0 = { 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa };
    const uint32x4_t elem_low_bit1 = { 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc };
    const uint32x4_t elem_low_bit2 = { 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0 };
    const uint32x4_t elem_low_bit3 = { 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00 };
    const uint32x4_t elem_low_bit4 = { 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000 };
    const uint32x4_t elem_low_bit5 = { 0x00000000, 0xffffffff, 0x00000000, 0xffffffff };
    const uint32x4_t elem_low_bit6 = { 0x00000000, 0x00000000, 0xffffffff, 0xffffffff };
    uint32x4_t v0;
    uint32x4_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = veorq_u32(v0, v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = veorq_u32(v0, v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = veorq_u32(v0, v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = veorq_u32(v0, v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = veorq_u32(v0, v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = veorq_u32(v0, v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = veorq_u32(v0, v1);
    output[6] = v0;
    v0 = elem_low_bit6;
    v1 = input[9];
    v0 = veorq_u32(v0, v1);
    output[7] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[10];
    v0 = veorq_u32(v0, v1);
    output[8] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[11];
    v0 = veorq_u32(v0, v1);
    output[9] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[12];
    v0 = veorq_u32(v0, v1);
    output[10] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[13];
    v0 = veorq_u32(v0, v1);
    output[11] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[14];
    v0 = veorq_u32(v0, v1);
    output[12] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[15];
    v0 = veorq_u32(v0, v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = veorq_u32(v0, v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 17 * idx + input_shift;
    const unsigned int ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = input[ivn + 2];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 3];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = input[ivn + 4];
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = input[ivn + 5];
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = input[ivn + 6];
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = input[ivn + 7];
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idx & 1) != 0) ? one : zero;
    v1 = input[ivn + 8];
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idx & 2) != 0) ? one : zero;
    v1 = input[ivn + 9];
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idx & 4) != 0) ? one : zero;
    v1 = input[ivn + 10];
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idx & 8) != 0) ? one : zero;
    v1 = input[ivn + 11];
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idx & 16) != 0) ? one : zero;
    v1 = input[ivn + 12];
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idx & 32) != 0) ? one : zero;
    v1 = input[ivn + 13];
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idx & 64) != 0) ? one : zero;
    v1 = input[ivn + 14];
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idx & 128) != 0) ? one : zero;
    v1 = input[ivn + 15];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 1];
    v1 = input[ivn + 16];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output) {
    const uint idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const unsigned int ivn = llen * (17 * idx) + input_shift;
    const unsigned int ovn = llen * (15 * idx) + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*2 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*0 + lidx] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + llen*3 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*1 + lidx] = v0;
    v0 = elem_low_bit1;
    v1 = input[ivn + llen*4 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*2 + lidx] = v0;
    v0 = elem_low_bit2;
    v1 = input[ivn + llen*5 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*3 + lidx] = v0;
    v0 = elem_low_bit3;
    v1 = input[ivn + llen*6 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*4 + lidx] = v0;
    v0 = elem_low_bit4;
    v1 = input[ivn + llen*7 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*5 + lidx] = v0;
    v0 = ((idx & 1) != 0) ? one : zero;
    v1 = input[ivn + llen*8 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*6 + lidx] = v0;
    v0 = ((idx & 2) != 0) ? one : zero;
    v1 = input[ivn + llen*9 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*7 + lidx] = v0;
    v0 = ((idx & 4) != 0) ? one : zero;
    v1 = input[ivn + llen*10 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*8 + lidx] = v0;
    v0 = ((idx & 8) != 0) ? one : zero;
    v1 = input[ivn + llen*11 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*9 + lidx] = v0;
    v0 = ((idx & 16) != 0) ? one : zero;
    v1 = input[ivn + llen*12 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*10 + lidx] = v0;
    v0 = ((idx & 32) != 0) ? one : zero;
    v1 = input[ivn + llen*13 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*11 + lidx] = v0;
    v0 = ((idx & 64) != 0) ? one : zero;
    v1 = input[ivn + llen*14 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*12 + lidx] = v0;
    v0 = ((idx & 128) != 0) ? one : zero;
    v1 = input[ivn + llen*15 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*13 + lidx] = v0;
    v0 = input[ivn + llen*1 + lidx];
    v1 = input[ivn + llen*16 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*14 + lidx] = v0;
}
"##
    );

    // with input_placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        Some((&(0..30 - 13).map(|i| 2 + 3 * i).collect::<Vec<_>>(), 100)),
        None,
        None,
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int task_id) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    uint32_t v0;
    uint32_t v1;
    v0 = input[2];
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[17];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[20];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[23];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = input[26];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = input[29];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = input[32];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = input[35];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = input[38];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = input[41];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = input[44];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((task_id & 128) != 0) ? one : zero;
    v1 = input[47];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[5];
    v1 = input[50];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    // with arg_input
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        Some(&(17..28).collect::<Vec<_>>()),
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, unsigned int task_id) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    uint32_t v0;
    uint32_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((task_id & 128) != 0) ? one : zero;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        Some(&(17..28).collect::<Vec<_>>()),
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 6 * idx + input_shift;
    const unsigned int ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = input[ivn + 2];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 3];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idx & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idx & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idx & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idx & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idx & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idx & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idx & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idx & 128) != 0) ? one : zero;
    v1 = input[ivn + 4];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 1];
    v1 = input[ivn + 5];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        None,
        None,
        Some(&(17..28).collect::<Vec<_>>()),
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const unsigned int ivn = llen * (6 * idx) + input_shift;
    const unsigned int ovn = llen * (15 * idx) + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*2 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*0 + lidx] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + llen*3 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*1 + lidx] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*2 + lidx] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*3 + lidx] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*4 + lidx] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*5 + lidx] = v0;
    v0 = ((idx & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*6 + lidx] = v0;
    v0 = ((idx & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*7 + lidx] = v0;
    v0 = ((idx & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*8 + lidx] = v0;
    v0 = ((idx & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*9 + lidx] = v0;
    v0 = ((idx & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*10 + lidx] = v0;
    v0 = ((idx & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*11 + lidx] = v0;
    v0 = ((idx & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*12 + lidx] = v0;
    v0 = ((idx & 128) != 0) ? one : zero;
    v1 = input[ivn + llen*4 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*13 + lidx] = v0;
    v0 = input[ivn + llen*1 + lidx];
    v1 = input[ivn + llen*5 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*14 + lidx] = v0;
}
"##
    );

    // with arg_input and input_placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        Some((
            &(0..30 - 13 - 11).map(|i| 9 + 11 * i).collect::<Vec<_>>(),
            100,
        )),
        None,
        Some(&(17..28).collect::<Vec<_>>()),
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, unsigned int task_id) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    uint32_t v0;
    uint32_t v1;
    v0 = input[9];
    v1 = input[31];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[42];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((task_id & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((task_id & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((task_id & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((task_id & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((task_id & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((task_id & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((task_id & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((task_id & 128) != 0) ? one : zero;
    v1 = input[53];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[20];
    v1 = input[64];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        Some((
            &(0..30 - 13 - 11).map(|i| 9 + 11 * i).collect::<Vec<_>>(),
            100,
        )),
        None,
        Some(&(17..28).collect::<Vec<_>>()),
        Some(&(1..14).collect::<Vec<_>>()),
        false,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned int n, 
    unsigned int input_shift, unsigned int output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 100 * idx + input_shift;
    const unsigned int ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 9];
    v1 = input[ivn + 31];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 42];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idx & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idx & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idx & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idx & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idx & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idx & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idx & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idx & 128) != 0) ? one : zero;
    v1 = input[ivn + 53];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 20];
    v1 = input[ivn + 64];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
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
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        Some((&[1, 0], 4)),
        Some((&[3, 2, 1, 0], 4)),
        Some(&[0, 2]),
        None,
        true,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(uint32_t* output, unsigned int arg, unsigned int arg2) {
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
    generate_code_ext(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        Some((&[1, 0], 4)),
        Some((&[3, 2, 1, 0], 4)),
        Some(&[0, 2]),
        None,
        true,
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(__m64* output, unsigned int arg, unsigned int arg2) {
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
}

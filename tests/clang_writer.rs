use gatenative::clang_writer::*;
use gatenative::*;

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
    uint32_t* output, unsigned int arg) {
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
    uint32_t* output, unsigned int arg) {
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
    uint32_t* output, unsigned int arg) {
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
    uint64_t* output, unsigned int arg) {
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
    assert_eq!(
        r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 3 * idx;
    const unsigned int ovn = 2 * idx;
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
        r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 68 * idx;
    const unsigned int ovn = 88 * idx;
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
        r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output, unsigned int arg) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 1 * idx;
    const unsigned int ovn = 2 * idx;
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
        r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output, unsigned int arg) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 68 * idx;
    const unsigned int ovn = 88 * idx;
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
}

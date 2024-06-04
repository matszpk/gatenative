use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_array_len() {
    let circuit = Circuit::new(
        3,
        [
            Gate::new_nimpl(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_nimpl(1, 0),
            Gate::new_nor(5, 6),
        ],
        [(4, false), (7, true)],
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U64.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "blab",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"#include <stdint.h>
#include <stddef.h>
typedef struct _gate_sys_type {
    uint64_t array[4];
} gate_sys_type;
#define TYPE_LEN (256)
#define TYPE_NAME gate_sys_type
#define __INT_GET_U32(D,X,I) { (D) = ((X) >> ((I)<<5)); }
#define __INT_GET_U32_ALL(D,X) { (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }
#define __INT_SET_U32(X,S,I) { uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }
#define __INT_SET_U32_ALL(X,S) { (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }
#define GET_U32(D,X,I) \
    __INT_GET_U32((D).array[(I) / 2], (X).array[(I) / 2], (I) % 2)
#define GET_ALL_U32(D,X) { \
    __INT_GET_U32_ALL((D).array[0], (X).array[0]); \
    __INT_GET_U32_ALL((D).array[1], (X).array[1]); \
    __INT_GET_U32_ALL((D).array[2], (X).array[2]); \
    __INT_GET_U32_ALL((D).array[3], (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 2], (S).array[(I) / 2], (I) % 2)
#define SET_ALL_U32(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (S).array[0]); \
    __INT_SET_U32_ALL((X).array[1], (S).array[1]); \
    __INT_SET_U32_ALL((X).array[2], (S).array[2]); \
    __INT_SET_U32_ALL((X).array[3], (S).array[3]); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, size_t idx) {
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    gate_sys_type v3;
    gate_sys_type v4;
    v0.array[0] = (input[0].array[0]);
    v0.array[1] = (input[0].array[1]);
    v0.array[2] = (input[0].array[2]);
    v0.array[3] = (input[0].array[3]);
    v1.array[0] = (input[1].array[0]);
    v1.array[1] = (input[1].array[1]);
    v1.array[2] = (input[1].array[2]);
    v1.array[3] = (input[1].array[3]);
    v2.array[0] = (v0.array[0] & ~v1.array[0]);
    v2.array[1] = (v0.array[1] & ~v1.array[1]);
    v2.array[2] = (v0.array[2] & ~v1.array[2]);
    v2.array[3] = (v0.array[3] & ~v1.array[3]);
    v3.array[0] = (input[2].array[0]);
    v3.array[1] = (input[2].array[1]);
    v3.array[2] = (input[2].array[2]);
    v3.array[3] = (input[2].array[3]);
    v4.array[0] = (v3.array[0] ^ v2.array[0]);
    v4.array[1] = (v3.array[1] ^ v2.array[1]);
    v4.array[2] = (v3.array[2] ^ v2.array[2]);
    v4.array[3] = (v3.array[3] ^ v2.array[3]);
    (output[0].array[0]) = v4.array[0];
    (output[0].array[1]) = v4.array[1];
    (output[0].array[2]) = v4.array[2];
    (output[0].array[3]) = v4.array[3];
    v2.array[0] = (v3.array[0] & v2.array[0]);
    v2.array[1] = (v3.array[1] & v2.array[1]);
    v2.array[2] = (v3.array[2] & v2.array[2]);
    v2.array[3] = (v3.array[3] & v2.array[3]);
    v0.array[0] = (v1.array[0] & ~v0.array[0]);
    v0.array[1] = (v1.array[1] & ~v0.array[1]);
    v0.array[2] = (v1.array[2] & ~v0.array[2]);
    v0.array[3] = (v1.array[3] & ~v0.array[3]);
    v0.array[0] = ~(v2.array[0] | v0.array[0]);
    v0.array[1] = ~(v2.array[1] | v0.array[1]);
    v0.array[2] = ~(v2.array[2] | v0.array[2]);
    v0.array[3] = ~(v2.array[3] | v0.array[3]);
    (output[1].array[0]) = ~v0.array[0];
    (output[1].array[1]) = ~v0.array[1];
    (output[1].array[2]) = ~v0.array[2];
    (output[1].array[3]) = ~v0.array[3];
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX2.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "blab",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    writer.epilog();
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
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
typedef struct _gate_sys_type {
    __m256i array[4];
} gate_sys_type;
#define TYPE_LEN (1024)
#define TYPE_NAME gate_sys_type
#define __INT_GET_U32(D,X,I) { uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define __INT_GET_U32_ALL(D,X) { _mm256_storeu_si256((__m256i*)(D), (X)); }
#define __INT_SET_U32(X,S,I) { uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_si256((__m256i*)temp); \
}
#define __INT_SET_U32_ALL(X,S) { (X) = _mm256_loadu_si256((__m256i*)(S)); }
#define GET_U32(D,X,I) \
    __INT_GET_U32((D).array[(I) / 8], (X).array[(I) / 8], (I) % 8)
#define GET_ALL_U32(D,X) { \
    __INT_GET_U32_ALL((D).array[0], (X).array[0]); \
    __INT_GET_U32_ALL((D).array[1], (X).array[1]); \
    __INT_GET_U32_ALL((D).array[2], (X).array[2]); \
    __INT_GET_U32_ALL((D).array[3], (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 8], (S).array[(I) / 8], (I) % 8)
#define SET_ALL_U32(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (S).array[0]); \
    __INT_SET_U32_ALL((X).array[1], (S).array[1]); \
    __INT_SET_U32_ALL((X).array[2], (S).array[2]); \
    __INT_SET_U32_ALL((X).array[3], (S).array[3]); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, size_t idx) {
    const __m256i one = *((const __m256i*)one_value);
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    gate_sys_type v3;
    gate_sys_type v4;
    v0.array[0] = _mm256_loadu_si256((const __m256i*)&(input[0].array[0]));
    v0.array[1] = _mm256_loadu_si256((const __m256i*)&(input[0].array[1]));
    v0.array[2] = _mm256_loadu_si256((const __m256i*)&(input[0].array[2]));
    v0.array[3] = _mm256_loadu_si256((const __m256i*)&(input[0].array[3]));
    v1.array[0] = _mm256_loadu_si256((const __m256i*)&(input[1].array[0]));
    v1.array[1] = _mm256_loadu_si256((const __m256i*)&(input[1].array[1]));
    v1.array[2] = _mm256_loadu_si256((const __m256i*)&(input[1].array[2]));
    v1.array[3] = _mm256_loadu_si256((const __m256i*)&(input[1].array[3]));
    v2.array[0] = _mm256_andnot_si256(v1.array[0], v0.array[0]);
    v2.array[1] = _mm256_andnot_si256(v1.array[1], v0.array[1]);
    v2.array[2] = _mm256_andnot_si256(v1.array[2], v0.array[2]);
    v2.array[3] = _mm256_andnot_si256(v1.array[3], v0.array[3]);
    v3.array[0] = _mm256_loadu_si256((const __m256i*)&(input[2].array[0]));
    v3.array[1] = _mm256_loadu_si256((const __m256i*)&(input[2].array[1]));
    v3.array[2] = _mm256_loadu_si256((const __m256i*)&(input[2].array[2]));
    v3.array[3] = _mm256_loadu_si256((const __m256i*)&(input[2].array[3]));
    v4.array[0] = _mm256_xor_si256(v3.array[0], v2.array[0]);
    v4.array[1] = _mm256_xor_si256(v3.array[1], v2.array[1]);
    v4.array[2] = _mm256_xor_si256(v3.array[2], v2.array[2]);
    v4.array[3] = _mm256_xor_si256(v3.array[3], v2.array[3]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[0]), v4.array[0]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[1]), v4.array[1]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[2]), v4.array[2]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[3]), v4.array[3]);
    v2.array[0] = _mm256_and_si256(v3.array[0], v2.array[0]);
    v2.array[1] = _mm256_and_si256(v3.array[1], v2.array[1]);
    v2.array[2] = _mm256_and_si256(v3.array[2], v2.array[2]);
    v2.array[3] = _mm256_and_si256(v3.array[3], v2.array[3]);
    v0.array[0] = _mm256_andnot_si256(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_andnot_si256(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_andnot_si256(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_andnot_si256(v0.array[3], v1.array[3]);
    v0.array[0] = _mm256_or_si256(v2.array[0], v0.array[0]);
    v0.array[1] = _mm256_or_si256(v2.array[1], v0.array[1]);
    v0.array[2] = _mm256_or_si256(v2.array[2], v0.array[2]);
    v0.array[3] = _mm256_or_si256(v2.array[3], v0.array[3]);
    _mm256_storeu_si256((__m256i*)&(output[1].array[0]), v0.array[0]);
    _mm256_storeu_si256((__m256i*)&(output[1].array[1]), v0.array[1]);
    _mm256_storeu_si256((__m256i*)&(output[1].array[2]), v0.array[2]);
    _mm256_storeu_si256((__m256i*)&(output[1].array[3]), v0.array[3]);
}
"##
    );
}

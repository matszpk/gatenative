use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

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
    __INT_GET_U32((D), (X).array[(I) / 2], (I) % 2)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[2])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[4])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[6])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 2], (S), (I) % 2)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[2]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[4]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[6]))); \
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
    __INT_GET_U32((D), (X).array[(I) / 8], (I) % 8)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[8])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[16])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[24])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 8], (S), (I) % 8)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[8]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[16]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[24]))); \
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

    let circuit = Circuit::new(
        8,
        [
            Gate::new_nimpl(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(4, 5),
            Gate::new_nimpl(6, 7),
            Gate::new_xor(8, 9),
            Gate::new_xor(10, 11),
        ],
        [(12, false), (13, true)],
    )
    .unwrap();
    // arg inputs
    let mut writer = CLANG_WRITER_U64.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "blab",
        circuit.clone(),
        false,
        CodeConfig::new().arg_inputs(Some(&[0, 1, 6, 7])),
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
    __INT_GET_U32((D), (X).array[(I) / 2], (I) % 2)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[2])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[4])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[6])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 2], (S), (I) % 2)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[2]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[4]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[6]))); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint64_t zero = 0ULL;
    const uint64_t one = 0xffffffffffffffffULL;
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    v0.array[0] = ((arg & 1) != 0) ? one : zero;
    v0.array[1] = ((arg & 1) != 0) ? one : zero;
    v0.array[2] = ((arg & 1) != 0) ? one : zero;
    v0.array[3] = ((arg & 1) != 0) ? one : zero;
    v1.array[0] = ((arg & 2) != 0) ? one : zero;
    v1.array[1] = ((arg & 2) != 0) ? one : zero;
    v1.array[2] = ((arg & 2) != 0) ? one : zero;
    v1.array[3] = ((arg & 2) != 0) ? one : zero;
    v0.array[0] = (v0.array[0] & ~v1.array[0]);
    v0.array[1] = (v0.array[1] & ~v1.array[1]);
    v0.array[2] = (v0.array[2] & ~v1.array[2]);
    v0.array[3] = (v0.array[3] & ~v1.array[3]);
    v1.array[0] = (input[0].array[0]);
    v1.array[1] = (input[0].array[1]);
    v1.array[2] = (input[0].array[2]);
    v1.array[3] = (input[0].array[3]);
    v2.array[0] = (input[1].array[0]);
    v2.array[1] = (input[1].array[1]);
    v2.array[2] = (input[1].array[2]);
    v2.array[3] = (input[1].array[3]);
    v1.array[0] = (v1.array[0] ^ v2.array[0]);
    v1.array[1] = (v1.array[1] ^ v2.array[1]);
    v1.array[2] = (v1.array[2] ^ v2.array[2]);
    v1.array[3] = (v1.array[3] ^ v2.array[3]);
    v0.array[0] = (v0.array[0] ^ v1.array[0]);
    v0.array[1] = (v0.array[1] ^ v1.array[1]);
    v0.array[2] = (v0.array[2] ^ v1.array[2]);
    v0.array[3] = (v0.array[3] ^ v1.array[3]);
    (output[0].array[0]) = v0.array[0];
    (output[0].array[1]) = v0.array[1];
    (output[0].array[2]) = v0.array[2];
    (output[0].array[3]) = v0.array[3];
    v0.array[0] = (input[2].array[0]);
    v0.array[1] = (input[2].array[1]);
    v0.array[2] = (input[2].array[2]);
    v0.array[3] = (input[2].array[3]);
    v1.array[0] = (input[3].array[0]);
    v1.array[1] = (input[3].array[1]);
    v1.array[2] = (input[3].array[2]);
    v1.array[3] = (input[3].array[3]);
    v0.array[0] = (v0.array[0] & v1.array[0]);
    v0.array[1] = (v0.array[1] & v1.array[1]);
    v0.array[2] = (v0.array[2] & v1.array[2]);
    v0.array[3] = (v0.array[3] & v1.array[3]);
    v1.array[0] = ((arg & 4) != 0) ? one : zero;
    v1.array[1] = ((arg & 4) != 0) ? one : zero;
    v1.array[2] = ((arg & 4) != 0) ? one : zero;
    v1.array[3] = ((arg & 4) != 0) ? one : zero;
    v2.array[0] = ((arg & 8) != 0) ? one : zero;
    v2.array[1] = ((arg & 8) != 0) ? one : zero;
    v2.array[2] = ((arg & 8) != 0) ? one : zero;
    v2.array[3] = ((arg & 8) != 0) ? one : zero;
    v1.array[0] = (v1.array[0] & ~v2.array[0]);
    v1.array[1] = (v1.array[1] & ~v2.array[1]);
    v1.array[2] = (v1.array[2] & ~v2.array[2]);
    v1.array[3] = (v1.array[3] & ~v2.array[3]);
    v0.array[0] = (v0.array[0] ^ v1.array[0]);
    v0.array[1] = (v0.array[1] ^ v1.array[1]);
    v0.array[2] = (v0.array[2] ^ v1.array[2]);
    v0.array[3] = (v0.array[3] ^ v1.array[3]);
    (output[1].array[0]) = ~v0.array[0];
    (output[1].array[1]) = ~v0.array[1];
    (output[1].array[2]) = ~v0.array[2];
    (output[1].array[3]) = ~v0.array[3];
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "blab",
        circuit.clone(),
        false,
        CodeConfig::new().arg_inputs(Some(&[0, 1, 6, 7])),
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
    __m256 array[4];
} gate_sys_type;
#define TYPE_LEN (1024)
#define TYPE_NAME gate_sys_type
#define __INT_GET_U32(D,X,I) { uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define __INT_GET_U32_ALL(D,X) { _mm256_storeu_ps((float*)(D), (X)); }
#define __INT_SET_U32(X,S,I) { uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_ps((float*)temp); \
}
#define __INT_SET_U32_ALL(X,S) { (X) = _mm256_loadu_ps((float*)(S)); }
#define GET_U32(D,X,I) \
    __INT_GET_U32((D), (X).array[(I) / 8], (I) % 8)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[8])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[16])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[24])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 8], (S), (I) % 8)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[8]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[16]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[24]))); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const __m256 zero = *((const __m256*)zero_value);
    const __m256 one = *((const __m256*)one_value);
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    v0.array[0] = ((arg & 1) != 0) ? one : zero;
    v0.array[1] = ((arg & 1) != 0) ? one : zero;
    v0.array[2] = ((arg & 1) != 0) ? one : zero;
    v0.array[3] = ((arg & 1) != 0) ? one : zero;
    v1.array[0] = ((arg & 2) != 0) ? one : zero;
    v1.array[1] = ((arg & 2) != 0) ? one : zero;
    v1.array[2] = ((arg & 2) != 0) ? one : zero;
    v1.array[3] = ((arg & 2) != 0) ? one : zero;
    v0.array[0] = _mm256_andnot_ps(v1.array[0], v0.array[0]);
    v0.array[1] = _mm256_andnot_ps(v1.array[1], v0.array[1]);
    v0.array[2] = _mm256_andnot_ps(v1.array[2], v0.array[2]);
    v0.array[3] = _mm256_andnot_ps(v1.array[3], v0.array[3]);
    v1.array[0] = _mm256_loadu_ps((const float*)&(input[0].array[0]));
    v1.array[1] = _mm256_loadu_ps((const float*)&(input[0].array[1]));
    v1.array[2] = _mm256_loadu_ps((const float*)&(input[0].array[2]));
    v1.array[3] = _mm256_loadu_ps((const float*)&(input[0].array[3]));
    v2.array[0] = _mm256_loadu_ps((const float*)&(input[1].array[0]));
    v2.array[1] = _mm256_loadu_ps((const float*)&(input[1].array[1]));
    v2.array[2] = _mm256_loadu_ps((const float*)&(input[1].array[2]));
    v2.array[3] = _mm256_loadu_ps((const float*)&(input[1].array[3]));
    v1.array[0] = _mm256_xor_ps(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_xor_ps(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_xor_ps(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_xor_ps(v1.array[3], v2.array[3]);
    v0.array[0] = _mm256_xor_ps(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_xor_ps(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_xor_ps(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_xor_ps(v0.array[3], v1.array[3]);
    _mm256_storeu_ps((float*)&(output[0].array[0]), v0.array[0]);
    _mm256_storeu_ps((float*)&(output[0].array[1]), v0.array[1]);
    _mm256_storeu_ps((float*)&(output[0].array[2]), v0.array[2]);
    _mm256_storeu_ps((float*)&(output[0].array[3]), v0.array[3]);
    v0.array[0] = _mm256_loadu_ps((const float*)&(input[2].array[0]));
    v0.array[1] = _mm256_loadu_ps((const float*)&(input[2].array[1]));
    v0.array[2] = _mm256_loadu_ps((const float*)&(input[2].array[2]));
    v0.array[3] = _mm256_loadu_ps((const float*)&(input[2].array[3]));
    v1.array[0] = _mm256_loadu_ps((const float*)&(input[3].array[0]));
    v1.array[1] = _mm256_loadu_ps((const float*)&(input[3].array[1]));
    v1.array[2] = _mm256_loadu_ps((const float*)&(input[3].array[2]));
    v1.array[3] = _mm256_loadu_ps((const float*)&(input[3].array[3]));
    v0.array[0] = _mm256_and_ps(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_and_ps(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_and_ps(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_and_ps(v0.array[3], v1.array[3]);
    v1.array[0] = ((arg & 4) != 0) ? one : zero;
    v1.array[1] = ((arg & 4) != 0) ? one : zero;
    v1.array[2] = ((arg & 4) != 0) ? one : zero;
    v1.array[3] = ((arg & 4) != 0) ? one : zero;
    v2.array[0] = ((arg & 8) != 0) ? one : zero;
    v2.array[1] = ((arg & 8) != 0) ? one : zero;
    v2.array[2] = ((arg & 8) != 0) ? one : zero;
    v2.array[3] = ((arg & 8) != 0) ? one : zero;
    v1.array[0] = _mm256_andnot_ps(v2.array[0], v1.array[0]);
    v1.array[1] = _mm256_andnot_ps(v2.array[1], v1.array[1]);
    v1.array[2] = _mm256_andnot_ps(v2.array[2], v1.array[2]);
    v1.array[3] = _mm256_andnot_ps(v2.array[3], v1.array[3]);
    v0.array[0] = _mm256_xor_ps(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_xor_ps(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_xor_ps(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_xor_ps(v0.array[3], v1.array[3]);
    _mm256_storeu_ps((float*)&(output[1].array[0]), _mm256_xor_ps(v0.array[0], one));
    _mm256_storeu_ps((float*)&(output[1].array[1]), _mm256_xor_ps(v0.array[1], one));
    _mm256_storeu_ps((float*)&(output[1].array[2]), _mm256_xor_ps(v0.array[2], one));
    _mm256_storeu_ps((float*)&(output[1].array[3]), _mm256_xor_ps(v0.array[3], one));
}
"##
    );

    let circuit = Circuit::new(
        16,
        [
            Gate::new_nimpl(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(4, 5),
            Gate::new_nimpl(6, 7),
            Gate::new_nimpl(8, 9),
            Gate::new_xor(10, 11),
            Gate::new_and(12, 13),
            Gate::new_nimpl(14, 15),
            Gate::new_xor(16, 17),
            Gate::new_xor(18, 19),
            Gate::new_xor(20, 21),
            Gate::new_xor(22, 23),
            Gate::new_nimpl(24, 25),
            Gate::new_nimpl(26, 27),
        ],
        [(28, false), (29, true)],
    )
    .unwrap();
    // elem inputs
    let mut writer = CLANG_WRITER_U64.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "blab",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&[0, 2, 4, 6, 8, 9, 10, 11, 12, 13, 14, 15])),
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
    __INT_GET_U32((D), (X).array[(I) / 2], (I) % 2)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[2])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[4])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[6])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 2], (S), (I) % 2)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[2]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[4]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[6]))); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, size_t idx) {
    const uint64_t zero = 0ULL;
    const uint64_t one = 0xffffffffffffffffULL;
    const uint64_t elem_low_bit0 = 0xaaaaaaaaaaaaaaaaULL;
    const uint64_t elem_low_bit1 = 0xccccccccccccccccULL;
    const uint64_t elem_low_bit2 = 0xf0f0f0f0f0f0f0f0ULL;
    const uint64_t elem_low_bit3 = 0xff00ff00ff00ff00ULL;
    const uint64_t elem_low_bit4 = 0xffff0000ffff0000ULL;
    const uint64_t elem_low_bit5 = 0xffffffff00000000ULL;
    const unsigned int idxl0 = (idx * 4 + 0) & 0xffffffff;
    const unsigned int idxh0 = (idx * 4 + 0) >> 32;
    const unsigned int idxl1 = (idx * 4 + 1) & 0xffffffff;
    const unsigned int idxh1 = (idx * 4 + 1) >> 32;
    const unsigned int idxl2 = (idx * 4 + 2) & 0xffffffff;
    const unsigned int idxh2 = (idx * 4 + 2) >> 32;
    const unsigned int idxl3 = (idx * 4 + 3) & 0xffffffff;
    const unsigned int idxh3 = (idx * 4 + 3) >> 32;
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    gate_sys_type v3;
    v0.array[0] = elem_low_bit0;
    v0.array[1] = elem_low_bit0;
    v0.array[2] = elem_low_bit0;
    v0.array[3] = elem_low_bit0;
    v1.array[0] = (input[0].array[0]);
    v1.array[1] = (input[0].array[1]);
    v1.array[2] = (input[0].array[2]);
    v1.array[3] = (input[0].array[3]);
    v0.array[0] = (v0.array[0] & ~v1.array[0]);
    v0.array[1] = (v0.array[1] & ~v1.array[1]);
    v0.array[2] = (v0.array[2] & ~v1.array[2]);
    v0.array[3] = (v0.array[3] & ~v1.array[3]);
    v1.array[0] = elem_low_bit1;
    v1.array[1] = elem_low_bit1;
    v1.array[2] = elem_low_bit1;
    v1.array[3] = elem_low_bit1;
    v2.array[0] = (input[1].array[0]);
    v2.array[1] = (input[1].array[1]);
    v2.array[2] = (input[1].array[2]);
    v2.array[3] = (input[1].array[3]);
    v1.array[0] = (v1.array[0] ^ v2.array[0]);
    v1.array[1] = (v1.array[1] ^ v2.array[1]);
    v1.array[2] = (v1.array[2] ^ v2.array[2]);
    v1.array[3] = (v1.array[3] ^ v2.array[3]);
    v0.array[0] = (v0.array[0] ^ v1.array[0]);
    v0.array[1] = (v0.array[1] ^ v1.array[1]);
    v0.array[2] = (v0.array[2] ^ v1.array[2]);
    v0.array[3] = (v0.array[3] ^ v1.array[3]);
    v1.array[0] = elem_low_bit2;
    v1.array[1] = elem_low_bit2;
    v1.array[2] = elem_low_bit2;
    v1.array[3] = elem_low_bit2;
    v2.array[0] = (input[2].array[0]);
    v2.array[1] = (input[2].array[1]);
    v2.array[2] = (input[2].array[2]);
    v2.array[3] = (input[2].array[3]);
    v1.array[0] = (v1.array[0] & v2.array[0]);
    v1.array[1] = (v1.array[1] & v2.array[1]);
    v1.array[2] = (v1.array[2] & v2.array[2]);
    v1.array[3] = (v1.array[3] & v2.array[3]);
    v2.array[0] = elem_low_bit3;
    v2.array[1] = elem_low_bit3;
    v2.array[2] = elem_low_bit3;
    v2.array[3] = elem_low_bit3;
    v3.array[0] = (input[3].array[0]);
    v3.array[1] = (input[3].array[1]);
    v3.array[2] = (input[3].array[2]);
    v3.array[3] = (input[3].array[3]);
    v2.array[0] = (v2.array[0] & ~v3.array[0]);
    v2.array[1] = (v2.array[1] & ~v3.array[1]);
    v2.array[2] = (v2.array[2] & ~v3.array[2]);
    v2.array[3] = (v2.array[3] & ~v3.array[3]);
    v1.array[0] = (v1.array[0] ^ v2.array[0]);
    v1.array[1] = (v1.array[1] ^ v2.array[1]);
    v1.array[2] = (v1.array[2] ^ v2.array[2]);
    v1.array[3] = (v1.array[3] ^ v2.array[3]);
    v0.array[0] = (v0.array[0] & ~v1.array[0]);
    v0.array[1] = (v0.array[1] & ~v1.array[1]);
    v0.array[2] = (v0.array[2] & ~v1.array[2]);
    v0.array[3] = (v0.array[3] & ~v1.array[3]);
    (output[0].array[0]) = v0.array[0];
    (output[0].array[1]) = v0.array[1];
    (output[0].array[2]) = v0.array[2];
    (output[0].array[3]) = v0.array[3];
    v0.array[0] = elem_low_bit4;
    v0.array[1] = elem_low_bit4;
    v0.array[2] = elem_low_bit4;
    v0.array[3] = elem_low_bit4;
    v1.array[0] = elem_low_bit5;
    v1.array[1] = elem_low_bit5;
    v1.array[2] = elem_low_bit5;
    v1.array[3] = elem_low_bit5;
    v0.array[0] = (v0.array[0] & ~v1.array[0]);
    v0.array[1] = (v0.array[1] & ~v1.array[1]);
    v0.array[2] = (v0.array[2] & ~v1.array[2]);
    v0.array[3] = (v0.array[3] & ~v1.array[3]);
    v1.array[0] = ((idxl0 & 1) != 0) ? one : zero;
    v1.array[1] = ((idxl1 & 1) != 0) ? one : zero;
    v1.array[2] = ((idxl2 & 1) != 0) ? one : zero;
    v1.array[3] = ((idxl3 & 1) != 0) ? one : zero;
    v2.array[0] = ((idxl0 & 2) != 0) ? one : zero;
    v2.array[1] = ((idxl1 & 2) != 0) ? one : zero;
    v2.array[2] = ((idxl2 & 2) != 0) ? one : zero;
    v2.array[3] = ((idxl3 & 2) != 0) ? one : zero;
    v1.array[0] = (v1.array[0] ^ v2.array[0]);
    v1.array[1] = (v1.array[1] ^ v2.array[1]);
    v1.array[2] = (v1.array[2] ^ v2.array[2]);
    v1.array[3] = (v1.array[3] ^ v2.array[3]);
    v0.array[0] = (v0.array[0] ^ v1.array[0]);
    v0.array[1] = (v0.array[1] ^ v1.array[1]);
    v0.array[2] = (v0.array[2] ^ v1.array[2]);
    v0.array[3] = (v0.array[3] ^ v1.array[3]);
    v1.array[0] = ((idxl0 & 4) != 0) ? one : zero;
    v1.array[1] = ((idxl1 & 4) != 0) ? one : zero;
    v1.array[2] = ((idxl2 & 4) != 0) ? one : zero;
    v1.array[3] = ((idxl3 & 4) != 0) ? one : zero;
    v2.array[0] = ((idxl0 & 8) != 0) ? one : zero;
    v2.array[1] = ((idxl1 & 8) != 0) ? one : zero;
    v2.array[2] = ((idxl2 & 8) != 0) ? one : zero;
    v2.array[3] = ((idxl3 & 8) != 0) ? one : zero;
    v1.array[0] = (v1.array[0] & v2.array[0]);
    v1.array[1] = (v1.array[1] & v2.array[1]);
    v1.array[2] = (v1.array[2] & v2.array[2]);
    v1.array[3] = (v1.array[3] & v2.array[3]);
    v2.array[0] = ((idxl0 & 16) != 0) ? one : zero;
    v2.array[1] = ((idxl1 & 16) != 0) ? one : zero;
    v2.array[2] = ((idxl2 & 16) != 0) ? one : zero;
    v2.array[3] = ((idxl3 & 16) != 0) ? one : zero;
    v3.array[0] = ((idxl0 & 32) != 0) ? one : zero;
    v3.array[1] = ((idxl1 & 32) != 0) ? one : zero;
    v3.array[2] = ((idxl2 & 32) != 0) ? one : zero;
    v3.array[3] = ((idxl3 & 32) != 0) ? one : zero;
    v2.array[0] = (v2.array[0] & ~v3.array[0]);
    v2.array[1] = (v2.array[1] & ~v3.array[1]);
    v2.array[2] = (v2.array[2] & ~v3.array[2]);
    v2.array[3] = (v2.array[3] & ~v3.array[3]);
    v1.array[0] = (v1.array[0] ^ v2.array[0]);
    v1.array[1] = (v1.array[1] ^ v2.array[1]);
    v1.array[2] = (v1.array[2] ^ v2.array[2]);
    v1.array[3] = (v1.array[3] ^ v2.array[3]);
    v0.array[0] = (v0.array[0] & ~v1.array[0]);
    v0.array[1] = (v0.array[1] & ~v1.array[1]);
    v0.array[2] = (v0.array[2] & ~v1.array[2]);
    v0.array[3] = (v0.array[3] & ~v1.array[3]);
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
        CodeConfig::new().elem_inputs(Some(&[0, 2, 4, 6, 8, 9, 10, 11, 12, 13, 14, 15])),
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
    __INT_GET_U32((D), (X).array[(I) / 8], (I) % 8)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[8])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[16])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[24])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 8], (S), (I) % 8)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[8]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[16]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[24]))); \
}
void gate_sys_blab(const gate_sys_type* input,
    gate_sys_type* output, size_t idx) {
    const __m256i zero = *((const __m256i*)zero_value);
    const __m256i one = *((const __m256i*)one_value);
    const __m256i elem_low_bit0 = *((const __m256i*)elem_index_low_tbl);
    const __m256i elem_low_bit1 = *((const __m256i*)(elem_index_low_tbl + 8));
    const __m256i elem_low_bit2 = *((const __m256i*)(elem_index_low_tbl + 16));
    const __m256i elem_low_bit3 = *((const __m256i*)(elem_index_low_tbl + 24));
    const __m256i elem_low_bit4 = *((const __m256i*)(elem_index_low_tbl + 32));
    const __m256i elem_low_bit5 = *((const __m256i*)(elem_index_low_tbl + 40));
    const __m256i elem_low_bit6 = *((const __m256i*)(elem_index_low_tbl + 48));
    const __m256i elem_low_bit7 = *((const __m256i*)(elem_index_low_tbl + 56));
    const unsigned int idxl0 = (idx * 4 + 0) & 0xffffffff;
    const unsigned int idxh0 = (idx * 4 + 0) >> 32;
    const unsigned int idxl1 = (idx * 4 + 1) & 0xffffffff;
    const unsigned int idxh1 = (idx * 4 + 1) >> 32;
    const unsigned int idxl2 = (idx * 4 + 2) & 0xffffffff;
    const unsigned int idxh2 = (idx * 4 + 2) >> 32;
    const unsigned int idxl3 = (idx * 4 + 3) & 0xffffffff;
    const unsigned int idxh3 = (idx * 4 + 3) >> 32;
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    gate_sys_type v3;
    v0.array[0] = elem_low_bit0;
    v0.array[1] = elem_low_bit0;
    v0.array[2] = elem_low_bit0;
    v0.array[3] = elem_low_bit0;
    v1.array[0] = _mm256_loadu_si256((const __m256i*)&(input[0].array[0]));
    v1.array[1] = _mm256_loadu_si256((const __m256i*)&(input[0].array[1]));
    v1.array[2] = _mm256_loadu_si256((const __m256i*)&(input[0].array[2]));
    v1.array[3] = _mm256_loadu_si256((const __m256i*)&(input[0].array[3]));
    v0.array[0] = _mm256_andnot_si256(v1.array[0], v0.array[0]);
    v0.array[1] = _mm256_andnot_si256(v1.array[1], v0.array[1]);
    v0.array[2] = _mm256_andnot_si256(v1.array[2], v0.array[2]);
    v0.array[3] = _mm256_andnot_si256(v1.array[3], v0.array[3]);
    v1.array[0] = elem_low_bit1;
    v1.array[1] = elem_low_bit1;
    v1.array[2] = elem_low_bit1;
    v1.array[3] = elem_low_bit1;
    v2.array[0] = _mm256_loadu_si256((const __m256i*)&(input[1].array[0]));
    v2.array[1] = _mm256_loadu_si256((const __m256i*)&(input[1].array[1]));
    v2.array[2] = _mm256_loadu_si256((const __m256i*)&(input[1].array[2]));
    v2.array[3] = _mm256_loadu_si256((const __m256i*)&(input[1].array[3]));
    v1.array[0] = _mm256_xor_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_xor_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_xor_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_xor_si256(v1.array[3], v2.array[3]);
    v0.array[0] = _mm256_xor_si256(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_xor_si256(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_xor_si256(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_xor_si256(v0.array[3], v1.array[3]);
    v1.array[0] = elem_low_bit2;
    v1.array[1] = elem_low_bit2;
    v1.array[2] = elem_low_bit2;
    v1.array[3] = elem_low_bit2;
    v2.array[0] = _mm256_loadu_si256((const __m256i*)&(input[2].array[0]));
    v2.array[1] = _mm256_loadu_si256((const __m256i*)&(input[2].array[1]));
    v2.array[2] = _mm256_loadu_si256((const __m256i*)&(input[2].array[2]));
    v2.array[3] = _mm256_loadu_si256((const __m256i*)&(input[2].array[3]));
    v1.array[0] = _mm256_and_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_and_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_and_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_and_si256(v1.array[3], v2.array[3]);
    v2.array[0] = elem_low_bit3;
    v2.array[1] = elem_low_bit3;
    v2.array[2] = elem_low_bit3;
    v2.array[3] = elem_low_bit3;
    v3.array[0] = _mm256_loadu_si256((const __m256i*)&(input[3].array[0]));
    v3.array[1] = _mm256_loadu_si256((const __m256i*)&(input[3].array[1]));
    v3.array[2] = _mm256_loadu_si256((const __m256i*)&(input[3].array[2]));
    v3.array[3] = _mm256_loadu_si256((const __m256i*)&(input[3].array[3]));
    v2.array[0] = _mm256_andnot_si256(v3.array[0], v2.array[0]);
    v2.array[1] = _mm256_andnot_si256(v3.array[1], v2.array[1]);
    v2.array[2] = _mm256_andnot_si256(v3.array[2], v2.array[2]);
    v2.array[3] = _mm256_andnot_si256(v3.array[3], v2.array[3]);
    v1.array[0] = _mm256_xor_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_xor_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_xor_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_xor_si256(v1.array[3], v2.array[3]);
    v0.array[0] = _mm256_andnot_si256(v1.array[0], v0.array[0]);
    v0.array[1] = _mm256_andnot_si256(v1.array[1], v0.array[1]);
    v0.array[2] = _mm256_andnot_si256(v1.array[2], v0.array[2]);
    v0.array[3] = _mm256_andnot_si256(v1.array[3], v0.array[3]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[0]), v0.array[0]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[1]), v0.array[1]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[2]), v0.array[2]);
    _mm256_storeu_si256((__m256i*)&(output[0].array[3]), v0.array[3]);
    v0.array[0] = elem_low_bit4;
    v0.array[1] = elem_low_bit4;
    v0.array[2] = elem_low_bit4;
    v0.array[3] = elem_low_bit4;
    v1.array[0] = elem_low_bit5;
    v1.array[1] = elem_low_bit5;
    v1.array[2] = elem_low_bit5;
    v1.array[3] = elem_low_bit5;
    v0.array[0] = _mm256_andnot_si256(v1.array[0], v0.array[0]);
    v0.array[1] = _mm256_andnot_si256(v1.array[1], v0.array[1]);
    v0.array[2] = _mm256_andnot_si256(v1.array[2], v0.array[2]);
    v0.array[3] = _mm256_andnot_si256(v1.array[3], v0.array[3]);
    v1.array[0] = elem_low_bit6;
    v1.array[1] = elem_low_bit6;
    v1.array[2] = elem_low_bit6;
    v1.array[3] = elem_low_bit6;
    v2.array[0] = elem_low_bit7;
    v2.array[1] = elem_low_bit7;
    v2.array[2] = elem_low_bit7;
    v2.array[3] = elem_low_bit7;
    v1.array[0] = _mm256_xor_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_xor_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_xor_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_xor_si256(v1.array[3], v2.array[3]);
    v0.array[0] = _mm256_xor_si256(v0.array[0], v1.array[0]);
    v0.array[1] = _mm256_xor_si256(v0.array[1], v1.array[1]);
    v0.array[2] = _mm256_xor_si256(v0.array[2], v1.array[2]);
    v0.array[3] = _mm256_xor_si256(v0.array[3], v1.array[3]);
    v1.array[0] = ((idxl0 & 1) != 0) ? one : zero;
    v1.array[1] = ((idxl1 & 1) != 0) ? one : zero;
    v1.array[2] = ((idxl2 & 1) != 0) ? one : zero;
    v1.array[3] = ((idxl3 & 1) != 0) ? one : zero;
    v2.array[0] = ((idxl0 & 2) != 0) ? one : zero;
    v2.array[1] = ((idxl1 & 2) != 0) ? one : zero;
    v2.array[2] = ((idxl2 & 2) != 0) ? one : zero;
    v2.array[3] = ((idxl3 & 2) != 0) ? one : zero;
    v1.array[0] = _mm256_and_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_and_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_and_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_and_si256(v1.array[3], v2.array[3]);
    v2.array[0] = ((idxl0 & 4) != 0) ? one : zero;
    v2.array[1] = ((idxl1 & 4) != 0) ? one : zero;
    v2.array[2] = ((idxl2 & 4) != 0) ? one : zero;
    v2.array[3] = ((idxl3 & 4) != 0) ? one : zero;
    v3.array[0] = ((idxl0 & 8) != 0) ? one : zero;
    v3.array[1] = ((idxl1 & 8) != 0) ? one : zero;
    v3.array[2] = ((idxl2 & 8) != 0) ? one : zero;
    v3.array[3] = ((idxl3 & 8) != 0) ? one : zero;
    v2.array[0] = _mm256_andnot_si256(v3.array[0], v2.array[0]);
    v2.array[1] = _mm256_andnot_si256(v3.array[1], v2.array[1]);
    v2.array[2] = _mm256_andnot_si256(v3.array[2], v2.array[2]);
    v2.array[3] = _mm256_andnot_si256(v3.array[3], v2.array[3]);
    v1.array[0] = _mm256_xor_si256(v1.array[0], v2.array[0]);
    v1.array[1] = _mm256_xor_si256(v1.array[1], v2.array[1]);
    v1.array[2] = _mm256_xor_si256(v1.array[2], v2.array[2]);
    v1.array[3] = _mm256_xor_si256(v1.array[3], v2.array[3]);
    v0.array[0] = _mm256_andnot_si256(v1.array[0], v0.array[0]);
    v0.array[1] = _mm256_andnot_si256(v1.array[1], v0.array[1]);
    v0.array[2] = _mm256_andnot_si256(v1.array[2], v0.array[2]);
    v0.array[3] = _mm256_andnot_si256(v1.array[3], v0.array[3]);
    _mm256_storeu_si256((__m256i*)&(output[1].array[0]), _mm256_xor_si256(v0.array[0], one));
    _mm256_storeu_si256((__m256i*)&(output[1].array[1]), _mm256_xor_si256(v0.array[1], one));
    _mm256_storeu_si256((__m256i*)&(output[1].array[2]), _mm256_xor_si256(v0.array[2], one));
    _mm256_storeu_si256((__m256i*)&(output[1].array[3]), _mm256_xor_si256(v0.array[3], one));
}
"##
    );

    let circuit = Circuit::<u32>::from_str(
        r##"{
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        12
        13
        14
        15
        xor(0,4):0
        xor(1,5)
        and(0,4)
        xor(17,18):1
        xor(2,6)
        and(17,18)
        and(1,5)
        nor(21,22)
        xor(20,23):2n
        xor(3,7)
        nimpl(20,23)
        and(2,6)
        nor(26,27)
        xor(25,28):3n
        xor(8,12):4
        xor(9,13)
        nimpl(8,12)
        nimpl(30,32)
        xor(31,33):5
        xor(10,14)
        nor(31,33)
        nimpl(9,13)
        nor(36,37)
        xor(35,38):6
        xor(11,15)
        nor(35,38)
        nimpl(10,14)
        nor(41,42)
        xor(40,43):7
    }(16)
"##,
    )
    .unwrap();
    // advanced aggr output, pop input, loop
    let mut writer = CLANG_WRITER_INTEL_SSE2.writer_with_array_len(Some(4));
    writer.prolog();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)buffer)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)buffer)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8)))
            .inner_loop(Some(10))
            .single_buffer(true),
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
typedef struct _gate_sys_type {
    __m128i array[4];
} gate_sys_type;
#define TYPE_LEN (512)
#define TYPE_NAME gate_sys_type
#define __INT_GET_U32(D,X,I) { uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    (D) = temp[(I)]; \
}
#define __INT_GET_U32_ALL(D,X) { _mm_storeu_si128((__m128i*)(D), (X)); }
#define __INT_SET_U32(X,S,I) { uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_si128((__m128i*)temp); \
}
#define __INT_SET_U32_ALL(X,S) { (X) = _mm_loadu_si128((__m128i*)(S)); }
#define GET_U32(D,X,I) \
    __INT_GET_U32((D), (X).array[(I) / 4], (I) % 4)
#define GET_U32_ALL(D,X) { \
    __INT_GET_U32_ALL((&((D)[0])), (X).array[0]); \
    __INT_GET_U32_ALL((&((D)[4])), (X).array[1]); \
    __INT_GET_U32_ALL((&((D)[8])), (X).array[2]); \
    __INT_GET_U32_ALL((&((D)[12])), (X).array[3]); \
}
#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / 4], (S), (I) % 4)
#define SET_U32_ALL(X,S) { \
    __INT_SET_U32_ALL((X).array[0], (&((S)[0]))); \
    __INT_SET_U32_ALL((X).array[1], (&((S)[4]))); \
    __INT_SET_U32_ALL((X).array[2], (&((S)[8]))); \
    __INT_SET_U32_ALL((X).array[3], (&((S)[12]))); \
}
void gate_sys_mulxx(gate_sys_type* output, unsigned int arg, unsigned int arg2, void* buffer, size_t idx) {
    const __m128i zero = *((const __m128i*)zero_value);
    const __m128i one = *((const __m128i*)one_value);
    const __m128i elem_low_bit0 = *((const __m128i*)elem_index_low_tbl);
    const __m128i elem_low_bit1 = *((const __m128i*)(elem_index_low_tbl + 4));
    const __m128i elem_low_bit2 = *((const __m128i*)(elem_index_low_tbl + 8));
    const __m128i elem_low_bit3 = *((const __m128i*)(elem_index_low_tbl + 12));
    const __m128i elem_low_bit4 = *((const __m128i*)(elem_index_low_tbl + 16));
    const __m128i elem_low_bit5 = *((const __m128i*)(elem_index_low_tbl + 20));
    const __m128i elem_low_bit6 = *((const __m128i*)(elem_index_low_tbl + 24));
    const unsigned int idxl0 = (idx * 4 + 0) & 0xffffffff;
    const unsigned int idxh0 = (idx * 4 + 0) >> 32;
    const unsigned int idxl1 = (idx * 4 + 1) & 0xffffffff;
    const unsigned int idxh1 = (idx * 4 + 1) >> 32;
    const unsigned int idxl2 = (idx * 4 + 2) & 0xffffffff;
    const unsigned int idxh2 = (idx * 4 + 2) >> 32;
    const unsigned int idxl3 = (idx * 4 + 3) & 0xffffffff;
    const unsigned int idxh3 = (idx * 4 + 3) >> 32;
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    gate_sys_type v0;
    gate_sys_type v1;
    gate_sys_type v2;
    gate_sys_type v3;
    gate_sys_type v4;
    gate_sys_type v5;
    gate_sys_type v6;
    gate_sys_type v7;
    gate_sys_type v8;
    gate_sys_type v9;
    gate_sys_type v10;
    gate_sys_type v11;
    gate_sys_type v12;
    gate_sys_type v13;
    gate_sys_type v14;
    gate_sys_type v15;
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i2 (v2)
#define i3 (v3)
#define i6 (v4)
#define i10 (v6)
    i2 = ((TYPE_NAME*)buffer)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    if (iter == 0) {
    v0.array[0] = _mm_loadu_si128((const __m128i*)&(output[0].array[0]));
    v0.array[1] = _mm_loadu_si128((const __m128i*)&(output[0].array[1]));
    v0.array[2] = _mm_loadu_si128((const __m128i*)&(output[0].array[2]));
    v0.array[3] = _mm_loadu_si128((const __m128i*)&(output[0].array[3]));
    v1.array[0] = _mm_loadu_si128((const __m128i*)&(output[2].array[0]));
    v1.array[1] = _mm_loadu_si128((const __m128i*)&(output[2].array[1]));
    v1.array[2] = _mm_loadu_si128((const __m128i*)&(output[2].array[2]));
    v1.array[3] = _mm_loadu_si128((const __m128i*)&(output[2].array[3]));
    v5.array[0] = _mm_loadu_si128((const __m128i*)&(output[1].array[0]));
    v5.array[1] = _mm_loadu_si128((const __m128i*)&(output[1].array[1]));
    v5.array[2] = _mm_loadu_si128((const __m128i*)&(output[1].array[2]));
    v5.array[3] = _mm_loadu_si128((const __m128i*)&(output[1].array[3]));
    v7.array[0] = _mm_loadu_si128((const __m128i*)&(output[3].array[0]));
    v7.array[1] = _mm_loadu_si128((const __m128i*)&(output[3].array[1]));
    v7.array[2] = _mm_loadu_si128((const __m128i*)&(output[3].array[2]));
    v7.array[3] = _mm_loadu_si128((const __m128i*)&(output[3].array[3]));
    v8.array[0] = _mm_loadu_si128((const __m128i*)&(output[5].array[0]));
    v8.array[1] = _mm_loadu_si128((const __m128i*)&(output[5].array[1]));
    v8.array[2] = _mm_loadu_si128((const __m128i*)&(output[5].array[2]));
    v8.array[3] = _mm_loadu_si128((const __m128i*)&(output[5].array[3]));
    v9.array[0] = _mm_loadu_si128((const __m128i*)&(output[7].array[0]));
    v9.array[1] = _mm_loadu_si128((const __m128i*)&(output[7].array[1]));
    v9.array[2] = _mm_loadu_si128((const __m128i*)&(output[7].array[2]));
    v9.array[3] = _mm_loadu_si128((const __m128i*)&(output[7].array[3]));
    }
    v10.array[0] = elem_low_bit0;
    v10.array[1] = elem_low_bit0;
    v10.array[2] = elem_low_bit0;
    v10.array[3] = elem_low_bit0;
    v11.array[0] = _mm_xor_si128(v0.array[0], v10.array[0]);
    v11.array[1] = _mm_xor_si128(v0.array[1], v10.array[1]);
    v11.array[2] = _mm_xor_si128(v0.array[2], v10.array[2]);
    v11.array[3] = _mm_xor_si128(v0.array[3], v10.array[3]);
    v12.array[0] = elem_low_bit1;
    v12.array[1] = elem_low_bit1;
    v12.array[2] = elem_low_bit1;
    v12.array[3] = elem_low_bit1;
    v13.array[0] = _mm_xor_si128(v1.array[0], v12.array[0]);
    v13.array[1] = _mm_xor_si128(v1.array[1], v12.array[1]);
    v13.array[2] = _mm_xor_si128(v1.array[2], v12.array[2]);
    v13.array[3] = _mm_xor_si128(v1.array[3], v12.array[3]);
    v0.array[0] = _mm_and_si128(v0.array[0], v10.array[0]);
    v0.array[1] = _mm_and_si128(v0.array[1], v10.array[1]);
    v0.array[2] = _mm_and_si128(v0.array[2], v10.array[2]);
    v0.array[3] = _mm_and_si128(v0.array[3], v10.array[3]);
    v10.array[0] = _mm_xor_si128(v13.array[0], v0.array[0]);
    v10.array[1] = _mm_xor_si128(v13.array[1], v0.array[1]);
    v10.array[2] = _mm_xor_si128(v13.array[2], v0.array[2]);
    v10.array[3] = _mm_xor_si128(v13.array[3], v0.array[3]);
    _mm_storeu_si128((__m128i*)&(output[5].array[0]), v10.array[0]);
    _mm_storeu_si128((__m128i*)&(output[5].array[1]), v10.array[1]);
    _mm_storeu_si128((__m128i*)&(output[5].array[2]), v10.array[2]);
    _mm_storeu_si128((__m128i*)&(output[5].array[3]), v10.array[3]);
    v14.array[0] = _mm_xor_si128(v2.array[0], v4.array[0]);
    v14.array[1] = _mm_xor_si128(v2.array[1], v4.array[1]);
    v14.array[2] = _mm_xor_si128(v2.array[2], v4.array[2]);
    v14.array[3] = _mm_xor_si128(v2.array[3], v4.array[3]);
    v0.array[0] = _mm_and_si128(v13.array[0], v0.array[0]);
    v0.array[1] = _mm_and_si128(v13.array[1], v0.array[1]);
    v0.array[2] = _mm_and_si128(v13.array[2], v0.array[2]);
    v0.array[3] = _mm_and_si128(v13.array[3], v0.array[3]);
    v1.array[0] = _mm_and_si128(v1.array[0], v12.array[0]);
    v1.array[1] = _mm_and_si128(v1.array[1], v12.array[1]);
    v1.array[2] = _mm_and_si128(v1.array[2], v12.array[2]);
    v1.array[3] = _mm_and_si128(v1.array[3], v12.array[3]);
    v0.array[0] = _mm_or_si128(v0.array[0], v1.array[0]);
    v0.array[1] = _mm_or_si128(v0.array[1], v1.array[1]);
    v0.array[2] = _mm_or_si128(v0.array[2], v1.array[2]);
    v0.array[3] = _mm_or_si128(v0.array[3], v1.array[3]);
    v1.array[0] = _mm_xor_si128(v14.array[0], v0.array[0]);
    v1.array[1] = _mm_xor_si128(v14.array[1], v0.array[1]);
    v1.array[2] = _mm_xor_si128(v14.array[2], v0.array[2]);
    v1.array[3] = _mm_xor_si128(v14.array[3], v0.array[3]);
    _mm_storeu_si128((__m128i*)&(output[7].array[0]), v1.array[0]);
    _mm_storeu_si128((__m128i*)&(output[7].array[1]), v1.array[1]);
    _mm_storeu_si128((__m128i*)&(output[7].array[2]), v1.array[2]);
    _mm_storeu_si128((__m128i*)&(output[7].array[3]), v1.array[3]);
    v12.array[0] = elem_low_bit2;
    v12.array[1] = elem_low_bit2;
    v12.array[2] = elem_low_bit2;
    v12.array[3] = elem_low_bit2;
    v3.array[0] = _mm_xor_si128(v3.array[0], v12.array[0]);
    v3.array[1] = _mm_xor_si128(v3.array[1], v12.array[1]);
    v3.array[2] = _mm_xor_si128(v3.array[2], v12.array[2]);
    v3.array[3] = _mm_xor_si128(v3.array[3], v12.array[3]);
    v0.array[0] = _mm_and_si128(v14.array[0], v0.array[0]);
    v0.array[1] = _mm_and_si128(v14.array[1], v0.array[1]);
    v0.array[2] = _mm_and_si128(v14.array[2], v0.array[2]);
    v0.array[3] = _mm_and_si128(v14.array[3], v0.array[3]);
    v2.array[0] = _mm_and_si128(v2.array[0], v4.array[0]);
    v2.array[1] = _mm_and_si128(v2.array[1], v4.array[1]);
    v2.array[2] = _mm_and_si128(v2.array[2], v4.array[2]);
    v2.array[3] = _mm_and_si128(v2.array[3], v4.array[3]);
    v0.array[0] = _mm_or_si128(v0.array[0], v2.array[0]);
    v0.array[1] = _mm_or_si128(v0.array[1], v2.array[1]);
    v0.array[2] = _mm_or_si128(v0.array[2], v2.array[2]);
    v0.array[3] = _mm_or_si128(v0.array[3], v2.array[3]);
    v0.array[0] = _mm_xor_si128(v3.array[0], v0.array[0]);
    v0.array[1] = _mm_xor_si128(v3.array[1], v0.array[1]);
    v0.array[2] = _mm_xor_si128(v3.array[2], v0.array[2]);
    v0.array[3] = _mm_xor_si128(v3.array[3], v0.array[3]);
    _mm_storeu_si128((__m128i*)&(output[3].array[0]), v0.array[0]);
    _mm_storeu_si128((__m128i*)&(output[3].array[1]), v0.array[1]);
    _mm_storeu_si128((__m128i*)&(output[3].array[2]), v0.array[2]);
    _mm_storeu_si128((__m128i*)&(output[3].array[3]), v0.array[3]);
    v2.array[0] = elem_low_bit3;
    v2.array[1] = elem_low_bit3;
    v2.array[2] = elem_low_bit3;
    v2.array[3] = elem_low_bit3;
    v3.array[0] = ((arg & 1) != 0) ? one : zero;
    v3.array[1] = ((arg & 1) != 0) ? one : zero;
    v3.array[2] = ((arg & 1) != 0) ? one : zero;
    v3.array[3] = ((arg & 1) != 0) ? one : zero;
    v4.array[0] = _mm_xor_si128(v2.array[0], v3.array[0]);
    v4.array[1] = _mm_xor_si128(v2.array[1], v3.array[1]);
    v4.array[2] = _mm_xor_si128(v2.array[2], v3.array[2]);
    v4.array[3] = _mm_xor_si128(v2.array[3], v3.array[3]);
    _mm_storeu_si128((__m128i*)&(output[2].array[0]), v4.array[0]);
    _mm_storeu_si128((__m128i*)&(output[2].array[1]), v4.array[1]);
    _mm_storeu_si128((__m128i*)&(output[2].array[2]), v4.array[2]);
    _mm_storeu_si128((__m128i*)&(output[2].array[3]), v4.array[3]);
    v12.array[0] = ((arg & 2) != 0) ? one : zero;
    v12.array[1] = ((arg & 2) != 0) ? one : zero;
    v12.array[2] = ((arg & 2) != 0) ? one : zero;
    v12.array[3] = ((arg & 2) != 0) ? one : zero;
    v13.array[0] = _mm_xor_si128(v5.array[0], v12.array[0]);
    v13.array[1] = _mm_xor_si128(v5.array[1], v12.array[1]);
    v13.array[2] = _mm_xor_si128(v5.array[2], v12.array[2]);
    v13.array[3] = _mm_xor_si128(v5.array[3], v12.array[3]);
    v2.array[0] = _mm_andnot_si128(v3.array[0], v2.array[0]);
    v2.array[1] = _mm_andnot_si128(v3.array[1], v2.array[1]);
    v2.array[2] = _mm_andnot_si128(v3.array[2], v2.array[2]);
    v2.array[3] = _mm_andnot_si128(v3.array[3], v2.array[3]);
    v2.array[0] = _mm_andnot_si128(v2.array[0], v4.array[0]);
    v2.array[1] = _mm_andnot_si128(v2.array[1], v4.array[1]);
    v2.array[2] = _mm_andnot_si128(v2.array[2], v4.array[2]);
    v2.array[3] = _mm_andnot_si128(v2.array[3], v4.array[3]);
    v3.array[0] = _mm_xor_si128(v13.array[0], v2.array[0]);
    v3.array[1] = _mm_xor_si128(v13.array[1], v2.array[1]);
    v3.array[2] = _mm_xor_si128(v13.array[2], v2.array[2]);
    v3.array[3] = _mm_xor_si128(v13.array[3], v2.array[3]);
    v14.array[0] = _mm_xor_si128(v6.array[0], v8.array[0]);
    v14.array[1] = _mm_xor_si128(v6.array[1], v8.array[1]);
    v14.array[2] = _mm_xor_si128(v6.array[2], v8.array[2]);
    v14.array[3] = _mm_xor_si128(v6.array[3], v8.array[3]);
    v2.array[0] = _mm_or_si128(v13.array[0], v2.array[0]);
    v2.array[1] = _mm_or_si128(v13.array[1], v2.array[1]);
    v2.array[2] = _mm_or_si128(v13.array[2], v2.array[2]);
    v2.array[3] = _mm_or_si128(v13.array[3], v2.array[3]);
    v5.array[0] = _mm_andnot_si128(v12.array[0], v5.array[0]);
    v5.array[1] = _mm_andnot_si128(v12.array[1], v5.array[1]);
    v5.array[2] = _mm_andnot_si128(v12.array[2], v5.array[2]);
    v5.array[3] = _mm_andnot_si128(v12.array[3], v5.array[3]);
    v2.array[0] = _mm_andnot_si128(v5.array[0], v2.array[0]);
    v2.array[1] = _mm_andnot_si128(v5.array[1], v2.array[1]);
    v2.array[2] = _mm_andnot_si128(v5.array[2], v2.array[2]);
    v2.array[3] = _mm_andnot_si128(v5.array[3], v2.array[3]);
    v5.array[0] = _mm_xor_si128(v14.array[0], v2.array[0]);
    v5.array[1] = _mm_xor_si128(v14.array[1], v2.array[1]);
    v5.array[2] = _mm_xor_si128(v14.array[2], v2.array[2]);
    v5.array[3] = _mm_xor_si128(v14.array[3], v2.array[3]);
    _mm_storeu_si128((__m128i*)&(output[0].array[0]), v5.array[0]);
    _mm_storeu_si128((__m128i*)&(output[0].array[1]), v5.array[1]);
    _mm_storeu_si128((__m128i*)&(output[0].array[2]), v5.array[2]);
    _mm_storeu_si128((__m128i*)&(output[0].array[3]), v5.array[3]);
    v7.array[0] = _mm_xor_si128(v7.array[0], v9.array[0]);
    v7.array[1] = _mm_xor_si128(v7.array[1], v9.array[1]);
    v7.array[2] = _mm_xor_si128(v7.array[2], v9.array[2]);
    v7.array[3] = _mm_xor_si128(v7.array[3], v9.array[3]);
    v2.array[0] = _mm_or_si128(v14.array[0], v2.array[0]);
    v2.array[1] = _mm_or_si128(v14.array[1], v2.array[1]);
    v2.array[2] = _mm_or_si128(v14.array[2], v2.array[2]);
    v2.array[3] = _mm_or_si128(v14.array[3], v2.array[3]);
    v6.array[0] = _mm_andnot_si128(v8.array[0], v6.array[0]);
    v6.array[1] = _mm_andnot_si128(v8.array[1], v6.array[1]);
    v6.array[2] = _mm_andnot_si128(v8.array[2], v6.array[2]);
    v6.array[3] = _mm_andnot_si128(v8.array[3], v6.array[3]);
    v2.array[0] = _mm_andnot_si128(v6.array[0], v2.array[0]);
    v2.array[1] = _mm_andnot_si128(v6.array[1], v2.array[1]);
    v2.array[2] = _mm_andnot_si128(v6.array[2], v2.array[2]);
    v2.array[3] = _mm_andnot_si128(v6.array[3], v2.array[3]);
    v2.array[0] = _mm_xor_si128(v7.array[0], v2.array[0]);
    v2.array[1] = _mm_xor_si128(v7.array[1], v2.array[1]);
    v2.array[2] = _mm_xor_si128(v7.array[2], v2.array[2]);
    v2.array[3] = _mm_xor_si128(v7.array[3], v2.array[3]);
    _mm_storeu_si128((__m128i*)&(output[1].array[0]), v2.array[0]);
    _mm_storeu_si128((__m128i*)&(output[1].array[1]), v2.array[1]);
    _mm_storeu_si128((__m128i*)&(output[1].array[2]), v2.array[2]);
    _mm_storeu_si128((__m128i*)&(output[1].array[3]), v2.array[3]);
#define o0 (v11)
#define o2 (v1)
#define o3 (v0)
#define o5 (v3)
    ((TYPE_NAME*)buffer)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    _mm_storeu_si128((__m128i*)&(output[5].array[0]), v10.array[0]);
    _mm_storeu_si128((__m128i*)&(output[5].array[1]), v10.array[1]);
    _mm_storeu_si128((__m128i*)&(output[5].array[2]), v10.array[2]);
    _mm_storeu_si128((__m128i*)&(output[5].array[3]), v10.array[3]);
    _mm_storeu_si128((__m128i*)&(output[7].array[0]), v1.array[0]);
    _mm_storeu_si128((__m128i*)&(output[7].array[1]), v1.array[1]);
    _mm_storeu_si128((__m128i*)&(output[7].array[2]), v1.array[2]);
    _mm_storeu_si128((__m128i*)&(output[7].array[3]), v1.array[3]);
    _mm_storeu_si128((__m128i*)&(output[3].array[0]), v0.array[0]);
    _mm_storeu_si128((__m128i*)&(output[3].array[1]), v0.array[1]);
    _mm_storeu_si128((__m128i*)&(output[3].array[2]), v0.array[2]);
    _mm_storeu_si128((__m128i*)&(output[3].array[3]), v0.array[3]);
    _mm_storeu_si128((__m128i*)&(output[2].array[0]), v4.array[0]);
    _mm_storeu_si128((__m128i*)&(output[2].array[1]), v4.array[1]);
    _mm_storeu_si128((__m128i*)&(output[2].array[2]), v4.array[2]);
    _mm_storeu_si128((__m128i*)&(output[2].array[3]), v4.array[3]);
    _mm_storeu_si128((__m128i*)&(output[0].array[0]), v5.array[0]);
    _mm_storeu_si128((__m128i*)&(output[0].array[1]), v5.array[1]);
    _mm_storeu_si128((__m128i*)&(output[0].array[2]), v5.array[2]);
    _mm_storeu_si128((__m128i*)&(output[0].array[3]), v5.array[3]);
    _mm_storeu_si128((__m128i*)&(output[1].array[0]), v2.array[0]);
    _mm_storeu_si128((__m128i*)&(output[1].array[1]), v2.array[1]);
    _mm_storeu_si128((__m128i*)&(output[1].array[2]), v2.array[2]);
    _mm_storeu_si128((__m128i*)&(output[1].array[3]), v2.array[3]);
    } else {
    v7 = v0;
    v9 = v1;
    v0 = v5;
    v5 = v2;
    v1 = v4;
    v8 = v10;
    }
    } // loop
}
"##
    );
}

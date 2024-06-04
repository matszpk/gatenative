use crate::*;

use crate::clang_transform::*;

use std::collections::{HashMap, HashSet};
use std::io::Write;

#[derive(Clone, Debug)]
pub struct ElemIndexConfig<'a> {
    low_bits_init: &'a str,
    low_bits_defs: [&'a str; 16],
}

#[derive(Clone, Debug)]
pub struct CLangWriterConfig<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    init_local_index: Option<&'a str>, // to initialize local index in OpenCL kernel
    buffer_shift: bool,
    include_name: Option<&'a str>,
    include_name_2: Option<&'a str>,
    include_name_3: Option<&'a str>,
    type_name: &'a str,
    type_bit_len: u32,
    arg_modifier: Option<&'a str>,
    and_op: &'a str,
    or_op: &'a str,
    xor_op: &'a str,
    impl_op: Option<&'a str>,
    nimpl_op: Option<&'a str>,
    not_op: Option<&'a str>,
    lop3_op: Option<(&'a str, &'a str)>,
    zero_value: (&'a str, &'a str), // for arg_input
    one_value: (&'a str, &'a str),  // for emulate NOT and arg_input
    elem_index: ElemIndexConfig<'a>,
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
    get_u32_op: &'a str,
    get_u32_all_op: &'a str,
    set_u32_op: &'a str,
    set_u32_all_op: &'a str,
    func_finish: Option<&'a str>,
    transform_config: &'a CLangTransformConfig<'a>,
}

pub const CLANG_WRITER_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint32_t",
    type_bit_len: 32,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0"),
    one_value: ("", "0xffffffff"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaa",
            "0xcccccccc",
            "0xf0f0f0f0",
            "0xff00ff00",
            "0xffff0000",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (X); }",
    get_u32_all_op: "{ (D)[0] = (X); }",
    set_u32_op: "{ (X) = (S); }",
    set_u32_all_op: "{ (X) = (S)[0]; }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_U32,
};

pub const CLANG_WRITER_U64: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0ULL"),
    one_value: ("", "0xffffffffffffffffULL"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaaaaaaaaaaULL",
            "0xccccccccccccccccULL",
            "0xf0f0f0f0f0f0f0f0ULL",
            "0xff00ff00ff00ff00ULL",
            "0xffff0000ffff0000ULL",
            "0xffffffff00000000ULL",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = ((X) >> ((I)<<5)); }",
    get_u32_all_op: "{ (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }",
    set_u32_op: r##"{ uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }"##,
    set_u32_all_op: "{ (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_U64,
};

pub const CLANG_WRITER_U64_TEST_LOP3: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: Some((
        r##"static inline uint64_t lop3_test(uint64_t a, uint64_t b, uint64_t c, uint8_t comb) {
    return ((~a & ~b & ~c & (0ULL - (((uint64_t)comb) & 1))) |
        (~a & ~b & c & (0ULL - ((((uint64_t)comb) >> 1) & 1))) |
        (~a & b & ~c & (0ULL - ((((uint64_t)comb) >> 2) & 1))) |
        (~a & b & c & (0ULL - ((((uint64_t)comb) >> 3) & 1))) |
        (a & ~b & ~c & (0ULL - ((((uint64_t)comb) >> 4) & 1))) |
        (a & ~b & c & (0ULL - ((((uint64_t)comb) >> 5) & 1))) |
        (a & b & ~c & (0ULL - ((((uint64_t)comb) >> 6) & 1))) |
        (a & b & c & (0ULL - ((((uint64_t)comb) >> 7) & 1))));
}"##,
        "lop3_test({}, {}, {}, {})",
    )),
    zero_value: ("", "0ULL"),
    one_value: ("", "0xffffffffffffffffULL"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaaaaaaaaaaULL",
            "0xccccccccccccccccULL",
            "0xf0f0f0f0f0f0f0f0ULL",
            "0xff00ff00ff00ff00ULL",
            "0xffff0000ffff0000ULL",
            "0xffffffff00000000ULL",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = ((X) >> ((I)<<5)); }",
    get_u32_all_op: "{ (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }",
    set_u32_op: r##"{ uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }"##,
    set_u32_all_op: "{ (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_U64,
};

pub const CLANG_WRITER_U64_TEST_IMPL: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: Some("(~{} | {})"),
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0ULL"),
    one_value: ("", "0xffffffffffffffffULL"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaaaaaaaaaaULL",
            "0xccccccccccccccccULL",
            "0xf0f0f0f0f0f0f0f0ULL",
            "0xff00ff00ff00ff00ULL",
            "0xffff0000ffff0000ULL",
            "0xffffffff00000000ULL",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = ((X) >> ((I)<<5)); }",
    get_u32_all_op: "{ (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }",
    set_u32_op: r##"{ uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }"##,
    set_u32_all_op: "{ (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_U64,
};

pub const CLANG_WRITER_U64_TEST_NIMPL: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: Some("({} & ~{})"),
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0ULL"),
    one_value: ("", "0xffffffffffffffffULL"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaaaaaaaaaaULL",
            "0xccccccccccccccccULL",
            "0xf0f0f0f0f0f0f0f0ULL",
            "0xff00ff00ff00ff00ULL",
            "0xffff0000ffff0000ULL",
            "0xffffffff00000000ULL",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = ((X) >> ((I)<<5)); }",
    get_u32_all_op: "{ (D)[0] = (uint32_t)(X); (D)[1] = (uint32_t)((X) >> 32); }",
    set_u32_op: r##"{ uint64_t mask = (0xffffffffULL << ((I)<<5)); \
    (X) = ((X) & ~mask) | (((uint64_t)(S) << ((I)<<5)) & mask); }"##,
    set_u32_all_op: "{ (X) = ((uint64_t)((S)[0])) | (((uint64_t)((S)[1]))<<32); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_U64,
};

pub const CLANG_WRITER_INTEL_MMX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("mmintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m64",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    xor_op: "_m_pxor({}, {})",
    impl_op: None,
    nimpl_op: Some("_m_pandn({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[2] = { 0, 0 };"##,
        "*((const __m64*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };"##,
        "*((const __m64*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[6*2] = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xcccccccc, 0xcccccccc, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xffff0000, 0xffff0000, 0x00000000, 0xffffffff
};"##,
        low_bits_defs: [
            "*((const __m64*)elem_index_low_tbl)",
            "*((const __m64*)(elem_index_low_tbl + 2))",
            "*((const __m64*)(elem_index_low_tbl + 4))",
            "*((const __m64*)(elem_index_low_tbl + 6))",
            "*((const __m64*)(elem_index_low_tbl + 8))",
            "*((const __m64*)(elem_index_low_tbl + 10))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (uint32_t)(_m_to_int(_mm_srli_si64((X), ((I) << 5)))); }",
    get_u32_all_op: r##"{ \
    (D)[0] = (uint32_t)(_m_to_int((X))); \
    (D)[1] = (uint32_t)(_m_to_int(_mm_srli_si64((X), 32))); \
}"##,
    set_u32_op: r##"{ uint32_t temp[2]; \
    *(__m64*)temp = (X); \
    temp[(I)] = (S); \
    (X) = *(__m64*)temp; \
}"##,
    set_u32_all_op: "{ (X) = *(__m64*)(S); }",
    func_finish: Some("_m_empty();"),
    transform_config: &CLANG_TRANSFORM_INTEL_MMX,
};

pub const CLANG_WRITER_INTEL_SSE: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("xmmintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m128",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "_mm_and_ps({}, {})",
    or_op: "_mm_or_ps({}, {})",
    xor_op: "_mm_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm_andnot_ps({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[4] __attribute__((aligned(16))) =
    { 0, 0, 0, 0 };"##,
        "*((const __m128*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[4] __attribute__((aligned(16))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };"##,
        "*((const __m128*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[7*4]
__attribute__((aligned(16))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff
};"##,
        low_bits_defs: [
            "*((const __m128*)elem_index_low_tbl)",
            "*((const __m128*)(elem_index_low_tbl + 4))",
            "*((const __m128*)(elem_index_low_tbl + 8))",
            "*((const __m128*)(elem_index_low_tbl + 12))",
            "*((const __m128*)(elem_index_low_tbl + 16))",
            "*((const __m128*)(elem_index_low_tbl + 20))",
            "*((const __m128*)(elem_index_low_tbl + 24))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: Some("_mm_loadu_ps((const float*)&{})"),
    store_op: Some("_mm_storeu_ps((float*)&{}, {})"),
    get_u32_op: r##"{ uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ _mm_storeu_ps((float*)(D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[4]; \
    _mm_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_ps((float*)temp); \
}"##,
    set_u32_all_op: "{ (X) = _mm_loadu_ps((float*)(S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_INTEL_SSE,
};

pub const CLANG_WRITER_INTEL_SSE2: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("xmmintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m128i",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "_mm_and_si128({}, {})",
    or_op: "_mm_or_si128({}, {})",
    xor_op: "_mm_xor_si128({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm_andnot_si128({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[4] __attribute__((aligned(16))) =
    { 0, 0, 0, 0 };"##,
        "*((const __m128i*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[4] __attribute__((aligned(16))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };"##,
        "*((const __m128i*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[7*4]
__attribute__((aligned(16))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff
};"##,
        low_bits_defs: [
            "*((const __m128i*)elem_index_low_tbl)",
            "*((const __m128i*)(elem_index_low_tbl + 4))",
            "*((const __m128i*)(elem_index_low_tbl + 8))",
            "*((const __m128i*)(elem_index_low_tbl + 12))",
            "*((const __m128i*)(elem_index_low_tbl + 16))",
            "*((const __m128i*)(elem_index_low_tbl + 20))",
            "*((const __m128i*)(elem_index_low_tbl + 24))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: Some("_mm_loadu_si128((const __m128i*)&{})"),
    store_op: Some("_mm_storeu_si128((__m128i*)&{}, {})"),
    get_u32_op: r##"{ uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ _mm_storeu_si128((__m128i*)(D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[4]; \
    _mm_storeu_si128((__m128i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm_loadu_si128((__m128i*)temp); \
}"##,
    set_u32_all_op: "{ (X) = _mm_loadu_si128((__m128i*)(S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_INTEL_SSE2,
};

pub const CLANG_WRITER_INTEL_AVX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m256",
    type_bit_len: 256,
    arg_modifier: None,
    and_op: "_mm256_and_ps({}, {})",
    or_op: "_mm256_or_ps({}, {})",
    xor_op: "_mm256_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm256_andnot_ps({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};"##,
        "*((const __m256*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[8] __attribute__((aligned(32))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        "*((const __m256*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[8*8]
__attribute__((aligned(32))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        low_bits_defs: [
            "*((const __m256*)elem_index_low_tbl)",
            "*((const __m256*)(elem_index_low_tbl + 8))",
            "*((const __m256*)(elem_index_low_tbl + 16))",
            "*((const __m256*)(elem_index_low_tbl + 24))",
            "*((const __m256*)(elem_index_low_tbl + 32))",
            "*((const __m256*)(elem_index_low_tbl + 40))",
            "*((const __m256*)(elem_index_low_tbl + 48))",
            "*((const __m256*)(elem_index_low_tbl + 56))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: Some("_mm256_loadu_ps((const float*)&{})"),
    store_op: Some("_mm256_storeu_ps((float*)&{}, {})"),
    get_u32_op: r##"{ uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ _mm256_storeu_ps((float*)(D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[8]; \
    _mm256_storeu_ps((float*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_ps((float*)temp); \
}"##,
    set_u32_all_op: "{ (X) = _mm256_loadu_ps((float*)(S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_INTEL_AVX,
};

pub const CLANG_WRITER_INTEL_AVX2: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m256i",
    type_bit_len: 256,
    arg_modifier: None,
    and_op: "_mm256_and_si256({}, {})",
    or_op: "_mm256_or_si256({}, {})",
    xor_op: "_mm256_xor_si256({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm256_andnot_si256({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};"##,
        "*((const __m256i*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[8] __attribute__((aligned(32))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        "*((const __m256i*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[8*8]
__attribute__((aligned(32))) = {
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff, 0x00000000, 0xffffffff,
    0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        low_bits_defs: [
            "*((const __m256i*)elem_index_low_tbl)",
            "*((const __m256i*)(elem_index_low_tbl + 8))",
            "*((const __m256i*)(elem_index_low_tbl + 16))",
            "*((const __m256i*)(elem_index_low_tbl + 24))",
            "*((const __m256i*)(elem_index_low_tbl + 32))",
            "*((const __m256i*)(elem_index_low_tbl + 40))",
            "*((const __m256i*)(elem_index_low_tbl + 48))",
            "*((const __m256i*)(elem_index_low_tbl + 56))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: Some("_mm256_loadu_si256((const float*)&{})"),
    store_op: Some("_mm256_storeu_si256((float*)&{}, {})"),
    get_u32_op: r##"{ uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ _mm256_storeu_si256((__m256i*)(D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[8]; \
    _mm256_storeu_si256((__m256i*)temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm256_loadu_si256((__m256i*)temp); \
}"##,
    set_u32_all_op: "{ (X) = _mm256_loadu_si256((__m256i*)(S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_INTEL_AVX2,
};

pub const CLANG_WRITER_INTEL_AVX512: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m512i",
    type_bit_len: 512,
    arg_modifier: None,
    and_op: "_mm512_and_epi64({}, {})",
    or_op: "_mm512_or_epi64({}, {})",
    xor_op: "_mm512_xor_epi64({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm512_andnot_epi64({1}, {0})"),
    not_op: None,
    lop3_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[16] __attribute__((aligned(64))) = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};"##,
        "*((const __m512i*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[16] __attribute__((aligned(64))) = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        "*((const __m512i*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[9*16]
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
};"##,
        low_bits_defs: [
            "*((const __m512i*)elem_index_low_tbl)",
            "*((const __m512i*)(elem_index_low_tbl + 16))",
            "*((const __m512i*)(elem_index_low_tbl + 32))",
            "*((const __m512i*)(elem_index_low_tbl + 48))",
            "*((const __m512i*)(elem_index_low_tbl + 64))",
            "*((const __m512i*)(elem_index_low_tbl + 80))",
            "*((const __m512i*)(elem_index_low_tbl + 96))",
            "*((const __m512i*)(elem_index_low_tbl + 112))",
            "*((const __m512i*)(elem_index_low_tbl + 128))",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: Some("_mm512_loadu_epi64(&{})"),
    store_op: Some("_mm512_storeu_epi64(&{}, {})"),
    get_u32_op: r##"{ uint32_t temp[16]; \
    _mm512_storeu_si512(temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ _mm512_storeu_si512((float*)(D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[16]; \
    _mm512_storeu_si512(temp, (X)); \
    temp[(I)] = (S); \
    (X) = _mm512_loadu_si512(temp); \
}"##,
    set_u32_all_op: "{ (X) = _mm512_loadu_si512((S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_INTEL_AVX512,
};

pub const CLANG_WRITER_ARM_NEON: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("arm_neon.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "uint32x4_t",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "vandq_u32({}, {})",
    or_op: "vorrq_u32({}, {})",
    xor_op: "veorq_u32({}, {})",
    impl_op: Some("vornq_u32({1}, {0})"),
    nimpl_op: None,
    not_op: Some("vmvnq_u32({})"),
    lop3_op: None,
    zero_value: ("", "{ 0, 0, 0, 0 }"),
    one_value: ("", "{ 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff }"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "{ 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa }",
            "{ 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc }",
            "{ 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0 }",
            "{ 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00 }",
            "{ 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000 }",
            "{ 0x00000000, 0xffffffff, 0x00000000, 0xffffffff }",
            "{ 0x00000000, 0x00000000, 0xffffffff, 0xffffffff }",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: r##"{ uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
}"##,
    get_u32_all_op: "{ vst4q_u32((D), (X)); }",
    set_u32_op: r##"{ uint32_t temp[4] __attribute__((aligned(16))); \
    vst4q_u32(temp, (X)); \
    (D) = temp[(I)]; \
    (X) = vld4q_u32(temp); \
}"##,
    set_u32_all_op: "{ (X) = vld4q_u32((S)); }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_ARM_NEON,
};

pub const CLANG_WRITER_OPENCL_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const size_t idx = get_global_id(0);"),
    init_local_index: None,
    buffer_shift: true,
    include_name: None,
    include_name_2: None,
    include_name_3: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0"),
    one_value: ("", "0xffffffff"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaa",
            "0xcccccccc",
            "0xf0f0f0f0",
            "0xff00ff00",
            "0xffff0000",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (X); }",
    get_u32_all_op: "{ (D)[0] = (X); }",
    set_u32_op: "{ (X) = (S); }",
    set_u32_all_op: "{ (X) = (S)[0]; }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_OPENCL_U32,
};

pub const CLANG_WRITER_OPENCL_U32_GROUP_VEC: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const size_t idx = get_group_id(0);"),
    init_local_index: Some(concat!(
        "const uint lidx = get_local_id(0);\n",
        "    const uint llen = get_local_size(0);"
    )),
    buffer_shift: true,
    include_name: None,
    include_name_2: None,
    include_name_3: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    lop3_op: None,
    zero_value: ("", "0"),
    one_value: ("", "0xffffffff"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaa",
            "0xcccccccc",
            "0xf0f0f0f0",
            "0xff00ff00",
            "0xffff0000",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (X); }",
    get_u32_all_op: "{ (D)[0] = (X); }",
    set_u32_op: "{ (X) = (S); }",
    set_u32_all_op: "{ (X) = (S)[0]; }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_OPENCL_U32,
};

pub const CLANG_WRITER_OPENCL_U32_LOP3: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const size_t idx = get_global_id(0);"),
    init_local_index: None,
    buffer_shift: true,
    include_name: None,
    include_name_2: None,
    include_name_3: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    // definition from:
    // https://github.com/vanhoefm/broadkey/blob/master/broadkey/opencl/opencl_misc.h
    // original name of function: lut3
    lop3_op: Some((
        r##"inline uint lop3(uint a, uint b, uint c, uint imm)
{
    uint r;
    asm("lop3.b32 %0, %1, %2, %3, %4;"
        : "=r" (r)
        : "r" (a), "r" (b), "r" (c), "i" (imm));
    return r;
}
"##,
        "lop3({}, {}, {}, {})",
    )),
    zero_value: ("", "0"),
    one_value: ("", "0xffffffff"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaa",
            "0xcccccccc",
            "0xf0f0f0f0",
            "0xff00ff00",
            "0xffff0000",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (X); }",
    get_u32_all_op: "{ (D)[0] = (X); }",
    set_u32_op: "{ (X) = (S); }",
    set_u32_all_op: "{ (X) = (S)[0]; }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_OPENCL_U32,
};

pub const CLANG_WRITER_OPENCL_U32_LOP3_GROUP_VEC: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const size_t idx = get_group_id(0);"),
    init_local_index: Some(concat!(
        "const uint lidx = get_local_id(0);\n",
        "    const uint llen = get_local_size(0);"
    )),
    buffer_shift: true,
    include_name: None,
    include_name_2: None,
    include_name_3: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    // definition from:
    // https://github.com/vanhoefm/broadkey/blob/master/broadkey/opencl/opencl_misc.h
    // original name of function: lut3
    lop3_op: Some((
        r##"inline uint lop3(uint a, uint b, uint c, uint imm)
{
    uint r;
    asm("lop3.b32 %0, %1, %2, %3, %4;"
        : "=r" (r)
        : "r" (a), "r" (b), "r" (c), "i" (imm));
    return r;
}
"##,
        "lop3({}, {}, {}, {})",
    )),
    zero_value: ("", "0"),
    one_value: ("", "0xffffffff"),
    elem_index: ElemIndexConfig {
        low_bits_init: "",
        low_bits_defs: [
            "0xaaaaaaaa",
            "0xcccccccc",
            "0xf0f0f0f0",
            "0xff00ff00",
            "0xffff0000",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    load_op: None,
    store_op: None,
    get_u32_op: "{ (D) = (X); }",
    get_u32_all_op: "{ (D)[0] = (X); }",
    set_u32_op: "{ (X) = (S); }",
    set_u32_all_op: "{ (X) = (S)[0]; }",
    func_finish: None,
    transform_config: &CLANG_TRANSFORM_OPENCL_U32,
};

pub struct CLangFuncWriter<'a, 'c> {
    writer: &'c mut CLangWriter<'a>,
    name: &'c str,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&'c [usize], usize)>,
    output_placement: Option<(&'c [usize], usize)>,
    input_map: HashMap<usize, usize>,
    output_map: HashMap<usize, usize>,
    arg_input_map: HashMap<usize, usize>,
    elem_input_map: HashMap<usize, usize>,
    pop_input_map: HashMap<usize, usize>,
    single_buffer: bool,
    init_code: Option<&'c str>,
    pop_input_code: Option<&'c str>,
    aggr_output_code: Option<&'c str>,
    output_vars: Option<Vec<(usize, usize)>>,
    aggr_to_buffer: bool,
    inner_loop: Option<u32>,
    // handling conditionals
    cond_nesting: usize,
}

pub struct CLangWriter<'a> {
    config: &'a CLangWriterConfig<'a>,
    elem_low_bits: u32,
    out: Vec<u8>,
    transform_helpers_added: bool,
    // array length
    array_len: Option<usize>,
}

impl<'a> CLangWriterConfig<'a> {
    pub fn writer_with_array_len(&'a self, array_len: Option<usize>) -> CLangWriter<'a> {
        assert!(!self.buffer_shift || self.init_index.is_some());
        if let Some(alen) = array_len {
            assert_ne!(alen, 0);
        }
        CLangWriter {
            config: self,
            elem_low_bits: self
                .elem_index
                .low_bits_defs
                .iter()
                .position(|x| *x == "")
                .unwrap_or(16) as u32,
            out: vec![],
            transform_helpers_added: false,
            array_len,
        }
    }
    pub fn writer(&'a self) -> CLangWriter<'a> {
        self.writer_with_array_len(None)
    }
}

impl<'a> CLangWriter<'a> {
    fn write_op(out: &mut Vec<u8>, op: &str, args: &[&[u8]]) {
        let mut rest = op;
        let mut arg_index = 0;
        while let Some(p) = rest.find('{') {
            out.extend(rest[..p].as_bytes());
            rest = &rest[p + 1..];
            if let Some(endr) = rest.find('}') {
                if rest[..endr].is_empty() {
                    // fetch next argument
                    out.extend(args[arg_index]);
                    arg_index += 1;
                } else {
                    // fetch argument with index given between {}
                    let index = usize::from_str_radix(&rest[..endr], 10).unwrap();
                    out.extend(args[index]);
                }
                rest = &rest[endr + 1..];
            } else {
                panic!("Unexpected");
            }
        }
        if !rest.is_empty() {
            out.extend(rest.as_bytes());
        }
    }

    fn write_neg(config: &CLangWriterConfig, out: &mut Vec<u8>, arg: &[u8]) {
        if let Some(op) = config.not_op {
            CLangWriter::write_op(out, op, &[arg]);
        } else {
            CLangWriter::write_op(out, config.xor_op, &[arg, b"one"]);
        }
    }

    // aidx - array_index for array_len
    fn format_neg_arg(&self, neg: bool, reg: usize, aidx: usize) -> String {
        if neg {
            let arg = self.format_reg(reg, aidx);
            let mut out_str = vec![];
            CLangWriter::write_neg(self.config, &mut out_str, arg.as_bytes());
            String::from_utf8_lossy(&out_str).to_string()
        } else {
            self.format_reg(reg, aidx)
        }
    }

    // to get calculation type name
    fn calc_type_name(&self) -> &'a str {
        if self.array_len.is_some() {
            "gate_sys_type"
        } else {
            self.config.type_name
        }
    }

    fn format_reg(&self, reg: usize, array_index: usize) -> String {
        if let Some(alen) = self.array_len {
            assert!(array_index < alen);
            format!("v{}.array[{}]", reg, array_index)
        } else {
            format!("v{}", reg)
        }
    }
}

impl<'a, 'c> FuncWriter for CLangFuncWriter<'a, 'c> {
    fn func_start(&mut self) {
        let shift_args = if self.writer.config.buffer_shift {
            if self.single_buffer {
                "\n    unsigned long output_shift,\n    "
            } else {
                "\n    unsigned long input_shift, unsigned long output_shift,\n    "
            }
        } else {
            ""
        };
        let shift_args = if self.writer.config.init_index.is_some()
            && (!self.pop_input_map.is_empty() || self.aggr_to_buffer)
        {
            shift_args.to_string() + "unsigned long buffer_shift, "
        } else {
            shift_args.to_string()
        };
        let arg_input = if !self.arg_input_map.is_empty() {
            ", unsigned int arg, unsigned int arg2"
        } else {
            ""
        };
        let buffer = if !self.pop_input_map.is_empty() || self.aggr_to_buffer {
            format!(
                ",{}{} void* buffer",
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.writer.config.arg_modifier.unwrap_or("")
            )
        } else {
            String::new()
        };
        let in_out_args = if self.single_buffer {
            format!(
                "{0}{1}{2}* output",
                self.writer.config.arg_modifier.unwrap_or(""),
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                if self.aggr_output_code.is_some()
                    && self.pop_input_map.is_empty()
                    && !self.aggr_to_buffer
                {
                    "void"
                } else {
                    self.writer.calc_type_name()
                }
            )
        } else {
            format!(
                "const {0}{1}{2}* input,\n    {0}{1}{3}* output",
                self.writer.config.arg_modifier.unwrap_or(""),
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                if self.pop_input_code.is_some() && self.pop_input_map.is_empty() {
                    "void"
                } else {
                    self.writer.calc_type_name()
                },
                if self.aggr_output_code.is_some() && !self.aggr_to_buffer {
                    "void"
                } else {
                    self.writer.calc_type_name()
                }
            )
        };
        if let Some(init_index) = self.writer.config.init_index {
            writeln!(
                self.writer.out,
                r##"{}{}void gate_sys_{}(unsigned long n, {}{}{}{}) {{
    {}"##,
                self.writer.config.func_modifier.unwrap_or(""),
                if self.writer.config.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.name,
                shift_args,
                in_out_args,
                arg_input,
                buffer,
                init_index
            )
            .unwrap();
            if let Some(init_local_index) = self.writer.config.init_local_index {
                writeln!(self.writer.out, "    {}", init_local_index).unwrap();
            }

            let (input_shift_part, output_shift_part) = if self.writer.config.buffer_shift {
                if self.single_buffer {
                    (" + output_shift", " + output_shift")
                } else {
                    (" + input_shift", " + output_shift")
                }
            } else {
                ("", "")
            };
            if self.writer.config.init_local_index.is_some() {
                write!(
                    self.writer.out,
                    concat!(
                        "    const size_t ivn = llen * ({} * idx){};\n",
                        "    const size_t ovn = llen * ({} * idx){};\n"
                    ),
                    self.input_placement.map(|(_, len)| len).unwrap_or(
                        self.input_len
                            - self.arg_input_map.len()
                            - self.elem_input_map.len()
                            - self.pop_input_map.len()
                    ),
                    input_shift_part,
                    self.output_placement.map(|(_, len)| len).unwrap_or(
                        if !self.output_map.is_empty() {
                            self.output_map.len()
                        } else {
                            self.output_len
                        }
                    ),
                    output_shift_part,
                )
                .unwrap();
            } else {
                write!(
                    self.writer.out,
                    concat!(
                        "    const size_t ivn = {} * idx{};\n",
                        "    const size_t ovn = {} * idx{};\n"
                    ),
                    self.input_placement.map(|(_, len)| len).unwrap_or(
                        self.input_len
                            - self.arg_input_map.len()
                            - self.elem_input_map.len()
                            - self.pop_input_map.len()
                    ),
                    input_shift_part,
                    self.output_placement.map(|(_, len)| len).unwrap_or(
                        if !self.output_map.is_empty() {
                            self.output_map.len()
                        } else {
                            self.output_len
                        }
                    ),
                    output_shift_part,
                )
                .unwrap();
            }
        } else {
            writeln!(
                self.writer.out,
                r##"{}{}void gate_sys_{}({}{}{}{}, size_t idx) {{"##,
                self.writer.config.func_modifier.unwrap_or(""),
                if self.writer.config.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.name,
                shift_args,
                in_out_args,
                arg_input,
                buffer
            )
            .unwrap();
        }
        let zero_value = self.writer.config.zero_value.1;
        if !self.arg_input_map.is_empty() || !self.elem_input_map.is_empty() {
            writeln!(
                self.writer.out,
                "    const {} zero = {};",
                self.writer.calc_type_name(),
                zero_value
            )
            .unwrap();
        }
        let one_value = self.writer.config.one_value.1;
        if self.writer.config.not_op.is_none()
            || !self.arg_input_map.is_empty()
            || !self.elem_input_map.is_empty()
        {
            writeln!(
                self.writer.out,
                "    const {} one = {};",
                self.writer.calc_type_name(),
                one_value
            )
            .unwrap();
        }
        if !self.elem_input_map.is_empty() {
            for i in 0..self.writer.elem_low_bits {
                writeln!(
                    self.writer.out,
                    "    const {} elem_low_bit{} = {};",
                    self.writer.calc_type_name(),
                    i,
                    self.writer.config.elem_index.low_bits_defs[i as usize]
                )
                .unwrap();
            }
            if let Some(alen) = self.writer.array_len {
                for i in 0..alen {
                    writeln!(
                        self.writer.out,
                        "    const unsigned int idxl{0} = (idx + {0}) & 0xffffffff;",
                        i
                    )
                    .unwrap();
                    writeln!(
                        self.writer.out,
                        "    const unsigned int idxh{0} = (idx + {0}) >> 32;",
                        i
                    )
                    .unwrap();
                }
            } else {
                self.writer
                    .out
                    .extend(b"    const unsigned int idxl = idx & 0xffffffff;\n");
                self.writer
                    .out
                    .extend(b"    const unsigned int idxh = idx >> 32;\n");
            }
        }
        if let Some(iter_max) = self.inner_loop {
            writeln!(
                self.writer.out,
                "    const unsigned int iter_max = {}U;",
                iter_max
            )
            .unwrap();
            self.writer
                .out
                .extend(b"    unsigned int iter;\n    unsigned int stop = 0;\n");
        }
    }

    fn gen_aggr_output_code(&mut self) {
        if let Some(aggr_output_code) = self.aggr_output_code {
            if let Some(output_vars) = self.output_vars.as_ref() {
                for (i, v) in output_vars {
                    writeln!(self.writer.out, "#define o{} (v{})", i, *v).unwrap();
                }
            }
            self.writer.out.extend(aggr_output_code.as_bytes());
            self.writer.out.push(b'\n');
            if let Some(output_vars) = self.output_vars.as_ref() {
                for (i, _) in output_vars {
                    writeln!(self.writer.out, "#undef o{}", i).unwrap();
                }
            }
        }
    }

    fn func_end(&mut self) {
        assert_eq!(self.cond_nesting, 0, "Conditional nesting");
        if self.inner_loop.is_none() {
            // generate aggr output code only if no inner loop.
            // in inner loop aggr output precedes storing circuit output.
            self.gen_aggr_output_code();
        }
        if let Some(func_finish) = self.writer.config.func_finish {
            writeln!(self.writer.out, "    {}", func_finish).unwrap();
        }
        if self.inner_loop.is_some() {
            self.writer.out.extend(b"    } // loop\n");
        }
        self.writer.out.extend(b"}\n");
    }

    fn alloc_vars(&mut self, var_num: usize) {
        for i in 0..var_num {
            writeln!(
                self.writer.out,
                "    {} v{};",
                self.writer.calc_type_name(),
                i
            )
            .unwrap();
        }
        if self.writer.config.init_index.is_some() {
            self.writer.out.extend(b"    if (idx >= n) return;\n");
            if !self.pop_input_map.is_empty() || self.aggr_to_buffer {
                if let Some(arg_modifier) = self.writer.config.arg_modifier {
                    writeln!(
                        self.writer.out,
                        concat!(
                            "    buffer = (const {0} void*)",
                            "(((const {0} char*)buffer) + 4*buffer_shift);"
                        ),
                        arg_modifier
                    )
                    .unwrap();
                } else {
                    self.writer.out.extend(
                        b"    buffer = (const void*)(((const char*)buffer) + 4*buffer_shift);\n",
                    );
                }
            } else {
                if self.pop_input_code.is_some() && !self.single_buffer {
                    if let Some(arg_modifier) = self.writer.config.arg_modifier {
                        writeln!(
                            self.writer.out,
                            concat!(
                                "    input = (const {0} void*)(((const {0} char*)input) + ",
                                "4*input_shift);"
                            ),
                            arg_modifier
                        )
                        .unwrap();
                    } else {
                        self.writer.out.extend(
                            b"    input = (const void*)(((const char*)input) + 4*input_shift);\n",
                        );
                    }
                }
                if self.aggr_output_code.is_some() {
                    if let Some(arg_modifier) = self.writer.config.arg_modifier {
                        writeln!(
                            self.writer.out,
                            "    output = ({0} void*)((({0} char*)output) + 4*output_shift);",
                            arg_modifier
                        )
                        .unwrap();
                    } else {
                        self.writer
                            .out
                            .extend(b"    output = (void*)(((char*)output) + 4*output_shift);\n");
                    }
                }
            }
        }
        if let Some(init_code) = self.init_code {
            self.writer.out.extend(init_code.as_bytes());
            self.writer.out.push(b'\n');
        }
        if self.inner_loop.is_some() {
            self.writer
                .out
                .extend(b"    for (iter = 0; iter < iter_max && stop == 0; iter++) {\n");
        }
        if let Some(pop_input_code) = self.pop_input_code {
            let pop_inputs = if !self.pop_input_map.is_empty() {
                let mut map = self
                    .pop_input_map
                    .iter()
                    .map(|(x, y)| (*x, *y))
                    .collect::<Vec<_>>();
                // include original order of pop_inputs
                map.sort_by_key(|(_, order)| *order);
                map.into_iter()
                    .map(|(pop_input, _)| pop_input)
                    .collect::<Vec<_>>()
            } else {
                (0..self.input_len).collect::<Vec<_>>()
            };
            for i in &pop_inputs {
                if let Some(iv) = self.pop_input_map.get(&i) {
                    writeln!(self.writer.out, "#define i{} (v{})", i, iv).unwrap();
                } else if !self.input_map.is_empty() {
                    if let Some(iv) = self.input_map.get(&i) {
                        writeln!(self.writer.out, "#define i{} (v{})", i, iv).unwrap();
                    }
                } else {
                    writeln!(self.writer.out, "#define i{} (v{})", i, i).unwrap();
                }
            }
            self.writer.out.extend(pop_input_code.as_bytes());
            self.writer.out.push(b'\n');
            for i in &pop_inputs {
                if self.input_map.is_empty()
                    || self.input_map.contains_key(&i)
                    || self.pop_input_map.contains_key(&i)
                {
                    writeln!(self.writer.out, "#undef i{0}", i).unwrap();
                }
            }
        }
    }

    fn gen_load(&mut self, reg: usize, input: usize) {
        let aidx = 0; // array index
        if let Some(arg_bit) = self.arg_input_map.get(&input) {
            if *arg_bit < 32 {
                writeln!(
                    self.writer.out,
                    "    {} = ((arg & {}) != 0) ? one : zero;",
                    self.writer.format_reg(reg, aidx),
                    1u32 << arg_bit
                )
                .unwrap();
            } else {
                writeln!(
                    self.writer.out,
                    "    {} = ((arg2 & {}) != 0) ? one : zero;",
                    self.writer.format_reg(reg, aidx),
                    1u32 << (*arg_bit - 32)
                )
                .unwrap();
            }
        } else if let Some(elem_bit) = self.elem_input_map.get(&input) {
            if *elem_bit < (self.writer.elem_low_bits as usize) {
                writeln!(self.writer.out, "    v{} = elem_low_bit{};", reg, *elem_bit).unwrap();
            } else {
                let idx_value_postfix = if self.writer.array_len.is_some() {
                    format!("{}", aidx)
                } else {
                    String::new()
                };
                let ebit = *elem_bit - (self.writer.elem_low_bits as usize);
                if ebit < 32 {
                    writeln!(
                        self.writer.out,
                        "    {} = ((idxl{} & {}) != 0) ? one : zero;",
                        self.writer.format_reg(reg, aidx),
                        idx_value_postfix,
                        1u32 << ebit
                    )
                    .unwrap();
                } else {
                    writeln!(
                        self.writer.out,
                        "    {} = ((idxh{} & {}) != 0) ? one : zero;",
                        self.writer.format_reg(reg, aidx),
                        idx_value_postfix,
                        1u32 << (ebit - 32)
                    )
                    .unwrap();
                }
            }
        } else {
            let arg_name = if self.single_buffer {
                "output"
            } else {
                "input"
            };
            let input = if let Some(real_input) = self.input_map.get(&input) {
                self.input_placement
                    .map(|(p, _)| p[*real_input])
                    .unwrap_or(*real_input)
            } else if self.input_map.is_empty() {
                self.input_placement.map(|(p, _)| p[input]).unwrap_or(input)
            } else {
                panic!("Unexpected input in gen_load!");
            };
            let (dst, r) = if self.writer.config.init_index.is_some() {
                if self.writer.config.init_local_index.is_some() {
                    (
                        self.writer.format_reg(reg, aidx),
                        format!("{}[ivn + llen*{} + lidx]", arg_name, input),
                    )
                } else {
                    (
                        self.writer.format_reg(reg, aidx),
                        format!("{}[ivn + {}]", arg_name, input),
                    )
                }
            } else {
                (
                    self.writer.format_reg(reg, aidx),
                    format!("{}[{}]", arg_name, input),
                )
            };
            if let Some(ld_op) = self.writer.config.load_op {
                write!(self.writer.out, "    {} = ", dst).unwrap();
                CLangWriter::<'a>::write_op(&mut self.writer.out, ld_op, &[r.as_bytes()]);
                self.writer.out.extend(b";\n");
            } else {
                writeln!(self.writer.out, "    {} = {};", dst, r).unwrap();
            }
        }
    }

    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize) {
        let aidx = 0;
        let arg0 = self.writer.format_reg(arg0, aidx);
        let arg1 = self
            .writer
            .format_neg_arg(negs == VNegs::NegInput1, arg1, aidx);
        let mut op_vec = vec![];
        let args = [arg0.as_bytes(), arg1.as_bytes()];
        match op {
            InstrOp::And => {
                CLangWriter::<'a>::write_op(&mut op_vec, self.writer.config.and_op, &args)
            }
            InstrOp::Or => {
                CLangWriter::<'a>::write_op(&mut op_vec, self.writer.config.or_op, &args)
            }
            InstrOp::Xor => {
                CLangWriter::<'a>::write_op(&mut op_vec, self.writer.config.xor_op, &args)
            }
            InstrOp::Impl => {
                CLangWriter::<'a>::write_op(&mut op_vec, self.writer.config.impl_op.unwrap(), &args)
            }
            InstrOp::Nimpl => CLangWriter::<'a>::write_op(
                &mut op_vec,
                self.writer.config.nimpl_op.unwrap(),
                &args,
            ),
            _ => {
                panic!("This is not 2-argument operation");
            }
        };
        write!(
            self.writer.out,
            "    {} = ",
            self.writer.format_reg(dst_arg, aidx)
        )
        .unwrap();
        if negs == VNegs::NegOutput {
            CLangWriter::<'a>::write_neg(self.writer.config, &mut self.writer.out, &op_vec);
        } else {
            self.writer.out.extend(op_vec);
        }
        self.writer.out.extend(b";\n");
    }

    fn gen_op3(&mut self, op: InstrOp, dst_arg: usize, arg0: usize, arg1: usize, arg2: usize) {
        let aidx = 0;
        let arg0 = self.writer.format_reg(arg0, aidx);
        let arg1 = self.writer.format_reg(arg1, aidx);
        let arg2 = self.writer.format_reg(arg2, aidx);
        let mut op_vec = vec![];
        let args = [arg0.as_bytes(), arg1.as_bytes(), arg2.as_bytes()];
        match op {
            InstrOp::Lop3(comb) => {
                let comb_str = comb.to_string();
                let args = [args[0], args[1], args[2], comb_str.as_bytes()];
                CLangWriter::<'a>::write_op(
                    &mut op_vec,
                    self.writer.config.lop3_op.unwrap().1,
                    &args,
                )
            }
            _ => {
                panic!("This is not 3-argument operation");
            }
        };
        write!(
            self.writer.out,
            "    {} = ",
            self.writer.format_reg(dst_arg, aidx)
        )
        .unwrap();
        self.writer.out.extend(op_vec);
        self.writer.out.extend(b";\n");
    }

    fn gen_not(&mut self, dst_arg: usize, arg: usize) {
        let aidx = 0;
        write!(
            self.writer.out,
            "    {} = ",
            self.writer.format_reg(dst_arg, aidx)
        )
        .unwrap();
        let arg = self.writer.format_reg(arg, aidx);
        CLangWriter::write_neg(&self.writer.config, &mut self.writer.out, arg.as_bytes());
        self.writer.out.extend(b";\n");
    }

    fn gen_store(&mut self, neg: bool, output: usize, reg: usize) {
        let aidx = 0;
        let output = if let Some(real_output) = self.output_map.get(&output) {
            self.output_placement
                .map(|(p, _)| p[*real_output])
                .unwrap_or(*real_output)
        } else if self.output_map.is_empty() {
            self.output_placement
                .map(|(p, _)| p[output])
                .unwrap_or(output)
        } else {
            panic!("Unexpected output in gen_store!");
        };
        let arg = self.writer.format_neg_arg(neg, reg, aidx);
        let (dst, src) = if self.writer.config.init_index.is_some() {
            if self.writer.config.init_local_index.is_some() {
                (
                    format!("output[ovn + llen*{} + lidx]", output),
                    format!("{}", arg),
                )
            } else {
                (format!("output[ovn + {}]", output), format!("{}", arg))
            }
        } else {
            (format!("output[{}]", output), format!("{}", arg))
        };
        if let Some(st_op) = self.writer.config.store_op {
            self.writer.out.extend(b"    ");
            CLangWriter::<'a>::write_op(
                &mut self.writer.out,
                st_op,
                &[dst.as_bytes(), src.as_bytes()],
            );
            self.writer.out.extend(b";\n");
        } else {
            writeln!(self.writer.out, "    {} = {};", dst, src).unwrap();
        }
    }

    fn gen_set(&mut self, dst_arg: usize, arg: usize) {
        writeln!(self.writer.out, "    v{} = v{};", dst_arg, arg).unwrap();
    }

    fn gen_if_loop_start(&mut self) {
        assert!(self.inner_loop.is_some());
        self.writer.out.extend(b"    if (iter == 0) {\n");
        self.cond_nesting += 1;
    }
    fn gen_if_loop_end(&mut self) {
        assert!(self.inner_loop.is_some());
        self.writer
            .out
            .extend(b"    if (iter == iter_max - 1 || stop != 0) {\n");
        self.cond_nesting += 1;
    }
    fn gen_else(&mut self) {
        assert!(self.cond_nesting > 0);
        self.writer.out.extend(b"    } else {\n");
    }
    fn gen_end_if(&mut self) {
        assert!(self.cond_nesting > 0);
        self.writer.out.extend(b"    }\n");
        self.cond_nesting -= 1;
    }
}

impl<'a, 'c> CodeWriter<'c, CLangFuncWriter<'a, 'c>> for CLangWriter<'a> {
    fn supported_ops(&self) -> u64 {
        let basic_ops = (1u64 << InstrOp::And.int_value())
            | (1u64 << InstrOp::Or.int_value())
            | (1u64 << InstrOp::Xor.int_value());
        let basic_impl_ops = basic_ops | (1u64 << InstrOp::Impl.int_value());
        let basic_nimpl_ops = basic_ops | (1u64 << InstrOp::Nimpl.int_value());
        let op = if self.config.impl_op.is_some() {
            basic_impl_ops
        } else if self.config.nimpl_op.is_some() {
            basic_nimpl_ops
        } else {
            basic_ops
        };
        if self.config.lop3_op.is_some() {
            op | (1u64 << InstrOp::Lop3(0).int_value())
        } else {
            op
        }
    }
    fn word_len(&self) -> u32 {
        if let Some(alen) = self.array_len {
            u32::try_from(u64::from(self.config.type_bit_len) * u64::try_from(alen).unwrap())
                .unwrap()
        } else {
            self.config.type_bit_len
        }
    }
    fn max_var_num(&self) -> usize {
        usize::MAX
    }
    fn preferred_var_num(&self) -> usize {
        10000
    }
    fn prolog(&mut self) {
        if let Some(include_name) = self.config.include_name {
            writeln!(self.out, "#include <{}>", include_name).unwrap();
        }
        if let Some(include_name_2) = self.config.include_name_2 {
            writeln!(self.out, "#include <{}>", include_name_2).unwrap();
        }
        if let Some(include_name_3) = self.config.include_name_3 {
            writeln!(self.out, "#include <{}>", include_name_3).unwrap();
        }
        if !self.config.zero_value.0.is_empty() {
            self.out.extend(self.config.zero_value.0.as_bytes());
            self.out.push(b'\n');
        }
        if !self.config.one_value.0.is_empty() {
            self.out.extend(self.config.one_value.0.as_bytes());
            self.out.push(b'\n');
        }
        if !self.config.elem_index.low_bits_init.is_empty() {
            self.out
                .extend(self.config.elem_index.low_bits_init.as_bytes());
            self.out.push(b'\n');
        }
        if let Some(alen) = self.array_len {
            write!(
                self.out,
                "typedef struct _gate_sys_type {{\n    {} array[{}];\n}} gate_sys_type;\n",
                self.config.type_name, alen
            )
            .unwrap();
        }
        write!(
            self.out,
            "#define TYPE_LEN ({})\n#define TYPE_NAME {}\n",
            self.word_len(),
            self.calc_type_name()
        )
        .unwrap();
        let macro_internal_prefix = if self.array_len.is_some() {
            "__INT_"
        } else {
            ""
        };
        writeln!(
            self.out,
            "#define {}GET_U32(D,X,I) {}",
            macro_internal_prefix, self.config.get_u32_op
        )
        .unwrap();
        writeln!(
            self.out,
            "#define {}GET_U32_ALL(D,X) {}",
            macro_internal_prefix, self.config.get_u32_all_op
        )
        .unwrap();
        writeln!(
            self.out,
            "#define {}SET_U32(X,S,I) {}",
            macro_internal_prefix, self.config.set_u32_op
        )
        .unwrap();
        writeln!(
            self.out,
            "#define {}SET_U32_ALL(X,S) {}",
            macro_internal_prefix, self.config.set_u32_all_op
        )
        .unwrap();

        if let Some(alen) = self.array_len {
            let int_type_word_len = self.config.type_bit_len >> 5;
            // real macro definition for array_len
            write!(
                self.out,
                r##"#define GET_U32(D,X,I) \
    __INT_GET_U32((D).array[(I) / {0}], (X).array[(I) / {0}], (I) % {0})
"##,
                int_type_word_len
            )
            .unwrap();

            // GET_ALL_U32
            self.out.extend(b"#define GET_ALL_U32(D,X) {\n");
            for i in 0..alen {
                writeln!(
                    self.out,
                    "    __INT_GET_U32_ALL((D).array[{0}], (X).array[{0}]);",
                    i
                )
                .unwrap();
            }
            self.out.extend(b"}\n");

            write!(
                self.out,
                r##"#define SET_U32(X,S,I) \
    __INT_SET_U32((X).array[(I) / {0}], (S).array[(I) / {0}], (I) % {0})
"##,
                int_type_word_len
            )
            .unwrap();

            // SET_ALL_U32
            self.out.extend(b"#define SET_ALL_U32(X,S) {\n");
            for i in 0..alen {
                writeln!(
                    self.out,
                    "    __INT_SET_U32_ALL((X).array[{0}], (S).array[{0}]);",
                    i
                )
                .unwrap();
            }
            self.out.extend(b"}\n");
        }

        if let Some((lop3_def, _)) = self.config.lop3_op {
            self.out.extend(lop3_def.as_bytes());
            self.out.extend(b"\n");
        }
    }

    fn user_defs(&mut self, user_defs: &str) {
        self.out.extend(user_defs.as_bytes());
        self.out.push(b'\n');
    }

    fn transform_helpers(&mut self) {
        if !self.transform_helpers_added {
            let alen_prefix = if self.array_len.is_some() {
                "__INT_"
            } else {
                ""
            };
            let mut transform = self.config.transform_config.transform();
            transform.init_defs();
            for i in 1..=32 {
                transform.gen_input_transform_with_prefix(i, alen_prefix);
                transform.gen_output_transform_with_prefix(i, alen_prefix);
            }
            self.user_defs(&transform.out());
            if let Some(alen) = self.array_len {
                // real macros for output transform
                use std::fmt::Write;
                let mut real_macros = String::new();
                for bits in 1..=32 {
                    writeln!(
                        real_macros,
                        r##"#define INPUT_TRANSFORM_B{0}({1}, {2}) {{ \
    unsigned int i; \
    for (i = 0; i < {4}; i++) {{ \
        __INT_INPUT_TRANSFORM_B{0}({3}, ({2}).array[i]) \
    }} \
}}
"##,
                        bits,
                        &((0..bits).map(|i| format!("D{}", i)).collect::<Vec<_>>()).join(", "),
                        "S",
                        &((0..bits)
                            .map(|i| format!("(D{}).array[i]", i))
                            .collect::<Vec<_>>())
                        .join(", "),
                        alen
                    )
                    .unwrap();
                    writeln!(
                        real_macros,
                        r##"#define OUTPUT_TRANSFORM_B{0}({1}, {2}) {{ \
    unsigned int i; \
    for (i = 0; i < {4}; i++) {{ \
        __INT_OUTPUT_TRANSFORM_B{0}(({1}).array[i], ({3})) \
    }} \
}}
"##,
                        bits,
                        "D",
                        &((0..bits).map(|i| format!("S{}", i)).collect::<Vec<_>>()).join(", "),
                        &((0..bits)
                            .map(|i| format!("(S{}).array[i]", i))
                            .collect::<Vec<_>>())
                        .join(", "),
                        alen
                    )
                    .unwrap();
                }
                self.user_defs(&real_macros);
            }
        }
    }

    fn epilog(&mut self) {}

    unsafe fn func_writer_internal(
        &'c mut self,
        name: &'c str,
        input_len: usize,
        output_len: usize,
        code_config: CodeConfig<'c>,
        output_vars: Option<Vec<(usize, usize)>>,
    ) -> CLangFuncWriter<'a, 'c> {
        if let Some(elem_inputs) = code_config.elem_inputs {
            assert!(elem_inputs.len() <= 64 + (self.elem_low_bits as usize));
        }

        let (input_map, arg_input_map, elem_input_map, pop_input_map) = {
            let arg_input_map = if let Some(arg_inputs) = code_config.arg_inputs {
                HashMap::from_iter(arg_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            };
            let elem_input_map = if let Some(elem_inputs) = code_config.elem_inputs {
                HashMap::from_iter(elem_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            };
            let pop_input_map = if code_config.pop_input_code.is_some() {
                if let Some(pop_inputs) = code_config.pop_from_buffer {
                    if code_config.inner_loop.is_some() {
                        let mut pop_input_map = HashMap::new();
                        let mut count = 0;
                        for i in 0..input_len {
                            if !arg_input_map.contains_key(&i) && !elem_input_map.contains_key(&i) {
                                if pop_inputs.iter().any(|x| *x == i) {
                                    pop_input_map.insert(i, count);
                                }
                                count += 1;
                            }
                        }
                        pop_input_map
                    } else {
                        HashMap::from_iter(pop_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            };
            let mut input_map = HashMap::new();
            if !arg_input_map.is_empty() || !elem_input_map.is_empty() || !pop_input_map.is_empty()
            {
                let mut count = 0;
                for i in 0..input_len {
                    if !arg_input_map.contains_key(&i)
                        && !elem_input_map.contains_key(&i)
                        && !pop_input_map.contains_key(&i)
                    {
                        input_map.insert(i, count);
                        count += 1;
                    }
                }
            }
            (input_map, arg_input_map, elem_input_map, pop_input_map)
        };
        let output_map = if let Some(excl_outputs) = code_config.exclude_outputs {
            let mut output_map = HashMap::new();
            let excl_set = HashSet::<usize>::from_iter(excl_outputs.iter().copied());
            let mut count = 0;
            for i in 0..output_len {
                if !excl_set.contains(&i) {
                    output_map.insert(i, count);
                    count += 1;
                }
            }
            output_map
        } else {
            HashMap::new()
        };

        let aggr_to_buffer =
            code_config.aggr_output_code.is_some() && code_config.aggr_to_buffer.is_some();
        CLangFuncWriter::<'a, 'c> {
            writer: self,
            name,
            input_len,
            output_len,
            input_placement: code_config.input_placement,
            output_placement: code_config.output_placement,
            input_map,
            output_map,
            arg_input_map,
            elem_input_map,
            pop_input_map,
            single_buffer: code_config.single_buffer,
            init_code: code_config.init_code,
            pop_input_code: code_config.pop_input_code,
            aggr_output_code: code_config.aggr_output_code,
            output_vars: if code_config.inner_loop.is_some() && aggr_to_buffer {
                Some(
                    code_config
                        .aggr_to_buffer
                        .unwrap()
                        .iter()
                        .map(|v| {
                            let output_vars = output_vars.as_ref().unwrap();
                            *output_vars.iter().find(|x| x.0 == *v).unwrap()
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                output_vars
            },
            aggr_to_buffer,
            inner_loop: code_config.inner_loop,
            cond_nesting: 0,
        }
    }

    fn out(self) -> Vec<u8> {
        self.out
    }
}

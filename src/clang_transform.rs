use crate::utils::*;

use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct FinalType<'a> {
    final_type_name: &'a str,
    final_type_bit_len: u32,
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct CLangTransformConfig<'a> {
    comp_type_name: &'a str,
    comp_type_bit_len: u32,
    // if final_type is not supplied then final type is comp_type.
    final_type: Option<FinalType<'a>>,
    // all operations defined for comp_type
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
    and_op: &'a str,
    or_op: &'a str,
    // shift_op: index - 2*i, value - operation of shift left for 2^(i+1) bit elements vector.
    // shift_op: index - 2*i+1, value - operation of shift right for 2^(i+1) bit elements vector.
    shift_op: [Option<&'a str>; 10 * 2],
    unpack_ops: [Option<&'a str>; 10 * 2],
    init_defs: &'a str,
    zero: &'a str,
    // masks for transposition operations (unpackings)
    constant_defs: [&'a str; 2 * 10],
    // masks for first 2^n bits
    constant2_defs: [&'a str; 5],
    collect_constants: bool,
}

pub const CLANG_TRANSFORM_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "uint32_t",
    comp_type_bit_len: 32,
    final_type: None,
    load_op: None,
    store_op: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("({} << {})"),
        Some("({} >> {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None,
    ],
    init_defs: "",
    zero: "0U",
    constant_defs: [
        "0x55555555U",
        "0xaaaaaaaaU",
        "0x33333333U",
        "0xccccccccU",
        "0x0f0f0f0fU",
        "0xf0f0f0f0U",
        "0x00ff00ffU",
        "0xff00ff00U",
        "0x0000ffffU",
        "0xffff0000U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
    ],
    constant2_defs: ["0x1U", "0x3U", "0xfU", "0xffU", "0xffffU"],
    collect_constants: false,
};

pub const CLANG_TRANSFORM_U64: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "uint64_t",
    comp_type_bit_len: 64,
    final_type: None,
    load_op: Some("(*((const uint64_t*)({})))"),
    store_op: Some("*((uint64_t*)({})) = {}"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("({} << {})"),
        Some("({} >> {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None,
    ],
    init_defs: "",
    zero: "0ULL",
    constant_defs: [
        "0x5555555555555555ULL",
        "0xaaaaaaaaaaaaaaaaULL",
        "0x3333333333333333ULL",
        "0xccccccccccccccccULL",
        "0x0f0f0f0f0f0f0f0fULL",
        "0xf0f0f0f0f0f0f0f0ULL",
        "0x00ff00ff00ff00ffULL",
        "0xff00ff00ff00ff00ULL",
        "0x0000ffff0000ffffULL",
        "0xffff0000ffff0000ULL",
        "0x00000000ffffffffULL",
        "0xffffffff00000000ULL",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
        "0U",
    ],
    constant2_defs: [
        "0x0000000100000001ULL",
        "0x0000000300000003ULL",
        "0x0000000f0000000fULL",
        "0x000000ff000000ffULL",
        "0x0000ffff0000ffffULL",
    ],
    collect_constants: false,
};

pub const CLANG_TRANSFORM_INTEL_MMX: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "__m64",
    comp_type_bit_len: 64,
    final_type: None,
    load_op: Some("(*((const __m64*)({})))"),
    store_op: Some("*((__m64*)({})) = {}"),
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_m_psllwi({}, {})"),
        Some("_m_psrlwi({}, {})"),
        Some("_m_pslldi({}, {})"),
        Some("_m_psrldi({}, {})"),
        Some("_m_psllqi({}, {})"),
        Some("_m_psrlqi({}, {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_m_punpckldq({}, {})"),
        Some("_m_punpckhdq({}, {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[6*2*2] = {
    0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U,
    0xffffffffU, 0x00000000U,
    0x00000000U, 0xffffffffU
};
static const unsigned int transform_const2_tbl[5*2] = {
    0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU
};
"##,
    zero: "_mm_setzero_si64()",
    constant_defs: [
        "(*(const __m64*)(transform_const_tbl + 2*0))",
        "(*(const __m64*)(transform_const_tbl + 2*1))",
        "(*(const __m64*)(transform_const_tbl + 2*2))",
        "(*(const __m64*)(transform_const_tbl + 2*3))",
        "(*(const __m64*)(transform_const_tbl + 2*4))",
        "(*(const __m64*)(transform_const_tbl + 2*5))",
        "(*(const __m64*)(transform_const_tbl + 2*6))",
        "(*(const __m64*)(transform_const_tbl + 2*7))",
        "(*(const __m64*)(transform_const_tbl + 2*8))",
        "(*(const __m64*)(transform_const_tbl + 2*9))",
        "(*(const __m64*)(transform_const_tbl + 2*10))",
        "(*(const __m64*)(transform_const_tbl + 2*11))",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    constant2_defs: [
        "(*(const __m64*)(transform_const2_tbl + 2*0))",
        "(*(const __m64*)(transform_const2_tbl + 2*1))",
        "(*(const __m64*)(transform_const2_tbl + 2*2))",
        "(*(const __m64*)(transform_const2_tbl + 2*3))",
        "(*(const __m64*)(transform_const2_tbl + 2*4))",
    ],
    collect_constants: true,
};

pub const CLANG_TRANSFORM_INTEL_SSE2: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "__m128i",
    comp_type_bit_len: 128,
    final_type: None,
    load_op: Some("_mm_loadu_si128((const __m128i*){})"),
    store_op: Some("_mm_storeu_si128((__m128i*){}, {})"),
    and_op: "_mm_and_si128({}, {})",
    or_op: "_mm_or_si128({}, {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm_slli_epi16({}, {})"),
        Some("_mm_srli_epi16({}, {})"),
        Some("_mm_slli_epi32({}, {})"),
        Some("_mm_srli_epi32({}, {})"),
        Some("_mm_slli_epi64({}, {})"),
        Some("_mm_srli_epi64({}, {})"),
        Some("_mm_slli_si128({}, ({})>>3)"),
        Some("_mm_srli_si128({}, ({})>>3)"),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm_unpacklo_epi64({}, {})"),
        Some("_mm_unpackhi_epi64({}, {})"),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[7*2*4]
__attribute__((aligned(16))) = {
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
};
static const unsigned int transform_const2_tbl[5*4]
__attribute__((aligned(16))) = {
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
};
"##,
    zero: "_mm_setzero_si128()",
    constant_defs: [
        "(*(const __m128i*)(transform_const_tbl + 4*0))",
        "(*(const __m128i*)(transform_const_tbl + 4*1))",
        "(*(const __m128i*)(transform_const_tbl + 4*2))",
        "(*(const __m128i*)(transform_const_tbl + 4*3))",
        "(*(const __m128i*)(transform_const_tbl + 4*4))",
        "(*(const __m128i*)(transform_const_tbl + 4*5))",
        "(*(const __m128i*)(transform_const_tbl + 4*6))",
        "(*(const __m128i*)(transform_const_tbl + 4*7))",
        "(*(const __m128i*)(transform_const_tbl + 4*8))",
        "(*(const __m128i*)(transform_const_tbl + 4*9))",
        "(*(const __m128i*)(transform_const_tbl + 4*10))",
        "(*(const __m128i*)(transform_const_tbl + 4*11))",
        "(*(const __m128i*)(transform_const_tbl + 4*12))",
        "(*(const __m128i*)(transform_const_tbl + 4*13))",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    constant2_defs: [
        "(*(const __m128i*)(transform_const2_tbl + 4*0))",
        "(*(const __m128i*)(transform_const2_tbl + 4*1))",
        "(*(const __m128i*)(transform_const2_tbl + 4*2))",
        "(*(const __m128i*)(transform_const2_tbl + 4*3))",
        "(*(const __m128i*)(transform_const2_tbl + 4*4))",
    ],
    collect_constants: true,
};

pub const CLANG_TRANSFORM_INTEL_AVX2: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "__m256i",
    comp_type_bit_len: 256,
    final_type: None,
    load_op: Some("_mm256_loadu_si256((const __m256i*){})"),
    store_op: Some("_mm256_storeu_si256((__m256i*){}, {})"),
    and_op: "_mm256_and_si256({}, {})",
    or_op: "_mm256_or_si256({}, {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm256_slli_epi16({}, {})"),
        Some("_mm256_srli_epi16({}, {})"),
        Some("_mm256_slli_epi32({}, {})"),
        Some("_mm256_srli_epi32({}, {})"),
        Some("_mm256_slli_epi64({}, {})"),
        Some("_mm256_srli_epi64({}, {})"),
        Some("_mm256_slli_si256({}, ({})>>3)"),
        Some("_mm256_srli_si256({}, ({})>>3)"),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm256_blend_epi16({}, _mm256_slli_epi32({}, 16), 0xaa)"),
        Some("_mm256_blend_epi16(_mm256_srli_epi32({}, 16), {}, 0xaa)"),
        Some("_mm256_blend_epi32({}, _mm256_slli_epi64({}, 32), 0xaa)"),
        Some("_mm256_blend_epi32(_mm256_srli_epi64({}, 32), {}, 0xaa)"),
        None,
        None,
        Some("_mm256_permute2x128_si256({}, {}, 0x20)"),
        Some("_mm256_permute2x128_si256({}, {}, 0x31)"),
        None,
        None,
        None,
        None,
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[8*2*8]
__attribute__((aligned(32))) = {
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU
};
static const unsigned int transform_const2_tbl[5*8]
__attribute__((aligned(32))) = {
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU
};
"##,
    zero: "_mm256_setzero_si256()",
    constant_defs: [
        "(*(const __m256i*)(transform_const_tbl + 8*0))",
        "(*(const __m256i*)(transform_const_tbl + 8*1))",
        "(*(const __m256i*)(transform_const_tbl + 8*2))",
        "(*(const __m256i*)(transform_const_tbl + 8*3))",
        "(*(const __m256i*)(transform_const_tbl + 8*4))",
        "(*(const __m256i*)(transform_const_tbl + 8*5))",
        "(*(const __m256i*)(transform_const_tbl + 8*6))",
        "(*(const __m256i*)(transform_const_tbl + 8*7))",
        "(*(const __m256i*)(transform_const_tbl + 8*8))",
        "(*(const __m256i*)(transform_const_tbl + 8*9))",
        "(*(const __m256i*)(transform_const_tbl + 8*10))",
        "(*(const __m256i*)(transform_const_tbl + 8*11))",
        "(*(const __m256i*)(transform_const_tbl + 8*12))",
        "(*(const __m256i*)(transform_const_tbl + 8*13))",
        "(*(const __m256i*)(transform_const_tbl + 8*14))",
        "(*(const __m256i*)(transform_const_tbl + 8*15))",
        "",
        "",
        "",
        "",
    ],
    constant2_defs: [
        "(*(const __m256i*)(transform_const2_tbl + 8*0))",
        "(*(const __m256i*)(transform_const2_tbl + 8*1))",
        "(*(const __m256i*)(transform_const2_tbl + 8*2))",
        "(*(const __m256i*)(transform_const2_tbl + 8*3))",
        "(*(const __m256i*)(transform_const2_tbl + 8*4))",
    ],
    collect_constants: true,
};

pub const CLANG_TRANSFORM_INTEL_AVX512: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "__m512i",
    comp_type_bit_len: 512,
    final_type: None,
    load_op: Some("_mm512_loadu_epi64({})"),
    store_op: Some("_mm512_storeu_epi64({}, {})"),
    and_op: "_mm512_and_epi64({}, {})",
    or_op: "_mm512_or_epi64({}, {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm512_slli_epi32({}, {})"),
        Some("_mm512_srli_epi32({}, {})"),
        Some("_mm512_slli_epi64({}, {})"),
        Some("_mm512_srli_epi64({}, {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm512_permutex2var_epi32({}, (*(const __m512i*)(transform_const4_tbl + 0)), {}))"),
        Some("_mm512_permutex2var_epi32({}, (*(const __m512i*)(transform_const4_tbl + 16)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*0)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*1)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*2)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*3)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*4)), {}))"),
        Some("_mm512_permutex2var_epi64({}, (*(const __m512i*)(transform_const3_tbl + 8*5)), {}))"),
        None,
        None,
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[8*2*16]
__attribute__((aligned(64))) = {
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU,
    0x00000000U, 0x00000000U, 0x00000000U, 0x00000000U,
    0xffffffffU, 0xffffffffU, 0xffffffffU, 0xffffffffU
};
static const unsigned int transform_const2_tbl[5*16]
__attribute__((aligned(64))) = {
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU
};
static const uint64_t transform_const3_tbl[3*16]
__attribute__((aligned(64))) = {
    0, 8, 2, 10, 4, 12, 6, 14, 1, 9, 3, 11, 5, 13, 7, 15,
    0, 1, 8, 9, 4, 5, 12, 13, 2, 3, 10, 11, 6, 7, 14, 15,
    0, 1, 2, 3, 8, 9, 10, 11, 4, 5, 6, 7, 12, 13, 14, 15
};
static const uint32_t transform_const4_tbl[32]
__attribute__((aligned(64))) = {
    0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30,
    1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31
};
"##,
    zero: "_mm512_setzero_si512()",
    constant_defs: [
        "(*(const __m512i*)(transform_const_tbl + 16*0))",
        "(*(const __m512i*)(transform_const_tbl + 16*1))",
        "(*(const __m512i*)(transform_const_tbl + 16*2))",
        "(*(const __m512i*)(transform_const_tbl + 16*3))",
        "(*(const __m512i*)(transform_const_tbl + 16*4))",
        "(*(const __m512i*)(transform_const_tbl + 16*5))",
        "(*(const __m512i*)(transform_const_tbl + 16*6))",
        "(*(const __m512i*)(transform_const_tbl + 16*7))",
        "(*(const __m512i*)(transform_const_tbl + 16*8))",
        "(*(const __m512i*)(transform_const_tbl + 16*9))",
        "(*(const __m512i*)(transform_const_tbl + 16*10))",
        "(*(const __m512i*)(transform_const_tbl + 16*11))",
        "(*(const __m512i*)(transform_const_tbl + 16*12))",
        "(*(const __m512i*)(transform_const_tbl + 16*13))",
        "(*(const __m512i*)(transform_const_tbl + 16*14))",
        "(*(const __m512i*)(transform_const_tbl + 16*15))",
        "",
        "",
        "",
        "",
    ],
    constant2_defs: [
        "(*(const __m512i*)(transform_const2_tbl + 16*0))",
        "(*(const __m512i*)(transform_const2_tbl + 16*1))",
        "(*(const __m512i*)(transform_const2_tbl + 16*2))",
        "(*(const __m512i*)(transform_const2_tbl + 16*3))",
        "(*(const __m512i*)(transform_const2_tbl + 16*4))",
    ],
    collect_constants: true,
};

pub const CLANG_TRANSFORM_ARM_NEON: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "uint32x4_t",
    comp_type_bit_len: 128,
    final_type: None,
    load_op: None,
    store_op: None,
    and_op: "vandq_u32({}, {})",
    or_op: "vorrq_u32({}, {})",
    shift_op: [
        None,
        None,
        None,
        None,
        Some("vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32({}), {}))"),
        Some("vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32({}), {}))"),
        Some("vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32({}), {}))"),
        Some("vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32({}), {}))"),
        Some("vshlq_n_u32({}, {})"),
        Some("vshrq_n_u32({}, {})"),
        Some("vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32({}), {}))"),
        Some("vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32({}), {}))"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(
            r##"vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32({}), vreinterpretq_u64_u32({})))"##,
        ),
        Some(
            r##"vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32({}), vreinterpretq_u64_u32({})))"##,
        ),
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    init_defs: "",
    zero: "{ 0U, 0U, 0U, 0U }",
    constant_defs: [
        "{ 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }",
        "{ 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }",
        "{ 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }",
        "{ 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }",
        "{ 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }",
        "{ 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }",
        "{ 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }",
        "{ 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }",
        "{ 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }",
        "{ 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }",
        "{ 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }",
        "{ 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }",
        "{ 0xffffffffU, 0xffffffffU, 0x00000000U, 0x00000000U }",
        "{ 0x00000000U, 0x00000000U, 0xffffffffU, 0xffffffffU }",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    constant2_defs: [
        "{ 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }",
        "{ 0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U }",
        "{ 0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU }",
        "{ 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }",
        "{ 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }",
    ],
    collect_constants: false,
};

pub const CLANG_TRANSFORM_OPENCL_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    comp_type_name: "uint",
    comp_type_bit_len: 32,
    final_type: None,
    load_op: None,
    store_op: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shift_op: [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("({} << {})"),
        Some("({} >> {})"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    unpack_ops: [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None,
    ],
    init_defs: "",
    zero: "0U",
    constant_defs: [
        "0x55555555U",
        "0xaaaaaaaaU",
        "0x33333333U",
        "0xccccccccU",
        "0x0f0f0f0fU",
        "0xf0f0f0f0U",
        "0x00ff00ffU",
        "0xff00ff00U",
        "0x0000ffffU",
        "0xffff0000U",
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
    constant2_defs: ["0x1U", "0x3U", "0xfU", "0xffU", "0xffffU"],
    collect_constants: false,
};

impl<'a> CLangTransformConfig<'a> {
    pub fn transform(&'a self) -> CLangTransform<'a> {
        CLangTransform {
            config: self,
            out: String::new(),
        }
    }
}

pub struct CLangTransform<'a> {
    config: &'a CLangTransformConfig<'a>,
    out: String,
}

const INPUT_TYPE: usize = usize::MAX;
const OUTPUT_TYPE: usize = usize::MAX - 1;

struct CLangMacroVars {
    var_types: Vec<String>,
    mvartool: MultiVarAllocTool<usize>,
    constants: BTreeMap<String, String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
    out: String,
    collect_constants: bool,
}

impl CLangMacroVars {
    fn new<'a>(
        var_types: impl IntoIterator<Item = &'a str>,
        inputs: impl IntoIterator<Item = String>,
        outputs: impl IntoIterator<Item = String>,
        collect_constants: bool,
    ) -> Self {
        let var_types = var_types
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let var_type_num = var_types.len();
        assert!(var_type_num < OUTPUT_TYPE);
        Self {
            var_types,
            mvartool: MultiVarAllocTool::new(var_type_num),
            constants: BTreeMap::new(),
            inputs: inputs.into_iter().collect::<Vec<_>>(),
            outputs: outputs.into_iter().collect::<Vec<_>>(),
            out: String::new(),
            collect_constants,
        }
    }

    // go to next pass
    fn set_usage_mode(&mut self) {
        self.mvartool.set_usage_mode();
    }

    fn write_var_defs(&mut self, out: &mut String) {
        for i in 0..self.var_types.len() {
            for j in 0..self.mvartool.alloc_var_num(i) {
                writeln!(out, "    {} t{}v{};\\", self.var_types[i], i, j).unwrap();
            }
        }
    }

    fn new_var(&mut self, var_type: usize) -> usize {
        self.mvartool.new_var(var_type)
    }

    fn use_var(&mut self, var_type: usize, v: usize) {
        self.mvartool.use_var(var_type, v);
    }

    fn format_var(&self, var_type: usize, v: usize) -> String {
        if var_type == INPUT_TYPE {
            self.inputs[v].clone()
        } else if var_type == OUTPUT_TYPE {
            self.outputs[v].clone()
        } else {
            format!("t{}v{}", var_type, v)
        }
    }

    fn write_constant_defs(&mut self, out: &mut String) {
        for (v, c) in &self.constants {
            writeln!(out, "    const {} {} = {};\\", self.var_types[0], c, v).unwrap();
        }
    }

    fn get_constant<'a>(&'a mut self, constant: &'a str) -> &'a str {
        if self.mvartool.usage_mode() {
            if self.collect_constants {
                &self.constants[constant]
            } else {
                constant
            }
        } else {
            if self.collect_constants {
                if !self.constants.contains_key(&constant.to_string()) {
                    let c1 = format!("c{}", self.constants.len());
                    self.constants.insert(constant.to_string(), c1);
                }
            }
            constant
        }
    }
}

impl Write for CLangMacroVars {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        if self.mvartool.usage_mode() {
            self.out.write_str(s)
        } else {
            Ok(())
        }
    }
}

const BIT_MASK_TBL: [u32; 5 * 2] = [
    0x55555555, 0xaaaaaaaa, 0x33333333, 0xcccccccc, 0x0f0f0f0f, 0xf0f0f0f0, 0x00ff00ff, 0xff00ff00,
    0x0000ffff, 0xffff0000,
];

const SINGLE_BIT_MASK_TBL: [u32; 5] = [0x1, 0x3, 0xf, 0xff, 0xffff];

impl<'a> CLangTransform<'a> {
    fn write_op(out: &mut String, op: &str, args: &[&str]) {
        let mut rest = op;
        let mut arg_index = 0;
        while let Some(p) = rest.find('{') {
            out.extend(rest[..p].chars());
            rest = &rest[p + 1..];
            if let Some(endr) = rest.find('}') {
                if rest[..endr].is_empty() {
                    // fetch next argument
                    out.extend(args[arg_index].chars());
                    arg_index += 1;
                } else {
                    // fetch argument with index given between {}
                    let index = usize::from_str_radix(&rest[..endr], 10).unwrap();
                    out.extend(args[index].chars());
                }
                rest = &rest[endr + 1..];
            } else {
                panic!("Unexpected");
            }
        }
        if !rest.is_empty() {
            out.extend(rest.chars());
        }
    }

    fn format_op(op: &str, args: &[&str]) -> String {
        let mut out = String::new();
        Self::write_op(&mut out, op, args);
        out
    }

    pub fn format_arg_s(arg: u32) -> String {
        format!("((S)[{}])", arg)
    }
    pub fn format_arg_d(arg: u32) -> String {
        format!("(D{})", arg)
    }
    pub fn format_load_input(&self, arg: u32) -> String {
        if let Some(load_op) = self.config.load_op {
            Self::format_op(load_op, &[&format!("((S) + {})", arg)])
        } else {
            Self::format_arg_s(arg)
        }
    }

    fn gen_unpack_low(
        &mut self,
        mvars: &mut CLangMacroVars,
        i: usize,
        bit_usage_f: bool,
        bit_usage_s: bool,
        t0: &str,
        t1: &str,
    ) -> String {
        if let Some(unpack) = self.config.unpack_ops[2 * i] {
            if bit_usage_f {
                if bit_usage_s {
                    Self::format_op(unpack, &[t0, t1])
                } else {
                    Self::format_op(unpack, &[t0, &self.config.zero])
                }
            } else {
                if bit_usage_s {
                    Self::format_op(unpack, &[&self.config.zero, t1])
                } else {
                    String::new()
                }
            }
        } else {
            let (shl, failed) = {
                let (shl, idx) = (i..10)
                    .filter_map(|x| self.config.shift_op[2 * x].map(|s| (s, x)))
                    .next()
                    .unwrap();
                (shl, idx != i)
            };
            // normal expression (bitwise logic and shifts)
            let p0 = if bit_usage_f {
                Self::format_op(
                    self.config.and_op,
                    &[t0, mvars.get_constant(self.config.constant_defs[2 * i])],
                )
            } else {
                String::new()
            };
            let p1 = if bit_usage_s {
                let p1 = if failed {
                    Self::format_op(
                        self.config.and_op,
                        &[t1, mvars.get_constant(self.config.constant_defs[2 * i])],
                    )
                } else {
                    t1.to_string()
                };
                Self::format_op(shl, &[&p1, &(1 << i).to_string()])
            } else {
                String::new()
            };
            if !p0.is_empty() {
                if !p1.is_empty() {
                    Self::format_op(self.config.or_op, &[&p0, &p1])
                } else {
                    p0
                }
            } else {
                p1
            }
        }
    }

    fn gen_unpack_high(
        &mut self,
        mvars: &mut CLangMacroVars,
        i: usize,
        bit_usage_f: bool,
        bit_usage_s: bool,
        t0: &str,
        t1: &str,
    ) -> String {
        if let Some(unpack) = self.config.unpack_ops[2 * i + 1] {
            if bit_usage_f {
                if bit_usage_s {
                    Self::format_op(unpack, &[t0, &t1])
                } else {
                    Self::format_op(unpack, &[t0, &self.config.zero])
                }
            } else {
                if bit_usage_s {
                    Self::format_op(unpack, &[&self.config.zero, t1])
                } else {
                    String::new()
                }
            }
        } else {
            let (shr, failed) = {
                let (shr, idx) = (i..10)
                    .filter_map(|x| self.config.shift_op[2 * x + 1].map(|s| (s, x)))
                    .next()
                    .unwrap();
                (shr, idx != i)
            };
            let p0 = if bit_usage_f {
                let p0 = if failed {
                    Self::format_op(
                        self.config.and_op,
                        &[t0, mvars.get_constant(self.config.constant_defs[2 * i + 1])],
                    )
                } else {
                    t0.to_string()
                };
                Self::format_op(shr, &[&p0, &(1 << i).to_string()])
            } else {
                String::new()
            };
            let p1 = if bit_usage_s {
                Self::format_op(
                    self.config.and_op,
                    &[t1, mvars.get_constant(self.config.constant_defs[2 * i + 1])],
                )
            } else {
                String::new()
            };
            if !p0.is_empty() {
                if !p1.is_empty() {
                    Self::format_op(self.config.or_op, &[&p0, &p1])
                } else {
                    p0
                }
            } else {
                p1
            }
        }
    }

    fn gen_input_transform_int(&mut self, mvars: &mut CLangMacroVars, bits: usize) {
        let bits_log = calc_log_bits(bits);
        let mut prev_type = INPUT_TYPE;
        let mut prev_pass = (0..32).collect::<Vec<_>>();
        if self.config.comp_type_bit_len > 32 {
            // use unpacking to transpose 32-bit words from sequential to form:
            // { TBL[0],TBL[32],TBL[64],TBL[96],TBL[1],TBL[33],TBL[65],TBL[97],...
            let type_len_log = calc_log_bits(self.config.comp_type_bit_len as usize);
            for i in (0..type_len_log - 5).rev() {
                let mut new_pass = vec![0; 32];
                for j in 0..16 {
                    let fj = j & ((1 << 4) - 1);
                    let sj = fj | (1 << 4);
                    let t0 = mvars.format_var(prev_type, prev_pass[fj]);
                    let t1 = mvars.format_var(prev_type, prev_pass[sj]);
                    if prev_type < OUTPUT_TYPE {
                        mvars.use_var(prev_type, prev_pass[fj]);
                        mvars.use_var(prev_type, prev_pass[sj]);
                    }
                    let (nt, ns0) = (0, mvars.new_var(0));
                    let i = i + 5;
                    let expr = self.gen_unpack_low(mvars, i, true, true, &t0, &t1);
                    write!(mvars, "    {} = ", mvars.format_var(nt, ns0)).unwrap();
                    writeln!(mvars, "{};\\", expr).unwrap();
                    new_pass[2 * j] = ns0;
                    if prev_type < OUTPUT_TYPE {
                        mvars.use_var(prev_type, prev_pass[fj]);
                        mvars.use_var(prev_type, prev_pass[sj]);
                    }
                    let (nt, ns1) = (0, mvars.new_var(0));
                    let expr = self.gen_unpack_high(mvars, i, true, true, &t0, &t1);
                    write!(mvars, "    {} = ", mvars.format_var(nt, ns1)).unwrap();
                    writeln!(mvars, "{};\\", expr).unwrap();
                    new_pass[2 * j + 1] = ns1;
                }
                prev_pass = new_pass;
                prev_type = 0;
            }
        }
        let mut bit_usage = vec![u32::try_from((1usize << bits) - 1).unwrap(); 32];
        if bits_log < 5 {
            // if bits < 16 then just join lower bits once:
            // example: { [TBL[0][0:3],TBL[8][0:3],TBL[16][0:3],TBL[24][0:3]],
            //            [TBL[0][4:7],TBL[8][4:7],TBL[16][4:7],TBL[24][4:7]],
            //            [TBL[0][8:11],TBL[8][8:11],TBL[16][8:11],TBL[24][8:11]],
            //            [TBL[0][12:15],TBL[8][12:15],TBL[16][12:15],TBL[24][12:15]], ... }
            // where [A,B,....] - number with joined bits A,B,... .
            //       TBL[x][a:b] - bits from a to b from value from table under index x.
            // After that we have operates on n-bits (where n is power of two) inside
            // 32-bit word.
            for i in 0..1 << bits_log {
                let mut final_expr = String::new();
                let mut bit_usg = 0;
                for j in 0..1 << (5 - bits_log) {
                    let idx = i | (j << bits_log);
                    let tv = mvars.format_var(prev_type, prev_pass[idx]);
                    if prev_type < OUTPUT_TYPE {
                        mvars.use_var(prev_type, prev_pass[idx]);
                    }
                    let (shl, failed) = {
                        let (shl, idx) = (4..10)
                            .filter_map(|x| self.config.shift_op[2 * x].map(|s| (s, x)))
                            .next()
                            .unwrap();
                        (shl, idx != 4)
                    };

                    let expr = if failed || j != ((1 << (5 - bits_log)) - 1) {
                        Self::format_op(
                            self.config.and_op,
                            &[
                                &tv,
                                mvars.get_constant(self.config.constant2_defs[bits_log]),
                            ],
                        )
                    } else {
                        tv
                    };
                    let expr = if j != 0 {
                        Self::format_op(shl, &[&expr, &(j << bits_log).to_string()])
                    } else {
                        expr
                    };
                    final_expr = if !final_expr.is_empty() {
                        Self::format_op(self.config.or_op, &[&final_expr, &expr])
                    } else {
                        expr
                    };
                    bit_usg |= (bit_usage[idx] & SINGLE_BIT_MASK_TBL[bits_log]) << (j << bits_log);
                    bit_usage[idx] = 0;
                }
                bit_usage[i] = bit_usg;
                let v = if bits_log != 0 {
                    let v = mvars.new_var(0);
                    write!(mvars, "    {} = ", mvars.format_var(0, v)).unwrap();
                    v
                } else {
                    write!(mvars, "    {} = ", mvars.format_var(OUTPUT_TYPE, 0)).unwrap();
                    0
                };
                write!(mvars, "{};\\\n", final_expr).unwrap();
                if bits_log != 0 {
                    prev_pass[i] = v;
                }
            }
            prev_type = 0;
        }
        for i in (0..bits_log).rev() {
            // main routine of transposing bits
            // using interleaving instructions or operations to quickly
            // transpose bit matrix where rows are words and columns are bits.
            let mut new_pass = vec![0; 1 << bits_log];
            for j in 0..1 << (bits_log - 1) {
                let fj = ((j >> i) << (i + 1)) | (j & ((1 << i) - 1));
                if i == 0 && fj >= bits {
                    continue;
                }
                let sj = fj | (1 << i);
                let bit_usage_f = (bit_usage[fj] & BIT_MASK_TBL[2 * i]) != 0;
                let bit_usage_s = (bit_usage[sj] & BIT_MASK_TBL[2 * i]) != 0;
                if prev_type < OUTPUT_TYPE {
                    if bit_usage_f {
                        mvars.use_var(prev_type, prev_pass[fj]);
                    }
                    if bit_usage_s {
                        mvars.use_var(prev_type, prev_pass[sj]);
                    }
                }
                let (nt, ns0) = if i != 0 {
                    (0, mvars.new_var(0))
                } else {
                    (OUTPUT_TYPE, fj)
                };
                let t0 = mvars.format_var(prev_type, prev_pass[fj]);
                let t1 = mvars.format_var(prev_type, prev_pass[sj]);

                let expr = self.gen_unpack_low(mvars, i, bit_usage_f, bit_usage_s, &t0, &t1);
                if !expr.is_empty() {
                    write!(mvars, "    {} = ", mvars.format_var(nt, ns0)).unwrap();
                    writeln!(mvars, "{};\\", expr).unwrap();
                }
                if i != 0 {
                    new_pass[fj] = ns0;
                }

                // second expression
                if i != 0 || sj < bits {
                    let bit_usage_f = (bit_usage[fj] & BIT_MASK_TBL[2 * i + 1]) != 0;
                    let bit_usage_s = (bit_usage[sj] & BIT_MASK_TBL[2 * i + 1]) != 0;
                    if prev_type < OUTPUT_TYPE {
                        if bit_usage_f {
                            mvars.use_var(prev_type, prev_pass[fj]);
                        }
                        if bit_usage_s {
                            mvars.use_var(prev_type, prev_pass[sj]);
                        }
                    }
                    let (nt, ns1) = if i != 0 {
                        (0, mvars.new_var(0))
                    } else {
                        (OUTPUT_TYPE, sj)
                    };
                    let expr = self.gen_unpack_high(mvars, i, bit_usage_f, bit_usage_s, &t0, &t1);
                    if !expr.is_empty() {
                        write!(mvars, "    {} = ", mvars.format_var(nt, ns1)).unwrap();
                        writeln!(mvars, "{};\\", expr).unwrap();
                    }
                    if i != 0 {
                        new_pass[sj] = ns1;
                    }
                }
                // update bit usage
                let bit_fj = (bit_usage[fj] & BIT_MASK_TBL[2 * i])
                    | ((bit_usage[sj] & BIT_MASK_TBL[2 * i]) << (1 << i));
                let bit_sj = ((bit_usage[fj] & BIT_MASK_TBL[2 * i + 1]) >> (1 << i))
                    | (bit_usage[sj] & BIT_MASK_TBL[2 * i + 1]);
                bit_usage[fj] = bit_fj;
                bit_usage[sj] = bit_sj;
            }
            prev_type = 0;
            if i != 0 {
                prev_pass = new_pass;
            }
        }
    }

    pub fn gen_input_transform(&mut self, bits: usize) {
        let mut mvars = CLangMacroVars::new(
            [self.config.comp_type_name],
            (0..32).map(|i| self.format_load_input((self.config.comp_type_bit_len >> 5) * i)),
            (0..bits as u32).map(|i| Self::format_arg_d(i)),
            self.config.collect_constants,
        );
        self.gen_input_transform_int(&mut mvars, bits);
        mvars.set_usage_mode();
        writeln!(
            &mut self.out,
            "#define INPUT_TRANSFORM_B{}({}, {}) \\",
            bits,
            &((0..bits).map(|i| format!("D{}", i)).collect::<Vec<_>>()).join(", "),
            "S",
        )
        .unwrap();
        self.out.write_str("{\\\n").unwrap();
        self.gen_input_transform_int(&mut mvars, bits);
        mvars.write_constant_defs(&mut self.out);
        mvars.write_var_defs(&mut self.out);
        self.out.push_str(&mvars.out);
        self.out.write_str("}\n").unwrap();
    }

    pub fn out(self) -> String {
        self.out
    }
}

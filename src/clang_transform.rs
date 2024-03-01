#[derive(Clone, Debug)]
pub struct CLangTransformConfig<'a> {
    func_modifier: Option<&'a str>,
    in_type_name: &'a str,
    out_type_name: &'a str,
    cast_type: Option<&'a str>,
    type_bit_len: u32,
    type_name_u32: &'a str,
    and_op: &'a str,
    or_op: &'a str,
    shl_op: &'a str,
    shr_op: &'a str,
    init: &'a str,
    values: [(&'a str, &'a str); 5],
    store_op: Option<&'a str>,
}

pub(crate) const CLANG_TRANSFORM_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "uint32_t",
    out_type_name: "uint32_t",
    cast_type: None,
    type_bit_len: 32,
    type_name_u32: "uint32_t",
    and_op: "{} & {}",
    or_op: "{} & {}",
    shl_op: "{} << {}",
    shr_op: "{} >> {}",
    init: "",
    values: [
        ("0x55555555U", "0xaaaaaaaaU"),
        ("0x33333333U", "0xccccccccU"),
        ("0x0f0f0f0fU", "0xf0f0f0f0U"),
        ("0x00ff00ffU", "0xff00ff00U"),
        ("0x0000ffffU", "0xffff0000U"),
    ],
    store_op: None,
};

pub(crate) const CLANG_TRANSFORM_U64: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "uint64_t",
    out_type_name: "uint64_t",
    cast_type: None,
    type_bit_len: 64,
    type_name_u32: "uint32_t",
    and_op: "{} & {}",
    or_op: "{} & {}",
    shl_op: "{} << {}",
    shr_op: "{} >> {}",
    init: "",
    values: [
        ("0x5555555555555555ULL", "0xaaaaaaaaaaaaaaaaULL"),
        ("0x3333333333333333ULL", "0xccccccccccccccccULL"),
        ("0x0f0f0f0f0f0f0f0fULL", "0xf0f0f0f0f0f0f0f0ULL"),
        ("0x00ff00ff00ff00ffULL", "0xff00ff00ff00ff00ULL"),
        ("0x0000ffff0000ffffULL", "0xffff0000ffff0000ULL"),
    ],
    store_op: None,
};

pub(crate) const CLANG_TRANSFORM_INTEL_MMX: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "__m64",
    out_type_name: "__m64",
    cast_type: None,
    type_bit_len: 64,
    type_name_u32: "uint32_t",
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    shl_op: "_m_pslldi({}, {})",
    shr_op: "_m_psrldi({}, {})",
    init: r##"static const unsigned int transform_mask_tbl[5*2*2] = {
    0x55555555, 0x55555555, 0xaaaaaaaa, 0xaaaaaaaa,
    0x33333333, 0x33333333, 0xcccccccc, 0xcccccccc,
    0x0f0f0f0f, 0x0f0f0f0f, 0xf0f0f0f0, 0xf0f0f0f0,
    0x00ff00ff, 0x00ff00ff, 0xff00ff00, 0xff00ff00,
    0x0000ffff, 0x0000ffff, 0xffff0000, 0xffff0000,
};
"##,
    values: [
        (
            "*((const __m64*)transform_mask_tbl)",
            "*((const __m64*)(transform_mask_tbl + 2))",
        ),
        (
            "*((const __m64*)(transform_mask_tbl + 4))",
            "*((const __m64*)(transform_mask_tbl + 6))",
        ),
        (
            "*((const __m64*)(transform_mask_tbl + 8))",
            "*((const __m64*)(transform_mask_tbl + 10))",
        ),
        (
            "*((const __m64*)(transform_mask_tbl + 12))",
            "*((const __m64*)(transform_mask_tbl + 14))",
        ),
        (
            "*((const __m64*)(transform_mask_tbl + 16))",
            "*((const __m64*)(transform_mask_tbl + 18))",
        ),
    ],
    store_op: None,
};

pub(crate) const CLANG_TRANSFORM_INTEL_SSE2: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "__m128",
    out_type_name: "__m128i",
    cast_type: Some("_mm_castps_si128({})"),
    type_bit_len: 128,
    type_name_u32: "uint32_t",
    and_op: "_mm_and_si128({}, {})",
    or_op: "_mm_or_si128({}, {})",
    shl_op: "_mm_slli_epi32({}, {})",
    shr_op: "_mm_srli_epi32({}, {})",
    init: r##"static const unsigned int transform_mask_tbl[5*4*2] = {
    0x55555555, 0x55555555, 0x55555555, 0x55555555, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0x33333333, 0x33333333, 0x33333333, 0x33333333, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
};
"##,
    values: [
        (
            "*((const __m128i*)transform_mask_tbl)",
            "*((const __m128i*)(transform_mask_tbl + 4))",
        ),
        (
            "*((const __m128i*)(transform_mask_tbl + 4*2))",
            "*((const __m128i*)(transform_mask_tbl + 4*3))",
        ),
        (
            "*((const __m128i*)(transform_mask_tbl + 4*4))",
            "*((const __m128i*)(transform_mask_tbl + 4*5))",
        ),
        (
            "*((const __m128i*)(transform_mask_tbl + 4*6))",
            "*((const __m128i*)(transform_mask_tbl + 4*7))",
        ),
        (
            "*((const __m128i*)(transform_mask_tbl + 4*8))",
            "*((const __m128i*)(transform_mask_tbl + 4*9))",
        ),
    ],
    store_op: None,
};

pub(crate) const CLANG_TRANSFORM_INTEL_AVX2: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "__m256",
    out_type_name: "__m256i",
    cast_type: Some("_mm256_castps_si256({})"),
    type_bit_len: 256,
    type_name_u32: "uint32_t",
    and_op: "_mm256_and_si256({}, {})",
    or_op: "_mm256_or_si256({}, {})",
    shl_op: "_mm256_sll_epi32({}, {})",
    shr_op: "_mm256_srl_epi32({}, {})",
    init: r##"static const unsigned int transform_mask_tbl[5*8*2] __attribute__((aligned(32))) = {
    0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555,
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
};
"##,
    values: [
        (
            "*((const __m256i*)transform_mask_tbl)",
            "*((const __m256i*)(transform_mask_tbl + 8))",
        ),
        (
            "*((const __m256i*)(transform_mask_tbl + 8*2))",
            "*((const __m256i*)(transform_mask_tbl + 8*3))",
        ),
        (
            "*((const __m256i*)(transform_mask_tbl + 8*4))",
            "*((const __m256i*)(transform_mask_tbl + 8*5))",
        ),
        (
            "*((const __m256i*)(transform_mask_tbl + 8*6))",
            "*((const __m256i*)(transform_mask_tbl + 8*7))",
        ),
        (
            "*((const __m256i*)(transform_mask_tbl + 8*8))",
            "*((const __m256i*)(transform_mask_tbl + 8*9))",
        ),
    ],
    store_op: Some("_mm256_storeu_si256((uint32_t*)&{}, {})"),
};

pub(crate) const CLANG_TRANSFORM_INTEL_AVX512: CLangTransformConfig<'_> = CLangTransformConfig {
    func_modifier: None,
    in_type_name: "__m512i",
    out_type_name: "__m256i",
    cast_type: None,
    type_bit_len: 512,
    type_name_u32: "uint32_t",
    and_op: "_mm512_and_epi64({}, {})",
    or_op: "_mm512_or_epi64({}, {})",
    shl_op: "_mm512_slli_epi64({}, {})",
    shr_op: "_mm512_srli_epi64({}, {})",
    init: r##"static const unsigned int transform_mask_tbl[5*16*2] __attribute__((aligned(64))) = {
    0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555,
    0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555, 0x55555555,
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa,
    0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333,
    0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc,
    0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f,
    0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0,
    0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff,
    0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff, 0x00ff00ff,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00,
    0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff,
    0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
    0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000,
};
"##,
    values: [
        (
            "*((const __m512i*)transform_mask_tbl)",
            "*((const __m512i*)(transform_mask_tbl + 16))",
        ),
        (
            "*((const __m512i*)(transform_mask_tbl + 16*2))",
            "*((const __m512i*)(transform_mask_tbl + 16*3))",
        ),
        (
            "*((const __m512i*)(transform_mask_tbl + 16*4))",
            "*((const __m512i*)(transform_mask_tbl + 16*5))",
        ),
        (
            "*((const __m512i*)(transform_mask_tbl + 16*6))",
            "*((const __m512i*)(transform_mask_tbl + 16*7))",
        ),
        (
            "*((const __m512i*)(transform_mask_tbl + 16*8))",
            "*((const __m512i*)(transform_mask_tbl + 16*9))",
        ),
    ],
    store_op: Some("_mm512_storeu_epi64(&{}, {})"),
};

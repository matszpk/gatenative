use gatesim::*;

use crate::*;

struct CLangWriter<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    include_name: Option<&'a str>,
    type_modifier: Option<&'a str>,
    type_name: &'a str,
    type_bit_len: u32,
    and_op: &'a str,
    or_op: &'a str,
    xor_op: &'a str,
    impl_op: Option<&'a str>,
    nimpl_op: Option<&'a str>,
    not_op: Option<&'a str>,
    one_value: Option<(&'a str, &'a str)>, // for emulate NOT
}

const CLANG_WRITER_U32: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_modifier: None,
    type_name: "uint32_t",
    type_bit_len: 32,
    and_op: "{} & {}",
    or_op: "{} | {}",
    xor_op: "{} ^ {}",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

const CLANG_WRITER_U64: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_modifier: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    and_op: "{} & {}",
    or_op: "{} | {}",
    xor_op: "{} ^ {}",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

const CLANG_WRITER_INTEL_MMX: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("mmintrin.h"),
    type_modifier: None,
    type_name: "__m64",
    type_bit_len: 64,
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    xor_op: "_m_pxor({}, {})",
    impl_op: None,
    nimpl_op: Some("_m_pandn({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsinged int one_value[2] = { 0xffffffff, 0xffffffff };"##,
        "*((__m64*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_SSE: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("xmmintrin.h"),
    type_modifier: None,
    type_name: "__m128",
    type_bit_len: 128,
    and_op: "_mm_and_ps({}, {})",
    or_op: "_mm_or_ps({}, {})",
    xor_op: "_mm_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm_andnot_ps({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsinged int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };"##,
        "*((__m128*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_AVX: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
    type_modifier: None,
    type_name: "__m256",
    type_bit_len: 256,
    and_op: "_mm256_and_ps({}, {})",
    or_op: "_mm256_or_ps({}, {})",
    xor_op: "_mm256_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm256_andnot_ps({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsigned int one_value[8] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        "*((__m256*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_AVX512: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
    type_modifier: None,
    type_name: "__m512i",
    type_bit_len: 512,
    and_op: "_mm512_and_epi64({}, {})",
    or_op: "_mm512_or_epi64({}, {})",
    xor_op: "_mm512_xor_epi64({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm512_andnot_epi64({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsigned int one_value[16] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};"##,
        "*((__m512i)one_value)",
    )),
};

const CLANG_WRITER_ARM_NEON: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("arm_neon.h"),
    type_modifier: None,
    type_name: "uint32x4_t",
    type_bit_len: 128,
    and_op: "vandq_u32({}, {})",
    or_op: "vorq_u32({}, {})",
    xor_op: "veorq_u32({}, {})",
    impl_op: Some("vornq_u32({1}, {0})"),
    nimpl_op: None,
    not_op: Some("vmvnq_u32({})"),
    one_value: None,
};

const CLANG_WRITER_OPENCL_U32: CLangWriter<'_> = CLangWriter {
    func_modifier: Some("kernel"),
    init_index: Some("uint idx = get_global_id(0);"),
    include_name: None,
    type_modifier: None,
    type_name: "uint",
    type_bit_len: 32,
    and_op: "{} & {}",
    or_op: "{} | {}",
    xor_op: "{} ^ {}",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

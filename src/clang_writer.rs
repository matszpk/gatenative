use crate::*;

use std::collections::HashMap;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct ElemIndexConfig<'a> {
    low_bits_init: &'a str,
    low_bits_defs: [&'a str; 16],
    func_arg_high: bool,
}

#[derive(Clone, Debug)]
pub struct CLangWriterConfig<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    init_local_index: Option<&'a str>, // to initialize local index in OpenCL kernel
    buffer_shift: bool,
    include_name: Option<&'a str>,
    include_name_2: Option<&'a str>,
    type_name: &'a str,
    type_bit_len: u32,
    arg_modifier: Option<&'a str>,
    and_op: &'a str,
    or_op: &'a str,
    xor_op: &'a str,
    impl_op: Option<&'a str>,
    nimpl_op: Option<&'a str>,
    not_op: Option<&'a str>,
    zero_value: (&'a str, &'a str), // for arg_input
    one_value: (&'a str, &'a str),  // for emulate NOT and arg_input
    elem_index: ElemIndexConfig<'a>,
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
}

pub const CLANG_WRITER_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    type_name: "uint32_t",
    type_bit_len: 32,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_U64: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_U64_TEST_IMPL: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: Some("(~{} | {})"),
    nimpl_op: None,
    not_op: Some("~{}"),
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_U64_TEST_NIMPL: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: Some("({} & ~{})"),
    not_op: Some("~{}"),
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_INTEL_MMX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("mmintrin.h"),
    include_name_2: Some("stddef.h"),
    type_name: "__m64",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    xor_op: "_m_pxor({}, {})",
    impl_op: None,
    nimpl_op: Some("_m_pandn({1}, {0})"),
    not_op: None,
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_INTEL_SSE: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("xmmintrin.h"),
    include_name_2: Some("stddef.h"),
    type_name: "__m128",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "_mm_and_ps({}, {})",
    or_op: "_mm_or_ps({}, {})",
    xor_op: "_mm_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm_andnot_ps({1}, {0})"),
    not_op: None,
    zero_value: (
        r##"static const unsigned int zero_value[4] = { 0, 0, 0, 0 };"##,
        "*((const __m128*)zero_value)",
    ),
    one_value: (
        r##"static const unsigned int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };"##,
        "*((const __m128*)one_value)",
    ),
    elem_index: ElemIndexConfig {
        low_bits_init: r##"static const unsigned int elem_index_low_tbl[7*4] = {
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_INTEL_AVX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    type_name: "__m256",
    type_bit_len: 256,
    arg_modifier: None,
    and_op: "_mm256_and_ps({}, {})",
    or_op: "_mm256_or_ps({}, {})",
    xor_op: "_mm256_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm256_andnot_ps({1}, {0})"),
    not_op: None,
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
        func_arg_high: true,
    },
    load_op: Some("_mm256_loadu_ps((const float*)&{})"),
    store_op: Some("_mm256_storeu_ps((float*)&{}, {})"),
};

pub const CLANG_WRITER_INTEL_AVX512: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    type_name: "__m512i",
    type_bit_len: 512,
    arg_modifier: None,
    and_op: "_mm512_and_epi64({}, {})",
    or_op: "_mm512_or_epi64({}, {})",
    xor_op: "_mm512_xor_epi64({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm512_andnot_epi64({1}, {0})"),
    not_op: None,
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
        func_arg_high: true,
    },
    load_op: Some("_mm512_loadu_epi64(&{})"),
    store_op: Some("_mm512_storeu_epi64(&{}, {})"),
};

pub const CLANG_WRITER_ARM_NEON: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    init_local_index: None,
    buffer_shift: false,
    include_name: Some("arm_neon.h"),
    include_name_2: Some("stddef.h"),
    type_name: "uint32x4_t",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "vandq_u32({}, {})",
    or_op: "vorrq_u32({}, {})",
    xor_op: "veorq_u32({}, {})",
    impl_op: Some("vornq_u32({1}, {0})"),
    nimpl_op: None,
    not_op: Some("vmvnq_u32({})"),
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
        func_arg_high: true,
    },
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_OPENCL_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const size_t idx = get_global_id(0);"),
    init_local_index: None,
    buffer_shift: true,
    include_name: None,
    include_name_2: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
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
        func_arg_high: false,
    },
    load_op: None,
    store_op: None,
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
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
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
        func_arg_high: false,
    },
    load_op: None,
    store_op: None,
};

pub struct CLangFuncWriter<'a, 'c> {
    writer: &'c mut CLangWriter<'a>,
    name: &'c str,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&'c [usize], usize)>,
    output_placement: Option<(&'c [usize], usize)>,
    input_map: HashMap<usize, usize>,
    arg_input_map: HashMap<usize, usize>,
    elem_input_map: HashMap<usize, usize>,
    single_buffer: bool,
}

pub struct CLangWriter<'a> {
    config: &'a CLangWriterConfig<'a>,
    elem_low_bits: u32,
    out: Vec<u8>,
}

impl<'a> CLangWriterConfig<'a> {
    pub fn writer(&'a self) -> CLangWriter<'a> {
        assert!(!self.buffer_shift || self.init_index.is_some());
        CLangWriter {
            config: self,
            elem_low_bits: self
                .elem_index
                .low_bits_defs
                .iter()
                .position(|x| *x == "")
                .unwrap_or(16) as u32,
            out: vec![],
        }
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

    fn format_neg_arg(config: &CLangWriterConfig, neg: bool, reg: usize) -> String {
        if neg {
            let arg = format!("v{}", reg);
            let mut out_str = vec![];
            CLangWriter::write_neg(&config, &mut out_str, arg.as_bytes());
            String::from_utf8_lossy(&out_str).to_string()
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
        let arg_input = if !self.arg_input_map.is_empty() {
            ", unsigned int arg, unsigned int arg2"
        } else {
            ""
        };
        let elem_input =
            if !self.elem_input_map.is_empty() && self.writer.config.elem_index.func_arg_high {
                ", size_t idx"
            } else {
                ""
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
                self.writer.config.type_name
            )
        } else {
            format!(
                "const {0}{1}{2}* input,\n    {0}{1}{2}* output",
                self.writer.config.arg_modifier.unwrap_or(""),
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.writer.config.type_name
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
                elem_input,
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
                        self.input_len - self.arg_input_map.len() - self.elem_input_map.len()
                    ),
                    input_shift_part,
                    self.output_placement
                        .map(|(_, len)| len)
                        .unwrap_or(self.output_len),
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
                        self.input_len - self.arg_input_map.len() - self.elem_input_map.len()
                    ),
                    input_shift_part,
                    self.output_placement
                        .map(|(_, len)| len)
                        .unwrap_or(self.output_len),
                    output_shift_part,
                )
                .unwrap();
            }
        } else {
            writeln!(
                self.writer.out,
                r##"{}{}void gate_sys_{}({}{}{}{}) {{"##,
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
                elem_input
            )
            .unwrap();
        }
        let zero_value = self.writer.config.zero_value.1;
        if !self.arg_input_map.is_empty() || !self.elem_input_map.is_empty() {
            writeln!(
                self.writer.out,
                "    const {} zero = {};",
                self.writer.config.type_name, zero_value
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
                self.writer.config.type_name, one_value
            )
            .unwrap();
        }
        if !self.elem_input_map.is_empty() {
            for i in 0..self.writer.elem_low_bits {
                writeln!(
                    self.writer.out,
                    "    const {} elem_low_bit{} = {};",
                    self.writer.config.type_name,
                    i,
                    self.writer.config.elem_index.low_bits_defs[i as usize]
                )
                .unwrap();
            }
            self.writer
                .out
                .extend(b"    const unsigned int idxl = idx & 0xffffffff;\n");
            self.writer
                .out
                .extend(b"    const unsigned int idxh = idx >> 32;\n");
        }
    }
    fn func_end(&mut self) {
        self.writer.out.extend(b"}\n");
    }
    fn alloc_vars(&mut self, var_num: usize) {
        for i in 0..var_num {
            writeln!(
                self.writer.out,
                "    {} v{};",
                self.writer.config.type_name, i
            )
            .unwrap();
        }
        if self.writer.config.init_index.is_some() {
            self.writer.out.extend(b"    if (idx >= n) return;\n");
        }
    }

    fn gen_load(&mut self, reg: usize, input: usize) {
        if let Some(arg_bit) = self.arg_input_map.get(&input) {
            if *arg_bit < 32 {
                writeln!(
                    self.writer.out,
                    "    v{} = ((arg & {}) != 0) ? one : zero;",
                    reg,
                    1u32 << arg_bit
                )
                .unwrap();
            } else {
                writeln!(
                    self.writer.out,
                    "    v{} = ((arg2 & {}) != 0) ? one : zero;",
                    reg,
                    1u32 << (*arg_bit - 32)
                )
                .unwrap();
            }
        } else if let Some(elem_bit) = self.elem_input_map.get(&input) {
            if *elem_bit < (self.writer.elem_low_bits as usize) {
                writeln!(self.writer.out, "    v{} = elem_low_bit{};", reg, *elem_bit).unwrap();
            } else {
                let ebit = *elem_bit - (self.writer.elem_low_bits as usize);
                if ebit < 32 {
                    writeln!(
                        self.writer.out,
                        "    v{} = ((idxl & {}) != 0) ? one : zero;",
                        reg,
                        1u32 << ebit
                    )
                    .unwrap();
                } else {
                    writeln!(
                        self.writer.out,
                        "    v{} = ((idxh & {}) != 0) ? one : zero;",
                        reg,
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
            } else {
                self.input_placement.map(|(p, _)| p[input]).unwrap_or(input)
            };
            let (dst, r) = if self.writer.config.init_index.is_some() {
                if self.writer.config.init_local_index.is_some() {
                    (
                        format!("v{}", reg),
                        format!("{}[ivn + llen*{} + lidx]", arg_name, input),
                    )
                } else {
                    (
                        format!("v{}", reg),
                        format!("{}[ivn + {}]", arg_name, input),
                    )
                }
            } else {
                (format!("v{}", reg), format!("{}[{}]", arg_name, input))
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
        let arg0 = format!("v{}", arg0);
        let arg1 =
            CLangWriter::<'a>::format_neg_arg(self.writer.config, negs == VNegs::NegInput1, arg1);
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
        };
        write!(self.writer.out, "    v{} = ", dst_arg).unwrap();
        if negs == VNegs::NegOutput {
            CLangWriter::<'a>::write_neg(self.writer.config, &mut self.writer.out, &op_vec);
        } else {
            self.writer.out.extend(op_vec);
        }
        self.writer.out.extend(b";\n");
    }

    fn gen_store(&mut self, neg: bool, output: usize, reg: usize) {
        let output = self
            .output_placement
            .map(|(p, _)| p[output])
            .unwrap_or(output);
        let arg = CLangWriter::<'a>::format_neg_arg(self.writer.config, neg, reg);
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
}

impl<'a, 'c> CodeWriter<'c, CLangFuncWriter<'a, 'c>> for CLangWriter<'a> {
    fn supported_ops(&self) -> u64 {
        let basic_ops = (1u64 << InstrOp::And.int_value())
            | (1u64 << InstrOp::Or.int_value())
            | (1u64 << InstrOp::Xor.int_value());
        let basic_impl_ops = basic_ops | (1u64 << InstrOp::Impl.int_value());
        let basic_nimpl_ops = basic_ops | (1u64 << InstrOp::Nimpl.int_value());
        if self.config.impl_op.is_some() {
            basic_impl_ops
        } else if self.config.nimpl_op.is_some() {
            basic_nimpl_ops
        } else {
            basic_ops
        }
    }
    fn word_len(&self) -> u32 {
        self.config.type_bit_len
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
    }

    fn epilog(&mut self) {}

    unsafe fn func_writer_internal(
        &'c mut self,
        name: &'c str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'c [usize], usize)>,
        output_placement: Option<(&'c [usize], usize)>,
        arg_inputs: Option<&'c [usize]>,
        elem_inputs: Option<&'c [usize]>,
        single_buffer: bool,
    ) -> CLangFuncWriter<'a, 'c> {
        let (input_map, arg_input_map, elem_input_map) = {
            let arg_input_map = if let Some(arg_inputs) = arg_inputs {
                HashMap::from_iter(arg_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            };
            let elem_input_map = if let Some(elem_inputs) = elem_inputs {
                HashMap::from_iter(elem_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            };
            let mut input_map = HashMap::new();
            if !arg_input_map.is_empty() || !elem_input_map.is_empty() {
                let mut count = 0;
                for i in 0..input_len {
                    if !arg_input_map.contains_key(&i) && !elem_input_map.contains_key(&i) {
                        input_map.insert(i, count);
                        count += 1;
                    }
                }
            }
            (input_map, arg_input_map, elem_input_map)
        };

        CLangFuncWriter::<'a, 'c> {
            writer: self,
            name,
            input_len,
            output_len,
            input_placement,
            output_placement,
            input_map,
            arg_input_map,
            elem_input_map,
            single_buffer,
        }
    }

    fn out(self) -> Vec<u8> {
        self.out
    }
}

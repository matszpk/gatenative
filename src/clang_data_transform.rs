use crate::*;

use crate::clang_transform::*;

use std::collections::{HashMap, HashSet};
use std::io::Write;

#[derive(Clone, Debug)]
pub struct CLangDataTransformConfig<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    buffer_shift: bool,
    include_name: Option<&'a str>,
    include_name_2: Option<&'a str>,
    include_name_3: Option<&'a str>,
    type_name: &'a str,
    type_bit_len: u32,
    arg_modifier: Option<&'a str>,
    zero_value: (&'a str, &'a str), // for arg_input
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
    transform_config: &'a CLangTransformConfig<'a>,
    previous: Option<&'a CLangDataTransformConfig<'a>>,
}

pub const CLANG_DATA_TRANSFORM_U32: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint32_t",
    type_bit_len: 32,
    arg_modifier: None,
    zero_value: ("", "0"),
    load_op: None,
    store_op: None,
    transform_config: &CLANG_TRANSFORM_U32,
    previous: None,
};

pub const CLANG_DATA_TRANSFORM_U64: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("stdint.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: None,
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    zero_value: ("", "0ULL"),
    load_op: None,
    store_op: None,
    transform_config: &CLANG_TRANSFORM_U64,
    previous: Some(&CLANG_DATA_TRANSFORM_U32),
};

pub const CLANG_DATA_TRANSFORM_INTEL_MMX: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("mmintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m64",
    type_bit_len: 64,
    arg_modifier: None,
    zero_value: (
        r##"static const unsigned int zero_value[2] = { 0, 0 };"##,
        "*((const __m64*)zero_value)",
    ),
    load_op: None,
    store_op: None,
    transform_config: &CLANG_TRANSFORM_INTEL_MMX,
    previous: Some(&CLANG_DATA_TRANSFORM_U32),
};

pub const CLANG_DATA_TRANSFORM_INTEL_SSE: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("xmmintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m128",
    type_bit_len: 128,
    arg_modifier: None,
    zero_value: (
        r##"static const unsigned int zero_value[4] __attribute__((aligned(16))) =
    { 0, 0, 0, 0 };"##,
        "*((const __m128*)zero_value)",
    ),
    load_op: Some("_mm_loadu_ps((const float*)&{})"),
    store_op: Some("_mm_storeu_ps((float*)&{}, {})"),
    transform_config: &CLANG_TRANSFORM_INTEL_SSE,
    previous: Some(&CLANG_DATA_TRANSFORM_INTEL_MMX),
};

pub const CLANG_DATA_TRANSFORM_INTEL_SSE2: CLangDataTransformConfig<'_> =
    CLangDataTransformConfig {
        func_modifier: None,
        init_index: None,
        buffer_shift: false,
        include_name: Some("xmmintrin.h"),
        include_name_2: Some("stddef.h"),
        include_name_3: Some("stdint.h"),
        type_name: "__m128i",
        type_bit_len: 128,
        arg_modifier: None,
        zero_value: (
            r##"static const unsigned int zero_value[4] __attribute__((aligned(16))) =
    { 0, 0, 0, 0 };"##,
            "*((const __m128i*)zero_value)",
        ),
        load_op: Some("_mm_loadu_si128((const __m128i*)&{})"),
        store_op: Some("_mm_storeu_si128((__m128i*)&{}, {})"),
        transform_config: &CLANG_TRANSFORM_INTEL_SSE2,
        previous: Some(&CLANG_DATA_TRANSFORM_INTEL_MMX),
    };

pub const CLANG_DATA_TRANSFORM_INTEL_AVX: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("immintrin.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "__m256",
    type_bit_len: 256,
    arg_modifier: None,
    zero_value: (
        r##"static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};"##,
        "*((const __m256*)zero_value)",
    ),
    load_op: Some("_mm256_loadu_ps((const float*)&{})"),
    store_op: Some("_mm256_storeu_ps((float*)&{}, {})"),
    transform_config: &CLANG_TRANSFORM_INTEL_AVX,
    previous: Some(&CLANG_DATA_TRANSFORM_INTEL_SSE2),
};

pub const CLANG_DATA_TRANSFORM_INTEL_AVX2: CLangDataTransformConfig<'_> =
    CLangDataTransformConfig {
        func_modifier: None,
        init_index: None,
        buffer_shift: false,
        include_name: Some("immintrin.h"),
        include_name_2: Some("stddef.h"),
        include_name_3: Some("stdint.h"),
        type_name: "__m256i",
        type_bit_len: 256,
        arg_modifier: None,
        zero_value: (
            r##"static const unsigned int zero_value[8] __attribute__((aligned(32))) = {
    0, 0, 0, 0, 0, 0, 0, 0
};"##,
            "*((const __m256i*)zero_value)",
        ),
        load_op: Some("_mm256_loadu_si256((const __m256i*)&{})"),
        store_op: Some("_mm256_storeu_si256((__m256i*)&{}, {})"),
        transform_config: &CLANG_TRANSFORM_INTEL_AVX2,
        previous: Some(&CLANG_DATA_TRANSFORM_INTEL_SSE2),
    };

pub const CLANG_DATA_TRANSFORM_INTEL_AVX512: CLangDataTransformConfig<'_> =
    CLangDataTransformConfig {
        func_modifier: None,
        init_index: None,
        buffer_shift: false,
        include_name: Some("immintrin.h"),
        include_name_2: Some("stddef.h"),
        include_name_3: Some("stdint.h"),
        type_name: "__m512i",
        type_bit_len: 512,
        arg_modifier: None,
        zero_value: (
            r##"static const unsigned int zero_value[16] __attribute__((aligned(64))) = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};"##,
            "*((const __m512i*)zero_value)",
        ),
        load_op: Some("_mm512_loadu_epi64(&{})"),
        store_op: Some("_mm512_storeu_epi64(&{}, {})"),
        transform_config: &CLANG_TRANSFORM_INTEL_AVX512,
        previous: Some(&CLANG_DATA_TRANSFORM_INTEL_AVX2),
    };

pub const CLANG_DATA_TRANSFORM_ARM_NEON: CLangDataTransformConfig<'_> = CLangDataTransformConfig {
    func_modifier: None,
    init_index: None,
    buffer_shift: false,
    include_name: Some("arm_neon.h"),
    include_name_2: Some("stddef.h"),
    include_name_3: Some("stdint.h"),
    type_name: "uint32x4_t",
    type_bit_len: 128,
    arg_modifier: None,
    zero_value: ("", "{ 0, 0, 0, 0 }"),
    load_op: None,
    store_op: None,
    transform_config: &CLANG_TRANSFORM_ARM_NEON,
    previous: Some(&CLANG_DATA_TRANSFORM_U64),
};

pub const CLANG_DATA_TRANSFORM_OPENCL_U32: CLangDataTransformConfig<'_> =
    CLangDataTransformConfig {
        func_modifier: Some("kernel"),
        init_index: Some("const size_t idx = get_group_id(0);"),
        buffer_shift: true,
        include_name: None,
        include_name_2: None,
        include_name_3: None,
        type_name: "uint",
        type_bit_len: 32,
        arg_modifier: Some("global"),
        zero_value: ("", "0"),
        load_op: None,
        store_op: None,
        transform_config: &CLANG_TRANSFORM_OPENCL_U32,
        previous: None,
    };

pub struct CLangDataTransform<'a> {
    config: &'a CLangDataTransformConfig<'a>,
    out: Vec<u8>,
    input_transform_helpers_added: bool,
    output_transform_helpers_added: bool,
    word_len: u32,
}

impl<'a> CLangDataTransformConfig<'a> {
    pub fn data_transform(&'a self, word_len: u32) -> CLangDataTransform<'a> {
        assert!(!self.buffer_shift || self.init_index.is_some());
        let mut config = self;
        // find config to match word_len (word_len must be multiple of type_bit_len.
        while (word_len % config.type_bit_len) != 0 {
            config = config.previous.unwrap();
        }
        CLangDataTransform {
            config,
            out: vec![],
            input_transform_helpers_added: false,
            output_transform_helpers_added: false,
            word_len,
        }
    }
}

impl<'a> CLangDataTransform<'a> {
    fn input_transform_helpers(&mut self) {
        if !self.input_transform_helpers_added {
            let mut transform = self.config.transform_config.transform();
            transform.init_defs();
            for i in 1..=32 {
                transform.gen_input_transform_with_prefix(i, "");
            }
            self.out.extend(transform.out().as_bytes());
            self.input_transform_helpers_added = true;
        }
    }
    fn output_transform_helpers(&mut self) {
        if !self.output_transform_helpers_added {
            let mut transform = self.config.transform_config.transform();
            transform.init_defs();
            for i in 1..=32 {
                transform.gen_output_transform_with_prefix(i, "");
            }
            self.out.extend(transform.out().as_bytes());
            self.output_transform_helpers_added = true;
        }
    }

    pub fn prolog(&mut self) {
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
    }

    fn function_start(&mut self, name: &str, output: bool) {
        writeln!(
            self.out,
            "{} void {}(unsigned long n, const {} {}* input, {} {}* output) {{",
            self.config.func_modifier.unwrap_or(""),
            name,
            self.config.arg_modifier.unwrap_or(""),
            if output {
                self.config.type_name
            } else {
                "unsigned int"
            },
            self.config.arg_modifier.unwrap_or(""),
            if output {
                "unsigned int"
            } else {
                self.config.type_name
            }
        )
        .unwrap();
        writeln!(
            self.out,
            "    const {} zero = {};",
            self.config.type_name, self.config.zero_value.1
        )
        .unwrap();
        if let Some(init_index) = self.config.init_index {
            writeln!(self.out, "    {}", init_index).unwrap();
        } else {
            self.out
                .extend(b"    size_t idx;\n    for (idx = 0; idx < n; idx++) {\n");
        }
    }

    fn function_end(&mut self) {
        if self.config.init_index.is_none() {
            self.out.extend(b"    }\n");
        }
        self.out.extend(b"}\n");
    }

    fn index_to_circuit_elem_ptr(&self, elem_len: usize) -> String {
        let typewords_per_word = self.word_len / self.config.type_bit_len;
        if typewords_per_word == 1 {
            format!("{}idx", elem_len)
        } else if typewords_per_word.count_ones() == 1 {
            // power of two
            let mask = typewords_per_word - 1;
            format!(
                "{0}*(idx & ~(size_t){1}) + (idx & {1})",
                (typewords_per_word as usize) * elem_len,
                mask
            )
        } else {
            format!(
                "{0}*((idx / {1}) * {1}) + (idx % {1})",
                (typewords_per_word as usize) * elem_len,
                typewords_per_word
            )
        }
    }

    pub fn input_transform(
        &mut self,
        input_elem_len: usize,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) {
        assert_eq!((input_elem_len & 31), 0);
        assert!(input_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(output_elem_len >= bit_mapping.len());
        self.function_start("input_data_transform", false);
        // define input and output elems pointers
        let typewords_per_word = self.word_len / self.config.type_bit_len;
        writeln!(
            self.out,
            "    const unsigned int {}* inelem = input + {}*idx",
            self.config.arg_modifier.unwrap_or(""),
            input_elem_len * (typewords_per_word as usize),
        )
        .unwrap();
        writeln!(
            self.out,
            "    {} {}* outelem = output + {};",
            self.config.arg_modifier.unwrap_or(""),
            self.config.type_name,
            self.index_to_circuit_elem_ptr(output_elem_len)
        )
        .unwrap();
        self.function_end();
    }

    pub fn output_transform(
        &mut self,
        input_elem_len: usize,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) {
        assert_eq!((output_elem_len & 31), 0);
        assert!(output_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(input_elem_len >= bit_mapping.len());
        self.function_start("output_data_transform", true);
        // define input and output elems pointers
        let typewords_per_word = self.word_len / self.config.type_bit_len;
        writeln!(
            self.out,
            "    const {} {}* inelem = input + {};",
            self.config.arg_modifier.unwrap_or(""),
            self.config.type_name,
            self.index_to_circuit_elem_ptr(input_elem_len)
        )
        .unwrap();
        writeln!(
            self.out,
            "    unsigned int {}* outelem = output + {}*idx",
            self.config.arg_modifier.unwrap_or(""),
            output_elem_len * (typewords_per_word as usize)
        )
        .unwrap();
        self.function_end();
    }
}

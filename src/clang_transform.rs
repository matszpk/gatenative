use crate::utils::*;

use std::collections::HashMap;
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct InterType<'a> {
    inter_type_name: &'a str,
    inter_type_bit_len: u32,
    inter_to_final: &'a str,
    final_to_inter: &'a str,
}

#[derive(Clone, Debug)]
pub struct CLangTransformConfig<'a> {
    final_type_name: &'a str,
    final_type_bit_len: u32,
    inter_type: Option<InterType<'a>>,
    and_op: &'a str,
    or_op: &'a str,
    shl32_op: &'a str,
    shr32_op: &'a str,
    unpack_ops: [Option<&'a str>; 5 * 2],
    init_defs: &'a str,
    // masks for transposition operations (unpackings)
    constant_defs: [&'a str; 2 * 5],
    // masks for first 2^n bits
    constant2_defs: [&'a str; 5],
}

pub const CLANG_TRANSFORM_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "uint32_t",
    final_type_bit_len: 32,
    inter_type: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shl32_op: "({} << {})",
    shr32_op: "({} >> {})",
    unpack_ops: [None, None, None, None, None, None, None, None, None, None],
    init_defs: "",
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
    ],
    constant2_defs: ["0x1U", "0x3U", "0xfU", "0xffU", "0xffffU"],
};

pub const CLANG_TRANSFORM_U64: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "uint64_t",
    final_type_bit_len: 64,
    inter_type: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shl32_op: "({} << {})",
    shr32_op: "({} >> {})",
    unpack_ops: [None, None, None, None, None, None, None, None, None, None],
    init_defs: "",
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
    ],
    constant2_defs: [
        "0x0000000100000001ULL",
        "0x0000000300000003ULL",
        "0x0000000f0000000fULL",
        "0x000000ff000000ffULL",
        "0x0000ffff0000ffffULL",
    ],
};

pub const CLANG_TRANSFORM_INTEL_MMX: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "__m64",
    final_type_bit_len: 64,
    inter_type: None,
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    shl32_op: "_m_pslldi({}, {})",
    shr32_op: "_m_psrldi({}, {})",
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_m_punpcklbw({}, {})"),
        Some("_m_punpckhbw({}, {})"),
        Some("_m_punpcklwd({}, {})"),
        Some("_m_punpckhwd({}, {})"),
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[5*2*2] = {
    0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U
};
static const unsigned int transform_const2_tbl[5*2] = {
    0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU
};
"##,
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
    ],
    constant2_defs: [
        "(*(const __m64*)(transform_const2_tbl + 2*0))",
        "(*(const __m64*)(transform_const2_tbl + 2*1))",
        "(*(const __m64*)(transform_const2_tbl + 2*2))",
        "(*(const __m64*)(transform_const2_tbl + 2*3))",
        "(*(const __m64*)(transform_const2_tbl + 2*4))",
    ],
};

pub const CLANG_TRANSFORM_INTEL_SSE2: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "__m128i",
    final_type_bit_len: 128,
    inter_type: None,
    and_op: "_mm_and_si128({}, {})",
    or_op: "_mm_or_si128({}, {})",
    shl32_op: "_mm_slli_epi32({}, {})",
    shr32_op: "_mm_srli_epi32({}, {})",
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm_unpacklo_epi8({}, {})"),
        Some("_mm_unpackhi_epi8({}, {})"),
        Some("_mm_unpacklo_epi16({}, {})"),
        Some("_mm_unpackhi_epi16({}, {})"),
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[5*2*4] = {
    0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U,
    0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU,
    0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U,
    0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU,
    0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU,
    0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U,
    0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU,
    0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU,
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U
};
static const unsigned int transform_const2_tbl[5*4] = {
    0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U,
    0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U,
    0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU,
    0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU,
    0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU
};
"##,
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
    ],
    constant2_defs: [
        "(*(const __m128i*)(transform_const2_tbl + 4*0))",
        "(*(const __m128i*)(transform_const2_tbl + 4*1))",
        "(*(const __m128i*)(transform_const2_tbl + 4*2))",
        "(*(const __m128i*)(transform_const2_tbl + 4*3))",
        "(*(const __m128i*)(transform_const2_tbl + 4*4))",
    ],
};

pub const CLANG_TRANSFORM_INTEL_AVX2: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "__m256i",
    final_type_bit_len: 256,
    inter_type: None,
    and_op: "_mm256_and_si256({}, {})",
    or_op: "_mm256_or_si256({}, {})",
    shl32_op: "_mm256_slli_epi32({}, {})",
    shr32_op: "_mm256_srli_epi32({}, {})",
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("_mm256_unpacklo_epi8({}, {})"),
        Some("_mm256_unpackhi_epi8({}, {})"),
        Some("_mm256_unpacklo_epi16({}, {})"),
        Some("_mm256_unpackhi_epi16({}, {})"),
    ],
    init_defs: r##"static const unsigned int transform_const_tbl[5*2*8]
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
    0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U
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
    ],
    constant2_defs: [
        "(*(const __m256i*)(transform_const2_tbl + 8*0))",
        "(*(const __m256i*)(transform_const2_tbl + 8*1))",
        "(*(const __m256i*)(transform_const2_tbl + 8*2))",
        "(*(const __m256i*)(transform_const2_tbl + 8*3))",
        "(*(const __m256i*)(transform_const2_tbl + 8*4))",
    ],
};

pub const CLANG_TRANSFORM_ARM_NEON: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "uint32x4_t",
    final_type_bit_len: 128,
    inter_type: None,
    and_op: "vandq_u32({}, {})",
    or_op: "vorrq_u32({}, {})",
    shl32_op: "vshlq_n_u32({}, {})",
    shr32_op: " vshrq_n_u32({}, {})",
    unpack_ops: [
        None,
        None,
        None,
        None,
        None,
        None,
        Some("vuzp1q_u8({}, {})"),
        Some("vuzp2q_u8({}, {})"),
        Some("vuzp1q_u16({}, {})"),
        Some("vuzp2q_u16({}, {})"),
    ],
    init_defs: "",
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
    ],
    constant2_defs: [
        "{ 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }",
        "{ 0x00000003U, 0x00000003U, 0x00000003U, 0x00000003U }",
        "{ 0x0000000fU, 0x0000000fU, 0x0000000fU, 0x0000000fU }",
        "{ 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }",
        "{ 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }",
    ],
};

pub const CLANG_TRANSFORM_OPENCL_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "uint",
    final_type_bit_len: 32,
    inter_type: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shl32_op: "({} << {})",
    shr32_op: "({} >> {})",
    unpack_ops: [None, None, None, None, None, None, None, None, None, None],
    init_defs: "",
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
    ],
    constant2_defs: ["0x1U", "0x3U", "0xfU", "0xffU", "0xffffU"],
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
    inputs: Vec<String>,
    outputs: Vec<String>,
    out: String,
}

impl CLangMacroVars {
    fn new<'a>(
        var_types: impl IntoIterator<Item = &'a str>,
        inputs: impl IntoIterator<Item = String>,
        outputs: impl IntoIterator<Item = String>,
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
            inputs: inputs.into_iter().collect::<Vec<_>>(),
            outputs: outputs.into_iter().collect::<Vec<_>>(),
            out: String::new(),
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

    pub fn format_arg_s(arg: usize) -> String {
        format!("((S)[{}])", arg)
    }
    pub fn format_arg_d(arg: usize) -> String {
        format!("(D{})", arg)
    }

    fn gen_input_transform_int(&mut self, mvars: &mut CLangMacroVars, bits: usize) {
        let bits_log = calc_log_bits(bits);
        let mut prev_type = INPUT_TYPE;
        let mut bit_usage = vec![u32::try_from((1usize << bits) - 1).unwrap(); 32];
        let mut prev_pass = (0..32).collect::<Vec<_>>();
        if bits_log < 5 {
            for i in 0..1 << bits_log {
                let v = if bits_log != 0 {
                    let v = mvars.new_var(0);
                    write!(mvars, "    {} = ", mvars.format_var(0, v)).unwrap();
                    v
                } else {
                    write!(mvars, "    {} = ", mvars.format_var(OUTPUT_TYPE, 0)).unwrap();
                    0
                };
                let mut final_expr = String::new();
                let mut bit_usg = 0;
                for j in 0..1 << (5 - bits_log) {
                    let idx = i | (j << bits_log);
                    let tv = mvars.format_var(prev_type, prev_pass[idx]);
                    let expr = if j != ((1 << (5 - bits_log)) - 1) {
                        Self::format_op(
                            self.config.and_op,
                            &[&tv, self.config.constant2_defs[bits_log]],
                        )
                    } else {
                        tv
                    };
                    let expr = if j != 0 {
                        Self::format_op(
                            self.config.shl32_op,
                            &[&expr, &(j << bits_log).to_string()],
                        )
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
                write!(mvars, "{};\\\n", final_expr).unwrap();
                if bits_log != 0 {
                    prev_pass[i] = v;
                }
            }
            prev_type = 0;
        }
        for i in (0..bits_log).rev() {
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
                let t0 = mvars.format_var(prev_type, prev_pass[fj]);
                let t1 = mvars.format_var(prev_type, prev_pass[sj]);
                let (nt, ns0) = if i != 0 {
                    (0, mvars.new_var(0))
                } else {
                    (OUTPUT_TYPE, fj)
                };

                let p0 = if bit_usage_f {
                    Self::format_op(self.config.and_op, &[&t0, self.config.constant_defs[2 * i]])
                } else {
                    String::new()
                };
                let p1 = if bit_usage_s {
                    let p1 = Self::format_op(
                        self.config.and_op,
                        &[&t1, self.config.constant_defs[2 * i]],
                    );
                    Self::format_op(self.config.shl32_op, &[&p1, &(1 << i).to_string()])
                } else {
                    String::new()
                };
                let expr = if !p0.is_empty() {
                    if !p1.is_empty() {
                        Self::format_op(self.config.or_op, &[&p0, &p1])
                    } else {
                        p0
                    }
                } else {
                    p1
                };
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
                    let p0 = if bit_usage_f {
                        let p0 = Self::format_op(
                            self.config.and_op,
                            &[&t0, self.config.constant_defs[2 * i + 1]],
                        );
                        Self::format_op(self.config.shr32_op, &[&p0, &(1 << i).to_string()])
                    } else {
                        String::new()
                    };
                    let p1 = if bit_usage_s {
                        Self::format_op(
                            self.config.and_op,
                            &[&t1, self.config.constant_defs[2 * i + 1]],
                        )
                    } else {
                        String::new()
                    };
                    let expr = if !p0.is_empty() {
                        if !p1.is_empty() {
                            Self::format_op(self.config.or_op, &[&p0, &p1])
                        } else {
                            p0
                        }
                    } else {
                        p1
                    };
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
            [self.config.final_type_name],
            (0..32).map(|i| Self::format_arg_s(i)),
            (0..bits).map(|i| Self::format_arg_d(i)),
        );
        self.gen_input_transform_int(&mut mvars, bits);
        mvars.set_usage_mode();
        writeln!(
            &mut self.out,
            "#define IN_TRANSFORM_B{}({}, {}) \\",
            bits,
            &((0..bits).map(|i| format!("D{}", i)).collect::<Vec<_>>()).join(", "),
            "S",
        )
        .unwrap();
        self.out.write_str("{\\\n").unwrap();
        self.gen_input_transform_int(&mut mvars, bits);
        mvars.write_var_defs(&mut self.out);
        self.out.push_str(&mvars.out);
        self.out.write_str("}\n").unwrap();
    }

    pub fn out(self) -> String {
        self.out
    }
}

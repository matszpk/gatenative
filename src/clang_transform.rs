use crate::utils::calc_log_bits;

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
        format!("(S[{}])", arg)
    }
    pub fn format_arg_d(arg: usize) -> String {
        format!("(D{})", arg)
    }

    pub fn write_left_side_assign(&mut self, new_var: bool, out: bool, prefix: &str, i: usize) {
        if out {
            write!(self.out, "    {} = ", Self::format_arg_d(i)).unwrap();
        } else if new_var {
            write!(
                self.out,
                "    {} {}{} = ",
                self.config.final_type_name, prefix, i
            )
            .unwrap();
        } else {
            write!(self.out, "    {}{} = ", prefix, i).unwrap();
        }
    }

    pub fn gen_input_transform(&mut self, bits: usize) {
        // definition
        writeln!(
            self.out,
            "#define IN_TRANSFORM_B{}({}, {}) \\",
            bits,
            &((0..bits).map(|i| format!("D{}", i)).collect::<Vec<_>>()).join(", "),
            "S",
        )
        .unwrap();
        self.out.push_str("{ \\\n");
        // TODO: Add passing out prepare to lower bits_log before main routine
        let bits_log = calc_log_bits(bits);
        for i in (0..bits_log).rev() {
            for j in 0..16 {
                let fj = ((j >> i) << (i + 1)) | (j & ((1 << i) - 1));
                if fj >= bits {
                    continue;
                }
                let sj = fj | (1 << i);
                let t0 = if i == bits_log - 1 {
                    Self::format_arg_s(fj)
                } else if (i & 1) ^ (bits_log & 1) == 0 {
                    format!("t{}", fj)
                } else {
                    format!("s{}", fj)
                };
                let t1 = if sj < bits {
                    if i == bits_log - 1 {
                        Self::format_arg_s(sj)
                    } else if (i & 1) ^ (bits_log & 1) == 0 {
                        format!("t{}", sj)
                    } else {
                        format!("s{}", sj)
                    }
                } else {
                    String::new()
                };
                self.write_left_side_assign(
                    i >= (bits_log - 2),
                    i == 0,
                    if (i & 1) ^ (bits_log & 1) != 0 {
                        "t"
                    } else {
                        "s"
                    },
                    fj,
                );
                let p0 =
                    Self::format_op(self.config.and_op, &[&t0, self.config.constant_defs[2 * i]]);
                let expr = if !t1.is_empty() {
                    let p1 = Self::format_op(
                        self.config.and_op,
                        &[&t1, self.config.constant_defs[2 * i]],
                    );
                    let p1 = Self::format_op(self.config.shl32_op, &[&p1, &(1 << i).to_string()]);
                    Self::format_op(self.config.or_op, &[&p0, &p1])
                } else {
                    p0
                };
                writeln!(self.out, "{};\\", expr).unwrap();
                if sj < bits {
                    self.write_left_side_assign(
                        i >= (bits_log - 2),
                        i == 0,
                        if (i & 1) ^ (bits_log & 1) != 0 {
                            "t"
                        } else {
                            "s"
                        },
                        sj,
                    );
                    let p0 = Self::format_op(
                        self.config.and_op,
                        &[&t0, self.config.constant_defs[2 * i + 1]],
                    );
                    let p0 = Self::format_op(self.config.shr32_op, &[&p0, &(1 << i).to_string()]);
                    let expr = if !t1.is_empty() {
                        let p1 = Self::format_op(
                            self.config.and_op,
                            &[&t1, self.config.constant_defs[2 * i + 1]],
                        );
                        Self::format_op(self.config.or_op, &[&p0, &p1])
                    } else {
                        p0
                    };
                    writeln!(self.out, "{};\\", expr).unwrap();
                }
            }
        }
        self.out.push_str("}\n");
    }

    pub fn out(self) -> String {
        self.out
    }
}

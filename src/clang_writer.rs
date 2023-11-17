use crate::*;

use std::io::Write;

pub struct CLangWriterConfig<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    include_name: Option<&'a str>,
    type_name: &'a str,
    type_bit_len: u32,
    arg_modifier: Option<&'a str>,
    and_op: &'a str,
    or_op: &'a str,
    xor_op: &'a str,
    impl_op: Option<&'a str>,
    nimpl_op: Option<&'a str>,
    not_op: Option<&'a str>,
    one_value: Option<(&'a str, &'a str)>, // for emulate NOT
}

#[cfg(target_pointer_width = "32")]
pub const CLANG_WRITER_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_name: "uint32_t",
    type_bit_len: 32,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

#[cfg(target_pointer_width = "64")]
pub const CLANG_WRITER_U64: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

pub const CLANG_WRITER_INTEL_MMX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("mmintrin.h"),
    type_name: "__m64",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "_m_pand({}, {})",
    or_op: "_m_por({}, {})",
    xor_op: "_m_pxor({}, {})",
    impl_op: None,
    nimpl_op: Some("_m_pandn({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };"##,
        "*((const __m64*)one_value)",
    )),
};

pub const CLANG_WRITER_INTEL_SSE: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("xmmintrin.h"),
    type_name: "__m128",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "_mm_and_ps({}, {})",
    or_op: "_mm_or_ps({}, {})",
    xor_op: "_mm_xor_ps({}, {})",
    impl_op: None,
    nimpl_op: Some("_mm_andnot_ps({1}, {0})"),
    not_op: None,
    one_value: Some((
        r##"static const unsigned int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };"##,
        "*((const __m128*)one_value)",
    )),
};

pub const CLANG_WRITER_INTEL_AVX: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
    type_name: "__m256",
    type_bit_len: 256,
    arg_modifier: None,
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
        "*((const __m256*)one_value)",
    )),
};

pub const CLANG_WRITER_INTEL_AVX512: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
    type_name: "__m512i",
    type_bit_len: 512,
    arg_modifier: None,
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
        "*((const __m512i*)one_value)",
    )),
};

pub const CLANG_WRITER_ARM_NEON: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("arm_neon.h"),
    type_name: "uint32x4_t",
    type_bit_len: 128,
    arg_modifier: None,
    and_op: "vandq_u32({}, {})",
    or_op: "vorrq_u32({}, {})",
    xor_op: "veorq_u32({}, {})",
    impl_op: Some("vornq_u32({1}, {0})"),
    nimpl_op: None,
    not_op: Some("vmvnq_u32({})"),
    one_value: None,
};

pub const CLANG_WRITER_OPENCL_U32: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: Some("kernel"),
    init_index: Some("const uint idx = get_global_id(0);"),
    include_name: None,
    type_name: "uint",
    type_bit_len: 32,
    arg_modifier: Some("global"),
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

pub struct CLangFuncWriter<'a, 'b, 'c> {
    writer: &'c mut CLangWriter<'a, 'b>,
    name: &'c str,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&'c [usize], usize)>,
    output_placement: Option<(&'c [usize], usize)>,
}

pub struct CLangWriter<'a, 'b> {
    config: &'a CLangWriterConfig<'a>,
    out: &'b mut Vec<u8>,
}

impl<'a> CLangWriterConfig<'a> {
    pub fn new<'b>(&'a self, out: &'b mut Vec<u8>) -> CLangWriter<'a, 'b> {
        CLangWriter { config: self, out }
    }
}

impl<'a, 'b> CLangWriter<'a, 'b> {
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

impl<'a, 'b, 'c> FuncWriter for CLangFuncWriter<'a, 'b, 'c> {
    fn func_start(&mut self) {
        if let Some(init_index) = self.writer.config.init_index {
            writeln!(
                self.writer.out,
                r##"{0}{1}void gate_sys_{2}(unsigned int n, const {3}{4}{5}* input,
    {3}{4}{5}* output) {{
    {6}"##,
                self.writer.config.func_modifier.unwrap_or(""),
                if self.writer.config.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.name,
                self.writer.config.arg_modifier.unwrap_or(""),
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.writer.config.type_name,
                init_index
            )
            .unwrap();
            write!(
                self.writer.out,
                concat!(
                    "    const unsigned int ivn = {} * idx;\n",
                    "    const unsigned int ovn = {} * idx;\n"
                ),
                self.input_placement
                    .map(|(_, len)| len)
                    .unwrap_or(self.input_len),
                self.output_placement
                    .map(|(_, len)| len)
                    .unwrap_or(self.output_len)
            )
            .unwrap();
        } else {
            writeln!(
                self.writer.out,
                r##"{0}{1}void gate_sys_{2}(const {3}{4}{5}* input,
    {3}{4}{5}* output) {{"##,
                self.writer.config.func_modifier.unwrap_or(""),
                if self.writer.config.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.name,
                self.writer.config.arg_modifier.unwrap_or(""),
                if self.writer.config.arg_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                self.writer.config.type_name
            )
            .unwrap();
        }
        if let Some((_, one_value)) = self.writer.config.one_value {
            writeln!(
                self.writer.out,
                "    const {} one = {};",
                self.writer.config.type_name, one_value
            )
            .unwrap();
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
        let input = self.input_placement.map(|(p, _)| p[input]).unwrap_or(input);
        if self.writer.config.init_index.is_some() {
            writeln!(self.writer.out, "    v{} = input[ivn + {}];", reg, input).unwrap();
        } else {
            writeln!(self.writer.out, "    v{} = input[{}];", reg, input).unwrap();
        }
    }
    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize) {
        let arg0 = format!("v{}", arg0);
        let arg1 = CLangWriter::<'a, 'b>::format_neg_arg(
            self.writer.config,
            negs == VNegs::NegInput1,
            arg1,
        );
        let mut op_vec = vec![];
        let args = [arg0.as_bytes(), arg1.as_bytes()];
        match op {
            InstrOp::And => {
                CLangWriter::<'a, 'b>::write_op(&mut op_vec, self.writer.config.and_op, &args)
            }
            InstrOp::Or => {
                CLangWriter::<'a, 'b>::write_op(&mut op_vec, self.writer.config.or_op, &args)
            }
            InstrOp::Xor => {
                CLangWriter::<'a, 'b>::write_op(&mut op_vec, self.writer.config.xor_op, &args)
            }
            InstrOp::Impl => CLangWriter::<'a, 'b>::write_op(
                &mut op_vec,
                self.writer.config.impl_op.unwrap(),
                &args,
            ),
            InstrOp::Nimpl => CLangWriter::<'a, 'b>::write_op(
                &mut op_vec,
                self.writer.config.nimpl_op.unwrap(),
                &args,
            ),
        };
        write!(self.writer.out, "    v{} = ", dst_arg).unwrap();
        if negs == VNegs::NegOutput {
            CLangWriter::<'a, 'b>::write_neg(self.writer.config, &mut self.writer.out, &op_vec);
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
        let arg = CLangWriter::<'a, 'b>::format_neg_arg(self.writer.config, neg, reg);
        if self.writer.config.init_index.is_some() {
            writeln!(self.writer.out, "    output[ovn + {}] = {};", output, arg).unwrap();
        } else {
            writeln!(self.writer.out, "    output[{}] = {};", output, arg).unwrap();
        }
    }
}

impl<'a, 'b, 'c> CodeWriter<'c, CLangFuncWriter<'a, 'b, 'c>> for CLangWriter<'a, 'b> {
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
        if let Some((init_one, _)) = self.config.one_value {
            self.out.extend(init_one.as_bytes());
            self.out.push(b'\n');
        }
    }

    fn epilog(&mut self) {}

    fn func_writer(
        &'c mut self,
        name: &'c str,
        input_len: usize,
        output_len: usize,
        input_placement: Option<(&'c [usize], usize)>,
        output_placement: Option<(&'c [usize], usize)>,
    ) -> CLangFuncWriter<'a, 'b, 'c> {
        CLangFuncWriter::<'a, 'b, 'c> {
            writer: self,
            name,
            input_len,
            output_len,
            input_placement,
            output_placement,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_test_code(cw_config: &CLangWriterConfig, inout_placement: bool) -> String {
        let mut out = vec![];
        let mut cw = cw_config.new(&mut out);
        let supported_ops = cw.supported_ops();
        cw.prolog();
        let mut fw = cw.func_writer(
            "func1",
            3,
            2,
            if inout_placement {
                Some((&[6, 11, 44], 68))
            } else {
                None
            },
            if inout_placement {
                Some((&[48, 72], 88))
            } else {
                None
            },
        );
        fw.func_start();
        fw.alloc_vars(5);
        fw.gen_load(2, 0);
        fw.gen_load(1, 1);
        fw.gen_load(0, 2);
        fw.gen_op(InstrOp::And, VNegs::NoNegs, 2, 0, 1);
        fw.gen_op(InstrOp::Or, VNegs::NoNegs, 1, 2, 1);
        fw.gen_op(InstrOp::Xor, VNegs::NoNegs, 3, 0, 1);
        fw.gen_op(InstrOp::And, VNegs::NegOutput, 3, 0, 1);
        if (supported_ops & (1u64 << INSTR_OP_VALUE_IMPL)) != 0 {
            fw.gen_op(InstrOp::Impl, VNegs::NoNegs, 3, 2, 1);
        }
        fw.gen_store(true, 1, 3);
        fw.gen_op(InstrOp::Or, VNegs::NegOutput, 2, 2, 3);
        fw.gen_op(InstrOp::Xor, VNegs::NegOutput, 4, 1, 3);
        fw.gen_op(InstrOp::And, VNegs::NegInput1, 4, 4, 1);
        fw.gen_op(InstrOp::Xor, VNegs::NegInput1, 4, 4, 1);
        if (supported_ops & (1u64 << INSTR_OP_VALUE_NIMPL)) != 0 {
            fw.gen_op(InstrOp::Nimpl, VNegs::NoNegs, 4, 2, 4);
        }
        fw.gen_store(false, 0, 4);
        fw.func_end();
        cw.epilog();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn test_clang_writer() {
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(
                r##"#include <stdint.h>
void gate_sys_func1(const uint32_t* input,
    uint32_t* output) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
                write_test_code(&CLANG_WRITER_U32, false)
            );
            assert_eq!(
                r##"#include <stdint.h>
void gate_sys_func1(const uint32_t* input,
    uint32_t* output) {
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    v2 = input[6];
    v1 = input[11];
    v0 = input[44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
                write_test_code(&CLANG_WRITER_U32, true)
            );
        }
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(
                r##"#include <stdint.h>
void gate_sys_func1(const uint64_t* input,
    uint64_t* output) {
    uint64_t v0;
    uint64_t v1;
    uint64_t v2;
    uint64_t v3;
    uint64_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[0] = v4;
}
"##,
                write_test_code(&CLANG_WRITER_U64, false)
            );
            assert_eq!(
                r##"#include <stdint.h>
void gate_sys_func1(const uint64_t* input,
    uint64_t* output) {
    uint64_t v0;
    uint64_t v1;
    uint64_t v2;
    uint64_t v3;
    uint64_t v4;
    v2 = input[6];
    v1 = input[11];
    v0 = input[44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[48] = v4;
}
"##,
                write_test_code(&CLANG_WRITER_U64, true)
            );
        }
        assert_eq!(
            r##"#include <mmintrin.h>
static const unsigned int one_value[2] = { 0xffffffff, 0xffffffff };
void gate_sys_func1(const __m64* input,
    __m64* output) {
    const __m64 one = *((const __m64*)one_value);
    __m64 v0;
    __m64 v1;
    __m64 v2;
    __m64 v3;
    __m64 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _m_pand(v0, v1);
    v1 = _m_por(v2, v1);
    v3 = _m_pxor(v0, v1);
    v3 = _m_pxor(_m_pand(v0, v1), one);
    output[1] = _m_pxor(v3, one);
    v2 = _m_pxor(_m_por(v2, v3), one);
    v4 = _m_pxor(_m_pxor(v1, v3), one);
    v4 = _m_pand(v4, _m_pxor(v1, one));
    v4 = _m_pxor(v4, _m_pxor(v1, one));
    v4 = _m_pandn(v4, v2);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_INTEL_MMX, false)
        );
        assert_eq!(
            r##"#include <xmmintrin.h>
static const unsigned int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
void gate_sys_func1(const __m128* input,
    __m128* output) {
    const __m128 one = *((const __m128*)one_value);
    __m128 v0;
    __m128 v1;
    __m128 v2;
    __m128 v3;
    __m128 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm_and_ps(v0, v1);
    v1 = _mm_or_ps(v2, v1);
    v3 = _mm_xor_ps(v0, v1);
    v3 = _mm_xor_ps(_mm_and_ps(v0, v1), one);
    output[1] = _mm_xor_ps(v3, one);
    v2 = _mm_xor_ps(_mm_or_ps(v2, v3), one);
    v4 = _mm_xor_ps(_mm_xor_ps(v1, v3), one);
    v4 = _mm_and_ps(v4, _mm_xor_ps(v1, one));
    v4 = _mm_xor_ps(v4, _mm_xor_ps(v1, one));
    v4 = _mm_andnot_ps(v4, v2);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_INTEL_SSE, false)
        );
        assert_eq!(
            r##"#include <immintrin.h>
static const unsigned int one_value[8] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
void gate_sys_func1(const __m256* input,
    __m256* output) {
    const __m256 one = *((const __m256*)one_value);
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm256_and_ps(v0, v1);
    v1 = _mm256_or_ps(v2, v1);
    v3 = _mm256_xor_ps(v0, v1);
    v3 = _mm256_xor_ps(_mm256_and_ps(v0, v1), one);
    output[1] = _mm256_xor_ps(v3, one);
    v2 = _mm256_xor_ps(_mm256_or_ps(v2, v3), one);
    v4 = _mm256_xor_ps(_mm256_xor_ps(v1, v3), one);
    v4 = _mm256_and_ps(v4, _mm256_xor_ps(v1, one));
    v4 = _mm256_xor_ps(v4, _mm256_xor_ps(v1, one));
    v4 = _mm256_andnot_ps(v4, v2);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_INTEL_AVX, false)
        );
        assert_eq!(
            r##"#include <immintrin.h>
static const unsigned int one_value[16] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
void gate_sys_func1(const __m512i* input,
    __m512i* output) {
    const __m512i one = *((const __m512i*)one_value);
    __m512i v0;
    __m512i v1;
    __m512i v2;
    __m512i v3;
    __m512i v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = _mm512_and_epi64(v0, v1);
    v1 = _mm512_or_epi64(v2, v1);
    v3 = _mm512_xor_epi64(v0, v1);
    v3 = _mm512_xor_epi64(_mm512_and_epi64(v0, v1), one);
    output[1] = _mm512_xor_epi64(v3, one);
    v2 = _mm512_xor_epi64(_mm512_or_epi64(v2, v3), one);
    v4 = _mm512_xor_epi64(_mm512_xor_epi64(v1, v3), one);
    v4 = _mm512_and_epi64(v4, _mm512_xor_epi64(v1, one));
    v4 = _mm512_xor_epi64(v4, _mm512_xor_epi64(v1, one));
    v4 = _mm512_andnot_epi64(v4, v2);
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_INTEL_AVX512, false)
        );
        assert_eq!(
            r##"#include <arm_neon.h>
void gate_sys_func1(const uint32x4_t* input,
    uint32x4_t* output) {
    uint32x4_t v0;
    uint32x4_t v1;
    uint32x4_t v2;
    uint32x4_t v3;
    uint32x4_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = vandq_u32(v0, v1);
    v1 = vorrq_u32(v2, v1);
    v3 = veorq_u32(v0, v1);
    v3 = vmvnq_u32(vandq_u32(v0, v1));
    v3 = vornq_u32(v1, v2);
    output[1] = vmvnq_u32(v3);
    v2 = vmvnq_u32(vorrq_u32(v2, v3));
    v4 = vmvnq_u32(veorq_u32(v1, v3));
    v4 = vandq_u32(v4, vmvnq_u32(v1));
    v4 = veorq_u32(v4, vmvnq_u32(v1));
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_ARM_NEON, false)
        );
        assert_eq!(
            r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 3 * idx;
    const unsigned int ovn = 2 * idx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = input[ivn + 0];
    v1 = input[ivn + 1];
    v0 = input[ivn + 2];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 1] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_OPENCL_U32, false)
        );
        assert_eq!(
            r##"kernel void gate_sys_func1(unsigned int n, const global uint* input,
    global uint* output) {
    const uint idx = get_global_id(0);
    const unsigned int ivn = 68 * idx;
    const unsigned int ovn = 88 * idx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    if (idx >= n) return;
    v2 = input[ivn + 6];
    v1 = input[ivn + 11];
    v0 = input[ivn + 44];
    v2 = (v0 & v1);
    v1 = (v2 | v1);
    v3 = (v0 ^ v1);
    v3 = ~(v0 & v1);
    output[ovn + 72] = ~v3;
    v2 = ~(v2 | v3);
    v4 = ~(v1 ^ v3);
    v4 = (v4 & ~v1);
    v4 = (v4 ^ ~v1);
    output[ovn + 48] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_OPENCL_U32, true)
        );
    }
}

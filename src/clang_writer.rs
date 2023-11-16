use crate::*;

use std::io::Write;

pub struct CLangWriter<'a> {
    func_modifier: Option<&'a str>,
    init_index: Option<&'a str>, // to initialize index in OpenCL kernel
    include_name: Option<&'a str>,
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
    type_name: "uint32_t",
    type_bit_len: 32,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

const CLANG_WRITER_U64: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

const CLANG_WRITER_INTEL_MMX: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("mmintrin.h"),
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
        "*((const __m64*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_SSE: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("xmmintrin.h"),
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
        "*((const __m128*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_AVX: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
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
        "*((const __m256*)one_value)",
    )),
};

const CLANG_WRITER_INTEL_AVX512: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("immintrin.h"),
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
        "*((const __m512i*)one_value)",
    )),
};

const CLANG_WRITER_ARM_NEON: CLangWriter<'_> = CLangWriter {
    func_modifier: None,
    init_index: None,
    include_name: Some("arm_neon.h"),
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
    init_index: Some("const uint idx = get_global_id(0);"),
    include_name: None,
    type_name: "uint",
    type_bit_len: 32,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: None,
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
};

impl<'a> CLangWriter<'a> {
    fn write_op(&self, out: &mut Vec<u8>, op: &str, args: &[&[u8]]) {
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

    fn write_neg(&self, out: &mut Vec<u8>, arg: &[u8]) {
        if let Some(op) = self.not_op {
            self.write_op(out, op, &[arg]);
        } else {
            self.write_op(out, self.xor_op, &[arg, b"one"]);
        }
    }

    fn format_neg_arg(&self, neg: bool, reg: usize) -> String {
        if neg {
            let arg = format!("v{}", reg);
            let mut out_str = vec![];
            self.write_neg(&mut out_str, arg.as_bytes());
            String::from_utf8_lossy(&out_str).to_string()
        } else {
            format!("v{}", reg)
        }
    }
}

impl<'a> CodeWriter for CLangWriter<'a> {
    fn supported_ops(&self) -> u64 {
        let basic_ops = (1u64 << InstrOp::And.int_value())
            | (1u64 << InstrOp::Or.int_value())
            | (1u64 << InstrOp::Xor.int_value());
        let basic_impl_ops = basic_ops | (1u64 << InstrOp::Impl.int_value());
        let basic_nimpl_ops = basic_ops | (1u64 << InstrOp::Nimpl.int_value());
        if self.impl_op.is_some() {
            basic_impl_ops
        } else if self.nimpl_op.is_some() {
            basic_nimpl_ops
        } else {
            basic_ops
        }
    }
    fn word_len(&self) -> u32 {
        self.type_bit_len
    }
    fn max_var_num(&self) -> usize {
        usize::MAX
    }
    fn preferred_var_num(&self) -> usize {
        10000
    }
    fn prolog(&self, out: &mut Vec<u8>) {
        if let Some(include_name) = self.include_name {
            writeln!(out, "#include <{}>", include_name).unwrap();
        }
        if let Some((init_one, _)) = self.one_value {
            out.extend(init_one.as_bytes());
            out.push(b'\n');
        }
    }

    fn epilog(&self, _out: &mut Vec<u8>) {}

    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize) {
        if let Some(init_index) = self.init_index {
            writeln!(
                out,
                r##"{0}{1}void gate_sys_{2}(unsigned int n, const {3}* input, {3}* output) {{
    {4}"##,
                self.func_modifier.unwrap_or(""),
                if self.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                name,
                self.type_name,
                init_index
            )
            .unwrap();
            write!(
                out,
                concat!(
                    "    const unsigned int ivn = {} * idx;\n",
                    "    const unsigned int ovn = {} * idx;\n"
                ),
                input_len, output_len
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "{0}{1}void gate_sys_{2}(const {3}* input, {3}* output) {{",
                self.func_modifier.unwrap_or(""),
                if self.func_modifier.is_some() {
                    " "
                } else {
                    ""
                },
                name,
                self.type_name
            )
            .unwrap();
        }
        if let Some((_, one_value)) = self.one_value {
            writeln!(out, "    const {} one = {};", self.type_name, one_value).unwrap();
        }
    }
    fn func_end(&self, out: &mut Vec<u8>, _name: &str) {
        out.extend(b"}\n");
    }
    fn alloc_vars(&self, out: &mut Vec<u8>, var_num: usize) {
        for i in 0..var_num {
            writeln!(out, "    {} v{};", self.type_name, i).unwrap();
        }
        if self.init_index.is_some() {
            out.extend(b"    if (idx >= n) return;\n");
        }
    }

    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize) {
        if self.init_index.is_some() {
            writeln!(out, "    v{} = input[ivn + {}];", reg, input).unwrap();
        } else {
            writeln!(out, "    v{} = input[{}];", reg, input).unwrap();
        }
    }
    fn gen_op(
        &self,
        out: &mut Vec<u8>,
        op: InstrOp,
        negs: VNegs,
        dst_arg: usize,
        arg0: usize,
        arg1: usize,
    ) {
        let arg0 = format!("v{}", arg0);
        let arg1 = self.format_neg_arg(negs == VNegs::NegInput1, arg1);
        let mut op_vec = vec![];
        let args = [arg0.as_bytes(), arg1.as_bytes()];
        match op {
            InstrOp::And => self.write_op(&mut op_vec, self.and_op, &args),
            InstrOp::Or => self.write_op(&mut op_vec, self.or_op, &args),
            InstrOp::Xor => self.write_op(&mut op_vec, self.xor_op, &args),
            InstrOp::Impl => self.write_op(&mut op_vec, self.impl_op.unwrap(), &args),
            InstrOp::Nimpl => self.write_op(&mut op_vec, self.nimpl_op.unwrap(), &args),
        };
        write!(out, "    v{} = ", dst_arg).unwrap();
        if negs == VNegs::NegOutput {
            self.write_neg(out, &op_vec);
        } else {
            out.extend(op_vec);
        }
        out.extend(b";\n");
    }

    fn gen_store(&self, out: &mut Vec<u8>, neg: bool, output: usize, reg: usize) {
        let arg = self.format_neg_arg(neg, reg);
        if self.init_index.is_some() {
            writeln!(out, "    output[ovn + {}] = {};", output, arg).unwrap();
        } else {
            writeln!(out, "    output[{}] = {};", output, arg).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_test_code<CW: CodeWriter>(cw: &CW) -> String {
        let mut out = vec![];
        cw.prolog(&mut out);
        cw.func_start(&mut out, "func1", 3, 2);
        cw.alloc_vars(&mut out, 5);
        cw.gen_load(&mut out, 2, 0);
        cw.gen_load(&mut out, 1, 1);
        cw.gen_load(&mut out, 0, 2);
        cw.gen_op(&mut out, InstrOp::And, VNegs::NoNegs, 2, 0, 1);
        cw.gen_op(&mut out, InstrOp::Or, VNegs::NoNegs, 1, 2, 1);
        cw.gen_op(&mut out, InstrOp::Xor, VNegs::NoNegs, 3, 0, 1);
        cw.gen_op(&mut out, InstrOp::And, VNegs::NegOutput, 3, 0, 1);
        if (cw.supported_ops() & (1u64 << INSTR_OP_VALUE_IMPL)) != 0 {
            cw.gen_op(&mut out, InstrOp::Impl, VNegs::NoNegs, 3, 2, 1);
        }
        cw.gen_store(&mut out, true, 1, 3);
        cw.gen_op(&mut out, InstrOp::Or, VNegs::NegOutput, 2, 2, 3);
        cw.gen_op(&mut out, InstrOp::Xor, VNegs::NegOutput, 4, 1, 3);
        cw.gen_op(&mut out, InstrOp::And, VNegs::NegInput1, 4, 4, 1);
        cw.gen_op(&mut out, InstrOp::Xor, VNegs::NegInput1, 4, 4, 1);
        if (cw.supported_ops() & (1u64 << INSTR_OP_VALUE_NIMPL)) != 0 {
            cw.gen_op(&mut out, InstrOp::Nimpl, VNegs::NoNegs, 4, 2, 4);
        }
        cw.gen_store(&mut out, false, 0, 4);
        cw.func_end(&mut out, "func1");
        cw.epilog(&mut out);
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn test_clang_writer() {
        assert_eq!(
            r##"#include <stdint.h>
void gate_sys_func1(const uint32_t* input, uint32_t* output) {
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
            write_test_code(&CLANG_WRITER_U32)
        );
        assert_eq!(
            r##"#include <stdint.h>
void gate_sys_func1(const uint64_t* input, uint64_t* output) {
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
            write_test_code(&CLANG_WRITER_U64)
        );
        assert_eq!(
            r##"#include <mmintrin.h>
static const unsinged int one_value[2] = { 0xffffffff, 0xffffffff };
void gate_sys_func1(const __m64* input, __m64* output) {
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
            write_test_code(&CLANG_WRITER_INTEL_MMX)
        );
        assert_eq!(
            r##"#include <xmmintrin.h>
static const unsinged int one_value[4] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
void gate_sys_func1(const __m128* input, __m128* output) {
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
            write_test_code(&CLANG_WRITER_INTEL_SSE)
        );
        assert_eq!(
            r##"#include <immintrin.h>
static const unsigned int one_value[8] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
void gate_sys_func1(const __m256* input, __m256* output) {
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
            write_test_code(&CLANG_WRITER_INTEL_AVX)
        );
        assert_eq!(
            r##"#include <immintrin.h>
static const unsigned int one_value[16] = {
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
};
void gate_sys_func1(const __m512i* input, __m512i* output) {
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
            write_test_code(&CLANG_WRITER_INTEL_AVX512)
        );
        assert_eq!(
            r##"#include <arm_neon.h>
void gate_sys_func1(const uint32x4_t* input, uint32x4_t* output) {
    uint32x4_t v0;
    uint32x4_t v1;
    uint32x4_t v2;
    uint32x4_t v3;
    uint32x4_t v4;
    v2 = input[0];
    v1 = input[1];
    v0 = input[2];
    v2 = vandq_u32(v0, v1);
    v1 = vorq_u32(v2, v1);
    v3 = veorq_u32(v0, v1);
    v3 = vmvnq_u32(vandq_u32(v0, v1));
    v3 = vornq_u32(v1, v2);
    output[1] = vmvnq_u32(v3);
    v2 = vmvnq_u32(vorq_u32(v2, v3));
    v4 = vmvnq_u32(veorq_u32(v1, v3));
    v4 = vandq_u32(v4, vmvnq_u32(v1));
    v4 = veorq_u32(v4, vmvnq_u32(v1));
    output[0] = v4;
}
"##,
            write_test_code(&CLANG_WRITER_ARM_NEON)
        );
        assert_eq!(
            r##"kernel void gate_sys_func1(unsigned int n, const uint* input, uint* output) {
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
            write_test_code(&CLANG_WRITER_OPENCL_U32)
        );
    }
}

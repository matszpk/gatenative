use crate::*;

use std::io::Write;

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
        "*((const __m64*)one_value)",
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
        "*((const __m128*)one_value)",
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
        "*((const __m256*)one_value)",
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
        "*((const __m512i*)one_value)",
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

impl<'a> CLangWriter<'a> {
    fn write_op(&self, out: &mut Vec<u8>, op: &str, args: &[&str]) {
        let mut rest = op;
        let mut arg_index = 0;
        while let Some(p) = rest.find('{') {
            out.extend(rest[..p].bytes());
            rest = &rest[p + 1..];
            if let Some(endr) = rest.find('}') {
                if rest[..endr].is_empty() {
                    // fetch next argument
                    out.extend(args[arg_index].bytes());
                    arg_index += 1;
                } else {
                    // fetch argument with index given between {}
                    let index = usize::from_str_radix(&rest[..endr], 10).unwrap();
                    out.extend(args[index].bytes());
                }
                rest = &rest[endr + 1..];
            } else {
                panic!("Unexpected");
            }
            rest = &rest[p..];
        }
        if !rest.is_empty() {
            out.extend(rest.bytes());
        }
    }

    fn write_neg(&self, out: &mut Vec<u8>, arg: &str) {
        if let Some(op) = self.not_op {
            self.write_op(out, op, &[arg]);
        } else {
            self.write_op(out, self.xor_op, &[arg, "one"]);
        }
    }

    fn format_neg_arg(&self, neg: bool, reg: usize) -> String {
        if neg {
            let arg = format!("v{}", reg);
            let mut out_str = vec![];
            self.write_neg(&mut out_str, &arg);
            String::from_utf8(out_str).unwrap()
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
            writeln!(out, "#include \"{}\"", include_name).unwrap();
        }
        if let Some((init_one, _)) = self.one_value {
            out.extend(init_one.bytes());
            out.push(b'\n');
        }
    }
    fn epilog(&self, out: &mut Vec<u8>) {}
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize) {
        if let Some(init_index) = self.init_index {
            writeln!(
                out,
                r##"{0} void gate_sys_{1}(unsigned int n, const {2}* input, {2}* output) {{
    {3}
    if idx < n {{ return; }}"##,
                self.func_modifier.unwrap_or(""),
                name,
                self.type_name,
                init_index
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "{0} void gate_sys_{1}(const {2}* input, {2}* output) {{",
                self.func_modifier.unwrap_or(""),
                name,
                self.type_name
            )
            .unwrap();
        }
        if let Some((_, one_value)) = self.one_value {
            writeln!(out, "    {} one = {}", self.type_name, one_value).unwrap();
        }
    }
    fn func_end(&self, out: &mut Vec<u8>, name: &str) {
        out.extend(b"}\n");
    }
    fn alloc_vars(&self, out: &mut Vec<u8>, var_num: usize) {
        for i in 0..var_num {
            writeln!(out, "    {} v{};", self.type_name, i).unwrap();
        }
    }

    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize) {
        if self.init_index.is_some() {
            writeln!(out, "    v{} = input[{} * n + idx];", reg, input).unwrap();
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
        match op {
            InstrOp::And => self.write_op(&mut op_vec, self.and_op, &[&arg0, &arg1]),
            InstrOp::Or => self.write_op(&mut op_vec, self.or_op, &[&arg0, &arg1]),
            InstrOp::Xor => self.write_op(&mut op_vec, self.or_op, &[&arg0, &arg1]),
            InstrOp::Impl => self.write_op(&mut op_vec, self.impl_op.unwrap(), &[&arg0, &arg1]),
            InstrOp::Nimpl => self.write_op(&mut op_vec, self.nimpl_op.unwrap(), &[&arg0, &arg1]),
        };
        if negs == VNegs::NegOutput {
            let s = String::from_utf8(op_vec).unwrap();
            self.write_neg(out, &s);
        } else {
            out.extend(op_vec);
        }
    }

    fn gen_store(&self, out: &mut Vec<u8>, neg: bool, output: usize, reg: usize) {
        let arg = self.format_neg_arg(neg, reg);
        if self.init_index.is_some() {
            writeln!(out, "    output[{} * n + idx] = {};", output, arg).unwrap();
        } else {
            writeln!(out, "    output[{}] = {};", output, arg).unwrap();
        }
    }
}

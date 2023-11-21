use crate::*;

use std::io::Write;

#[derive(Clone, Debug)]
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
    load_op: Option<&'a str>,
    store_op: Option<&'a str>,
}

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
    load_op: None,
    store_op: None,
};

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
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_U64_TEST_IMPL: CLangWriterConfig<'_> = CLangWriterConfig {
    func_modifier: None,
    init_index: None,
    include_name: Some("stdint.h"),
    type_name: "uint64_t",
    type_bit_len: 64,
    arg_modifier: None,
    and_op: "({} & {})",
    or_op: "({} | {})",
    xor_op: "({} ^ {})",
    impl_op: Some("(~{} | {})"),
    nimpl_op: None,
    not_op: Some("~{}"),
    one_value: None,
    load_op: None,
    store_op: None,
};

pub const CLANG_WRITER_U64_TEST_NIMPL: CLangWriterConfig<'_> = CLangWriterConfig {
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
    nimpl_op: Some("({} & ~{})"),
    not_op: Some("~{}"),
    one_value: None,
    load_op: None,
    store_op: None,
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
    load_op: None,
    store_op: None,
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
    load_op: None,
    store_op: None,
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
    load_op: Some("_mm256_loadu_ps((const float*)&{})"),
    store_op: Some("_mm256_storeu_ps((float*)&{}, {})"),
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
    load_op: Some("_mm512_loadu_epi64(&{})"),
    store_op: Some("_mm512_storeu_epi64(&{}, {})"),
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
    load_op: None,
    store_op: None,
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
}

pub struct CLangWriter<'a> {
    config: &'a CLangWriterConfig<'a>,
    out: Vec<u8>,
}

impl<'a> CLangWriterConfig<'a> {
    pub fn writer(&'a self) -> CLangWriter<'a> {
        CLangWriter {
            config: self,
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
        let (dst, r) = if self.writer.config.init_index.is_some() {
            (format!("v{}", reg), format!("input[ivn + {}]", input))
        } else {
            (format!("v{}", reg), format!("input[{}]", input))
        };
        if let Some(ld_op) = self.writer.config.load_op {
            write!(self.writer.out, "    {} = ", dst).unwrap();
            CLangWriter::<'a>::write_op(&mut self.writer.out, ld_op, &[r.as_bytes()]);
            self.writer.out.extend(b";\n");
        } else {
            writeln!(self.writer.out, "    {} = {};", dst, r).unwrap();
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
            (format!("output[ovn + {}]", output), format!("{}", arg))
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
    ) -> CLangFuncWriter<'a, 'c> {
        CLangFuncWriter::<'a, 'c> {
            writer: self,
            name,
            input_len,
            output_len,
            input_placement,
            output_placement,
        }
    }

    fn out(self) -> Vec<u8> {
        self.out
    }
}

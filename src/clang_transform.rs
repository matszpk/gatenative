use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct CLangTransformConfig<'a> {
    final_type_name: &'a str,
    final_type_bit_len: u32,
    inter_type_name: &'a str,
    inter_type_bit_len: u32,
    and_op: &'a str,
    or_op: &'a str,
    shl32_op: &'a str,
    shr32_op: &'a str,
    unpack_ops: [Option<&'a str>; 5],
    init_defs: &'a str,
    constant_defs: [&'a str; 2 * 5],
}

pub const CLANG_TRANSFORM_U32: CLangTransformConfig<'_> = CLangTransformConfig {
    final_type_name: "uint32_t",
    final_type_bit_len: 32,
    inter_type_name: "uint32_t",
    inter_type_bit_len: 32,
    and_op: "({} & {})",
    or_op: "({} | {})",
    shl32_op: "({} << {})",
    shr32_op: "({} >> {})",
    unpack_ops: [None, None, None, None, None],
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
        format!("(S{})", arg)
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
                self.config.inter_type_name, prefix, i
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
            &((0..32).map(|i| format!("D{}", i)).collect::<Vec<_>>()).join(", "),
            &((0..32).map(|i| format!("S{}", i)).collect::<Vec<_>>()).join(", "),
        )
        .unwrap();
        self.out.push_str("{ \\\n");
        for i in (0..5).rev() {
            for j in 0..16 {
                let fj = ((j >> i) << (i + 1)) | (j & ((1 << i) - 1));
                let sj = fj | (1 << i);
                let t0 = if i == 4 {
                    Self::format_arg_s(fj)
                } else if (i & 1) != 0 {
                    format!("t{}", fj)
                } else {
                    format!("s{}", fj)
                };
                let t1 = if i == 4 {
                    Self::format_arg_s(sj)
                } else if (i & 1) != 0 {
                    format!("t{}", sj)
                } else {
                    format!("s{}", sj)
                };
                self.write_left_side_assign(
                    i >= 3,
                    i == 0,
                    if (i & 1) == 0 { "t" } else { "s" },
                    fj,
                );
                let p0 =
                    Self::format_op(self.config.and_op, &[&t0, self.config.constant_defs[2 * i]]);
                let p1 =
                    Self::format_op(self.config.and_op, &[&t1, self.config.constant_defs[2 * i]]);
                let p1 = Self::format_op(self.config.shl32_op, &[&p1, &(1 << i).to_string()]);
                let expr = Self::format_op(self.config.or_op, &[&p0, &p1]);
                writeln!(self.out, "{};\\", expr).unwrap();
                self.write_left_side_assign(
                    i >= 3,
                    i == 0,
                    if (i & 1) == 0 { "t" } else { "s" },
                    sj,
                );
                let p0 = Self::format_op(
                    self.config.and_op,
                    &[&t0, self.config.constant_defs[2 * i + 1]],
                );
                let p0 = Self::format_op(self.config.shr32_op, &[&p0, &(1 << i).to_string()]);
                let p1 = Self::format_op(
                    self.config.and_op,
                    &[&t1, self.config.constant_defs[2 * i + 1]],
                );
                let expr = Self::format_op(self.config.or_op, &[&p0, &p1]);
                writeln!(self.out, "{};\\", expr).unwrap();
            }
        }
        self.out.push_str("}");
    }

    pub fn out(self) -> String {
        self.out
    }
}

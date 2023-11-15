use gatenative::*;
use gatesim::*;
use int_enum::IntEnum;

struct TestCodeWriter {
    supp_ops: u64,
}

use std::io::Write;

impl CodeWriter for TestCodeWriter {
    fn supported_ops(&self) -> u64 {
        self.supp_ops
    }
    fn word_len(&self) -> u32 {
        32
    }
    fn max_var_num(&self) -> usize {
        usize::MAX
    }
    fn preferred_var_num(&self) -> usize {
        1000000
    }
    /// Generates prolog.
    fn prolog(&self, out: &mut Vec<u8>) {
        writeln!(out, "Start").unwrap();
    }
    /// Generates epilog;
    fn epilog(&self, out: &mut Vec<u8>) {
        writeln!(out, "End").unwrap();
    }
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize) {
        writeln!(out, "Func {}({} {})", name, input_len, output_len).unwrap();
    }
    fn func_end(&self, out: &mut Vec<u8>, _name: &str) {
        writeln!(out, "EndFunc").unwrap();
    }
    fn alloc_vars(&self, out: &mut Vec<u8>, var_num: usize) {
        writeln!(out, "  vars v0..{}", var_num).unwrap();
    }
    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize) {
        writeln!(out, "  v{} = I{}", reg, input).unwrap();
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
        writeln!(
            out,
            "  v{} = {}(v{} {} {}v{})",
            dst_arg,
            if negs == VNegs::NegOutput { "~" } else { "" },
            arg0,
            match op {
                InstrOp::And => "and",
                InstrOp::Or => "or",
                InstrOp::Xor => "xor",
                InstrOp::Impl => "impl",
                InstrOp::Nimpl => "nimpl",
            },
            if negs == VNegs::NegInput1 { "~" } else { "" },
            arg1
        )
        .unwrap();
    }
    fn gen_store(&self, out: &mut Vec<u8>, neg: bool, output: usize, reg: usize) {
        writeln!(
            out,
            "  O{} = {}v{}",
            output,
            if neg { "~" } else { "" },
            reg
        )
        .unwrap();
    }
}

#[test]
fn test_generate_code() {
    let circuit = Circuit::new(
        3,
        [
            Gate::new_xor(0, 1),
            Gate::new_xor(2, 3),
            Gate::new_and(2, 3),
            Gate::new_nimpl(0, 1),
            Gate::new_nor(5, 6),
        ],
        [(4, false), (7, false)],
    )
    .unwrap();

    let basic_ops = (1u64 << InstrOp::And.int_value())
        | (1u64 << InstrOp::Or.int_value())
        | (1u64 << InstrOp::Xor.int_value());
    let basic_impl_ops = basic_ops | (1u64 << InstrOp::Impl.int_value());
    let basic_nimpl_ops = basic_ops | (1u64 << InstrOp::Nimpl.int_value());
    let cw = TestCodeWriter {
        supp_ops: basic_impl_ops,
    };
    let mut out = vec![];
    generate_code(&cw, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
  vars v0..5
  v0 = I0
  v1 = I1
  v2 = I2
  v3 = (v0 xor v1)
  v4 = (v2 xor v3)
  O0 = v4
  v2 = (v2 and v3)
  v0 = (v0 impl v1)
  v0 = (v0 impl v2)
  O1 = ~v0
EndFunc
"##
    );

    let cw = TestCodeWriter {
        supp_ops: basic_nimpl_ops,
    };
    let mut out = vec![];
    generate_code(&cw, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
  vars v0..5
  v0 = I0
  v1 = I1
  v2 = I2
  v3 = (v0 xor v1)
  v4 = (v2 xor v3)
  O0 = v4
  v2 = (v2 and v3)
  v0 = (v0 nimpl v1)
  v0 = (v2 or v0)
  O1 = ~v0
EndFunc
"##
    );

    let cw = TestCodeWriter {
        supp_ops: basic_ops,
    };
    let mut out = vec![];
    generate_code(&cw, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
  vars v0..5
  v0 = I0
  v1 = I1
  v2 = I2
  v3 = (v0 xor v1)
  v4 = (v2 xor v3)
  O0 = v4
  v2 = (v2 and v3)
  v0 = (v0 and ~v1)
  v0 = ~(v2 or v0)
  O1 = v0
EndFunc
"##
    );
}

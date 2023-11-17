use gatenative::gencode::*;
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
    fn prolog(&self, out: &mut Vec<u8>) {
        writeln!(out, "Start").unwrap();
    }
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
        writeln!(out, "    vars v0..{}", var_num).unwrap();
    }
    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize) {
        writeln!(out, "    v{} = I{}", reg, input).unwrap();
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
            "    v{} = {}(v{} {} {}v{})",
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
            "    O{} = {}v{}",
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
    let cw_impl = TestCodeWriter {
        supp_ops: basic_impl_ops,
    };
    let cw_nimpl = TestCodeWriter {
        supp_ops: basic_nimpl_ops,
    };
    let cw_basic = TestCodeWriter {
        supp_ops: basic_ops,
    };
    let mut out = vec![];
    generate_code(&cw_impl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = (v0 xor v1)
    v4 = (v2 xor v3)
    v2 = (v2 and v3)
    v0 = (v0 impl v1)
    v0 = (v0 impl v2)
    O0 = v4
    O1 = ~v0
EndFunc
"##
    );

    let mut out = vec![];
    generate_code(&cw_nimpl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = (v0 xor v1)
    v4 = (v2 xor v3)
    v2 = (v2 and v3)
    v0 = (v0 nimpl v1)
    v0 = (v2 or v0)
    O0 = v4
    O1 = ~v0
EndFunc
"##
    );

    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = (v0 xor v1)
    v4 = (v2 xor v3)
    v2 = (v2 and v3)
    v0 = (v0 and ~v1)
    v0 = ~(v2 or v0)
    O0 = v4
    O1 = v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        4,
        [
            Gate::new_nimpl(0, 1),   // 4
            Gate::new_and(0, 3),     // 5
            Gate::new_xor(1, 4),     // 6
            Gate::new_and(3, 5),     // 7
            Gate::new_xor(2, 6),     // 8
            Gate::new_xor(3, 7),     // 9
            Gate::new_nor(8, 9),     // 10
            Gate::new_and(8, 9),     // 11
            Gate::new_nimpl(8, 9),   // 12
            Gate::new_nor(0, 10),    // 13
            Gate::new_nor(1, 11),    // 14
            Gate::new_xor(2, 12),    // 15
            Gate::new_xor(13, 14),   // 16
            Gate::new_and(0, 10),    // 17 tree4
            Gate::new_nor(15, 16),   // 18 tree3
            Gate::new_nimpl(1, 12),  // 19 tree4
            Gate::new_nimpl(11, 17), // 20
            Gate::new_nimpl(3, 19),  // 21
            Gate::new_xor(20, 21),   // 22
        ],
        [(18, true), (22, false)],
    )
    .unwrap();

    let mut out = vec![];
    generate_code(&cw_impl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 impl v1)
    v4 = (v1 xor v4)
    v4 = (v2 xor v4)
    v5 = (v0 and v3)
    v5 = (v3 and v5)
    v5 = (v3 xor v5)
    v6 = (v4 or v5)
    v2 = (v2 xor v6)
    v7 = (v4 impl v5)
    v8 = (v7 impl v0)
    v4 = (v5 impl v4)
    v5 = (v4 impl v1)
    v5 = (v8 xor v5)
    v2 = (v2 impl v5)
    v0 = (v0 impl v7)
    v0 = (v0 impl v4)
    v1 = (v1 and v6)
    v1 = (v3 impl v1)
    v0 = (v0 xor v1)
    O0 = v2
    O1 = v0
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_nimpl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 nimpl v1)
    v4 = (v1 xor v4)
    v4 = (v2 xor v4)
    v5 = (v0 and v3)
    v5 = (v3 and v5)
    v5 = (v3 xor v5)
    v6 = (v4 nimpl v5)
    v2 = (v2 xor v6)
    v7 = (v4 or v5)
    v8 = (v7 nimpl v0)
    v4 = (v4 and v5)
    v5 = (v1 or v4)
    v5 = (v8 xor v5)
    v2 = (v5 nimpl v2)
    v0 = (v0 nimpl v7)
    v0 = (v4 nimpl v0)
    v1 = (v1 nimpl v6)
    v1 = (v3 nimpl v1)
    v0 = (v0 xor v1)
    O0 = ~v2
    O1 = v0
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 and ~v1)
    v4 = (v1 xor v4)
    v4 = (v2 xor v4)
    v5 = (v0 and v3)
    v5 = (v3 and v5)
    v5 = (v3 xor v5)
    v6 = (v4 and ~v5)
    v2 = (v2 xor v6)
    v7 = ~(v4 or v5)
    v8 = ~(v0 or v7)
    v4 = (v4 and v5)
    v5 = ~(v1 or v4)
    v5 = (v8 xor v5)
    v2 = ~(v2 or v5)
    v0 = (v0 and v7)
    v0 = (v4 and ~v0)
    v1 = (v1 and ~v6)
    v1 = (v3 and ~v1)
    v0 = (v0 xor v1)
    O0 = ~v2
    O1 = v0
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), true);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 and ~v1)
    v4 = (v1 xor v4)
    v4 = (v2 xor v4)
    v5 = (v0 and v3)
    v5 = (v3 and v5)
    v5 = (v3 xor v5)
    v6 = (v4 and ~v5)
    v2 = (v2 xor v6)
    v7 = ~(v4 or v5)
    v8 = (v0 or v7)
    v4 = (v4 and v5)
    v5 = (v1 or v4)
    v5 = (v8 xor v5)
    v2 = (v2 or v5)
    v0 = (v0 and v7)
    v0 = (v4 and ~v0)
    v1 = (v6 or ~v1)
    v1 = (v3 and v1)
    v0 = (v0 xor v1)
    O0 = v2
    O1 = v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        3,
        [
            Gate::new_and(0, 1),
            Gate::new_and(1, 2),
            Gate::new_nimpl(3, 4),
            Gate::new_nimpl(5, 1),
        ],
        [(6, true)],
    )
    .unwrap();
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 1)
    vars v0..3
    v0 = I0
    v1 = I1
    v2 = I2
    v0 = (v0 and v1)
    v2 = (v1 and v2)
    v0 = (v0 and ~v2)
    v0 = (v0 and ~v1)
    O0 = ~v0
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), true);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(3 1)
    vars v0..3
    v0 = I0
    v1 = I1
    v2 = I2
    v0 = (v0 and v1)
    v2 = (v1 and v2)
    v0 = (v2 or ~v0)
    v0 = (v0 or v1)
    O0 = v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(0, 2),
            Gate::new_and(1, 2),
            Gate::new_and(0, 3),
            Gate::new_and(1, 3),
            // add a1*b0 + a0*b1
            Gate::new_xor(5, 6),
            Gate::new_and(5, 6),
            // add c(a1*b0 + a0*b1) + a1*b1
            Gate::new_xor(7, 9),
            Gate::new_and(7, 9),
        ],
        [(4, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();
    let mut out = vec![];
    generate_code(&cw_impl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 4)
    vars v0..6
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 and v2)
    v2 = (v1 and v2)
    v0 = (v0 and v3)
    v5 = (v2 xor v0)
    v1 = (v1 and v3)
    v0 = (v2 and v0)
    v2 = (v1 xor v0)
    v0 = (v1 and v0)
    O0 = v4
    O1 = ~v5
    O2 = v2
    O3 = ~v0
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 4)
    vars v0..6
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    v4 = (v0 and v2)
    v2 = (v1 and v2)
    v0 = (v0 and v3)
    v5 = (v2 xor v0)
    v1 = (v1 and v3)
    v0 = (v2 and v0)
    v2 = (v1 xor v0)
    v0 = (v1 and v0)
    O0 = v4
    O1 = ~v5
    O2 = v2
    O3 = ~v0
EndFunc
"##
    );

    let circuit = Circuit::new(4, [], [(0, false), (3, true), (2, false), (1, true)]).unwrap();
    let mut out = vec![];
    generate_code(&cw_impl, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 4)
    vars v0..4
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    O0 = v0
    O1 = ~v3
    O2 = v2
    O3 = ~v1
EndFunc
"##
    );
    let mut out = vec![];
    generate_code(&cw_basic, &mut out, "test1", circuit.clone(), false);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        r##"Func test1(4 4)
    vars v0..4
    v0 = I0
    v1 = I1
    v2 = I2
    v3 = I3
    O0 = v0
    O1 = ~v3
    O2 = v2
    O3 = ~v1
EndFunc
"##
    );
}

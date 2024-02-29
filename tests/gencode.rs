use gatenative::gencode::*;
use gatenative::*;
use gatesim::*;
use int_enum::IntEnum;

use std::collections::HashMap;

struct TestFuncWriter<'c> {
    writer: &'c mut TestCodeWriter,
    name: &'c str,
    input_len: usize,
    output_len: usize,
    input_placement: Option<(&'c [usize], usize)>,
    output_placement: Option<(&'c [usize], usize)>,
    arg_input_map: HashMap<usize, usize>,
    single_buffer: bool,
}

impl<'c> FuncWriter for TestFuncWriter<'c> {
    fn func_start(&mut self) {
        let sb = if self.single_buffer { " sb" } else { "" };
        if !self.arg_input_map.is_empty() {
            writeln!(
                self.writer.out,
                "Func {}({} {} {}{})",
                self.name,
                self.input_len,
                self.output_len,
                self.arg_input_map.len(),
                sb
            )
            .unwrap();
        } else {
            writeln!(
                self.writer.out,
                "Func {}({} {}{})",
                self.name, self.input_len, self.output_len, sb
            )
            .unwrap();
        }
    }
    fn func_end(&mut self) {
        writeln!(self.writer.out, "EndFunc").unwrap();
    }
    fn alloc_vars(&mut self, var_num: usize) {
        writeln!(self.writer.out, "    vars v0..{}", var_num).unwrap();
    }
    fn gen_load(&mut self, reg: usize, input: usize) {
        let input = self.input_placement.map(|(p, _)| p[input]).unwrap_or(input);
        if let Some(arg_bit) = self.arg_input_map.get(&input) {
            writeln!(self.writer.out, "    v{} = bit(arg, {})", reg, arg_bit).unwrap();
        } else if !self.single_buffer {
            writeln!(self.writer.out, "    v{} = I{}", reg, input).unwrap();
        } else {
            writeln!(self.writer.out, "    v{} = O{}", reg, input).unwrap();
        }
    }
    fn gen_op(&mut self, op: InstrOp, negs: VNegs, dst_arg: usize, arg0: usize, arg1: usize) {
        writeln!(
            self.writer.out,
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
    fn gen_not(&mut self, dst_arg: usize, arg: usize) {
        writeln!(self.writer.out, "    v{} = ~v{}", dst_arg, arg).unwrap();
    }
    fn gen_store(&mut self, neg: bool, output: usize, reg: usize) {
        let output = self
            .output_placement
            .map(|(p, _)| p[output])
            .unwrap_or(output);
        writeln!(
            self.writer.out,
            "    O{} = {}v{}",
            output,
            if neg { "~" } else { "" },
            reg
        )
        .unwrap();
    }
}

struct TestCodeWriter {
    supp_ops: u64,
    out: Vec<u8>,
}

use std::io::Write;

impl<'c> CodeWriter<'c, TestFuncWriter<'c>> for TestCodeWriter {
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
    fn prolog(&mut self) {
        writeln!(self.out, "Start").unwrap();
    }
    fn epilog(&mut self) {
        writeln!(self.out, "End").unwrap();
    }
    unsafe fn func_writer_internal(
        &'c mut self,
        name: &'c str,
        input_len: usize,
        output_len: usize,
        code_config: CodeConfig<'c>,
        _output_vars: Option<Vec<usize>>,
    ) -> TestFuncWriter<'c> {
        TestFuncWriter::<'c> {
            writer: self,
            name,
            input_len,
            output_len,
            input_placement: code_config.input_placement,
            output_placement: code_config.output_placement,
            arg_input_map: HashMap::from_iter(
                code_config
                    .arg_inputs
                    .unwrap_or(&[])
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| (*x, i)),
            ),
            single_buffer: code_config.single_buffer,
        }
    }

    fn out(self) -> Vec<u8> {
        self.out
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
    let mut cw_impl = TestCodeWriter {
        supp_ops: basic_impl_ops,
        out: vec![],
    };
    let mut cw_nimpl = TestCodeWriter {
        supp_ops: basic_nimpl_ops,
        out: vec![],
    };
    let mut cw_basic = TestCodeWriter {
        supp_ops: basic_ops,
        out: vec![],
    };
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = (v0 xor v1)
    v3 = I2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 impl v1)
    v0 = (v0 impl v2)
    O1 = ~v0
EndFunc
"##
    );

    // in/out placement
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        Some((&[6, 11, 44], 68)),
        Some((&[48, 72], 88)),
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I6
    v1 = I11
    v2 = (v0 xor v1)
    v3 = I44
    v4 = (v3 xor v2)
    O48 = v4
    v2 = (v3 and v2)
    v0 = (v0 impl v1)
    v0 = (v0 impl v2)
    O72 = ~v0
EndFunc
"##
    );
    // edn in/out placement

    // arg input
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        Some(&[0, 2]),
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(3 2 2)
    vars v0..5
    v0 = bit(arg, 0)
    v1 = I1
    v2 = (v0 xor v1)
    v3 = bit(arg, 1)
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 impl v1)
    v0 = (v0 impl v2)
    O1 = ~v0
EndFunc
"##
    );
    // end arg input

    cw_nimpl.out.clear();
    generate_code(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = (v0 xor v1)
    v3 = I2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 nimpl v1)
    v0 = (v2 or v0)
    O1 = ~v0
EndFunc
"##
    );

    // single_buffer
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[0, 1, 2], 3)))
            .output_placement(Some((&[0, 1], 3)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(3 2 sb)
    vars v0..5
    v0 = O0
    v1 = O1
    v2 = (v0 xor v1)
    v3 = O2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 nimpl v1)
    v0 = (v2 or v0)
    O1 = ~v0
EndFunc
"##
    );
    cw_impl.out.clear();
    generate_code_with_config(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[0, 1, 2], 3)))
            .output_placement(Some((&[0, 1], 3)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(3 2 sb)
    vars v0..5
    v0 = O0
    v1 = O1
    v2 = (v0 xor v1)
    v3 = O2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 impl v1)
    v0 = (v0 impl v2)
    O1 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[0, 1, 2], 3)))
            .output_placement(Some((&[0, 1], 3)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(3 2 sb)
    vars v0..5
    v0 = O0
    v1 = O1
    v2 = (v0 xor v1)
    v3 = O2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 and ~v1)
    v0 = ~(v2 or v0)
    O1 = v0
EndFunc
"##
    );

    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(3 2)
    vars v0..5
    v0 = I0
    v1 = I1
    v2 = (v0 xor v1)
    v3 = I2
    v4 = (v3 xor v2)
    O0 = v4
    v2 = (v3 and v2)
    v0 = (v0 and ~v1)
    v0 = ~(v2 or v0)
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

    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 impl v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 or v5)
    v3 = (v3 xor v6)
    v7 = (v2 impl v5)
    v8 = (v7 impl v0)
    v2 = (v5 impl v2)
    v5 = (v2 impl v1)
    v5 = (v8 xor v5)
    v3 = (v3 impl v5)
    O0 = v3
    v0 = (v0 impl v7)
    v0 = (v0 impl v2)
    v1 = (v1 and v6)
    v1 = (v4 impl v1)
    v0 = (v0 xor v1)
    O1 = v0
EndFunc
"##
    );
    cw_nimpl.out.clear();
    generate_code(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 nimpl v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 nimpl v5)
    v3 = (v3 xor v6)
    v7 = (v2 or v5)
    v8 = (v7 nimpl v0)
    v2 = (v2 and v5)
    v5 = (v1 or v2)
    v5 = (v8 xor v5)
    v3 = (v5 nimpl v3)
    O0 = ~v3
    v0 = (v0 nimpl v7)
    v0 = (v2 nimpl v0)
    v1 = (v1 nimpl v6)
    v1 = (v4 nimpl v1)
    v0 = (v0 xor v1)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 and ~v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 and ~v5)
    v3 = (v3 xor v6)
    v7 = ~(v2 or v5)
    v8 = ~(v0 or v7)
    v2 = (v2 and v5)
    v5 = ~(v1 or v2)
    v5 = (v8 xor v5)
    v3 = ~(v3 or v5)
    O0 = ~v3
    v0 = (v0 and v7)
    v0 = (v2 and ~v0)
    v1 = (v1 and ~v6)
    v1 = (v4 and ~v1)
    v0 = (v0 xor v1)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        true,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 2)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 and ~v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 and ~v5)
    v3 = (v3 xor v6)
    v7 = ~(v2 or v5)
    v8 = (v0 or v7)
    v2 = (v2 and v5)
    v5 = (v1 or v2)
    v5 = (v8 xor v5)
    v3 = (v3 or v5)
    O0 = v3
    v0 = (v0 and v7)
    v0 = (v2 and ~v0)
    v1 = (v6 or ~v1)
    v1 = (v4 and v1)
    v0 = (v0 xor v1)
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
        [
            (18, true),
            (18, false),
            (22, true),
            (22, false),
            (0, false),
            (1, false),
            (1, true),
            (0, true),
        ],
    )
    .unwrap();
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 impl v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 or v5)
    v3 = (v3 xor v6)
    v7 = (v2 impl v5)
    v8 = (v7 impl v0)
    v2 = (v5 impl v2)
    v5 = (v2 impl v1)
    v5 = (v8 xor v5)
    v3 = (v3 impl v5)
    O0 = v3
    O1 = ~v3
    v5 = (v0 impl v7)
    v2 = (v5 impl v2)
    v5 = (v1 and v6)
    v4 = (v4 impl v5)
    v2 = (v2 xor v4)
    O2 = ~v2
    O3 = v2
    O4 = v0
    O5 = v1
    O6 = ~v1
    O7 = ~v0
EndFunc
"##
    );
    cw_nimpl.out.clear();
    generate_code(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 nimpl v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 nimpl v5)
    v3 = (v3 xor v6)
    v7 = (v2 or v5)
    v8 = (v7 nimpl v0)
    v2 = (v2 and v5)
    v5 = (v1 or v2)
    v5 = (v8 xor v5)
    v3 = (v5 nimpl v3)
    O0 = ~v3
    O1 = v3
    v5 = (v0 nimpl v7)
    v2 = (v2 nimpl v5)
    v5 = (v1 nimpl v6)
    v4 = (v4 nimpl v5)
    v2 = (v2 xor v4)
    O2 = ~v2
    O3 = v2
    O4 = v0
    O5 = v1
    O6 = ~v1
    O7 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 and ~v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 and ~v5)
    v3 = (v3 xor v6)
    v7 = ~(v2 or v5)
    v8 = ~(v0 or v7)
    v2 = (v2 and v5)
    v5 = ~(v1 or v2)
    v5 = (v8 xor v5)
    v3 = ~(v3 or v5)
    O0 = ~v3
    O1 = v3
    v5 = (v0 and v7)
    v2 = (v2 and ~v5)
    v5 = (v1 and ~v6)
    v4 = (v4 and ~v5)
    v2 = (v2 xor v4)
    O2 = ~v2
    O3 = v2
    O4 = v0
    O5 = v1
    O6 = ~v1
    O7 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        true,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..9
    v0 = I0
    v1 = I1
    v2 = (v0 and ~v1)
    v2 = (v1 xor v2)
    v3 = I2
    v2 = (v3 xor v2)
    v4 = I3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 and ~v5)
    v3 = (v3 xor v6)
    v7 = ~(v2 or v5)
    v8 = (v0 or v7)
    v2 = (v2 and v5)
    v5 = (v1 or v2)
    v5 = (v8 xor v5)
    v3 = (v3 or v5)
    O0 = v3
    O1 = ~v3
    v5 = (v0 and v7)
    v2 = (v2 and ~v5)
    v5 = (v6 or ~v1)
    v4 = (v4 and v5)
    v2 = (v2 xor v4)
    O2 = ~v2
    O3 = v2
    O4 = v0
    O5 = v1
    O6 = ~v1
    O7 = ~v0
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
        [(18, true), (22, false), (18, false), (22, true)],
    )
    .unwrap();
    cw_impl.out.clear();
    generate_code_with_config(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..9
    v0 = O0
    v1 = O1
    v2 = (v0 impl v1)
    v2 = (v1 xor v2)
    v3 = O2
    v2 = (v3 xor v2)
    v4 = O3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 or v5)
    v3 = (v3 xor v6)
    v7 = (v2 impl v5)
    v8 = (v7 impl v0)
    v2 = (v5 impl v2)
    v5 = (v2 impl v1)
    v5 = (v8 xor v5)
    v3 = (v3 impl v5)
    O0 = v3
    O2 = ~v3
    v0 = (v0 impl v7)
    v0 = (v0 impl v2)
    v1 = (v1 and v6)
    v1 = (v4 impl v1)
    v0 = (v0 xor v1)
    O1 = v0
    O3 = ~v0
EndFunc
"##
    );
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..9
    v0 = O0
    v1 = O1
    v2 = (v0 nimpl v1)
    v2 = (v1 xor v2)
    v3 = O2
    v2 = (v3 xor v2)
    v4 = O3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 nimpl v5)
    v3 = (v3 xor v6)
    v7 = (v2 or v5)
    v8 = (v7 nimpl v0)
    v2 = (v2 and v5)
    v5 = (v1 or v2)
    v5 = (v8 xor v5)
    v3 = (v5 nimpl v3)
    O0 = ~v3
    O2 = v3
    v0 = (v0 nimpl v7)
    v0 = (v2 nimpl v0)
    v1 = (v1 nimpl v6)
    v1 = (v4 nimpl v1)
    v0 = (v0 xor v1)
    O1 = v0
    O3 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..9
    v0 = O0
    v1 = O1
    v2 = (v0 and ~v1)
    v2 = (v1 xor v2)
    v3 = O2
    v2 = (v3 xor v2)
    v4 = O3
    v5 = (v0 and v4)
    v5 = (v4 and v5)
    v5 = (v4 xor v5)
    v6 = (v2 and ~v5)
    v3 = (v3 xor v6)
    v7 = ~(v2 or v5)
    v8 = ~(v0 or v7)
    v2 = (v2 and v5)
    v5 = ~(v1 or v2)
    v5 = (v8 xor v5)
    v3 = ~(v3 or v5)
    O0 = ~v3
    O2 = v3
    v0 = (v0 and v7)
    v0 = (v2 and ~v0)
    v1 = (v1 and ~v6)
    v1 = (v4 and ~v1)
    v0 = (v0 xor v1)
    O1 = v0
    O3 = ~v0
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
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(3 1)
    vars v0..3
    v0 = I0
    v1 = I1
    v0 = (v0 and v1)
    v2 = I2
    v2 = (v1 and v2)
    v0 = (v0 and ~v2)
    v0 = (v0 and ~v1)
    O0 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        true,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(3 1)
    vars v0..3
    v0 = I0
    v1 = I1
    v0 = (v0 and v1)
    v2 = I2
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
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..5
    v0 = I0
    v1 = I2
    v2 = (v0 and v1)
    O0 = v2
    v2 = I1
    v1 = (v2 and v1)
    v3 = I3
    v0 = (v0 and v3)
    v4 = (v1 xor v0)
    O1 = ~v4
    v2 = (v2 and v3)
    v0 = (v1 and v0)
    v1 = (v2 xor v0)
    O2 = v1
    v0 = (v2 and v0)
    O3 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..5
    v0 = I0
    v1 = I2
    v2 = (v0 and v1)
    O0 = v2
    v2 = I1
    v1 = (v2 and v1)
    v3 = I3
    v0 = (v0 and v3)
    v4 = (v1 xor v0)
    O1 = ~v4
    v2 = (v2 and v3)
    v0 = (v1 and v0)
    v1 = (v2 xor v0)
    O2 = v1
    v0 = (v2 and v0)
    O3 = ~v0
EndFunc
"##
    );

    let circuit = Circuit::new(4, [], [(0, false), (3, true), (2, false), (1, true)]).unwrap();
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..1
    v0 = I0
    O0 = v0
    v0 = I3
    O1 = ~v0
    v0 = I2
    O2 = v0
    v0 = I1
    O3 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..1
    v0 = I0
    O0 = v0
    v0 = I3
    O1 = ~v0
    v0 = I2
    O2 = v0
    v0 = I1
    O3 = ~v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        true,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..1
    v0 = I0
    O0 = v0
    v0 = I3
    O1 = ~v0
    v0 = I2
    O2 = v0
    v0 = I1
    O3 = ~v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        4,
        [],
        [
            (0, false),
            (3, true),
            (2, false),
            (1, true),
            (1, false),
            (2, true),
            (0, true),
            (3, false),
        ],
    )
    .unwrap();
    cw_impl.out.clear();
    generate_code(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..4
    v0 = I0
    O0 = v0
    v1 = I3
    O1 = ~v1
    v2 = I2
    O2 = v2
    v3 = I1
    O3 = ~v3
    O4 = v3
    O5 = ~v2
    O6 = ~v0
    O7 = v1
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        None,
        None,
        None,
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 8)
    vars v0..4
    v0 = I0
    O0 = v0
    v1 = I3
    O1 = ~v1
    v2 = I2
    O2 = v2
    v3 = I1
    O3 = ~v3
    O4 = v3
    O5 = ~v2
    O6 = ~v0
    O7 = v1
EndFunc
"##
    );

    //
    // extra single buffer tests
    //
    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(0, 1),
            Gate::new_xor(0, 1),
            Gate::new_nor(0, 1),
            Gate::new_nor(2, 3),
        ],
        [(4, false), (5, true), (6, false), (7, true)],
    )
    .unwrap();
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..3
    v0 = O0
    v1 = O1
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = (v0 or v1)
    v1 = O2
    O2 = ~v0
    v0 = O3
    v0 = (v1 or v0)
    O3 = v0
EndFunc
"##
    );
    cw_impl.out.clear();
    generate_code_with_config(
        &mut cw_impl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_impl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..3
    v0 = O0
    v1 = O1
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = (v0 or v1)
    v1 = O2
    O2 = ~v0
    v0 = O3
    v0 = (v1 or v0)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..3
    v0 = O0
    v1 = O1
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = ~(v0 or v1)
    v1 = O2
    O2 = v0
    v0 = O3
    v0 = ~(v1 or v0)
    O3 = ~v0
EndFunc
"##
    );
    // no single buffer
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..3
    v0 = I0
    v1 = I1
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = (v0 or v1)
    O2 = ~v0
    v0 = I2
    v1 = I3
    v0 = (v0 or v1)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..3
    v0 = I0
    v1 = I1
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = ~(v0 or v1)
    O2 = v0
    v0 = I2
    v1 = I3
    v0 = ~(v0 or v1)
    O3 = ~v0
EndFunc
"##
    );
    // additional tests with placements
    // both input and output placements
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    O2 = ~v2
    v0 = (v0 or v1)
    v1 = O0
    O0 = ~v0
    v0 = (v3 or v1)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    O2 = ~v2
    v0 = ~(v0 or v1)
    v1 = O0
    O0 = v0
    v0 = ~(v3 or v1)
    O1 = ~v0
EndFunc
"##
    );
    // both input and output placements
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 0, 3], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    O2 = ~v2
    v0 = (v0 or v1)
    v1 = O0
    O0 = ~v0
    v0 = (v1 or v3)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 0, 3], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    O2 = ~v2
    v0 = ~(v0 or v1)
    v1 = O0
    O0 = v0
    v0 = ~(v1 or v3)
    O1 = ~v0
EndFunc
"##
    );
    // output placements
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O0
    v1 = O1
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    v4 = O2
    O2 = ~v2
    v0 = (v0 or v1)
    O0 = ~v0
    v0 = (v4 or v3)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O0
    v1 = O1
    v2 = (v0 and v1)
    v3 = O3
    O3 = v2
    v2 = (v0 xor v1)
    v4 = O2
    O2 = ~v2
    v0 = ~(v0 or v1)
    O0 = v0
    v0 = ~(v4 or v3)
    O1 = ~v0
EndFunc
"##
    );
    // input placements
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 0, 3], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O0
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = (v0 or v1)
    O2 = ~v0
    v0 = O3
    v0 = (v3 or v0)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 0, 3], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O1
    v1 = O2
    v2 = (v0 and v1)
    v3 = O0
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = ~(v0 or v1)
    O2 = v0
    v0 = O3
    v0 = ~(v3 or v0)
    O3 = ~v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(2, 3),
            Gate::new_xor(2, 3),
            Gate::new_nor(2, 3),
            Gate::new_nor(0, 1),
        ],
        [(4, false), (5, true), (6, false), (7, true)],
    )
    .unwrap();
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O2
    v1 = O3
    v2 = (v0 and v1)
    v3 = O0
    O0 = v2
    v2 = (v0 xor v1)
    v4 = O1
    O1 = ~v2
    v0 = (v0 or v1)
    O2 = ~v0
    v0 = (v3 or v4)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O2
    v1 = O3
    v2 = (v0 and v1)
    v3 = O0
    O0 = v2
    v2 = (v0 xor v1)
    v4 = O1
    O1 = ~v2
    v0 = ~(v0 or v1)
    O2 = v0
    v0 = ~(v3 or v4)
    O3 = ~v0
EndFunc
"##
    );
    // no single buffer
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..3
    v0 = I2
    v1 = I3
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = (v0 or v1)
    O2 = ~v0
    v0 = I0
    v1 = I1
    v0 = (v0 or v1)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..3
    v0 = I2
    v1 = I3
    v2 = (v0 and v1)
    O0 = v2
    v2 = (v0 xor v1)
    O1 = ~v2
    v0 = ~(v0 or v1)
    O2 = v0
    v0 = I0
    v1 = I1
    v0 = ~(v0 or v1)
    O3 = ~v0
EndFunc
"##
    );

    let circuit = Circuit::new(
        4,
        [
            Gate::new_and(2, 3),
            Gate::new_xor(2, 3),
            Gate::new_nor(0, 3),
            Gate::new_and(4, 5),
            Gate::new_nimpl(4, 6),
            Gate::new_xor(5, 6),
            Gate::new_xor(8, 9),
            Gate::new_nimpl(9, 1),
        ],
        [(7, false), (8, true), (10, false), (11, true)],
    )
    .unwrap();
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O2
    v1 = O3
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    v4 = O0
    O0 = v3
    v1 = (v4 or v1)
    v2 = (v2 and v1)
    v3 = O1
    O1 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O2 = ~v1
    v0 = (v0 or v3)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new().single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..5
    v0 = O2
    v1 = O3
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    v4 = O0
    O0 = v3
    v1 = ~(v4 or v1)
    v2 = (v2 and ~v1)
    v3 = O1
    O1 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O2 = v1
    v0 = (v0 and ~v3)
    O3 = ~v0
EndFunc
"##
    );
    // no single buffer
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..4
    v0 = I2
    v1 = I3
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    O0 = v3
    v3 = I0
    v1 = (v3 or v1)
    v2 = (v2 and v1)
    O1 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O2 = ~v1
    v1 = I1
    v0 = (v0 or v1)
    O3 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new(),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4)
    vars v0..4
    v0 = I2
    v1 = I3
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    O0 = v3
    v3 = I0
    v1 = ~(v3 or v1)
    v2 = (v2 and ~v1)
    O1 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O2 = v1
    v1 = I1
    v0 = (v0 and ~v1)
    O3 = ~v0
EndFunc
"##
    );
    // with placements
    cw_nimpl.out.clear();
    generate_code_with_config(
        &mut cw_nimpl,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_nimpl.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O3
    v1 = O0
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    O3 = v3
    v3 = O1
    v1 = (v3 or v1)
    v2 = (v2 and v1)
    v3 = O2
    O2 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O0 = ~v1
    v0 = (v0 or v3)
    O1 = v0
EndFunc
"##
    );
    cw_basic.out.clear();
    generate_code_with_config(
        &mut cw_basic,
        "test1",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((&[1, 2, 3, 0], 4)))
            .output_placement(Some((&[3, 2, 0, 1], 4)))
            .single_buffer(true),
    );
    assert_eq!(
        String::from_utf8(cw_basic.out.clone()).unwrap(),
        r##"Func test1(4 4 sb)
    vars v0..4
    v0 = O3
    v1 = O0
    v2 = (v0 and v1)
    v0 = (v0 xor v1)
    v3 = (v2 and v0)
    O3 = v3
    v3 = O1
    v1 = ~(v3 or v1)
    v2 = (v2 and ~v1)
    v3 = O2
    O2 = ~v2
    v0 = (v0 xor v1)
    v1 = (v2 xor v0)
    O0 = v1
    v0 = (v0 and ~v3)
    O1 = ~v0
EndFunc
"##
    );
}

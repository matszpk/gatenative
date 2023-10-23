use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Nand = 1,
    Or = 2,
    Nor = 3,
    Nimpl = 4,
    Xor = 5,
    Equal = 6,
    BNot = 7,
}

pub trait CodeWriter {
    // returns bit mask of where bit position is InstrOp integer value.
    fn supported_ops(&self) -> u64;
    // generate prolog
    fn prolog(&self, out: &mut Vec<u8>);
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize);
    fn func_end(&self, out: &mut Vec<u8>, name: &str);
    // allocate structures of sizes
    fn alloc(&self, out: &mut Vec<u8>, alloc_size: usize);
    // generate operation
    fn gen_op(
        &self,
        out: &mut Vec<u8>,
        op: InstrOp,
        dst_arg: usize,
        arg1: usize,
        arg2: Option<usize>,
    );
    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize);
    fn gen_store(&self, out: &mut Vec<u8>, output: usize, reg: usize);
    fn epilog(&self, out: &mut Vec<u8>);
}

#[cfg(test)]
mod tests {
    use super::*;
}

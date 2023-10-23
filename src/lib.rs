use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum InstrOp {
    And = 0,
    Nand = 1,
    Or = 2,
    Nor = 3,
    Impl = 4,
    Nimpl = 5,
    Xor = 6,
    Equal = 7,
    BNot = 8,
}

pub trait CodeWriter {
    /// It returns bit mask of where bit position is InstrOp integer value - support
    // Instr Ops.
    fn supported_ops(&self) -> u64;
    // Returns Word length in bits
    fn word_len(&self) -> u32;
    // Returns maximal possible allocation size in words.
    fn max_alloc_size(&self) -> usize;
    // Returns preferred allocation size in words.
    fn preferred_alloc_size(&self) -> usize;
    /// Generates prolog.
    fn prolog(&self, out: &mut Vec<u8>);
    /// Generates epilog;
    fn epilog(&self, out: &mut Vec<u8>);
    /// Generates function start with definition.
    fn func_start(&self, out: &mut Vec<u8>, name: &str, input_len: usize, output_len: usize);
    /// Generates function end.
    fn func_end(&self, out: &mut Vec<u8>, name: &str);
    /// Generates allocation of local words to make operations.
    fn alloc(&self, out: &mut Vec<u8>, alloc_size: usize);

    /// Generates Load instruction from input.
    fn gen_load(&self, out: &mut Vec<u8>, reg: usize, input: usize);
    /// Generates operation.
    fn gen_op(
        &self,
        out: &mut Vec<u8>,
        op: InstrOp,
        dst_arg: usize,
        arg1: usize,
        arg2: Option<usize>,
    );
    /// Generates Store instruction into output.
    fn gen_store(&self, out: &mut Vec<u8>, output: usize, reg: usize);
}

#[cfg(test)]
mod tests {
    use super::*;
}

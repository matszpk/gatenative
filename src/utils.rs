use gatesim::*;

use crate::*;

use crate::vbinopcircuit::*;
use crate::vcircuit::*;

use std::collections::HashMap;
use std::fmt::Debug;

use static_init::dynamic;
use std::collections::BinaryHeap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct VarAllocator<T> {
    free_list: BinaryHeap<std::cmp::Reverse<T>>,
    alloc_map: Vec<bool>,
}

impl<T> VarAllocator<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    pub fn new() -> Self {
        Self {
            free_list: BinaryHeap::new(),
            alloc_map: vec![],
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.alloc_map.len()
    }

    pub fn alloc(&mut self) -> T {
        if let Some(std::cmp::Reverse(index)) = self.free_list.pop() {
            let index_u = usize::try_from(index).unwrap();
            self.alloc_map[index_u] = true;
            index
        } else {
            let index = self.alloc_map.len();
            self.alloc_map.push(true);
            T::try_from(index).unwrap()
        }
    }

    pub fn free(&mut self, index: T) -> bool {
        let index_u = usize::try_from(index).unwrap();
        assert!(index_u < self.len());
        if self.alloc_map[index_u] {
            self.free_list.push(std::cmp::Reverse(index));
            self.alloc_map[index_u] = false;
            true
        } else {
            false
        }
    }
}

// trait for Circuits
pub(crate) trait CircuitTrait<T> {
    fn input_len(&self) -> T;
    fn len(&self) -> usize;
    fn gate_input_num(&self, gate: usize) -> usize;
    fn gate_input(&self, gate: usize, input: usize) -> T;
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs);
    fn outputs(&self) -> &[(T, bool)];
}

impl<T> CircuitTrait<T> for Circuit<T>
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.input_len()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn gate_input_num(&self, _gate: usize) -> usize {
        2
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => self.gates()[gate].i0,
            1 => self.gates()[gate].i1,
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, _gate: usize) -> (InstrOp, VNegs) {
        panic!("Unsupported");
    }
    fn outputs(&self) -> &[(T, bool)] {
        self.outputs()
    }
}

// vcircuit with swap_args - vcircuit with table of swapping arguments
impl<T> CircuitTrait<T> for (VCircuit<T>, &[bool])
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.0.input_len
    }
    fn len(&self) -> usize {
        self.0.gates.len()
    }
    fn gate_input_num(&self, _gate: usize) -> usize {
        2
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => {
                if self.1[gate] {
                    self.0.gates[gate].i1
                } else {
                    self.0.gates[gate].i0
                }
            }
            1 => {
                if self.1[gate] {
                    self.0.gates[gate].i0
                } else {
                    self.0.gates[gate].i1
                }
            }
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs) {
        (
            match self.0.gates[gate].func {
                VGateFunc::And => InstrOp::And,
                VGateFunc::Or => InstrOp::Or,
                VGateFunc::Impl => InstrOp::Impl,
                VGateFunc::Nimpl => InstrOp::Nimpl,
                VGateFunc::Xor => InstrOp::Xor,
                _ => {
                    panic!("Unsupported InstrOp")
                }
            },
            VNegs::NoNegs,
        )
    }
    fn outputs(&self) -> &[(T, bool)] {
        &self.0.outputs
    }
}

// vbinopcircuit with swap_args - vcircuit with table of swapping arguments
impl<T> CircuitTrait<T> for (VBinOpCircuit<T>, &[bool])
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.0.input_len
    }
    fn len(&self) -> usize {
        self.0.gates.len()
    }
    fn gate_input_num(&self, _gate: usize) -> usize {
        2
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => {
                if self.1[gate] {
                    self.0.gates[gate].0.i1
                } else {
                    self.0.gates[gate].0.i0
                }
            }
            1 => {
                if self.1[gate] {
                    self.0.gates[gate].0.i0
                } else {
                    self.0.gates[gate].0.i1
                }
            }
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs) {
        (
            match self.0.gates[gate].0.func {
                VGateFunc::And => InstrOp::And,
                VGateFunc::Or => InstrOp::Or,
                VGateFunc::Xor => InstrOp::Xor,
                _ => {
                    panic!("Unsupported InstrOp")
                }
            },
            self.0.gates[gate].1,
        )
    }
    fn outputs(&self) -> &[(T, bool)] {
        &self.0.outputs
    }
}

// var usage - just counter of var usage.

pub(crate) fn gen_var_usage<T>(circuit: &Circuit<T>) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let mut var_usage = vec![T::default(); input_len + circuit.len()];
    for g in circuit.gates() {
        let gi0 = usize::try_from(g.i0).unwrap();
        let gi1 = usize::try_from(g.i1).unwrap();
        let var_usage_0 = usize::try_from(var_usage[gi0]).unwrap() + 1;
        var_usage[gi0] = T::try_from(var_usage_0).unwrap();
        let var_usage_1 = usize::try_from(var_usage[gi1]).unwrap() + 1;
        var_usage[gi1] = T::try_from(var_usage_1).unwrap();
    }
    for (o, _) in circuit.outputs() {
        let o = usize::try_from(*o).unwrap();
        let var_usage_0 = usize::try_from(var_usage[o]).unwrap() + 1;
        var_usage[o] = T::try_from(var_usage_0).unwrap();
    }
    var_usage
}

#[dynamic]
static mut TIMESTAMP: u128 = {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
};

pub(crate) fn get_timestamp() -> u128 {
    let mut lock = TIMESTAMP.write();
    let old = *lock;
    *lock += 1;
    old
}

pub const fn calc_log_bits(n: usize) -> usize {
    let nbits = usize::BITS - n.leading_zeros();
    if (1 << (nbits - 1)) == n {
        (nbits - 1) as usize
    } else {
        nbits as usize
    }
}

pub struct MultiVarAllocTool<T> {
    var_allocs: Vec<VarAllocator<T>>,
    var_usages: Vec<Vec<T>>,
    var_maps: Vec<HashMap<T, T>>,
    var_new_history: Vec<T>,
    usage_mode: bool, // if true then use variables, if false then allocates variables
}

impl<T> MultiVarAllocTool<T>
where
    T: Default + Clone + Copy + Ord + PartialEq + Eq + std::hash::Hash,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    pub fn new(var_type_num: usize) -> Self {
        Self {
            var_allocs: vec![VarAllocator::new(); var_type_num],
            var_usages: vec![Vec::new(); var_type_num],
            var_maps: vec![HashMap::new(); var_type_num],
            var_new_history: vec![T::default(); var_type_num],
            usage_mode: false,
        }
    }

    pub fn set_usage_mode(&mut self) {
        self.usage_mode = true;
    }

    pub fn usage_mode(&self) -> bool {
        self.usage_mode
    }

    pub fn var_type_num(&self) -> usize {
        self.var_allocs.len()
    }

    fn use_or_remove_var(&mut self, var_type: usize, vv: T) {
        let vt = self.var_maps[var_type][&vv];
        let v = usize::try_from(vt).unwrap();
        let var_usage = self.var_usages[var_type][v];
        let mut var_usage = usize::try_from(var_usage).unwrap();
        assert_ne!(var_usage, 0);
        var_usage -= 1;
        self.var_usages[var_type][v] = T::try_from(var_usage).unwrap();
        if var_usage == 0 {
            self.var_allocs[var_type].free(vv);
            self.var_maps[var_type].remove(&vv);
        }
    }

    pub fn new_var(&mut self, var_type: usize) -> T {
        if !self.usage_mode {
            // returned value is variable original index
            let v = self.var_usages[var_type].len();
            self.var_usages[var_type].push(T::try_from(1).unwrap());
            T::try_from(v).unwrap()
        } else {
            // returned value is allocated variable index
            let vt = self.var_new_history[var_type];
            let v = usize::try_from(vt).unwrap();
            self.var_new_history[var_type] = T::try_from(v + 1).unwrap();
            let vv = self.var_allocs[var_type].alloc();
            self.var_maps[var_type].insert(vv, vt);
            self.use_or_remove_var(var_type, vv);
            vv
        }
    }

    pub fn use_var(&mut self, var_type: usize, vt: T) {
        if !self.usage_mode {
            // vt is original variable index
            let v = usize::try_from(vt).unwrap();
            let var_usage = self.var_usages[var_type][v];
            let var_usage = usize::try_from(var_usage).unwrap();
            self.var_usages[var_type][v] = T::try_from(var_usage + 1).unwrap();
        } else {
            // vt is allocated variable index
            self.use_or_remove_var(var_type, vt);
        }
    }

    pub fn alloc_var_num(&self, var_type: usize) -> usize {
        self.var_allocs[var_type].len()
    }
}

#[dynamic]
pub(crate) static GATE_SYS_DUMP_SOURCE: bool = match env::var("GATE_SYS_DUMP_SOURCE")
    .unwrap_or("0".to_string())
    .to_lowercase()
    .as_str()
{
    "0" => false,
    "1" => true,
    "false" => false,
    "true" => true,
    "off" => false,
    "on" => true,
    "no" => false,
    "yes" => true,
    _ => false,
};

pub(crate) fn dump_source_code(name: &str, source: &[u8]) {
    if *GATE_SYS_DUMP_SOURCE {
        eprintln!(
            "------ SOURCE: {0} ------\n{1}\n-- END OF SOURCE: {0} ---\n",
            name,
            std::str::from_utf8(source).unwrap_or("UNKNOWN!!!!")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_allocator() {
        let mut vacc = VarAllocator::new();
        assert_eq!(0, vacc.alloc());
        assert_eq!(1, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(3, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert!(vacc.free(2));
        assert!(!vacc.free(2));
        assert!(vacc.free(1));
        assert!(!vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(0));
        assert!(!vacc.free(0));
        assert_eq!(0, vacc.alloc());
        assert_eq!(2, vacc.alloc());
        assert_eq!(5, vacc.alloc());
        assert!(vacc.free(4));
        assert!(vacc.free(1));
        assert_eq!(1, vacc.alloc());
        assert!(vacc.free(3));
        assert_eq!(3, vacc.alloc());
        assert!(vacc.free(2));
        assert_eq!(2, vacc.alloc());
        assert_eq!(4, vacc.alloc());
        assert_eq!(6, vacc.alloc());
    }

    #[test]
    fn test_multi_var_alloc_tool() {
        let mut mvar_alloc_tool = MultiVarAllocTool::<usize>::new(2);
        {
            let t1 = mvar_alloc_tool.new_var(0);
            assert_eq!(t1, 0);
            mvar_alloc_tool.use_var(0, t1);
            let t2 = mvar_alloc_tool.new_var(0);
            assert_eq!(t2, 1);
            mvar_alloc_tool.use_var(0, t2);
            let t3 = mvar_alloc_tool.new_var(1);
            assert_eq!(t3, 0);
            mvar_alloc_tool.use_var(1, t3);
            mvar_alloc_tool.use_var(0, t1);
            let t4 = mvar_alloc_tool.new_var(0);
            assert_eq!(t4, 2);
            mvar_alloc_tool.use_var(0, t4);
            mvar_alloc_tool.use_var(1, t3);
            let t5 = mvar_alloc_tool.new_var(1);
            assert_eq!(t5, 1);
            mvar_alloc_tool.use_var(1, t5);
            mvar_alloc_tool.use_var(0, t2);
            let t6 = mvar_alloc_tool.new_var(0);
            assert_eq!(t6, 3);
            mvar_alloc_tool.use_var(0, t1);
            let t7 = mvar_alloc_tool.new_var(1);
            assert_eq!(t7, 2);
            mvar_alloc_tool.use_var(0, t4);
            let t8 = mvar_alloc_tool.new_var(1);
            assert_eq!(t8, 3);
            let t9 = mvar_alloc_tool.new_var(0);
            assert_eq!(t9, 4);
            let t10 = mvar_alloc_tool.new_var(1);
            assert_eq!(t10, 4);
            let t11 = mvar_alloc_tool.new_var(1);
            assert_eq!(t11, 5);
            mvar_alloc_tool.use_var(1, t8);
            mvar_alloc_tool.use_var(1, t10);
            mvar_alloc_tool.use_var(0, t6);
        }
        mvar_alloc_tool.set_usage_mode();
        {
            let t1 = mvar_alloc_tool.new_var(0); // *.., ...
            assert_eq!(t1, 0);
            mvar_alloc_tool.use_var(0, t1);
            let t2 = mvar_alloc_tool.new_var(0); // **., ...
            assert_eq!(t2, 1);
            mvar_alloc_tool.use_var(0, t2);
            let t3 = mvar_alloc_tool.new_var(1); // **., *..
            assert_eq!(t3, 0);
            mvar_alloc_tool.use_var(1, t3);
            mvar_alloc_tool.use_var(0, t1);
            let t4 = mvar_alloc_tool.new_var(0); // ***, *..
            assert_eq!(t4, 2);
            mvar_alloc_tool.use_var(0, t4);
            mvar_alloc_tool.use_var(1, t3); // ***, ...
            let t5 = mvar_alloc_tool.new_var(1); // ***, *..
            assert_eq!(t5, 0);
            mvar_alloc_tool.use_var(1, t5); // ***, ...
            mvar_alloc_tool.use_var(0, t2); // *.*, ...
            let t6 = mvar_alloc_tool.new_var(0); // ***, ...
            assert_eq!(t6, 1);
            mvar_alloc_tool.use_var(0, t1); // .**, ...
            let t7 = mvar_alloc_tool.new_var(1); // .**, *..
            assert_eq!(t7, 0);
            mvar_alloc_tool.use_var(0, t4); // .*., ...
            let t8 = mvar_alloc_tool.new_var(1); // .*., *..
            assert_eq!(t8, 0);
            let t9 = mvar_alloc_tool.new_var(0); // **., *..
            assert_eq!(t9, 0);
            let t10 = mvar_alloc_tool.new_var(1); // **., **.
            assert_eq!(t10, 1);
            let t11 = mvar_alloc_tool.new_var(1); // **., ***
            assert_eq!(t11, 2);
            mvar_alloc_tool.use_var(1, t8); // **., .**
            mvar_alloc_tool.use_var(1, t10); // **., ..*
            mvar_alloc_tool.use_var(0, t6); // *.., ..*
        }
    }
}

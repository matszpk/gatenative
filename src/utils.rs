#![cfg_attr(docsrs, feature(doc_cfg))]
//! Utilities sometimes to internal usage.

use crate::*;

use crate::vbinopcircuit::*;
use crate::vcircuit::*;
use crate::vlop3circuit::*;

use std::collections::HashMap;
use std::fmt::Debug;

use static_init::dynamic;
use std::collections::BinaryHeap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// Variable allocator.
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
    /// Creates new allocator.
    pub fn new() -> Self {
        Self {
            free_list: BinaryHeap::new(),
            alloc_map: vec![],
        }
    }

    /// Returns number of variables.
    #[inline]
    pub fn len(&self) -> usize {
        self.alloc_map.len()
    }

    /// Allocate variable and returns new its index.
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

    /// Free variable with given index. Returns true if variables existed.
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
impl<T> CircuitTrait<T> for VCircuit<T>
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.input_len
    }
    fn len(&self) -> usize {
        self.gates.len()
    }
    fn gate_input_num(&self, _gate: usize) -> usize {
        2
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => self.gates[gate].i0,
            1 => self.gates[gate].i1,
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs) {
        (
            match self.gates[gate].func {
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
        &self.outputs
    }
}

// vbinopcircuit with swap_args - vcircuit with table of swapping arguments
impl<T> CircuitTrait<T> for VBinOpCircuit<T>
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.input_len
    }
    fn len(&self) -> usize {
        self.gates.len()
    }
    fn gate_input_num(&self, _gate: usize) -> usize {
        2
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => self.gates[gate].0.i0,
            1 => self.gates[gate].0.i1,
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs) {
        (
            match self.gates[gate].0.func {
                VGateFunc::And => InstrOp::And,
                VGateFunc::Or => InstrOp::Or,
                VGateFunc::Xor => InstrOp::Xor,
                _ => {
                    panic!("Unsupported InstrOp")
                }
            },
            self.gates[gate].1,
        )
    }
    fn outputs(&self) -> &[(T, bool)] {
        &self.outputs
    }
}

impl<T> CircuitTrait<T> for VLOP3Circuit<T>
where
    T: Clone + Copy,
{
    fn input_len(&self) -> T {
        self.input_len
    }
    fn len(&self) -> usize {
        self.gates.len()
    }
    fn gate_input_num(&self, gate: usize) -> usize {
        if matches!(self.gates[gate].func, VLOP3GateFunc::LOP3(_)) {
            3
        } else {
            2
        }
    }
    fn gate_input(&self, gate: usize, input: usize) -> T {
        match input {
            0 => self.gates[gate].i0,
            1 => self.gates[gate].i1,
            2 => self.gates[gate].i2,
            _ => {
                panic!("No more input");
            }
        }
    }
    fn gate_op(&self, gate: usize) -> (InstrOp, VNegs) {
        (
            match self.gates[gate].func {
                VLOP3GateFunc::And => InstrOp::And,
                VLOP3GateFunc::Or => InstrOp::Or,
                VLOP3GateFunc::Xor => InstrOp::Xor,
                VLOP3GateFunc::LOP3(f) => InstrOp::Lop3(f),
            },
            self.gates[gate].negs,
        )
    }
    fn outputs(&self) -> &[(T, bool)] {
        &self.outputs
    }
}

// var usage - just counter of var usage.

pub(crate) fn gen_var_usage<T, CT>(circuit: &CT) -> Vec<T>
where
    CT: CircuitTrait<T>,
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len()).unwrap();
    let mut var_usage = vec![T::default(); input_len + circuit.len()];
    for gi in 0..circuit.len() {
        for ii in 0..circuit.gate_input_num(gi) {
            let gi0 = usize::try_from(circuit.gate_input(gi, ii)).unwrap();
            let var_usage_0 = usize::try_from(var_usage[gi0]).unwrap() + 1;
            var_usage[gi0] = T::try_from(var_usage_0).unwrap();
        }
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

pub(super) use gate_calc_log_bits::calc_log_bits;

/// Tool to variable allocation at many types (diferrent spaces).
///
/// This tool can works in two modes. In normal mode then allocation method allocates
/// new variable and use_var count usage of variable. In usage mode allocation method
/// do nothing and use_var decrease counter of usage and if it zeroed then remove variable.
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
    /// Creates multi var allocator with specified number of types.
    pub fn new(var_type_num: usize) -> Self {
        Self {
            var_allocs: vec![VarAllocator::new(); var_type_num],
            var_usages: vec![Vec::new(); var_type_num],
            var_maps: vec![HashMap::new(); var_type_num],
            var_new_history: vec![T::default(); var_type_num],
            usage_mode: false,
        }
    }

    /// Sets usage mode.
    pub fn set_usage_mode(&mut self) {
        self.usage_mode = true;
    }

    /// Returns if allocator in usage mode.
    pub fn usage_mode(&self) -> bool {
        self.usage_mode
    }

    /// Returns number of types.
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

    /// Method to allocate variable.
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

    /// Method to use variable.
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

    /// Returns number of allocated variables for given type.
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

#[dynamic]
pub(crate) static GATE_SYS_UNTESTED: bool = match env::var("GATE_SYS_UNTESTED")
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

// Returns final placements in buffer:
// First tuple for circuit input, second for circuit output.
// structure of input tuples:
// * list of position in input:
//   index - circuit input
//   value - position in input buffer
// * length of bits in input buffer
// structure of output tuples:
// * list of position in output:
//   index - circuit output
//   value - position in output buffer
// * length of bits in output buffer

/// Utility that get final placements for circuit inputs and outputs.
///
/// It returns:
// structure of input tuples:
/// * list of position in input: index - circuit input, value - pack element index.
/// * pack length
/// structure of output tuples:
/// * list of position in output: index - circuit output, value - pack element index.
/// * pack length
pub fn get_final_placements(
    input_len: usize,
    output_len: usize,
    code_config: &CodeConfig,
) -> ((Vec<Option<usize>>, usize), (Vec<Option<usize>>, usize)) {
    let input_placement = if !code_config.pop_input_code.is_some()
        || !code_config.pop_from_buffer.is_none()
    {
        let arg_input_map = if let Some(arg_inputs) = code_config.arg_inputs {
            HashMap::from_iter(arg_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let elem_input_map = if let Some(elem_inputs) = code_config.elem_inputs {
            HashMap::from_iter(elem_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
        } else {
            HashMap::new()
        };
        let pop_input_map = if code_config.pop_input_code.is_some() {
            if let Some(pop_inputs) = code_config.pop_from_buffer {
                HashMap::from_iter(pop_inputs.into_iter().enumerate().map(|(i, x)| (*x, i)))
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };
        let mut input_map = vec![];
        let input_map =
            if !arg_input_map.is_empty() || !elem_input_map.is_empty() || !pop_input_map.is_empty()
            {
                let mut count = 0;
                for i in 0..input_len {
                    if !arg_input_map.contains_key(&i)
                        && !elem_input_map.contains_key(&i)
                        && !pop_input_map.contains_key(&i)
                    {
                        input_map.push(Some(count));
                        count += 1;
                    } else {
                        input_map.push(None);
                    }
                }
                (input_map, count)
            } else {
                ((0..input_len).map(|x| Some(x)).collect(), input_len)
            };
        if let Some((ip, is)) = code_config.input_placement {
            (
                input_map
                    .0
                    .into_iter()
                    .map(|p| p.map(|x| ip[x]))
                    .collect::<Vec<_>>(),
                is,
            )
        } else {
            input_map
        }
    } else {
        (vec![None; input_len], 0)
    };

    let output_placement =
        if !code_config.aggr_output_code.is_some() || !code_config.aggr_to_buffer.is_none() {
            let output_map = if let Some(excl_outputs) = code_config.exclude_outputs {
                let mut output_map = vec![];
                let excl_set = HashSet::<usize>::from_iter(excl_outputs.iter().copied());
                let mut count = 0;
                for i in 0..output_len {
                    if !excl_set.contains(&i) {
                        output_map.push(Some(count));
                        count += 1;
                    } else {
                        output_map.push(None)
                    }
                }
                (output_map, count)
            } else {
                ((0..output_len).map(|x| Some(x)).collect(), output_len)
            };
            if let Some((op, os)) = code_config.output_placement {
                (
                    output_map
                        .0
                        .into_iter()
                        .map(|p| p.map(|x| op[x]))
                        .collect::<Vec<_>>(),
                    os,
                )
            } else {
                output_map
            }
        } else {
            (vec![None; output_len], 0)
        };

    (input_placement, output_placement)
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

    #[test]
    fn test_get_final_placements() {
        assert_eq!(
            (
                (vec![Some(0), Some(1), Some(2), Some(3)], 4),
                (vec![Some(0), Some(1), Some(2), Some(3), Some(4)], 5),
            ),
            get_final_placements(4, 5, &CodeConfig::new()),
        );
        assert_eq!(
            (
                (vec![Some(4), Some(5), Some(1), Some(7)], 8),
                (vec![Some(2), Some(4), Some(6), Some(1), Some(9)], 11),
            ),
            get_final_placements(
                4,
                5,
                &CodeConfig::new()
                    .input_placement(Some((&[4, 5, 1, 7], 8)))
                    .output_placement(Some((&[2, 4, 6, 1, 9], 11))),
            ),
        );
        assert_eq!(
            (
                (
                    vec![
                        Some(0),
                        None,
                        None,
                        Some(1),
                        None,
                        Some(2),
                        Some(3),
                        Some(4),
                        Some(5)
                    ],
                    6
                ),
                (vec![Some(0), Some(1), Some(2), Some(3), Some(4)], 5),
            ),
            get_final_placements(9, 5, &CodeConfig::new().arg_inputs(Some(&[2, 4, 1]))),
        );
        assert_eq!(
            (
                (
                    vec![
                        None,
                        Some(0),
                        None,
                        Some(1),
                        None,
                        None,
                        Some(2),
                        None,
                        Some(3)
                    ],
                    4
                ),
                (vec![Some(0), Some(1), Some(2), Some(3), Some(4)], 5),
            ),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .pop_from_buffer(Some(&[7, 0]))
            ),
        );
        assert_eq!(
            (
                (
                    vec![
                        None,
                        Some(6),
                        None,
                        Some(5),
                        None,
                        None,
                        Some(9),
                        None,
                        Some(2)
                    ],
                    12
                ),
                (vec![Some(7), None, Some(2), None, Some(11)], 14),
            ),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .pop_from_buffer(Some(&[7, 0]))
                    .exclude_outputs(Some(&[1, 3]))
                    .input_placement(Some((&[6, 5, 9, 2], 12)))
                    .output_placement(Some((&[7, 2, 11], 14)))
            ),
        );
        assert_eq!(
            (
                (
                    vec![
                        None,
                        Some(6),
                        None,
                        Some(5),
                        None,
                        None,
                        Some(9),
                        None,
                        Some(2)
                    ],
                    12
                ),
                (vec![Some(7), None, Some(2), None, Some(11)], 14),
            ),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .pop_from_buffer(Some(&[7, 0]))
                    .exclude_outputs(Some(&[1, 3]))
                    .aggr_output_code(Some("yyy"))
                    .aggr_to_buffer(Some(&[3, 4]))
                    .input_placement(Some((&[6, 5, 9, 2], 12)))
                    .output_placement(Some((&[7, 2, 11], 14)))
            ),
        );
        assert_eq!(
            (
                (
                    vec![
                        None,
                        Some(6),
                        None,
                        Some(5),
                        None,
                        None,
                        Some(9),
                        None,
                        Some(2)
                    ],
                    12
                ),
                (vec![None; 5], 0),
            ),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .pop_from_buffer(Some(&[7, 0]))
                    .aggr_output_code(Some("yyyy"))
                    .exclude_outputs(Some(&[1, 3]))
                    .input_placement(Some((&[6, 5, 9, 2], 12)))
                    .output_placement(Some((&[7, 2, 11], 14)))
            ),
        );
        assert_eq!(
            (
                (vec![None; 9], 0),
                (vec![Some(7), None, Some(2), None, Some(11)], 14),
            ),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .exclude_outputs(Some(&[1, 3]))
                    .input_placement(Some((&[6, 5, 9, 2], 12)))
                    .output_placement(Some((&[7, 2, 11], 14)))
            ),
        );
        assert_eq!(
            ((vec![None; 9], 0), (vec![None; 5], 0)),
            get_final_placements(
                9,
                5,
                &CodeConfig::new()
                    .arg_inputs(Some(&[2, 4]))
                    .elem_inputs(Some(&[5]))
                    .pop_input_code(Some("xxx"))
                    .aggr_output_code(Some("xxx"))
                    .exclude_outputs(Some(&[1, 3]))
                    .input_placement(Some((&[6, 5, 9, 2], 12)))
                    .output_placement(Some((&[7, 2, 11], 14)))
            ),
        );
    }
}

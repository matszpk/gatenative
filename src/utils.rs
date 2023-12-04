use gatesim::*;

use std::fmt::Debug;

use static_init::dynamic;
use std::time::{SystemTime, UNIX_EPOCH};

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

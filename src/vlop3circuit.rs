use gatesim::*;

use std::fmt::Debug;

use crate::vcircuit::*;
use crate::VNegs::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum VLop3GateFunc {
    And,
    Or,
    Nimpl,
    Xor,
    Lop3(u8),
}

impl TryFrom<VGateFunc> for VLop3GateFunc {
    type Error = String;
    #[inline]
    fn try_from(gf: VGateFunc) -> Result<Self, Self::Error> {
        match gf {
            VGateFunc::And => Ok(VLop3GateFunc::And),
            VGateFunc::Or => Ok(VLop3GateFunc::Or),
            VGateFunc::Nimpl => Ok(VLop3GateFunc::Nimpl),
            VGateFunc::Xor => Ok(VLop3GateFunc::Xor),
            _ => Err("Unsupported!".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct VLop3Gate<T: Clone + Copy> {
    pub(crate) i0: T,
    pub(crate) i1: T,
    pub(crate) i2: T,
    pub(crate) func: VLop3GateFunc,
}

impl<T: Clone + Copy + Default> TryFrom<VGate<T>> for VLop3Gate<T> {
    type Error = String;
    fn try_from(g: VGate<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            i0: g.i0,
            i1: g.i1,
            i2: T::default(),
            func: VLop3GateFunc::try_from(g.func)?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct VLop3Circuit<T: Clone + Copy> {
    pub(crate) input_len: T,
    pub(crate) gates: Vec<VLop3Gate<T>>,
    pub(crate) outputs: Vec<(T, bool)>,
}

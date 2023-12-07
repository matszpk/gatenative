use crate::divide::*;
use crate::*;

use std::marker::PhantomData;

pub struct DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    executor: Vec<E>,
    input_placements: Vec<Placement>,
    output_placements: Vec<Placement>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

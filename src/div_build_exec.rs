use crate::divide::*;
use crate::*;

use std::marker::PhantomData;

struct DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    executor: &'a E,
    d: PhantomData<D>,
    dr: PhantomData<DR>,
    dw: PhantomData<DW>,
}

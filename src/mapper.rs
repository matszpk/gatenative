use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;

struct BasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    executor: E,
    arg_input_max: u32,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

impl<'a, DR, DW, D, E> MapperExecutor<'a, DR, DW, D> for BasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    type ErrorType = E::ErrorType;

    #[inline]
    fn input_len(&self) -> usize {
        self.executor.input_len()
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.executor.output_len()
    }

    fn execute<Out, F>(&mut self, input: &D, init: Out, mut f: F) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, u32) -> Out,
    {
        let input_len = self.input_len();
        let count = if input_len != 0 {
            input.len() / input_len
        } else {
            0
        };
        let mut output = self.executor.new_data(count * self.output_len());
        let mut out = init;
        for arg in 0..=self.arg_input_max {
            self.executor.execute_reuse(input, arg, &mut output)?;
            out = f(out, input, &output, arg);
        }
        Ok(out)
    }

    fn new_data(&mut self, len: usize) -> D {
        self.executor.new_data(len)
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D {
        self.executor.new_data_from_vec(data)
    }
}

pub struct BasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    builder: B,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
    e: PhantomData<&'a E>,
}

impl<'a, DR, DW, D, E, B> MapperBuilder<'a, DR, DW, D, BasicMapperExecutor<'a, DR, DW, D, E>>
    for BasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    type ErrorType = B::ErrorType;

    fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.builder
            .add(name, circuit, None, None, Some(arg_inputs));
    }

    fn build(self) -> Result<Vec<BasicMapperExecutor<'a, DR, DW, D, E>>, Self::ErrorType> {
        Ok(vec![])
    }

    fn word_len(&self) -> u32 {
        self.builder.word_len()
    }

    fn is_data_holder_global() -> bool {
        B::is_data_holder_global()
    }
    fn is_data_holder_in_builder() -> bool {
        B::is_data_holder_in_builder()
    }
}

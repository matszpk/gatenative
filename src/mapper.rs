use crate::*;

use rayon::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;

pub struct BasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    executor: E,
    arg_input_max: u64,
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
    fn real_input_len(&self) -> usize {
        self.executor.real_input_len()
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.executor.output_len()
    }

    fn execute<Out, F>(&mut self, input: &D, init: Out, mut f: F) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, u64) -> Out,
    {
        let input_len = self.real_input_len();
        // calculate chunk count
        let count = if input_len != 0 {
            input.len() / input_len
        } else {
            0
        };
        let mut output = self.executor.new_data(count * self.output_len());
        let mut out = init;
        // just execute
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

    fn new_data_from_slice(&mut self, data: &[u32]) -> D {
        self.executor.new_data_from_slice(data)
    }

    fn word_len(&self) -> u32 {
        self.executor.word_len()
    }
}

impl<'a, DR, DW, D, E, IDT, ODT> DataTransforms<'a, DR, DW, D, IDT, ODT>
    for BasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D> + DataTransforms<'a, DR, DW, D, IDT, ODT>,
    IDT: DataTransformer<'a, DR, DW, D>,
    ODT: DataTransformer<'a, DR, DW, D>,
{
    type ErrorType = <E as DataTransforms<'a, DR, DW, D, IDT, ODT>>::ErrorType;

    fn input_tx(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType> {
        self.executor.input_tx(input_elem_len, bit_mapping)
    }
    fn output_tx(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<ODT, Self::ErrorType> {
        self.executor.output_tx(output_elem_len, bit_mapping)
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
    arg_input_lens: Vec<usize>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
    e: PhantomData<&'a E>,
}

impl<'a, DR, DW, D, E, B> BasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    pub fn new(builder: B) -> Self {
        assert!(builder.is_empty());
        Self {
            builder,
            arg_input_lens: vec![],
            d: PhantomData,
            dr: PhantomData,
            dw: PhantomData,
            e: PhantomData,
        }
    }
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
        assert!(arg_inputs.len() <= 32);
        self.arg_input_lens.push(arg_inputs.len());
        self.builder
            .add(name, circuit, None, None, Some(arg_inputs));
    }

    fn build(self) -> Result<Vec<BasicMapperExecutor<'a, DR, DW, D, E>>, Self::ErrorType> {
        self.builder.build().map(|execs| {
            execs
                .into_iter()
                .zip(self.arg_input_lens)
                .map(|(e, arg_len)| BasicMapperExecutor {
                    executor: e,
                    arg_input_max: u64::try_from((1u128 << arg_len) - 1u128).unwrap(),
                    d: PhantomData,
                    dr: PhantomData,
                    dw: PhantomData,
                })
                .collect::<Vec<_>>()
        })
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.builder.word_len()
    }

    #[inline]
    fn is_data_holder_global() -> bool {
        B::is_data_holder_global()
    }
    #[inline]
    fn is_data_holder_in_builder() -> bool {
        B::is_data_holder_in_builder()
    }

    #[inline]
    fn preferred_input_count(&self) -> usize {
        self.builder.preferred_input_count()
    }
}

// parallel
pub struct ParBasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    E::ErrorType: Send,
{
    executor: E,
    arg_input_max: u64,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

impl<'a, DR, DW, D, E> ParMapperExecutor<'a, DR, DW, D> for ParBasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    E::ErrorType: Send,
{
    type ErrorType = E::ErrorType;

    #[inline]
    fn input_len(&self) -> usize {
        self.executor.input_len()
    }
    #[inline]
    fn real_input_len(&self) -> usize {
        self.executor.real_input_len()
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.executor.output_len()
    }

    fn execute<Out, F, G>(
        &mut self,
        input: &D,
        init: Out,
        f: F,
        g: G,
    ) -> Result<Out, Self::ErrorType>
    where
        F: Fn(&D, &D, u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Out: Clone + Send + Sync,
    {
        (0..=self.arg_input_max)
            .into_par_iter()
            .map(|arg| {
                // just execute executor
                self.executor
                    .try_clone()
                    .unwrap()
                    .execute(input, arg)
                    .map(|output| f(input, &output, arg))
            })
            .reduce(
                || Ok(init.clone()),
                |a, b| {
                    // check whether is ok otherwise return error
                    if let Ok(av) = a {
                        if let Ok(bv) = b {
                            // join results
                            Ok(g(av, bv))
                        } else {
                            b
                        }
                    } else {
                        a
                    }
                },
            )
    }

    fn new_data(&mut self, len: usize) -> D {
        self.executor.new_data(len)
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D {
        self.executor.new_data_from_vec(data)
    }

    fn new_data_from_slice(&mut self, data: &[u32]) -> D {
        self.executor.new_data_from_slice(data)
    }

    fn word_len(&self) -> u32 {
        self.executor.word_len()
    }
}

impl<'a, DR, DW, D, E, IDT, ODT> DataTransforms<'a, DR, DW, D, IDT, ODT>
    for ParBasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + DataTransforms<'a, DR, DW, D, IDT, ODT> + Send + Sync,
    IDT: DataTransformer<'a, DR, DW, D>,
    ODT: DataTransformer<'a, DR, DW, D>,
    <E as Executor<'a, DR, DW, D>>::ErrorType: Send,
    <E as DataTransforms<'a, DR, DW, D, IDT, ODT>>::ErrorType: Send,
{
    type ErrorType = <E as DataTransforms<'a, DR, DW, D, IDT, ODT>>::ErrorType;

    fn input_tx(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType> {
        self.executor.input_tx(input_elem_len, bit_mapping)
    }
    fn output_tx(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<ODT, Self::ErrorType> {
        self.executor.output_tx(output_elem_len, bit_mapping)
    }
}

pub struct ParBasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    B: Builder<'a, DR, DW, D, E>,
    E::ErrorType: Send,
{
    builder: B,
    arg_input_lens: Vec<usize>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
    e: PhantomData<&'a E>,
}

impl<'a, DR, DW, D, E, B> ParBasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    B: Builder<'a, DR, DW, D, E>,
    E::ErrorType: Send,
{
    pub fn new(builder: B) -> Self {
        assert!(B::is_data_holder_global() && B::is_executor_per_thread());
        assert!(builder.is_empty());
        Self {
            builder,
            arg_input_lens: vec![],
            d: PhantomData,
            dr: PhantomData,
            dw: PhantomData,
            e: PhantomData,
        }
    }
}

impl<'a, DR, DW, D, E, B> ParMapperBuilder<'a, DR, DW, D, ParBasicMapperExecutor<'a, DR, DW, D, E>>
    for ParBasicMapperBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    B: Builder<'a, DR, DW, D, E>,
    E::ErrorType: Send,
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
        assert!(arg_inputs.len() <= 32);
        self.arg_input_lens.push(arg_inputs.len());
        self.builder
            .add(name, circuit, None, None, Some(arg_inputs));
    }

    fn build(self) -> Result<Vec<ParBasicMapperExecutor<'a, DR, DW, D, E>>, Self::ErrorType> {
        self.builder.build().map(|execs| {
            execs
                .into_iter()
                .zip(self.arg_input_lens)
                .map(|(e, arg_len)| ParBasicMapperExecutor {
                    executor: e,
                    arg_input_max: u64::try_from((1u128 << arg_len) - 1u128).unwrap(),
                    d: PhantomData,
                    dr: PhantomData,
                    dw: PhantomData,
                })
                .collect::<Vec<_>>()
        })
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.builder.word_len()
    }

    #[inline]
    fn is_data_holder_global() -> bool {
        B::is_data_holder_global()
    }
    #[inline]
    fn is_data_holder_in_builder() -> bool {
        B::is_data_holder_in_builder()
    }

    #[inline]
    fn preferred_input_count(&self) -> usize {
        self.builder.preferred_input_count()
    }
}

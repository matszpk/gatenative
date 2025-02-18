//! The module provides basic mappers.
//!
//! This module implements mappers defined in main module. Mapper builders requires
//! child builder that builds and creates executor to run simulation execution.

use crate::*;

use rayon::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{
    atomic::{self, AtomicBool},
    Arc,
};

use std::marker::PhantomData;

/// Basic mapper executor.
///
/// This mapper executor can reuses output buffer between consecutive executions efficiently
/// and if dont_clear_outputs is set then omit clearing output buffer between executions.
///
/// This executor provides data transformers by [DataTransforms]. See more in [MapperExecutor].
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

    fn execute<Out, F, Stop>(
        &mut self,
        input: &D,
        init: Out,
        mut f: F,
        mut stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, u64) -> Out,
        Stop: FnMut(&Out) -> bool,
    {
        let mut out = init;
        // just execute
        let mut output = self
            .executor
            .new_data_output_elems(self.executor.elem_count(input.len()));
        let need_clear_output = self.executor.need_clear_outputs();
        for arg in 0..=self.arg_input_max {
            if need_clear_output {
                output.fill(0);
            }
            self.executor.execute_reuse(input, arg, &mut output)?;
            out = f(out, input, &output, arg);
            if stop(&out) {
                break;
            }
        }
        Ok(out)
    }

    fn execute_buffer<Out, F, Stop>(
        &mut self,
        input: &D,
        buffer: &mut D,
        init: Out,
        mut f: F,
        mut stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, &D, u64) -> Out,
        Stop: FnMut(&Out) -> bool,
    {
        let mut out = init;
        // just execute
        let mut output = self
            .executor
            .new_data_output_elems(self.executor.elem_count(input.len()));
        let need_clear_output = self.executor.need_clear_outputs();
        for arg in 0..=self.arg_input_max {
            if need_clear_output {
                output.fill(0);
            }
            self.executor
                .execute_buffer_reuse(input, arg, &mut output, buffer)?;
            out = f(out, input, &output, &buffer, arg);
            if stop(&out) {
                break;
            }
        }
        Ok(out)
    }

    #[inline]
    fn elem_count(&self, input_len: usize) -> usize {
        self.executor.elem_count(input_len)
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

    #[inline]
    fn input_data_len(&self, elem_num: usize) -> usize {
        self.executor.input_data_len(elem_num)
    }

    #[inline]
    fn output_data_len(&self, elem_num: usize) -> usize {
        self.executor.output_data_len(elem_num)
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.executor.word_len()
    }

    #[inline]
    fn output_is_aggregated(&self) -> bool {
        self.executor.output_is_aggregated()
    }

    #[inline]
    fn is_aggregated_to_buffer(&self) -> bool {
        self.executor.is_aggregated_to_buffer()
    }

    #[inline]
    fn aggr_output_len(&self) -> Option<usize> {
        self.executor.aggr_output_len()
    }

    #[inline]
    fn input_is_populated(&self) -> bool {
        self.executor.input_is_populated()
    }

    #[inline]
    fn is_populated_from_buffer(&self) -> bool {
        self.executor.is_populated_from_buffer()
    }

    #[inline]
    fn pop_input_len(&self) -> Option<usize> {
        self.executor.pop_input_len()
    }

    fn is_sequential_execution(&self) -> bool {
        self.executor.is_sequential_execution()
    }

    #[inline]
    fn inner_loop(&self) -> Option<u32> {
        self.executor.inner_loop()
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

    fn input_transformer(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType> {
        self.executor.input_transformer(input_elem_len, bit_mapping)
    }
    fn output_transformer(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<ODT, Self::ErrorType> {
        self.executor
            .output_transformer(output_elem_len, bit_mapping)
    }
}

impl<'a, DR, DW, D, E> BasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    /// Returns child executor used to execute simulation.
    pub fn executor(&self) -> &E {
        &self.executor
    }
}

/// Basic mapper builder.
///
/// See more in [MapperBuilder].
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
    /// Creates new Basic mapper builder using child builder given in `builder`.
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

    fn user_defs(&mut self, user_defs: &str) {
        self.builder.user_defs(user_defs);
    }
    fn transform_helpers(&mut self) {
        self.builder.transform_helpers();
    }

    unsafe fn add_internal<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.arg_input_lens
            .push(code_config.arg_inputs.as_ref().unwrap().len());
        self.builder.add_with_config(name, circuit, code_config);
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
    fn type_len(&self) -> u32 {
        self.builder.type_len()
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

// iterator helper

struct StoppableIterator<I: Iterator> {
    inner: I,
    do_stop: Arc<AtomicBool>,
}

impl<I: Iterator> StoppableIterator<I> {
    fn new(inner: I, do_stop: Arc<AtomicBool>) -> Self {
        Self { inner, do_stop }
    }
}

impl<I: Iterator> Iterator for StoppableIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.do_stop.load(atomic::Ordering::SeqCst) {
            self.inner.next()
        } else {
            None
        }
    }
}

// parallel
/// Basic parallel mapper executor.
///
/// This executor provides data transformers by [DataTransforms].
/// See more in [ParMapperExecutor].
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

    fn execute<Out, F, G, Stop>(
        &mut self,
        input: &D,
        init: Out,
        f: F,
        g: G,
        stop: Stop,
    ) -> Result<Out, Self::ErrorType>
    where
        F: Fn(&D, &D, u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Stop: Fn(&Out) -> bool + Send + Sync,
        Out: Clone + Send + Sync,
    {
        let do_stop = Arc::new(AtomicBool::new(false));
        StoppableIterator::new(0..=self.arg_input_max, do_stop.clone())
            .par_bridge()
            .map(|arg| {
                // just execute executor
                let r = self
                    .executor
                    .try_clone()
                    .unwrap()
                    .execute(input, arg)
                    .map(|output| f(input, &output, arg))?;
                do_stop.fetch_or(stop(&r), atomic::Ordering::SeqCst);
                Ok(r)
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

    #[inline]
    fn elem_count(&self, input_len: usize) -> usize {
        self.executor.elem_count(input_len)
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

    #[inline]
    fn input_data_len(&self, elem_num: usize) -> usize {
        self.executor.input_data_len(elem_num)
    }

    #[inline]
    fn output_data_len(&self, elem_num: usize) -> usize {
        self.executor.output_data_len(elem_num)
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.executor.word_len()
    }

    #[inline]
    fn output_is_aggregated(&self) -> bool {
        self.executor.output_is_aggregated()
    }

    #[inline]
    fn aggr_output_len(&self) -> Option<usize> {
        self.executor.aggr_output_len()
    }

    #[inline]
    fn input_is_populated(&self) -> bool {
        self.executor.input_is_populated()
    }

    #[inline]
    fn pop_input_len(&self) -> Option<usize> {
        self.executor.pop_input_len()
    }

    fn is_sequential_execution(&self) -> bool {
        self.executor.is_sequential_execution()
    }

    #[inline]
    fn inner_loop(&self) -> Option<u32> {
        self.executor.inner_loop()
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

    fn input_transformer(
        &self,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<IDT, Self::ErrorType> {
        self.executor.input_transformer(input_elem_len, bit_mapping)
    }
    fn output_transformer(
        &self,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<ODT, Self::ErrorType> {
        self.executor
            .output_transformer(output_elem_len, bit_mapping)
    }
}

impl<'a, DR, DW, D, E> ParBasicMapperExecutor<'a, DR, DW, D, E>
where
    DR: DataReader + Send + Sync,
    DW: DataWriter + Send + Sync,
    D: DataHolder<'a, DR, DW> + Send + Sync,
    E: Executor<'a, DR, DW, D> + Send + Sync,
    E::ErrorType: Send,
{
    pub fn executor(&self) -> &E {
        &self.executor
    }
}

/// Basic parallel mapper builder.
///
/// See more in [ParMapperBuilder].
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
    /// Creates new Basic parallel mapper builder using child builder given in `builder`.
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

    fn user_defs(&mut self, user_defs: &str) {
        self.builder.user_defs(user_defs);
    }
    fn transform_helpers(&mut self) {
        self.builder.transform_helpers();
    }

    unsafe fn add_internal<T>(&mut self, name: &str, circuit: Circuit<T>, code_config: CodeConfig)
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.arg_input_lens
            .push(code_config.arg_inputs.as_ref().unwrap().len());
        self.builder.add_with_config(name, circuit, code_config);
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
    fn type_len(&self) -> u32 {
        self.builder.type_len()
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

use crate::cpu_build_exec::*;
use crate::opencl_build_exec::*;

/// Type of Basic mapper executor that uses CPUExecutor.
pub type CPUBasicMapperExecutor<'a> =
    BasicMapperExecutor<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder, CPUExecutor>;
/// Type of Basic mapper builder that uses CPUBuilder.
pub type CPUBasicMapperBuilder<'a> = BasicMapperBuilder<
    'a,
    CPUDataReader<'a>,
    CPUDataWriter<'a>,
    CPUDataHolder,
    CPUExecutor,
    CPUBuilder<'a>,
>;

/// Type of Basic mapper executor that uses OpenCLExecutor.
pub type OpenCLBasicMapperExecutor<'a> = BasicMapperExecutor<
    'a,
    OpenCLDataReader<'a>,
    OpenCLDataWriter<'a>,
    OpenCLDataHolder,
    OpenCLExecutor,
>;
/// Type of Basic mapper builder that uses OpenCLBuilder.
pub type OpenCLBasicMapperBuilder<'a> = BasicMapperBuilder<
    'a,
    OpenCLDataReader<'a>,
    OpenCLDataWriter<'a>,
    OpenCLDataHolder,
    OpenCLExecutor,
    OpenCLBuilder<'a>,
>;

/// Type of Basic parallel mapper executor that uses CPUExecutor.
pub type CPUParBasicMapperExecutor<'a> =
    ParBasicMapperExecutor<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder, CPUExecutor>;
/// Type of Basic parallel mapper builder that uses CPUBuilder.
pub type CPUParBasicMapperBuilder<'a> = ParBasicMapperBuilder<
    'a,
    CPUDataReader<'a>,
    CPUDataWriter<'a>,
    CPUDataHolder,
    CPUExecutor,
    CPUBuilder<'a>,
>;

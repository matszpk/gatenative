use crate::*;

use rayon::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;

// TODO: Add adapter from Mapper to ParMapper

// TODO: Add mapper that join parallel-mapper and one or more sequential-mappers.

pub struct BasicMapperExecutor<'a, DR, DW, D, E>
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
    fn real_input_len(&self) -> usize {
        self.executor.real_input_len()
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.executor.output_len()
    }

    fn execute<Out, F>(&mut self, input: &D, init: Out, mut f: F) -> Result<Out, Self::ErrorType>
    where
        F: FnMut(Out, &D, &D, u32) -> Out,
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
                    arg_input_max: u32::try_from((1u64 << arg_len) - 1u64).unwrap(),
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
    arg_input_max: u32,
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
        F: Fn(&D, &D, u32) -> Out + Send + Sync,
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
                    arg_input_max: u32::try_from((1u64 << arg_len) - 1u64).unwrap(),
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

// ParSeqMapper - mapper that join parallel and sequential mapper

pub struct ParSeqDataWriter<PDW: DataWriter + Send + Sync, SDW: DataWriter> {
    par_writer: PDW,
    seq_writers: Vec<SDW>,
}

impl<PDW: DataWriter + Send + Sync, SDW: DataWriter> DataWriter for ParSeqDataWriter<PDW, SDW> {
    fn get_mut(&mut self) -> &mut [u32] {
        self.par_writer.get_mut()
    }
}

impl<PDW: DataWriter + Send + Sync, SDW: DataWriter> Drop for ParSeqDataWriter<PDW, SDW> {
    fn drop(&mut self) {
        let par_data = self.par_writer.get_mut();
        for sw in &mut self.seq_writers {
            sw.get_mut().copy_from_slice(par_data);
        }
    }
}

pub struct ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader,
    SDW: DataWriter,
    SD: DataHolder<'a, SDR, SDW>,
{
    par: PD,       // parallel data holder
    seqs: Vec<SD>, // sequential data holders
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
}

impl<'a, PDR, PDW, PD, SDR, SDW, SD> DataHolder<'a, PDR, ParSeqDataWriter<PDW, SDW>>
    for ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader,
    SDW: DataWriter,
    SD: DataHolder<'a, SDR, SDW>,
{
    fn len(&self) -> usize {
        self.par.len()
    }

    fn set_range(&mut self, range: Range<usize>) {
        self.par.set_range(range.clone());
        for s in &mut self.seqs {
            s.set_range(range.clone());
        }
    }

    fn get(&'a self) -> PDR {
        self.par.get()
    }
    fn get_mut(&'a mut self) -> ParSeqDataWriter<PDW, SDW> {
        ParSeqDataWriter {
            par_writer: self.par.get_mut(),
            seq_writers: self
                .seqs
                .iter_mut()
                .map(|x| x.get_mut())
                .collect::<Vec<_>>(),
        }
    }
    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        self.par.process(f)
    }

    fn process_mut<F, Out>(&mut self, mut f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        self.par.process_mut(|par_data| {
            let out = f(par_data);
            for s in &mut self.seqs {
                s.process_mut(|data| data.copy_from_slice(par_data));
            }
            out
        })
    }

    fn release(self) -> Vec<u32> {
        let out = self.par.release();
        for s in self.seqs {
            s.free();
        }
        out
    }

    fn free(self) {
        self.par.free();
        for s in self.seqs {
            s.free();
        }
    }
}

pub enum ParSeqExecutorError<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    PE::ErrorType: Send,
    SDR: DataReader,
    SDW: DataWriter,
    SD: DataHolder<'a, SDR, SDW>,
    SE: Executor<'a, SDR, SDW, SD>,
    SE::ErrorType: Send,
{
    ParError(PE::ErrorType),
    SeqError(SE::ErrorType),
}

pub struct ParSeqDataExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    SDR: DataReader,
    SDW: DataWriter,
    SD: DataHolder<'a, SDR, SDW>,
    SE: Executor<'a, SDR, SDW, SD>,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
{
    par: PE,
    seqs: Vec<SE>,
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    pd: PhantomData<&'a PD>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
    sd: PhantomData<&'a SD>,
}

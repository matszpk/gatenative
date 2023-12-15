use crate::*;

use rayon::prelude::*;
use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::Mutex;

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

pub enum ParSeqDataReader<PDR: DataReader + Send + Sync, SDR: DataReader> {
    Par(PDR),
    Seq(SDR),
}

impl<PDR: DataReader + Send + Sync, SDR: DataReader> DataReader for ParSeqDataReader<PDR, SDR> {
    fn get(&self) -> &[u32] {
        match self {
            ParSeqDataReader::Par(p) => p.get(),
            ParSeqDataReader::Seq(s) => s.get(),
        }
    }
}

pub struct ParSeqDataWriter<PDW, SDW>
where
    PDW: DataWriter + Send + Sync,
    SDW: DataWriter + Send + Sync,
{
    par_writer: PDW,
    seq_writers: Vec<SDW>,
}

impl<PDW, SDW> DataWriter for ParSeqDataWriter<PDW, SDW>
where
    PDW: DataWriter + Send + Sync,
    SDW: DataWriter + Send + Sync,
{
    fn get_mut(&mut self) -> &mut [u32] {
        self.par_writer.get_mut()
    }
}

impl<PDW, SDW> Drop for ParSeqDataWriter<PDW, SDW>
where
    PDW: DataWriter + Send + Sync,
    SDW: DataWriter + Send + Sync,
{
    fn drop(&mut self) {
        let par_data = self.par_writer.get_mut();
        for sw in &mut self.seq_writers {
            sw.get_mut().copy_from_slice(par_data);
        }
    }
}

pub enum ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
{
    All { par: PD, seqs: Vec<SD> },
    ParRef(&'a PD),
    SeqRef(usize, &'a SD),
    PDR(PhantomData<&'a PDR>),
    PDW(PhantomData<&'a PDW>),
    SDR(PhantomData<&'a SDR>),
    SDW(PhantomData<&'a SDW>),
}

impl<'a, PDR, PDW, PD, SDR, SDW, SD> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
{
    fn par_ref(&'a self) -> Self {
        match self {
            Self::All { par, .. } => Self::ParRef(&par),
            Self::ParRef(par) => Self::ParRef(par),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
    fn seq_ref(&'a self, index: usize) -> Self {
        match self {
            Self::All { seqs, .. } => Self::SeqRef(index, &seqs[index]),
            Self::SeqRef(r_index, seq) => {
                assert_eq!(*r_index, index);
                Self::SeqRef(index, seq)
            }
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
}

impl<'a, PDR, PDW, PD, SDR, SDW, SD>
    DataHolder<'a, ParSeqDataReader<PDR, SDR>, ParSeqDataWriter<PDW, SDW>>
    for ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
{
    fn len(&self) -> usize {
        match self {
            Self::All { par, .. } => par.len(),
            Self::ParRef(par) => par.len(),
            Self::SeqRef(_, seq) => seq.len(),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn set_range(&mut self, range: Range<usize>) {
        if let Self::All { par, seqs } = self {
            par.set_range(range.clone());
            for s in seqs {
                s.set_range(range.clone());
            }
        } else {
            panic!("Unsupported");
        }
    }

    fn get(&'a self) -> ParSeqDataReader<PDR, SDR> {
        match self {
            Self::All { par, .. } => ParSeqDataReader::Par(par.get()),
            Self::ParRef(par) => ParSeqDataReader::Par(par.get()),
            Self::SeqRef(_, seq) => ParSeqDataReader::Seq(seq.get()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
    fn get_mut(&'a mut self) -> ParSeqDataWriter<PDW, SDW> {
        if let Self::All { par, seqs } = self {
            ParSeqDataWriter {
                par_writer: par.get_mut(),
                seq_writers: seqs.iter_mut().map(|x| x.get_mut()).collect::<Vec<_>>(),
            }
        } else {
            panic!("Unsupported");
        }
    }

    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        match self {
            Self::All { par, .. } => par.process(f),
            Self::ParRef(par) => par.process(f),
            Self::SeqRef(_, seq) => seq.process(f),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn process_mut<F, Out>(&mut self, mut f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        if let Self::All { par, seqs } = self {
            par.process_mut(|par_data| {
                let out = f(par_data);
                for s in seqs.iter_mut() {
                    s.process_mut(|data| data.copy_from_slice(par_data));
                }
                out
            })
        } else {
            panic!("Unsupported");
        }
    }

    fn release(self) -> Vec<u32> {
        match self {
            Self::All { par, seqs } => {
                let out = par.release();
                for s in seqs {
                    s.free();
                }
                out
            }
            Self::ParRef(par) => par.get().get().to_vec(),
            Self::SeqRef(_, seq) => seq.get().get().to_vec(),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn free(self) {
        match self {
            Self::All { par, seqs } => {
                par.free();
                for s in seqs {
                    s.free();
                }
            }
            Self::ParRef(_) => {}
            Self::SeqRef(_, _) => {}
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ParSeqMapperExecutorError<PE, SE> {
    #[error("ParError {0}")]
    ParError(#[from] PE),
    #[error("SeqError for {0} {1}")]
    SeqError(usize, SE),
}

pub struct ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
{
    par: PE,
    seqs: Vec<Mutex<SE>>,
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    pd: PhantomData<&'a PD>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
    sd: PhantomData<&'a SD>,
}

impl<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
    ParMapperExecutor<
        'a,
        ParSeqDataReader<PDR, SDR>,
        ParSeqDataWriter<PDW, SDW>,
        ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
    > for ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
{
    type ErrorType = ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>;

    fn input_len(&self) -> usize {
        self.par.input_len()
    }
    fn real_input_len(&self) -> usize {
        self.par.real_input_len()
    }
    fn output_len(&self) -> usize {
        self.par.output_len()
    }
    fn execute<Out, F, G>(
        &mut self,
        input: &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
    ) -> Result<Out, Self::ErrorType>
    where
        F: Fn(
                &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
                &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
                u32,
            ) -> Out
            + Send
            + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Out: Clone + Send + Sync,
    {
        Ok(init)
    }

    fn new_data(&mut self, len: usize) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::All {
            par: self.par.new_data(len),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data(len))
                .collect::<Vec<_>>(),
        }
    }

    fn new_data_from_vec(
        &mut self,
        data: Vec<u32>,
    ) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::All {
            par: self.par.new_data_from_slice(&data),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data_from_slice(&data))
                .collect::<Vec<_>>(),
        }
    }

    fn new_data_from_slice(
        &mut self,
        data: &[u32],
    ) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::All {
            par: self.par.new_data_from_slice(data),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data_from_slice(data))
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParSeqMapperBuilderError<PE, SE> {
    #[error("ParError {0}")]
    ParError(#[from] PE),
    #[error("SeqError for {0} {1}")]
    SeqError(usize, SE),
}

pub struct ParSeqMapperBuilder<'a, PDR, PDW, PD, PE, PB, SDR, SDW, SD, SE, SB>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    PB: Builder<'a, PDR, PDW, PD, PE>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
    SB: Builder<'a, SDR, SDW, SD, SE>,
{
    par: PB,
    seqs: Vec<SB>,
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    pd: PhantomData<&'a PD>,
    pe: PhantomData<&'a PE>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
    sd: PhantomData<&'a SD>,
    se: PhantomData<&'a SE>,
}

impl<'a, PDR, PDW, PD, PE, PB, SDR, SDW, SD, SE, SB>
    ParMapperBuilder<
        'a,
        ParSeqDataReader<PDR, SDR>,
        ParSeqDataWriter<PDW, SDW>,
        ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>,
    > for ParSeqMapperBuilder<'a, PDR, PDW, PD, PE, PB, SDR, SDW, SD, SE, SB>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    PB: Builder<'a, PDR, PDW, PD, PE>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
    SB: Builder<'a, SDR, SDW, SD, SE>,
{
    type ErrorType = ParSeqMapperBuilderError<PB::ErrorType, SB::ErrorType>;

    fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.par
            .add(name, circuit.clone(), None, None, Some(arg_inputs));
        for s in &mut self.seqs {
            s.add(name, circuit.clone(), None, None, Some(arg_inputs));
        }
    }

    fn build(
        self,
    ) -> Result<Vec<ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>>, Self::ErrorType>
    {
        let par_execs = self.par.build()?;
        let mut seq_execs = HashMap::new();
        let seqs_len = self.seqs.len();
        for (i, s) in self.seqs.into_iter().enumerate() {
            match s.build() {
                Ok(execs) => {
                    for (j, e) in execs.into_iter().enumerate() {
                        seq_execs.insert((i, j), Mutex::new(e));
                    }
                }
                Err(err) => {
                    return Err(ParSeqMapperBuilderError::SeqError(i, err));
                }
            }
        }
        Ok(par_execs
            .into_iter()
            .enumerate()
            .map(|(i, par)| ParSeqMapperExecutor {
                par,
                seqs: (0..seqs_len)
                    .map(|x| seq_execs.remove(&(x, i)).unwrap())
                    .collect::<Vec<_>>(),
                pdr: PhantomData,
                pdw: PhantomData,
                pd: PhantomData,
                sdr: PhantomData,
                sdw: PhantomData,
                sd: PhantomData,
            })
            .collect::<Vec<_>>())
    }

    fn word_len(&self) -> u32 {
        panic!("Use other way!");
    }

    fn is_data_holder_global() -> bool {
        PB::is_data_holder_global() && SB::is_data_holder_global()
    }
    fn is_data_holder_in_builder() -> bool {
        PB::is_data_holder_in_builder() && SB::is_data_holder_in_builder()
    }
    fn preferred_input_count(&self) -> usize {
        std::cmp::max(
            self.par.preferred_input_count(),
            self.seqs
                .iter()
                .map(|x| x.preferred_input_count())
                .max()
                .unwrap(),
        )
    }
}

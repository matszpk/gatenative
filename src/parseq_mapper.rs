#![cfg_attr(docsrs, feature(doc_cfg))]
//! Mapper that joins together two types of simulations (CPU and OpenCL).
//!
//! This mapper simplifies organization of heterogeneous simulations.
//! The ParSeqMapper is mapper that join two types of simulation executions:
//! * parallel executed (multiple executions on multiple threads) (for example on CPU).
//! * sequentially executed (by only one thread) (for example on OpenCL device).
//!
//! It is working similar to ParMapper but runs simulation on devices of one of two types.
//! It provides similar functionality by different interface, because uses two types of
//! builder and executor objects. For parrallel simulation, mapper executes multiple simulations,
//! however it can uses only one builder and executor (for circuit).
//! Mapper can use multiple sequential simulations concurrently (for example multiple GPU).

use crate::*;

use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::{
    atomic::{self, AtomicBool, AtomicU64},
    Arc, Mutex,
};

// ParSeqMapper - mapper that join parallel and sequential mapper

/// It can holds object of one of two types (for parallel and sequntial).
#[derive(Clone, Debug)]
pub enum ParSeqObject<T1, T2> {
    /// Object for parrallel simulation.
    Par(T1),
    /// Object for sequential simulation.
    Seq(T2),
}

impl<T1, T2> ParSeqObject<T1, T2> {
    /// Returns object for parrallel simulation.
    pub fn par(self) -> Option<T1> {
        if let Self::Par(o) = self {
            Some(o)
        } else {
            None
        }
    }
    /// Returns object for sequential simulation.
    pub fn seq(self) -> Option<T2> {
        if let Self::Seq(o) = self {
            Some(o)
        } else {
            None
        }
    }
}

/// Object that refers to selection of object for simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParSeqSelection {
    /// This is parrallel simulation.
    Par,
    /// This is sequential simulation with given index.
    Seq(usize),
}

/// Data holder that holds all data for all simulations.
///
/// Parameter types:
/// * `PDR`, `PDW` and `PD` - parrallel data reader, data writer and data holder.
/// * `SDR`, `SDW` and `SD` - sequential data reader, data writer and data holder.
pub struct ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
{
    par: PD,
    seqs: Vec<SD>,
    real_input_len: usize,
    max_word_len: usize, // in 32-bit words
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
}

impl<'a, PDR, PDW, PD, SDR, SDW, SD> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
{
    /// Returns length of data in data holder in 32-bit words.
    #[inline]
    pub fn len(&self) -> usize {
        self.par.len()
    }

    /// Process data for given simulation.
    pub fn process_single<F, Out>(&self, sel: ParSeqSelection, mut f: F) -> Out
    where
        F: FnMut(ParSeqObject<&PD, (usize, &SD)>) -> Out,
    {
        match sel {
            ParSeqSelection::Par => f(ParSeqObject::Par(&self.par)),
            ParSeqSelection::Seq(i) => f(ParSeqObject::Seq((i, &self.seqs[i]))),
        }
    }

    /// Process mutually data for given simulation.
    pub fn process_single_mut<F, Out>(&mut self, sel: ParSeqSelection, mut f: F) -> Out
    where
        F: FnMut(ParSeqObject<&mut PD, (usize, &mut SD)>) -> Out,
    {
        match sel {
            ParSeqSelection::Par => f(ParSeqObject::Par(&mut self.par)),
            ParSeqSelection::Seq(i) => f(ParSeqObject::Seq((i, &mut self.seqs[i]))),
        }
    }

    /// Process data for all simulations.
    pub fn process<F, Out>(&self, mut f: F) -> Vec<Out>
    where
        F: FnMut(ParSeqObject<&PD, (usize, &SD)>) -> Out,
    {
        let mut out = vec![];
        out.push(f(ParSeqObject::Par(&self.par)));
        for (i, s) in self.seqs.iter().enumerate() {
            out.push(f(ParSeqObject::Seq((i, s))));
        }
        out
    }

    /// Process mutually data for all simulations.
    pub fn process_mut<F, Out>(&mut self, mut f: F) -> Vec<Out>
    where
        F: FnMut(ParSeqObject<&mut PD, (usize, &mut SD)>) -> Out,
    {
        let mut out = vec![];
        out.push(f(ParSeqObject::Par(&mut self.par)));
        for (i, s) in self.seqs.iter_mut().enumerate() {
            out.push(f(ParSeqObject::Seq((i, s))));
        }
        out
    }

    fn check_length(&self) {
        assert!((self.par.len() % self.max_word_len) == 0);
        assert!(self.seqs.iter().all(|s| (s.len() % self.max_word_len) == 0));
        let expected_len = self.par.len();
        assert!(self.seqs.iter().all(|s| s.len() == expected_len));
        let expected_word_num = expected_len / self.max_word_len;
        if self.real_input_len != 0 {
            let expected_chunks = (expected_word_num) / self.real_input_len;
            assert_eq!((expected_word_num) % self.real_input_len, 0);
            assert!(self.seqs.iter().all(|s| {
                let s_len = s.len() / self.max_word_len;
                s_len % self.real_input_len == 0 && s_len / self.real_input_len == expected_chunks
            }));
        }
    }
}

impl<'a, PDR, PDW, PD, SDR, SDW, SD> RangedData
    for ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync + RangedData,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync + RangedData,
{
    fn set_range(&mut self, range: Range<usize>) {
        self.par.set_range(range.clone());
        for s in &mut self.seqs {
            s.set_range(range.clone());
        }
        self.check_length();
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
    PE::ErrorType: Send,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    SE::ErrorType: Send,
{
    par: PE,
    seqs: Vec<Mutex<SE>>,
    arg_input_max: u32,
    thread_pool: rayon::ThreadPool,
    num_threads: usize,
    max_word_len: usize, // in 32-bit words
    pdr: PhantomData<&'a PDR>,
    pdw: PhantomData<&'a PDW>,
    pd: PhantomData<&'a PD>,
    sdr: PhantomData<&'a SDR>,
    sdw: PhantomData<&'a SDW>,
    sd: PhantomData<&'a SD>,
}

impl<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
    ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    PE::ErrorType: Send,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    SE::ErrorType: Send,
{
    pub fn input_len(&self) -> usize {
        self.par.input_len()
    }
    pub fn real_input_len(&self) -> usize {
        self.par.real_input_len()
    }
    pub fn output_len(&self) -> usize {
        self.par.output_len()
    }
    pub fn execute<Out, F, G, Stop>(
        &mut self,
        input: &'a ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
        stop: Stop,
    ) -> Result<Out, ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>>
    where
        F: Fn(
                ParSeqSelection,
                &ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
                ParSeqObject<&PD, &SD>,
                u64,
            ) -> Out
            + Send
            + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Stop: Fn(&Out) -> bool + Send + Sync,
        Out: Clone + Send + Sync,
    {
        let arg_count = Arc::new(AtomicU64::new(0));
        let do_stop = Arc::new(AtomicBool::new(false));
        let seq_outputs = self
            .seqs
            .iter()
            .map(|seq| {
                let mut seq = seq.lock().unwrap();
                let elem_count = seq.elem_count(input.len());
                Mutex::new(seq.new_data_output_elems(elem_count))
            })
            .collect::<Vec<_>>();
        let results = self.thread_pool.broadcast(|ctx| {
            let mut thread_result = Ok(init.clone());
            loop {
                let thread_idx = ctx.index();
                if do_stop.load(atomic::Ordering::SeqCst) {
                    break;
                }
                let arg_u64 = arg_count.fetch_add(1, atomic::Ordering::SeqCst);
                if arg_u64 > u64::from(self.arg_input_max) {
                    break;
                }
                let result = if thread_idx < self.num_threads {
                    self.par
                        .try_clone()
                        .unwrap()
                        .execute(&input.par, arg_u64)
                        .map(|output| {
                            f(
                                ParSeqSelection::Par,
                                input,
                                ParSeqObject::Par(&output),
                                arg_u64,
                            )
                        })
                        .map_err(|e| ParSeqMapperExecutorError::ParError(e))
                } else {
                    let i = thread_idx - self.num_threads;
                    let mut output = seq_outputs[i].lock().unwrap();
                    let mut seq = self.seqs[i].lock().unwrap();
                    if seq.need_clear_outputs() {
                        output.fill(0);
                    }
                    match seq.execute_reuse(&input.seqs[i], arg_u64, &mut output) {
                        Ok(()) => Ok(f(
                            ParSeqSelection::Seq(i),
                            input,
                            ParSeqObject::Seq(&output),
                            arg_u64,
                        )),
                        Err(e) => Err(ParSeqMapperExecutorError::SeqError(i, e)),
                    }
                };
                if let Ok(a) = thread_result {
                    do_stop.fetch_or(stop(&a), atomic::Ordering::SeqCst);
                    thread_result = match result {
                        Ok(result) => Ok(g(a.clone(), result)),
                        Err(e) => Err(e),
                    };
                }
            }
            thread_result
        });
        results.into_iter().fold(Ok(init), |a, b| {
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
        })
    }

    pub fn execute_direct<Out: Clone, F, G, Stop>(
        &mut self,
        input: &'a ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
        stop: Stop,
    ) -> Result<Out, ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>>
    where
        F: Fn(ParSeqSelection, &[u32], &[u32], u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Stop: Fn(&Out) -> bool + Send + Sync,
        Out: Clone + Send + Sync,
    {
        self.execute(
            input,
            init,
            |sel, input, output, arg_input| match sel {
                ParSeqSelection::Par => {
                    let output = output.par().unwrap();
                    input.process_single(sel, |inputx| {
                        inputx.par().unwrap().process(|inputx| {
                            output.process(|outputx| f(sel, inputx, outputx, arg_input))
                        })
                    })
                }
                ParSeqSelection::Seq(_) => {
                    let output = output.seq().unwrap();
                    input.process_single(sel, |inputx| {
                        inputx.seq().unwrap().1.process(|inputx| {
                            output.process(|outputx| f(sel, inputx, outputx, arg_input))
                        })
                    })
                }
            },
            g,
            stop,
        )
    }

    pub fn new_data(&mut self, len: usize) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        assert!((len & 15) == 0);
        let out = ParSeqAllDataHolder {
            par: self.par.new_data(len),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data(len))
                .collect::<Vec<_>>(),
            real_input_len: self.real_input_len(),
            max_word_len: self.max_word_len,
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        };
        out.check_length();
        out
    }

    pub fn new_data_from_vec(
        &mut self,
        mut data: impl FnMut(ParSeqSelection) -> Vec<u32>,
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        let out = ParSeqAllDataHolder {
            par: self.par.new_data_from_vec(data(ParSeqSelection::Par)),
            seqs: self
                .seqs
                .iter_mut()
                .enumerate()
                .map(|(i, s)| {
                    s.lock()
                        .unwrap()
                        .new_data_from_vec(data(ParSeqSelection::Seq(i)))
                })
                .collect::<Vec<_>>(),
            real_input_len: self.real_input_len(),
            max_word_len: self.max_word_len,
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        };
        out.check_length();
        out
    }

    pub fn new_data_with_executor(
        &mut self,
        mut data: impl FnMut(ParSeqObject<&mut PE, (usize, &mut SE)>) -> ParSeqObject<PD, SD>,
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        let out = ParSeqAllDataHolder {
            par: match data(ParSeqObject::Par(&mut self.par)) {
                ParSeqObject::Par(pd) => pd,
                ParSeqObject::Seq(_) => {
                    panic!("Unexpected!");
                }
            },
            seqs: self
                .seqs
                .iter_mut()
                .enumerate()
                .map(|(i, s)| {
                    let mut s = s.lock().unwrap();
                    match data(ParSeqObject::Seq((i, &mut s))) {
                        ParSeqObject::Par(_) => {
                            panic!("Unexpected!");
                        }
                        ParSeqObject::Seq(sd) => sd,
                    }
                })
                .collect::<Vec<_>>(),
            real_input_len: self.real_input_len(),
            max_word_len: self.max_word_len,
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        };
        out.check_length();
        out
    }

    pub fn new_data_from_slice<'b>(
        &mut self,
        mut data: impl FnMut(ParSeqSelection) -> &'b [u32],
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        let out = ParSeqAllDataHolder {
            par: self.par.new_data_from_slice(data(ParSeqSelection::Par)),
            seqs: self
                .seqs
                .iter_mut()
                .enumerate()
                .map(|(i, s)| {
                    s.lock()
                        .unwrap()
                        .new_data_from_slice(data(ParSeqSelection::Seq(i)))
                })
                .collect::<Vec<_>>(),
            real_input_len: self.real_input_len(),
            max_word_len: self.max_word_len,
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        };
        out.check_length();
        out
    }

    pub fn new_data_input_elems(
        &mut self,
        elem_num: usize,
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        let out = ParSeqAllDataHolder {
            par: self.par.new_data(self.par.input_data_len(elem_num)),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| {
                    let mut s = s.lock().unwrap();
                    let len = s.input_data_len(elem_num);
                    s.new_data(len)
                })
                .collect::<Vec<_>>(),
            real_input_len: self.real_input_len(),
            max_word_len: self.max_word_len,
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        };
        out.check_length();
        out
    }

    #[inline]
    pub fn output_is_aggregated(&self) -> bool {
        self.par.output_is_aggregated()
    }

    #[inline]
    pub fn input_is_populated(&self) -> bool {
        self.par.input_is_populated()
    }

    #[inline]
    pub fn pop_input_len(&self, sel: ParSeqSelection) -> Option<usize> {
        match sel {
            ParSeqSelection::Par => self.par.pop_input_len(),
            ParSeqSelection::Seq(i) => self.seqs[i].lock().unwrap().pop_input_len(),
        }
    }

    pub fn with_executor<F>(&self, mut f: F)
    where
        F: FnMut(ParSeqObject<&PE, (usize, &SE)>),
    {
        f(ParSeqObject::Par(&self.par));
        for (i, seq) in self.seqs.iter().enumerate() {
            let s = seq.lock().unwrap();
            f(ParSeqObject::Seq((i, &s)));
        }
    }

    pub fn is_sequential_execution(&self, sel: ParSeqSelection) -> bool {
        match sel {
            ParSeqSelection::Par => self.par.is_sequential_execution(),
            ParSeqSelection::Seq(i) => self.seqs[i].lock().unwrap().is_sequential_execution(),
        }
    }

    #[inline]
    pub fn inner_loop(&self) -> Option<u32> {
        self.par.inner_loop()
    }
}

#[derive(Error, Debug)]
pub enum ParSeqMapperTransformsError<PE, SE> {
    #[error("ParError {0}")]
    ParError(#[from] PE),
    #[error("SeqError for {0} {1}")]
    SeqError(usize, SE),
}

pub struct ParSeqMapperTransforms<
    'b,
    'a,
    PDR,
    PDW,
    PD,
    PE,
    PIDT,
    PODT,
    SDR,
    SDW,
    SD,
    SE,
    SIDT,
    SODT,
> where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + DataTransforms<'a, PDR, PDW, PD, PIDT, PODT> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    PIDT: DataTransformer<'a, PDR, PDW, PD>,
    PODT: DataTransformer<'a, PDR, PDW, PD>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + DataTransforms<'a, SDR, SDW, SD, SIDT, SODT> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
    SIDT: DataTransformer<'a, SDR, SDW, SD>,
    SODT: DataTransformer<'a, SDR, SDW, SD>,
{
    executor: &'b mut ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>,
    pidt: PhantomData<&'a PIDT>,
    podt: PhantomData<&'a PODT>,
    sidt: PhantomData<&'a SIDT>,
    sodt: PhantomData<&'a SODT>,
}

impl<'b, 'a, PDR, PDW, PD, PE, PIDT, PODT, SDR, SDW, SD, SE, SIDT, SODT>
    ParSeqMapperTransforms<'b, 'a, PDR, PDW, PD, PE, PIDT, PODT, SDR, SDW, SD, SE, SIDT, SODT>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + DataTransforms<'a, PDR, PDW, PD, PIDT, PODT> + Send + Sync,
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    PIDT: DataTransformer<'a, PDR, PDW, PD>,
    PODT: DataTransformer<'a, PDR, PDW, PD>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + DataTransforms<'a, SDR, SDW, SD, SIDT, SODT> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
    SIDT: DataTransformer<'a, SDR, SDW, SD>,
    SODT: DataTransformer<'a, SDR, SDW, SD>,
{
    pub fn new(e: &'b mut ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>) -> Self {
        Self {
            executor: e,
            pidt: PhantomData,
            podt: PhantomData,
            sidt: PhantomData,
            sodt: PhantomData,
        }
    }

    pub fn with_input_transforms<F>(
        &'b mut self,
        mut f: F,
        input_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<
        (),
        ParSeqMapperTransformsError<
            <PE as DataTransforms<'a, PDR, PDW, PD, PIDT, PODT>>::ErrorType,
            <SE as DataTransforms<'a, SDR, SDW, SD, SIDT, SODT>>::ErrorType,
        >,
    >
    where
        F: FnMut(
            &'b mut ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>,
            PIDT,
            Vec<SIDT>,
        ),
    {
        let pidt = self
            .executor
            .par
            .input_transformer(input_elem_len, bit_mapping)?;
        let sidts = self
            .executor
            .seqs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let s = s.lock().unwrap();
                Ok::<
                    _,
                    ParSeqMapperTransformsError<
                        <PE as DataTransforms<'a, PDR, PDW, PD, PIDT, PODT>>::ErrorType,
                        <SE as DataTransforms<'a, SDR, SDW, SD, SIDT, SODT>>::ErrorType,
                    >,
                >(
                    s.input_transformer(input_elem_len, bit_mapping)
                        .map_err(|e| ParSeqMapperTransformsError::SeqError(i, e))?,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        f(self.executor, pidt, sidts);
        Ok(())
    }

    pub fn with_output_transforms<F>(
        &'b mut self,
        mut f: F,
        output_elem_len: usize,
        bit_mapping: &[usize],
    ) -> Result<
        (),
        ParSeqMapperTransformsError<
            <PE as DataTransforms<'a, PDR, PDW, PD, PIDT, PODT>>::ErrorType,
            <SE as DataTransforms<'a, SDR, SDW, SD, SIDT, SODT>>::ErrorType,
        >,
    >
    where
        F: FnMut(
            &'b mut ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>,
            PODT,
            Vec<SODT>,
        ),
    {
        let podt = self
            .executor
            .par
            .output_transformer(output_elem_len, bit_mapping)?;
        let sodts = self
            .executor
            .seqs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let s = s.lock().unwrap();
                Ok::<
                    _,
                    ParSeqMapperTransformsError<
                        <PE as DataTransforms<'a, PDR, PDW, PD, PIDT, PODT>>::ErrorType,
                        <SE as DataTransforms<'a, SDR, SDW, SD, SIDT, SODT>>::ErrorType,
                    >,
                >(
                    s.output_transformer(output_elem_len, bit_mapping)
                        .map_err(|e| ParSeqMapperTransformsError::SeqError(i, e))?,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        f(self.executor, podt, sodts);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParSeqMapperBuilderError<PE, SE> {
    #[error("ParError {0}")]
    ParError(#[from] PE),
    #[error("SeqError for {0} {1}")]
    SeqError(usize, SE),
}

#[derive(Clone, Copy, Debug)]
pub struct ParSeqDynamicConfig<'a> {
    pub init_code: Option<&'a str>,
    pub pop_input_code: Option<&'a str>,
    pub pop_input_len: Option<usize>,
    pub aggr_output_code: Option<&'a str>,
    pub aggr_output_len: Option<usize>,
    // exclude outputs
    pub exclude_outputs: Option<&'a [usize]>,
    pub dont_clear_outputs: bool,
    pub inner_loop: Option<u32>,
}

impl<'a> ParSeqDynamicConfig<'a> {
    pub fn new() -> Self {
        Self {
            init_code: None,
            pop_input_code: None,
            pop_input_len: None,
            aggr_output_code: None,
            aggr_output_len: None,
            exclude_outputs: None,
            dont_clear_outputs: false,
            inner_loop: None,
        }
    }
    pub fn init_code(mut self, init: Option<&'a str>) -> Self {
        self.init_code = init;
        self
    }
    pub fn pop_input_code(mut self, pop: Option<&'a str>) -> Self {
        self.pop_input_code = pop;
        self
    }
    pub fn pop_input_len(mut self, pop: Option<usize>) -> Self {
        self.pop_input_len = pop;
        self
    }
    pub fn aggr_output_code(mut self, aggr: Option<&'a str>) -> Self {
        self.aggr_output_code = aggr;
        self
    }
    pub fn aggr_output_len(mut self, aggr: Option<usize>) -> Self {
        self.aggr_output_len = aggr;
        self
    }

    pub fn exclude_outputs(mut self, excl: Option<&'a [usize]>) -> Self {
        self.exclude_outputs = excl;
        self
    }
    pub fn dont_clear_outputs(mut self, ignore: bool) -> Self {
        self.dont_clear_outputs = ignore;
        self
    }
    pub fn inner_loop(mut self, l: Option<u32>) -> Self {
        self.inner_loop = l;
        self
    }
}

pub struct ParSeqMapperBuilder<'a, PDR, PDW, PD, PE, PB, SDR, SDW, SD, SE, SB>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    PE::ErrorType: Send,
    PB: Builder<'a, PDR, PDW, PD, PE>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    SE::ErrorType: Send,
    SB: Builder<'a, SDR, SDW, SD, SE>,
{
    par: PB,
    seqs: Vec<SB>,
    arg_input_lens: Vec<usize>,
    num_threads: Option<usize>,
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
    ParSeqMapperBuilder<'a, PDR, PDW, PD, PE, PB, SDR, SDW, SD, SE, SB>
where
    PDR: DataReader + Send + Sync,
    PDW: DataWriter + Send + Sync,
    PD: DataHolder<'a, PDR, PDW> + Send + Sync,
    PE: Executor<'a, PDR, PDW, PD> + Send + Sync,
    PE::ErrorType: Send,
    PB: Builder<'a, PDR, PDW, PD, PE>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    SE::ErrorType: Send,
    SB: Builder<'a, SDR, SDW, SD, SE>,
{
    pub fn new(par_builder: PB, seq_builders: impl IntoIterator<Item = SB>) -> Self {
        assert!(par_builder.is_empty());
        Self {
            par: par_builder,
            seqs: seq_builders
                .into_iter()
                .map(|sb| {
                    assert!(sb.is_empty());
                    sb
                })
                .collect::<Vec<_>>(),
            arg_input_lens: vec![],
            num_threads: None,
            pdr: PhantomData,
            pdw: PhantomData,
            pd: PhantomData,
            pe: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
            sd: PhantomData,
            se: PhantomData,
        }
    }

    pub fn new_with_num_threads(
        par_builder: PB,
        seq_builders: impl IntoIterator<Item = SB>,
        par_num_threads: usize,
    ) -> Self {
        assert!(par_builder.is_empty());
        Self {
            par: par_builder,
            seqs: seq_builders
                .into_iter()
                .map(|sb| {
                    assert!(sb.is_empty());
                    sb
                })
                .collect::<Vec<_>>(),
            arg_input_lens: vec![],
            num_threads: Some(par_num_threads),
            pdr: PhantomData,
            pdw: PhantomData,
            pd: PhantomData,
            pe: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
            sd: PhantomData,
            se: PhantomData,
        }
    }

    pub fn user_defs<'b>(&mut self, mut user_defs: impl FnMut(ParSeqSelection) -> &'b str) {
        self.par.user_defs(user_defs(ParSeqSelection::Par));
        for (i, s) in self.seqs.iter_mut().enumerate() {
            s.user_defs(user_defs(ParSeqSelection::Seq(i)));
        }
    }

    pub fn transform_helpers(&mut self) {
        self.par.transform_helpers();
        for s in self.seqs.iter_mut() {
            s.transform_helpers();
        }
    }

    pub fn add_with_config<'b, T, DCF>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        arg_inputs: &[usize],
        elem_inputs: Option<&[usize]>,
        mut dyn_config: DCF,
    ) where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
        DCF: FnMut(ParSeqSelection) -> ParSeqDynamicConfig<'b>,
    {
        assert!(arg_inputs.len() < 64);
        self.arg_input_lens.push(arg_inputs.len());
        let dyncfg = dyn_config(ParSeqSelection::Par);
        self.par.add_with_config(
            name,
            circuit.clone(),
            CodeConfig::new()
                .arg_inputs(Some(arg_inputs))
                .elem_inputs(elem_inputs)
                .init_code(dyncfg.init_code)
                .pop_input_code(dyncfg.pop_input_code)
                .pop_input_len(dyncfg.pop_input_len)
                .aggr_output_code(dyncfg.aggr_output_code)
                .aggr_output_len(dyncfg.aggr_output_len)
                .exclude_outputs(dyncfg.exclude_outputs)
                .dont_clear_outputs(dyncfg.dont_clear_outputs)
                .inner_loop(dyncfg.inner_loop),
        );
        for (i, s) in self.seqs.iter_mut().enumerate() {
            let dyncfg = dyn_config(ParSeqSelection::Seq(i));
            s.add_with_config(
                name,
                circuit.clone(),
                CodeConfig::new()
                    .arg_inputs(Some(arg_inputs))
                    .elem_inputs(elem_inputs)
                    .init_code(dyncfg.init_code)
                    .pop_input_code(dyncfg.pop_input_code)
                    .pop_input_len(dyncfg.pop_input_len)
                    .aggr_output_code(dyncfg.aggr_output_code)
                    .aggr_output_len(dyncfg.aggr_output_len)
                    .exclude_outputs(dyncfg.exclude_outputs)
                    .dont_clear_outputs(dyncfg.dont_clear_outputs)
                    .inner_loop(dyncfg.inner_loop),
            );
        }
    }

    pub fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.add_with_config(name, circuit, arg_inputs, None, |_| {
            ParSeqDynamicConfig::new()
        });
    }

    pub fn build(
        self,
    ) -> Result<
        Vec<ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>>,
        ParSeqMapperBuilderError<PB::ErrorType, SB::ErrorType>,
    > {
        let max_word_len = std::iter::once(&self.par)
            .map(|b| b.word_len())
            .chain(self.seqs.iter().map(|b| b.word_len()))
            .max()
            .unwrap_or_default();
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
        let num_threads = self.num_threads.unwrap_or(rayon::current_num_threads());
        Ok(par_execs
            .into_iter()
            .enumerate()
            .map(|(i, par)| ParSeqMapperExecutor {
                par,
                seqs: (0..seqs_len)
                    .map(|x| seq_execs.remove(&(x, i)).unwrap())
                    .collect::<Vec<_>>(),
                arg_input_max: u32::try_from((1u64 << self.arg_input_lens[i]) - 1u64).unwrap(),
                thread_pool: rayon::ThreadPoolBuilder::new()
                    .num_threads(num_threads + seqs_len)
                    .build()
                    .unwrap(),
                num_threads,
                max_word_len: (max_word_len as usize) >> 5,
                pdr: PhantomData,
                pdw: PhantomData,
                pd: PhantomData,
                sdr: PhantomData,
                sdw: PhantomData,
                sd: PhantomData,
            })
            .collect::<Vec<_>>())
    }

    pub fn word_len(&self, sel: ParSeqSelection) -> u32 {
        match sel {
            ParSeqSelection::Par => self.par.word_len(),
            ParSeqSelection::Seq(i) => self.seqs[i].word_len(),
        }
    }

    pub fn type_len(&self, sel: ParSeqSelection) -> u32 {
        match sel {
            ParSeqSelection::Par => self.par.type_len(),
            ParSeqSelection::Seq(i) => self.seqs[i].type_len(),
        }
    }

    pub fn seq_builder_num(&self) -> usize {
        self.seqs.len()
    }

    pub fn is_data_holder_global() -> bool {
        PB::is_data_holder_global() && SB::is_data_holder_global()
    }
    pub fn is_data_holder_in_builder() -> bool {
        PB::is_data_holder_in_builder() && SB::is_data_holder_in_builder()
    }
    pub fn preferred_input_count(&self) -> usize {
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

use crate::cpu_build_exec::*;
use crate::opencl_build_exec::*;

pub type CPUOpenCLParSeqMapperExecutor<'a> = ParSeqMapperExecutor<
    'a,
    CPUDataReader<'a>,
    CPUDataWriter<'a>,
    CPUDataHolder,
    CPUExecutor,
    OpenCLDataReader<'a>,
    OpenCLDataWriter<'a>,
    OpenCLDataHolder,
    OpenCLExecutor,
>;
pub type CPUOpenCLParSeqMapperBuilder<'a> = ParSeqMapperBuilder<
    'a,
    CPUDataReader<'a>,
    CPUDataWriter<'a>,
    CPUDataHolder,
    CPUExecutor,
    CPUBuilder<'a>,
    OpenCLDataReader<'a>,
    OpenCLDataWriter<'a>,
    OpenCLDataHolder,
    OpenCLExecutor,
    OpenCLBuilder<'a>,
>;

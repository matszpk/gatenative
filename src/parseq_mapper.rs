use crate::*;

use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::{
    atomic::{self, AtomicU64},
    Arc, Mutex,
};

// ParSeqMapper - mapper that join parallel and sequential mapper

pub enum ParSeqObject<T1, T2> {
    Par(T1),
    Seq(T2),
}

impl<T1, T2> ParSeqObject<T1, T2> {
    pub fn par(self) -> Option<T1> {
        if let Self::Par(o) = self {
            Some(o)
        } else {
            None
        }
    }
    pub fn seq(self) -> Option<T2> {
        if let Self::Seq(o) = self {
            Some(o)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParSeqSelection {
    Par,
    Seq(usize),
}

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
    #[inline]
    pub fn len(&self) -> usize {
        self.par.len()
    }

    pub fn process_single<F, Out>(&self, sel: ParSeqSelection, mut f: F) -> Out
    where
        F: FnMut(ParSeqObject<&PD, (usize, &SD)>) -> Out,
    {
        match sel {
            ParSeqSelection::Par => f(ParSeqObject::Par(&self.par)),
            ParSeqSelection::Seq(i) => f(ParSeqObject::Seq((i, &self.seqs[i]))),
        }
    }

    pub fn process_single_mut<F, Out>(&mut self, sel: ParSeqSelection, mut f: F) -> Out
    where
        F: FnMut(ParSeqObject<&mut PD, (usize, &mut SD)>) -> Out,
    {
        match sel {
            ParSeqSelection::Par => f(ParSeqObject::Par(&mut self.par)),
            ParSeqSelection::Seq(i) => f(ParSeqObject::Seq((i, &mut self.seqs[i]))),
        }
    }

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
        let expected_chunks = (expected_word_num) / self.real_input_len;
        assert_eq!((expected_word_num) % self.real_input_len, 0);
        assert!(self.seqs.iter().all(|s| {
            let s_len = s.len() / self.max_word_len;
            s_len % self.real_input_len == 0 && s_len / self.real_input_len == expected_chunks
        }));
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
    pub fn execute<Out, F, G>(
        &mut self,
        input: &'a ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
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
        Out: Clone + Send + Sync,
    {
        let arg_count = Arc::new(AtomicU64::new(0));
        let results = self.thread_pool.broadcast(|ctx| {
            let mut thread_result = Ok(init.clone());
            loop {
                let thread_idx = ctx.index();
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
                    self.seqs[i]
                        .lock()
                        .unwrap()
                        .execute(&input.seqs[i], arg_u64)
                        .map(|output| {
                            f(
                                ParSeqSelection::Seq(i),
                                input,
                                ParSeqObject::Seq(&output),
                                arg_u64,
                            )
                        })
                        .map_err(|e| ParSeqMapperExecutorError::SeqError(i, e))
                };
                if let Ok(a) = thread_result {
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

    pub fn execute_direct<Out: Clone, F, G>(
        &mut self,
        input: &'a ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
    ) -> Result<Out, ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>>
    where
        F: Fn(ParSeqSelection, &[u32], &[u32], u64) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
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
    executor: &'b ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>,
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
    pub fn new(e: &'b ParSeqMapperExecutor<'a, PDR, PDW, PD, PE, SDR, SDW, SD, SE>) -> Self {
        Self {
            executor: e,
            pidt: PhantomData,
            podt: PhantomData,
            sidt: PhantomData,
            sodt: PhantomData,
        }
    }

    pub fn with_input_transforms<F>(
        &self,
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
        F: FnMut(ParSeqObject<PIDT, (usize, SIDT)>),
    {
        f(ParSeqObject::Par(
            self.executor.par.input_tx(input_elem_len, bit_mapping)?,
        ));
        for (i, s) in self.executor.seqs.iter().enumerate() {
            let sitx = {
                let s = s.lock().unwrap();
                s.input_tx(input_elem_len, bit_mapping)
                    .map_err(|e| ParSeqMapperTransformsError::SeqError(i, e))?
            };
            f(ParSeqObject::Seq((i, sitx)));
        }
        Ok(())
    }

    pub fn with_output_transforms<F>(
        &self,
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
        F: FnMut(ParSeqObject<PODT, (usize, SODT)>),
    {
        f(ParSeqObject::Par(
            self.executor.par.output_tx(output_elem_len, bit_mapping)?,
        ));
        for (i, s) in self.executor.seqs.iter().enumerate() {
            let sotx = {
                let s = s.lock().unwrap();
                s.output_tx(output_elem_len, bit_mapping)
                    .map_err(|e| ParSeqMapperTransformsError::SeqError(i, e))?
            };
            f(ParSeqObject::Seq((i, sotx)));
        }
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

    pub fn add<T>(&mut self, name: &str, circuit: Circuit<T>, arg_inputs: &[usize])
    where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        self.arg_input_lens.push(arg_inputs.len());
        self.par
            .add(name, circuit.clone(), None, None, Some(arg_inputs));
        for s in &mut self.seqs {
            s.add(name, circuit.clone(), None, None, Some(arg_inputs));
        }
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
        let num_threads = rayon::current_num_threads();
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

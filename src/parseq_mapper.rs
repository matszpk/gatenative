use crate::*;

use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::{
    atomic::{self, AtomicBool, AtomicU32},
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

pub enum ParSeqDataReader<PDR, SDR>
where
    PDR: DataReader + Send + Sync,
    SDR: DataReader + Send + Sync,
{
    Par(PDR),
    Seq(SDR),
}

impl<PDR, SDR> DataReader for ParSeqDataReader<PDR, SDR>
where
    PDR: DataReader + Send + Sync,
    SDR: DataReader + Send + Sync,
{
    fn get(&self) -> &[u32] {
        match self {
            ParSeqDataReader::Par(p) => p.get(),
            ParSeqDataReader::Seq(s) => s.get(),
        }
    }
}

pub enum ParSeqDataWriter<PDW, SDW>
where
    PDW: DataWriter + Send + Sync,
    SDW: DataWriter + Send + Sync,
{
    Par(PDW),
    Seq(SDW),
}

impl<PDW, SDW> DataWriter for ParSeqDataWriter<PDW, SDW>
where
    PDW: DataWriter + Send + Sync,
    SDW: DataWriter + Send + Sync,
{
    fn get_mut(&mut self) -> &mut [u32] {
        match self {
            ParSeqDataWriter::Par(p) => p.get_mut(),
            ParSeqDataWriter::Seq(s) => s.get_mut(),
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
    ParRef(&'a PD),
    SeqRef(usize, &'a SD),
    ParRefMut(&'a mut PD),
    SeqRefMut(usize, &'a mut SD),
    PDR(PhantomData<&'a PDR>),
    PDW(PhantomData<&'a PDW>),
    SDR(PhantomData<&'a SDR>),
    SDW(PhantomData<&'a SDW>),
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
            Self::ParRef(par) => par.len(),
            Self::SeqRef(_, seq) => seq.len(),
            Self::ParRefMut(par) => par.len(),
            Self::SeqRefMut(_, seq) => seq.len(),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn set_range(&mut self, range: Range<usize>) {
        match self {
            Self::ParRefMut(par) => par.set_range(range.clone()),
            Self::SeqRefMut(_, seq) => seq.set_range(range.clone()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn get(&'a self) -> ParSeqDataReader<PDR, SDR> {
        match self {
            Self::ParRef(par) => ParSeqDataReader::Par(par.get()),
            Self::SeqRef(_, seq) => ParSeqDataReader::Seq(seq.get()),
            Self::ParRefMut(par) => ParSeqDataReader::Par(par.get()),
            Self::SeqRefMut(_, seq) => ParSeqDataReader::Seq(seq.get()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
    fn get_mut(&'a mut self) -> ParSeqDataWriter<PDW, SDW> {
        match self {
            Self::ParRefMut(par) => ParSeqDataWriter::Par(par.get_mut()),
            Self::SeqRefMut(_, seq) => ParSeqDataWriter::Seq(seq.get_mut()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn process<F, Out>(&self, f: F) -> Out
    where
        F: FnMut(&[u32]) -> Out,
    {
        match self {
            Self::ParRef(par) => par.process(f),
            Self::SeqRef(_, seq) => seq.process(f),
            Self::ParRefMut(par) => par.process(f),
            Self::SeqRefMut(_, seq) => seq.process(f),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn process_mut<F, Out>(&mut self, f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        match self {
            Self::ParRefMut(par) => par.process_mut(f),
            Self::SeqRefMut(_, seq) => seq.process_mut(f),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn release(self) -> Vec<u32> {
        match self {
            Self::ParRef(par) => par.get().get().to_vec(),
            Self::SeqRef(_, seq) => seq.get().get().to_vec(),
            Self::ParRefMut(par) => par.get().get().to_vec(),
            Self::SeqRefMut(_, seq) => seq.get().get().to_vec(),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn free(self) {
        match self {
            Self::ParRef(_) => {}
            Self::SeqRef(_, _) => {}
            Self::ParRefMut(_) => {}
            Self::SeqRefMut(_, _) => {}
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
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
    pub fn par_ref(&'a self) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::ParRef(&self.par)
    }
    #[inline]
    pub fn seq_ref(&'a self, index: usize) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::SeqRef(index, &self.seqs[index])
    }
    #[inline]
    pub fn par_ref_mut(&'a mut self) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::ParRefMut(&mut self.par)
    }
    #[inline]
    pub fn seq_ref_mut(
        &'a mut self,
        index: usize,
    ) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::SeqRefMut(index, &mut self.seqs[index])
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
    arg_input_max: u32,
    thread_pool: rayon::ThreadPool,
    num_threads: usize,
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
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
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
                &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
                ParSeqObject<&PD, &SD>,
                u32,
            ) -> Out
            + Send
            + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Out: Clone + Send + Sync,
    {
        let arg_count = Arc::new(AtomicU32::new(0));
        let end = Arc::new(AtomicBool::new(false));

        let results = self.thread_pool.broadcast(|ctx| {
            let mut thread_result = Ok(init.clone());
            while !end.load(atomic::Ordering::SeqCst) {
                let thread_idx = ctx.index();
                let arg = arg_count.fetch_add(1, atomic::Ordering::SeqCst);
                if arg == self.arg_input_max {
                    end.store(true, atomic::Ordering::SeqCst);
                }
                let result = if thread_idx < self.num_threads {
                    self.par
                        .try_clone()
                        .unwrap()
                        .execute(&input.par, arg)
                        .map(|output| f(&input.par_ref(), ParSeqObject::Par(&output), arg))
                        .map_err(|e| ParSeqMapperExecutorError::ParError(e))
                } else {
                    let i = thread_idx - self.num_threads;
                    self.seqs[i]
                        .lock()
                        .unwrap()
                        .execute(&input.seqs[i], arg)
                        .map(|output| f(&input.seq_ref(i), ParSeqObject::Seq(&output), arg))
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
        F: Fn(ParSeqSelection, &[u32], &[u32], u32) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Out: Clone + Send + Sync,
    {
        self.execute(
            input,
            init,
            |input, output, arg_input| match input {
                ParSeqDataHolder::ParRefMut(_) => {
                    let output = output.par().unwrap();
                    input.process(|inputx| {
                        output
                            .process(|outputx| f(ParSeqSelection::Par, inputx, outputx, arg_input))
                    })
                }
                ParSeqDataHolder::SeqRefMut(i, _) => {
                    let output = output.seq().unwrap();
                    input.process(|inputx| {
                        output.process(|outputx| {
                            f(ParSeqSelection::Seq(*i), inputx, outputx, arg_input)
                        })
                    })
                }
                _ => {
                    panic!("Unexpected");
                }
            },
            g,
        )
    }

    pub fn new_data(&mut self, len: usize) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        assert!((len & 15) == 0);
        ParSeqAllDataHolder {
            par: self.par.new_data(len),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data(len))
                .collect::<Vec<_>>(),
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        }
    }

    pub fn new_data_from_vec(
        &mut self,
        data: Vec<u32>,
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        assert!((data.len() & 15) == 0);
        ParSeqAllDataHolder {
            par: self.par.new_data_from_slice(&data),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data_from_slice(&data))
                .collect::<Vec<_>>(),
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
        }
    }

    pub fn new_data_from_slice(
        &mut self,
        data: &[u32],
    ) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        assert!((data.len() & 15) == 0);
        ParSeqAllDataHolder {
            par: self.par.new_data_from_slice(data),
            seqs: self
                .seqs
                .iter_mut()
                .map(|s| s.lock().unwrap().new_data_from_slice(data))
                .collect::<Vec<_>>(),
            pdr: PhantomData,
            pdw: PhantomData,
            sdr: PhantomData,
            sdw: PhantomData,
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
    <PE as Executor<'a, PDR, PDW, PD>>::ErrorType: Send,
    PB: Builder<'a, PDR, PDW, PD, PE>,
    SDR: DataReader + Send + Sync,
    SDW: DataWriter + Send + Sync,
    SD: DataHolder<'a, SDR, SDW> + Send + Sync,
    SE: Executor<'a, SDR, SDW, SD> + Send,
    <SE as Executor<'a, SDR, SDW, SD>>::ErrorType: Send,
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
                pdr: PhantomData,
                pdw: PhantomData,
                pd: PhantomData,
                sdr: PhantomData,
                sdw: PhantomData,
                sd: PhantomData,
            })
            .collect::<Vec<_>>())
    }

    pub fn par_word_len(&self) -> u32 {
        self.par.word_len()
    }

    pub fn seq_word_len(&self, index: usize) -> u32 {
        self.seqs[index].word_len()
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

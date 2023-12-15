use crate::*;

use rayon::prelude::*;
use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::Mutex;

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

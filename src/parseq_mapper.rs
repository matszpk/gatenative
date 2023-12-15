use crate::*;

use rayon::prelude::*;
use thiserror::Error;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;
use std::sync::Mutex;

// ParSeqMapper - mapper that join parallel and sequential mapper

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
    ParRef(&'a mut PD),
    SeqRef(usize, &'a mut SD),
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
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn set_range(&mut self, range: Range<usize>) {
        match self {
            Self::ParRef(par) => par.set_range(range.clone()),
            Self::SeqRef(_, seq) => seq.set_range(range.clone()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn get(&'a self) -> ParSeqDataReader<PDR, SDR> {
        match self {
            Self::ParRef(par) => ParSeqDataReader::Par(par.get()),
            Self::SeqRef(_, seq) => ParSeqDataReader::Seq(seq.get()),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }
    fn get_mut(&'a mut self) -> ParSeqDataWriter<PDW, SDW> {
        match self {
            Self::ParRef(par) => ParSeqDataWriter::Par(par.get_mut()),
            Self::SeqRef(_, seq) => ParSeqDataWriter::Seq(seq.get_mut()),
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
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn process_mut<F, Out>(&mut self, mut f: F) -> Out
    where
        F: FnMut(&mut [u32]) -> Out,
    {
        match self {
            Self::ParRef(par) => par.process_mut(f),
            Self::SeqRef(_, seq) => seq.process_mut(f),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn release(self) -> Vec<u32> {
        match self {
            Self::ParRef(par) => par.get().get().to_vec(),
            Self::SeqRef(_, seq) => seq.get().get().to_vec(),
            _ => {
                panic!("Unexpected kind");
            }
        }
    }

    fn free(self) {
        match self {
            Self::ParRef(_) => {}
            Self::SeqRef(_, _) => {}
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
    pub fn par_ref(&'a mut self) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::ParRef(&mut self.par)
    }
    #[inline]
    pub fn seq_ref(&'a mut self, index: usize) -> ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
        ParSeqDataHolder::SeqRef(index, &mut self.seqs[index])
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
        input: &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
    ) -> Result<Out, ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>>
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

    pub fn execute_direct<'b, Out: Clone, F, G>(
        &mut self,
        input: &ParSeqDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD>,
        init: Out,
        f: F,
        g: G,
    ) -> Result<Out, ParSeqMapperExecutorError<PE::ErrorType, SE::ErrorType>>
    where
        F: Fn(&[u32], &[u32], u32) -> Out + Send + Sync,
        G: Fn(Out, Out) -> Out + Send + Sync,
        Out: Clone + Send + Sync,
    {
        self.execute(
            input,
            init,
            |input, output, arg_input| {
                input.process(|inputx| output.process(|outputx| f(inputx, outputx, arg_input)))
            },
            g,
        )
    }

    pub fn new_data(&mut self, len: usize) -> ParSeqAllDataHolder<'a, PDR, PDW, PD, SDR, SDW, SD> {
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

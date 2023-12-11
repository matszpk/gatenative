use crate::divide::*;
use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DivPlacements {
    input_ps: Option<Placement>,  // input placement
    output_ps: Option<Placement>, // output placement
}

pub struct DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    executors: Vec<E>,
    buffer_len: usize,
    single_buffer: bool,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

impl<'a, DR, DW, D, E> DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    pub fn buffer_len(&self) -> usize {
        self.buffer_len
    }
}

impl<'a, DR, DW, D, E> Executor<'a, DR, DW, D> for DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    type ErrorType = E::ErrorType;

    #[inline]
    fn input_len(&self) -> usize {
        self.executors.first().unwrap().input_len()
    }
    #[inline]
    fn output_len(&self) -> usize {
        self.executors.last().unwrap().output_len()
    }
    #[inline]
    fn real_input_len(&self) -> usize {
        self.executors.first().unwrap().real_input_len()
    }
    #[inline]
    fn real_output_len(&self) -> usize {
        self.executors.last().unwrap().real_output_len()
    }

    unsafe fn execute_internal(&mut self, input: &D, arg_input: u32) -> Result<D, E::ErrorType> {
        let exec_len = self.executors.len();
        if exec_len != 1 {
            let input_len = input.len();
            let input_chunk_len = self.real_input_len();
            let num = if input_chunk_len != 0 {
                input_len / input_chunk_len
            } else {
                0
            };
            let mut buffer = self.new_data(num * self.buffer_len);
            let mut output = self.new_data(num * self.real_output_len());
            for (i, exec) in self.executors.iter_mut().enumerate() {
                if i == 0 {
                    exec.execute_reuse_internal(input, arg_input, &mut buffer)?;
                } else if exec_len == i + 1 {
                    exec.execute_reuse_internal(&buffer, 0, &mut output)?;
                } else {
                    exec.execute_single_internal(&mut buffer, 0)?;
                }
            }
            Ok(output)
        } else {
            unsafe { self.executors[0].execute_internal(input, arg_input) }
        }
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u32,
        output: &mut D,
    ) -> Result<(), E::ErrorType> {
        let exec_len = self.executors.len();
        if exec_len != 1 {
            let input_len = input.len();
            let output_len = output.len();
            let input_chunk_len = self.real_input_len();
            let output_chunk_len = self.real_output_len();
            let num = if input_chunk_len != 0 {
                input_len / input_chunk_len
            } else {
                output_len / output_chunk_len
            };
            let mut buffer = self.new_data(num * self.buffer_len);
            for (i, exec) in self.executors.iter_mut().enumerate() {
                if i == 0 {
                    exec.execute_reuse_internal(input, arg_input, &mut buffer)?;
                } else if exec_len == i + 1 {
                    exec.execute_reuse_internal(&buffer, 0, output)?;
                } else {
                    exec.execute_single_internal(&mut buffer, 0)?;
                }
            }
            Ok(())
        } else {
            unsafe { self.executors[0].execute_reuse_internal(input, arg_input, output) }
        }
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u32,
    ) -> Result<(), E::ErrorType> {
        let exec_len = self.executors.len();
        if exec_len != 1 {
            let output_len = output.len();
            let output_chunk_len = self.real_output_len();
            let num = output_len / output_chunk_len;
            let mut buffer = self.new_data(num * self.buffer_len);
            for (i, exec) in self.executors.iter_mut().enumerate() {
                if i == 0 {
                    exec.execute_reuse_internal(output, arg_input, &mut buffer)?;
                } else if exec_len == i + 1 {
                    exec.execute_reuse_internal(&buffer, 0, output)?;
                } else {
                    exec.execute_single_internal(&mut buffer, 0)?;
                }
            }
            Ok(())
        } else {
            unsafe { self.executors[0].execute_single_internal(output, arg_input) }
        }
    }

    fn new_data(&mut self, len: usize) -> D {
        self.executors.first_mut().unwrap().new_data(len)
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D {
        self.executors.first_mut().unwrap().new_data_from_vec(data)
    }

    fn new_data_from_slice(&mut self, data: &[u32]) -> D {
        self.executors
            .first_mut()
            .unwrap()
            .new_data_from_slice(data)
    }

    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        let cloned_execs = self
            .executors
            .iter()
            .map(|ex| ex.try_clone())
            .collect::<Vec<_>>();
        if cloned_execs.iter().all(|ex| ex.is_some()) {
            Some(DivExecutor {
                executors: cloned_execs
                    .into_iter()
                    .map(|ex| ex.unwrap())
                    .collect::<Vec<_>>(),
                buffer_len: self.buffer_len,
                single_buffer: self.single_buffer,
                d: PhantomData,
                dr: PhantomData,
                dw: PhantomData,
            })
        } else {
            None
        }
    }

    #[inline]
    fn is_single_buffer(&self) -> bool {
        self.single_buffer
    }
}

struct CircuitInfo {
    subcircuit_num: usize,
    buffer_len: usize,
    single_buffer: bool,
}

pub struct DivBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    builder: B,
    max_gates: usize,
    circuit_infos: Vec<CircuitInfo>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
    e: PhantomData<&'a E>,
}

impl<'a, DR, DW, D, E, B> DivBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    pub fn new(builder: B, max_gates: usize) -> Self {
        assert!(builder.is_empty());
        DivBuilder {
            builder,
            max_gates,
            circuit_infos: vec![],
            d: PhantomData,
            dr: PhantomData,
            dw: PhantomData,
            e: PhantomData,
        }
    }
}

impl<'a, DR, DW, D, E, B> Builder<'a, DR, DW, D, DivExecutor<'a, DR, DW, D, E>>
    for DivBuilder<'a, DR, DW, D, E, B>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
    B: Builder<'a, DR, DW, D, E>,
{
    type ErrorType = B::ErrorType;
    fn add_ext<T>(
        &mut self,
        name: &str,
        circuit: Circuit<T>,
        input_placement: Option<(&[usize], usize)>,
        output_placement: Option<(&[usize], usize)>,
        arg_inputs: Option<&[usize]>,
        single_buffer: bool,
    ) where
        T: Clone + Copy + Ord + PartialEq + Eq + Hash,
        T: Default + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
        usize: TryFrom<T>,
        <usize as TryFrom<T>>::Error: Debug,
    {
        let subcircuits = divide_circuit_traverse(circuit, self.max_gates);
        let subcircuit_num = subcircuits.len();
        let buffer_len = subcircuits[0]
            .output_ps
            .as_ref()
            .map(|ps| ps.real_len)
            .unwrap_or_default();
        for (i, subcircuit) in subcircuits.into_iter().enumerate() {
            let last_placement = DivPlacements {
                input_ps: subcircuit.input_ps,
                output_ps: subcircuit.output_ps,
            };
            let name_0 = format!("{}_{}", name, i);
            self.builder.add_ext(
                &name_0,
                subcircuit.circuit,
                if i == 0 {
                    input_placement
                } else {
                    // use placement from subcircuit
                    last_placement
                        .input_ps
                        .as_ref()
                        .map(|p| (p.ps.as_slice(), p.real_len))
                },
                if i + 1 == subcircuit_num {
                    output_placement
                } else {
                    // use placement from subcircuit
                    last_placement
                        .output_ps
                        .as_ref()
                        .map(|p| (p.ps.as_slice(), p.real_len))
                },
                if i == 0 { arg_inputs } else { None },
                if i + 1 == subcircuit_num {
                    // if only one subcircuit then apply single_buffer
                    single_buffer && subcircuit_num == 1
                } else {
                    // for subcircuits after first and before last
                    i != 0
                },
            );
        }
        self.circuit_infos.push(CircuitInfo {
            subcircuit_num,
            buffer_len,
            single_buffer,
        });
    }

    fn build(self) -> Result<Vec<DivExecutor<'a, DR, DW, D, E>>, B::ErrorType> {
        let all_circuit_execs = self.builder.build()?;
        let mut execs = vec![];
        let mut all_circuit_execs = all_circuit_execs.into_iter();
        for circuit_infos in self.circuit_infos {
            let subcircuit_num = circuit_infos.subcircuit_num;
            let mut circuit_execs = vec![];
            for _ in 0..subcircuit_num {
                circuit_execs.push(all_circuit_execs.next().unwrap());
            }
            execs.push(DivExecutor {
                executors: circuit_execs,
                buffer_len: circuit_infos.buffer_len,
                single_buffer: circuit_infos.single_buffer,
                d: PhantomData,
                dr: PhantomData,
                dw: PhantomData,
            });
        }
        Ok(execs)
    }

    #[inline]
    fn word_len(&self) -> u32 {
        self.builder.word_len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.circuit_infos.is_empty()
    }

    #[inline]
    fn is_executor_per_thread() -> bool {
        B::is_executor_per_thread()
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

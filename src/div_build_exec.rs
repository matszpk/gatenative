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
    placements: Vec<DivPlacements>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
}

impl<'a, DR, DW, D, E> Executor<'a, DR, DW, D> for DivExecutor<'a, DR, DW, D, E>
where
    DR: DataReader,
    DW: DataWriter,
    D: DataHolder<'a, DR, DW>,
    E: Executor<'a, DR, DW, D>,
{
    type ErrorType = E::ErrorType;

    fn input_len(&self) -> usize {
        self.executors.first().unwrap().input_len()
    }
    fn output_len(&self) -> usize {
        self.executors.last().unwrap().output_len()
    }
    fn real_input_len(&self) -> usize {
        self.executors.first().unwrap().real_input_len()
    }
    fn real_output_len(&self) -> usize {
        self.executors.last().unwrap().real_output_len()
    }

    unsafe fn execute_internal(&mut self, input: &D, arg_input: u32) -> Result<D, E::ErrorType> {
        unsafe {
            self.executors
                .first_mut()
                .unwrap()
                .execute_internal(input, arg_input)
        }
    }

    unsafe fn execute_reuse_internal(
        &mut self,
        input: &D,
        arg_input: u32,
        output: &mut D,
    ) -> Result<(), E::ErrorType> {
        unsafe {
            self.executors
                .first_mut()
                .unwrap()
                .execute_reuse_internal(input, arg_input, output)
        }
    }

    unsafe fn execute_single_internal(
        &mut self,
        output: &mut D,
        arg_input: u32,
    ) -> Result<(), E::ErrorType> {
        unsafe {
            self.executors
                .first_mut()
                .unwrap()
                .execute_single_internal(output, arg_input)
        }
    }

    fn new_data(&mut self, len: usize) -> D {
        self.executors.first_mut().unwrap().new_data(len)
    }

    fn new_data_from_vec(&mut self, data: Vec<u32>) -> D {
        self.executors.first_mut().unwrap().new_data_from_vec(data)
    }

    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn is_single_buffer(&self) -> bool {
        self.executors.first().unwrap().is_single_buffer()
    }
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
    placements: Vec<DivPlacements>,
    d: PhantomData<&'a D>,
    dr: PhantomData<&'a DR>,
    dw: PhantomData<&'a DW>,
    e: PhantomData<&'a E>,
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
        let subcircuits_num = subcircuits.len();
        for (i, subcircuit) in subcircuits.into_iter().enumerate() {
            self.placements.push(DivPlacements {
                input_ps: subcircuit.input_ps,
                output_ps: subcircuit.output_ps,
            });
            let last_placement = self.placements.last().unwrap();
            let name_0 = format!("{}_{}", name, i);
            self.builder.add_ext(
                &name_0,
                subcircuit.circuit,
                if i == 0 {
                    input_placement
                } else {
                    last_placement
                        .input_ps
                        .as_ref()
                        .map(|p| (p.ps.as_slice(), p.real_len))
                },
                if i + 1 == subcircuits_num {
                    output_placement
                } else {
                    last_placement
                        .output_ps
                        .as_ref()
                        .map(|p| (p.ps.as_slice(), p.real_len))
                },
                if i == 0 { arg_inputs } else { None },
                single_buffer && subcircuits_num == 1,
            );
        }
    }

    fn build(self) -> Result<Vec<DivExecutor<'a, DR, DW, D, E>>, B::ErrorType> {
        self.builder.build()?;
        Ok(vec![])
    }

    fn word_len(&self) -> u32 {
        self.builder.word_len()
    }

    fn is_executor_per_thread() -> bool {
        B::is_executor_per_thread()
    }
}

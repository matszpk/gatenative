use crate::cpu_build_exec::*;
use crate::*;

use rayon::prelude::*;

use std::convert::Infallible;

/// convert input data into circuit input form.
#[derive(Clone)]
pub struct CPUDataInputTransformer {
    word_len: u32,
    input_elem_len: usize,
    output_elem_len: usize,
    bit_mapping: Vec<usize>,
    parallel: bool,
}

impl CPUDataInputTransformer {
    /// A bit_mapping - index is bit of output's element, value is bit of input's element.
    pub fn new(
        word_len: u32,
        input_elem_len: usize,
        output_elem_len: usize,
        bit_mapping: &[usize],
        parallel: bool,
    ) -> Self {
        assert_eq!((word_len & 31), 0);
        assert_eq!((input_elem_len & 31), 0);
        assert!(input_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(output_elem_len >= bit_mapping.len());
        Self {
            word_len,
            input_elem_len: ((input_elem_len + 31) >> 5) << 5,
            output_elem_len,
            bit_mapping: bit_mapping.to_vec(),
            parallel,
        }
    }

    fn transform_int(&self, input: &[u32], output: &mut [u32]) {
        let input_elem_word_num = self.input_elem_len >> 5;
        let elem_num = input.len() / input_elem_word_num;
        let words_per_word = (self.word_len as usize) >> 5;
        let mut gidx = 0;
        let mut widx = 0;
        let mut sbit = 0;
        for i in 0..elem_num {
            let input_elem = &input[i * input_elem_word_num..(i + 1) * input_elem_word_num];
            for (outbit, inbit) in self.bit_mapping.iter().enumerate() {
                let inbit_val = (input_elem[inbit >> 5] >> (inbit & 31)) & 1;
                let oi = words_per_word * (gidx * self.output_elem_len + outbit) + widx;
                output[oi] = (output[oi] & !(1 << sbit)) | (inbit_val << sbit);
            }
            sbit += 1;
            if sbit >= 32 {
                sbit = 0;
                widx += 1;
                if widx >= words_per_word {
                    widx = 0;
                    gidx += 1;
                }
            }
        }
    }
}

impl<'a> DataTransformer<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder>
    for CPUDataInputTransformer
{
    type ErrorType = Infallible;

    fn transform(&mut self, input: &CPUDataHolder) -> Result<CPUDataHolder, Self::ErrorType> {
        let mut output = CPUDataHolder::new(vec![0; self.output_data_len(input.len())]);
        self.transform_reuse(input, &mut output)?;
        Ok(output)
    }

    fn transform_reuse(
        &mut self,
        input: &CPUDataHolder,
        output: &mut CPUDataHolder,
    ) -> Result<(), Self::ErrorType> {
        assert_eq!(
            input.len() % (((self.word_len as usize) * self.input_elem_len) >> 5),
            0
        );
        assert_eq!(
            output.len() % (((self.word_len as usize) * self.output_elem_len) >> 5),
            0
        );
        if self.parallel {
            const CHUNK_LEN: usize = 128;
            let input_r = input.get();
            let input = input_r.get();
            let mut output_w = output.get_mut();
            let output = output_w.get_mut();

            let word_len = self.word_len as usize;
            let input_elem_word_num = self.input_elem_len >> 5;
            let elem_num = input.len() / input_elem_word_num;
            let output_group_word_num = (self.output_elem_len * word_len) >> 5;
            output[0..(elem_num * self.output_elem_len) >> 5]
                .chunks_mut(output_group_word_num * CHUNK_LEN)
                .enumerate()
                .par_bridge()
                .for_each(|(i, x)| {
                    let end = std::cmp::min((i + 1) * word_len * CHUNK_LEN, elem_num);
                    // println!("RangeX: {} {}", word_len * i * CHUNK_LEN, end);
                    self.transform_int(
                        &input[input_elem_word_num * word_len * i * CHUNK_LEN
                            ..input_elem_word_num * end],
                        x,
                    );
                });
        } else {
            let input_r = input.get();
            let input = input_r.get();
            let mut output_w = output.get_mut();
            let output = output_w.get_mut();
            self.transform_int(input, output);
        }
        Ok(())
    }

    fn input_elem_len(&self) -> usize {
        self.input_elem_len
    }
    fn output_elem_len(&self) -> usize {
        self.output_elem_len
    }
}

#[derive(Clone)]
pub struct CPUDataOutputTransformer {
    word_len: u32,
    input_elem_len: usize,
    output_elem_len: usize,
    bit_mapping: Vec<usize>,
    parallel: bool,
}

/// convert output data from circuit input form into output form.
impl CPUDataOutputTransformer {
    /// An output_elem_len - number of bits of really single input element.
    /// An input_elem_len - number of bits of really single output element.
    /// A bit_mapping - index is bit of really input's element,
    //  value is bit of really output's element.
    pub fn new(
        word_len: u32,
        output_elem_len: usize,
        input_elem_len: usize,
        bit_mapping: &[usize],
        parallel: bool,
    ) -> Self {
        assert_eq!((word_len & 31), 0);
        assert_eq!((input_elem_len & 31), 0);
        assert!(input_elem_len >= bit_mapping.iter().copied().max().unwrap());
        assert!(output_elem_len >= bit_mapping.len());
        Self {
            word_len,
            input_elem_len: ((input_elem_len + 31) >> 5) << 5,
            output_elem_len,
            bit_mapping: bit_mapping.to_vec(),
            parallel,
        }
    }

    fn transform_int(&self, output: &[u32], input: &mut [u32]) {
        let input_elem_word_num = self.input_elem_len >> 5;
        let elem_num = input.len() / input_elem_word_num;
        let words_per_word = (self.word_len as usize) >> 5;
        let mut gidx = 0;
        let mut widx = 0;
        let mut sbit = 0;
        for i in 0..elem_num {
            let input_elem = &mut input[i * input_elem_word_num..(i + 1) * input_elem_word_num];
            for (outbit, inbit) in self.bit_mapping.iter().enumerate() {
                let outbit_val = (output
                    [words_per_word * (gidx * self.output_elem_len + outbit) + widx]
                    >> sbit)
                    & 1;
                let iv = inbit >> 5;
                let ibit = inbit & 31;
                input_elem[iv] = (input_elem[iv] & !(1 << ibit)) | (outbit_val << ibit);
            }
            sbit += 1;
            if sbit >= 32 {
                sbit = 0;
                widx += 1;
                if widx >= words_per_word {
                    widx = 0;
                    gidx += 1;
                }
            }
        }
    }
}

impl<'a> DataTransformer<'a, CPUDataReader<'a>, CPUDataWriter<'a>, CPUDataHolder>
    for CPUDataOutputTransformer
{
    type ErrorType = Infallible;

    fn transform(&mut self, input: &CPUDataHolder) -> Result<CPUDataHolder, Self::ErrorType> {
        let mut output = CPUDataHolder::new(vec![0; self.output_data_len(input.len())]);
        self.transform_reuse(input, &mut output)?;
        Ok(output)
    }

    /// changed names of arguments:
    /// output - really input data, input - really output data
    fn transform_reuse(
        &mut self,
        output: &CPUDataHolder,
        input: &mut CPUDataHolder,
    ) -> Result<(), Self::ErrorType> {
        assert_eq!(
            input.len() % (((self.word_len as usize) * self.input_elem_len) >> 5),
            0
        );
        assert_eq!(
            output.len() % (((self.word_len as usize) * self.output_elem_len) >> 5),
            0
        );
        if self.parallel {
            const CHUNK_LEN: usize = 128;
            let mut input_w = input.get_mut();
            let input = input_w.get_mut();
            let output_r = output.get();
            let output = output_r.get();

            let word_len = self.word_len as usize;
            let input_elem_word_num = self.input_elem_len >> 5;
            let output_group_word_num = (self.output_elem_len * word_len) >> 5;
            input
                .chunks_mut(word_len * input_elem_word_num * CHUNK_LEN)
                .enumerate()
                .par_bridge()
                .for_each(|(i, x)| {
                    let end =
                        std::cmp::min(output_group_word_num * (i + 1) * CHUNK_LEN, output.len());
                    self.transform_int(&output[output_group_word_num * i * CHUNK_LEN..end], x);
                });
        } else {
            let output_r = output.get();
            let output = output_r.get();
            let mut input_w = input.get_mut();
            let input = input_w.get_mut();
            self.transform_int(output, input);
        }
        Ok(())
    }

    fn input_elem_len(&self) -> usize {
        self.output_elem_len
    }
    fn output_elem_len(&self) -> usize {
        self.input_elem_len
    }
}

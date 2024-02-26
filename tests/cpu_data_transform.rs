use gatenative::cpu_build_exec::*;
use gatenative::cpu_data_transform::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_cpu_input_output_data_transformer() {
    for parallel in [false, true] {
        let word_len = 160; // 1 32-bit word
        let input_elem_len = 96;
        let output_elem_len = 77;
        let input_elem_word_num = input_elem_len * (word_len >> 5);
        let output_elem_word_num = output_elem_len * (word_len >> 5);
        let group_num = 17541;
        let mut input = CPUDataHolder::new(vec![0u32; input_elem_word_num * group_num]);
        let mut input_2 = CPUDataHolder::new(vec![0u32; input_elem_word_num * group_num]);
        let mut output = CPUDataHolder::new(vec![0u32; output_elem_word_num * group_num]);
        let bit_mapping = (0..27)
            .chain(32..32 + 22)
            .chain(64..64 + 28)
            .collect::<Vec<_>>();
        {
            let mut input = input.get_mut();
            let input = input.get_mut();
            for (i, v) in input.iter_mut().enumerate() {
                *v = (100 + i + 51 * i * i) as u32;
            }
        }
        let mut expected = vec![0u32; output.len()];
        let wl = word_len >> 5;
        {
            let input = input.get();
            let input = input.get();
            for i in 0..32 * wl * group_num {
                let input_slice = &input[i * 3..(i + 1) * 3];
                let idx = (i >> 5) as usize;
                let gidx = idx / wl;
                let widx = idx % wl;
                let idx = gidx * wl * output_elem_len + widx;
                let sbit = i & 31;
                //println!("X{}: {} {} {} {}", i, idx, gidx, widx, sbit);
                for bit in 0..output_elem_len {
                    let xbit = bit_mapping[bit];
                    expected[idx + bit * wl] |=
                        u32::from((input_slice[xbit >> 5] >> (xbit & 31)) & 1) << sbit;
                }
            }
        }
        let mut transformer = CPUDataInputTransformer::new(
            u32::try_from(word_len).unwrap(),
            input_elem_len,
            output_elem_len,
            &bit_mapping,
            parallel,
        );
        use std::time::SystemTime;
        let start = SystemTime::now();
        transformer.transform_reuse(&input, &mut output).unwrap();
        let elapsed = start.elapsed().unwrap();
        println!("Time: {} s", elapsed.as_secs_f64());
        {
            let output = output.get();
            let output = output.get();
            for i in 0..output.len() {
                assert_eq!(expected[i], output[i], "{}: {}", parallel, i);
            }
        }

        let mut transformer = CPUDataOutputTransformer::new(
            u32::try_from(word_len).unwrap(),
            output_elem_len,
            input_elem_len,
            &bit_mapping,
            parallel,
        );
        let start = SystemTime::now();
        transformer.transform_reuse(&output, &mut input_2).unwrap();
        let elapsed = start.elapsed().unwrap();
        println!("Time: {} s", elapsed.as_secs_f64());
        {
            let input = input.release();
            let input_2 = input_2.release();
            for i in 0..input.len() {
                let expected = match i % 3 {
                    0 => input[i] & ((1 << 27) - 1),
                    1 => input[i] & ((1 << 22) - 1),
                    2 => input[i] & ((1 << 28) - 1),
                    _ => {
                        panic!("Unexpected!");
                    }
                };
                assert_eq!(expected, input_2[i], "{}: {}", parallel, i);
            }
        }
    }
}

#[test]
fn test_cpu_data_transforms() {
    let mut builder = CPUBuilder::new(None);
    let circuit = Circuit::new(
        40,
        (0..20).map(|i| Gate::new_xor(i, 20 + i)),
        (0..20).map(|i| (40 + i, false)),
    )
    .unwrap();
    builder.add("xor", circuit.clone(), None, None, None);
    let mut execs = builder.build().unwrap();
    let input = execs[0].new_data_from_vec(
        (0..1024u32 * 128u32)
            .map(|i| 124 * i + 21 * i * i + 1342)
            .collect::<Vec<_>>(),
    );
    let mut it = execs[0]
        .input_tx(64, &(0..20).chain(32..32 + 20).collect::<Vec<_>>())
        .unwrap();
    let mut ot = execs[0]
        .output_tx(32, &(0..20).collect::<Vec<_>>())
        .unwrap();
    let mut input_circ =
        execs[0].new_data((input.len() * it.output_elem_len()) / it.input_elem_len());
    it.transform_reuse(&input, &mut input_circ).unwrap();
    let output_circ = execs[0].execute(&input_circ, 0).unwrap();
    let mut output =
        execs[0].new_data((output_circ.len() * ot.output_elem_len()) / ot.input_elem_len());
    ot.transform_reuse(&output_circ, &mut output).unwrap();
    {
        let input = input.get();
        let input = input.get();
        let output = output.get();
        let output = output.get();
        for (i, v) in output.iter().enumerate() {
            assert_eq!(
                (input[2 * i] ^ input[2 * i + 1]) & ((1 << 20) - 1),
                *v,
                "{}",
                i
            );
        }
    }
    // use helpers
    let input_circ = it.transform(&input).unwrap();
    let output_circ = execs[0].execute(&input_circ, 0).unwrap();
    let output = ot.transform(&output_circ).unwrap();
    {
        let input = input.release();
        let output = output.release();
        for (i, v) in output.iter().enumerate() {
            assert_eq!(
                (input[2 * i] ^ input[2 * i + 1]) & ((1 << 20) - 1),
                *v,
                "{}",
                i
            );
        }
    }
}

use gatenative::opencl_build_exec::*;
use gatenative::*;
use gatesim::*;

use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};

#[test]
fn test_opencl_builder_and_exec() {
    let no_opt_neg_config = OpenCLBuilderConfig {
        optimize_negs: false,
    };
    let opt_neg_config = OpenCLBuilderConfig {
        optimize_negs: true,
    };

    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)
            .unwrap()
            .get(0)
            .expect("No device in platform"),
    );

    for (config_num, builder_config) in [no_opt_neg_config, opt_neg_config].into_iter().enumerate()
    {
        let mut builder = OpenCLBuilder::new(&device, Some(builder_config.clone()));
        let word_len = (builder.word_len() >> 5) as usize;
        let circuit = Circuit::new(
            4,
            [
                Gate::new_and(0, 2),
                Gate::new_and(1, 2),
                Gate::new_and(0, 3),
                Gate::new_and(1, 3),
                // add a1*b0 + a0*b1
                Gate::new_xor(5, 6),
                Gate::new_and(5, 6),
                // add c(a1*b0 + a0*b1) + a1*b1
                Gate::new_xor(7, 9),
                Gate::new_and(7, 9),
            ],
            [(4, false), (8, false), (10, false), (11, false)],
        )
        .unwrap();
        builder.add("mul2x2", circuit.clone(), None, None);
        let input_ps = (&[3, 1, 6, 4][..], 8);
        let output_ps = (&[5, 3, 1, 4][..], 7);
        builder.add(
            "mul2x2p",
            circuit.clone(),
            Some(input_ps.clone()),
            Some(output_ps.clone()),
        );
        let mut execs = builder.build().unwrap();
        const MUL2X2_INPUT_TEMPLATE: [u32; 4] = [
            0b1010101010101010,
            0b1100110011001100,
            0b1111000011110000,
            0b1111111100000000,
        ];
        let mul2x2_input = MUL2X2_INPUT_TEMPLATE
            .into_iter()
            .map(|x| std::iter::once(x).chain(std::iter::repeat(0).take(word_len - 1)))
            .flatten()
            .collect::<Vec<_>>();
        let input_holder = execs[0].new_data_from_vec(mul2x2_input.clone());
        let out = execs[0].execute(&input_holder).unwrap().release();
        for i in 0..16 {
            let a = i & 3;
            let b = i >> 2;
            let c = ((out[0] >> i) & 1)
                + (((out[1 * word_len] >> i) & 1) << 1)
                + (((out[2 * word_len] >> i) & 1) << 2)
                + (((out[3 * word_len] >> i) & 1) << 3);
            assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
        }
        // more inputs
        let mul2x2_more_input_combs = {
            let mut input = vec![];
            let mut s = 0x34251u32;
            for _ in 0..64 * 24 {
                input.push(s & 15);
                s = (s ^ (s * 1895952115 + 159502151)) ^ 0xba001a4;
                s = s.rotate_right(s & 15);
            }
            input
        };
        let mut more_input = vec![0; (mul2x2_more_input_combs.len() >> 6) * 4 * 2];
        for (i, &v) in mul2x2_more_input_combs.iter().enumerate() {
            let idx = (i >> 5) / word_len;
            let half_idx = (i >> 5) % word_len;
            let shift = i & 31;
            more_input[idx * 4 * word_len + 0 + half_idx] |= (v & 1) << shift;
            more_input[idx * 4 * word_len + word_len + half_idx] |= ((v >> 1) & 1) << shift;
            more_input[idx * 4 * word_len + 2 * word_len + half_idx] |= ((v >> 2) & 1) << shift;
            more_input[idx * 4 * word_len + 3 * word_len + half_idx] |= ((v >> 3) & 1) << shift;
        }
        let more_input_holder = execs[0].new_data_from_vec(more_input);
        let out = execs[0].execute(&more_input_holder).unwrap().release();
        for (i, v) in mul2x2_more_input_combs.into_iter().enumerate() {
            let idx = (i >> 5) / word_len;
            let half_idx = (i >> 5) % word_len;
            let shift = i & 31;
            let a = v & 3;
            let b = v >> 2;
            let c = ((out[idx * 4 * word_len + 0 + half_idx] >> shift) & 1)
                + (((out[idx * 4 * word_len + word_len + half_idx] >> shift) & 1) << 1)
                + (((out[idx * 4 * word_len + 2 * word_len + half_idx] >> shift) & 1) << 2)
                + (((out[idx * 4 * word_len + 3 * word_len + half_idx] >> shift) & 1) << 3);
            assert_eq!((a * b) & 15, c, "{}: {}", config_num, i);
        }
    }
}

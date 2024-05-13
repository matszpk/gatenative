use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

use std::str::FromStr;

#[test]
fn test_clang_writer_loop_config_opencl() {
    let circuit = Circuit::<u32>::from_str(
        r##"{
        0
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10
        11
        12
        13
        14
        15
        xor(0,4):0
        xor(1,5)
        and(0,4)
        xor(17,18):1
        xor(2,6)
        and(17,18)
        and(1,5)
        nor(21,22)
        xor(20,23):2n
        xor(3,7)
        nimpl(20,23)
        and(2,6)
        nor(26,27)
        xor(25,28):3n
        xor(8,12):4
        xor(9,13)
        nimpl(8,12)
        nimpl(30,32)
        xor(31,33):5
        xor(10,14)
        nor(31,33)
        nimpl(9,13)
        nor(36,37)
        xor(35,38):6
        xor(11,15)
        nor(35,38)
        nimpl(10,14)
        nor(41,42)
        xor(40,43):7
    }(16)
"##,
    )
    .unwrap();
    // pop_from_buffer, aggr_to_buffer with output exclusion, arg_inputs and elem_inputs
    // with placements
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)input)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)output)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8)))
            .inner_loop(Some(10)),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_mulxx(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    unsigned long buffer_shift, const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 8 * idx + input_shift;
    const size_t ovn = 8 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    uint v6;
    uint v7;
    uint v8;
    uint v9;
    uint v10;
    uint v11;
    uint v12;
    uint v13;
    uint v14;
    uint v15;
    if (idx >= n) return;
    buffer = (const global void*)(((const global char*)buffer) + 4*buffer_shift);
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i2 (v2)
#define i3 (v3)
#define i6 (v4)
#define i10 (v6)
    i2 = ((TYPE_NAME*)input)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    if (iter == 0) {
    v0 = input[ivn + 0];
    v1 = input[ivn + 2];
    v5 = input[ivn + 1];
    v7 = input[ivn + 3];
    v8 = input[ivn + 5];
    v9 = input[ivn + 7];
    }
    v10 = elem_low_bit0;
    v11 = (v0 ^ v10);
    v12 = elem_low_bit1;
    v13 = (v1 ^ v12);
    v0 = (v0 & v10);
    v10 = (v13 ^ v0);
    output[ovn + 5] = v10;
    v14 = (v2 ^ v4);
    v0 = (v13 & v0);
    v1 = (v1 & v12);
    v0 = ~(v0 | v1);
    v1 = (v14 ^ v0);
    output[ovn + 7] = ~v1;
    v12 = elem_low_bit2;
    v3 = (v3 ^ v12);
    v0 = (v14 & ~v0);
    v2 = (v2 & v4);
    v0 = ~(v0 | v2);
    v0 = (v3 ^ v0);
    output[ovn + 3] = ~v0;
    v2 = elem_low_bit3;
    v3 = ((arg & 1) != 0) ? one : zero;
    v4 = (v2 ^ v3);
    output[ovn + 2] = v4;
    v12 = ((arg & 2) != 0) ? one : zero;
    v13 = (v5 ^ v12);
    v2 = (v2 & ~v3);
    v2 = (v4 & ~v2);
    v3 = (v13 ^ v2);
    v14 = (v6 ^ v8);
    v2 = ~(v13 | v2);
    v5 = (v5 & ~v12);
    v2 = ~(v2 | v5);
    v5 = (v14 ^ v2);
    output[ovn + 0] = v5;
    v7 = (v7 ^ v9);
    v2 = ~(v14 | v2);
    v6 = (v6 & ~v8);
    v2 = ~(v2 | v6);
    v2 = (v7 ^ v2);
    output[ovn + 1] = v2;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v11)
#define o2 (v1)
#define o3 (v0)
#define o5 (v3)
    ((TYPE_NAME*)output)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[ovn + 5] = v10;
    output[ovn + 7] = v1;
    output[ovn + 3] = v0;
    output[ovn + 2] = v4;
    output[ovn + 0] = v5;
    output[ovn + 1] = v2;
    } else {
    v7 = v0;
    v9 = v1;
    v0 = v5;
    v5 = v2;
    v1 = v4;
    v8 = v10;
    }
    } // loop
}
"##
    );
    // pop_from_buffer, aggr_to_buffer with output exclusion, arg_inputs and elem_inputs
    // with placements with single_buffer
    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "mulxx",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&[12, 13]))
            .elem_inputs(Some(&[4, 5, 7, 8]))
            .pop_input_code(Some("    i2 = ((TYPE_NAME*)buffer)[1];"))
            .pop_from_buffer(Some(&[2, 3, 6, 10]))
            .aggr_output_code(Some("    ((TYPE_NAME*)buffer)[1] = o2;"))
            .aggr_to_buffer(Some(&[0, 2, 3, 5]))
            .exclude_outputs(Some(&[0, 5]))
            .input_placement(Some((&[0, 2, 1, 3, 5, 7], 8)))
            .output_placement(Some((&[5, 7, 3, 2, 0, 1], 8)))
            .inner_loop(Some(10))
            .single_buffer(true),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_mulxx(unsigned long n, 
    unsigned long output_shift,
    unsigned long buffer_shift, global uint* output, unsigned int arg, unsigned int arg2, global void* buffer) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 8 * idx + output_shift;
    const size_t ovn = 8 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    const unsigned int iter_max = 10U;
    unsigned int iter;
    unsigned int stop = 0;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    uint v6;
    uint v7;
    uint v8;
    uint v9;
    uint v10;
    uint v11;
    uint v12;
    uint v13;
    uint v14;
    uint v15;
    if (idx >= n) return;
    buffer = (const global void*)(((const global char*)buffer) + 4*buffer_shift);
    for (iter = 0; iter < iter_max && stop == 0; iter++) {
#define i2 (v2)
#define i3 (v3)
#define i6 (v4)
#define i10 (v6)
    i2 = ((TYPE_NAME*)buffer)[1];
#undef i2
#undef i3
#undef i6
#undef i10
    if (iter == 0) {
    v0 = output[ivn + 0];
    v1 = output[ivn + 2];
    v5 = output[ivn + 1];
    v7 = output[ivn + 3];
    v8 = output[ivn + 5];
    v9 = output[ivn + 7];
    }
    v10 = elem_low_bit0;
    v11 = (v0 ^ v10);
    v12 = elem_low_bit1;
    v13 = (v1 ^ v12);
    v0 = (v0 & v10);
    v10 = (v13 ^ v0);
    output[ovn + 5] = v10;
    v14 = (v2 ^ v4);
    v0 = (v13 & v0);
    v1 = (v1 & v12);
    v0 = ~(v0 | v1);
    v1 = (v14 ^ v0);
    output[ovn + 7] = ~v1;
    v12 = elem_low_bit2;
    v3 = (v3 ^ v12);
    v0 = (v14 & ~v0);
    v2 = (v2 & v4);
    v0 = ~(v0 | v2);
    v0 = (v3 ^ v0);
    output[ovn + 3] = ~v0;
    v2 = elem_low_bit3;
    v3 = ((arg & 1) != 0) ? one : zero;
    v4 = (v2 ^ v3);
    output[ovn + 2] = v4;
    v12 = ((arg & 2) != 0) ? one : zero;
    v13 = (v5 ^ v12);
    v2 = (v2 & ~v3);
    v2 = (v4 & ~v2);
    v3 = (v13 ^ v2);
    v14 = (v6 ^ v8);
    v2 = ~(v13 | v2);
    v5 = (v5 & ~v12);
    v2 = ~(v2 | v5);
    v5 = (v14 ^ v2);
    output[ovn + 0] = v5;
    v7 = (v7 ^ v9);
    v2 = ~(v14 | v2);
    v6 = (v6 & ~v8);
    v2 = ~(v2 | v6);
    v2 = (v7 ^ v2);
    output[ovn + 1] = v2;
    v1 = ~v1;
    v0 = ~v0;
#define o0 (v11)
#define o2 (v1)
#define o3 (v0)
#define o5 (v3)
    ((TYPE_NAME*)buffer)[1] = o2;
#undef o0
#undef o2
#undef o3
#undef o5
    if (iter == iter_max - 1 || stop != 0) {
    output[ovn + 5] = v10;
    output[ovn + 7] = v1;
    output[ovn + 3] = v0;
    output[ovn + 2] = v4;
    output[ovn + 0] = v5;
    output[ovn + 1] = v2;
    } else {
    v7 = v0;
    v9 = v1;
    v0 = v5;
    v5 = v2;
    v1 = v4;
    v8 = v10;
    }
    } // loop
}
"##
    );
}

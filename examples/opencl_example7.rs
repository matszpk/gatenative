use gategen::boolvar::*;
use gategen::gatesim::*;
use gategen::intvar::*;
use gatenative::{opencl_build_exec::*, *};
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};

// generate circuit
fn mul_add_circuit() -> Circuit<u32> {
    call32(|| {
        let a = U16Var32::var();
        let b = U16Var32::var();
        let c = U16Var32::var();
        let ra = &a * &b + &c;
        let rb = &c * &a + &b;
        let rc = &b * &c + &a;
        let out = ra.concat(rb).concat(rc);
        // Circuit has 48-bit input divided into:
        // 0..16 - 'a' argument
        // 16..32 - 'b' argument
        // 32..48 - 'c' argument
        out.to_translated_circuit(a.concat(b).concat(c).iter())
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::new(
        *get_all_devices(CL_DEVICE_TYPE_GPU)?
            .get(0)
            .expect("No OpenCL devices"),
    );
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create builder.
    let mut builder = OpenCLBuilder::new(&device, None);
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new().single_buffer(true).inner_loop(Some(10)),
    );
    let mut execs = builder.build()?;
    // Get input data transformer that converts 96-bit structure into 48-bit circuit input:
    // 0 32-bit word - 'a', 1 32-bit word - 'b', 2 32-bit word - 'c'.
    let mut it = execs[0].input_transformer(
        96,
        &((0..16).chain(32..48).chain(64..80).collect::<Vec<_>>()),
    )?;
    // Get output data transformer that converts 48-bit circuit input to 96-bit structure:
    // 0 32-bit word - 'a', 1 32-bit word - 'b', 2 32-bit word - 'c'.
    let mut ot = execs[0].output_transformer(
        96,
        &((0..16).chain(32..48).chain(64..80).collect::<Vec<_>>()),
    )?;
    let input = execs[0].new_data_from_vec(
        (0..16384u32)
            .map(|x| [(x + 3) & 0xffff, (x + 1489u32) & 0xffff, (5 * x) & 0xffff])
            .flatten()
            .collect::<Vec<_>>(),
    );
    // Transform input to internal form.
    let mut data = it.transform(&input)?;
    // Execute simulation with single buffer.
    execs[0].execute_single(&mut data, 0)?;
    // Transform output to 32-bit array.
    let output = ot.transform(&data)?;
    // Release output data holder - just get its data.
    let output = output.release();
    // Print that data (3 values per element)
    for (i, v) in output.chunks(3).into_iter().enumerate() {
        println!("{}: {} {} {}", i, v[0], v[1], v[2]);
    }
    Ok(())
}

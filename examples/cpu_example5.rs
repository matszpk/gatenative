use gategen::boolvar::*;
use gategen::gatesim::*;
use gategen::intvar::*;
use gatenative::{cpu_build_exec::*, *};

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

// This example generates output equal to results from cpu_example4.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create builder.
    let mut builder = CPUBuilder::new(None);
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            .single_buffer(true)
            // Sets pack of elements (inputs): input mapping:
            // input[0] = pack[2], input[1] = pac[0], input[2] = pack[3]
            .input_placement(Some((
                &((32..48).chain(0..16).chain(48..64).collect::<Vec<_>>()),
                64,
            )))
            // Sets pack of elements (outputs): output mapping:
            // output[0] = pack[2], output[1] = pac[1], output[2] = pack[3]
            .output_placement(Some((
                &((32..48).chain(16..32).chain(48..64).collect::<Vec<_>>()),
                64,
            ))),
    );
    let mut execs = builder.build()?;
    // Get input data transformer that converts 96-bit structure into 48-bit circuit input:
    // 4 32-bit words.
    let mut it = execs[0].input_transformer(
        128,
        &((0..16)
            .chain(32..48)
            .chain(64..80)
            .chain(96..112)
            .collect::<Vec<_>>()),
    )?;
    // Get output data transformer that converts 48-bit circuit input to 96-bit structure:
    // 4 32-bit words.
    let mut ot = execs[0].output_transformer(
        128,
        &((0..16)
            .chain(32..48)
            .chain(64..80)
            .chain(96..112)
            .collect::<Vec<_>>()),
    )?;
    let input = execs[0].new_data_from_vec(
        (0..16384u32)
            .map(|x| {
                [
                    (x + 1489u32) & 0xffff, // 1
                    (x + 442) & 0xffff,     // fake
                    (x + 3) & 0xffff,       // 0
                    (5 * x) & 0xffff,       // 2
                ]
            })
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
    for (i, v) in output.chunks(4).into_iter().enumerate() {
        println!("{}: {} {} {}", i, v[2], v[1], v[3]);
    }
    Ok(())
}

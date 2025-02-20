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
        let r = &a * &b + &c;
        // Circuit has 48-bit input divided into:
        // 0..16 - 'a' argument
        // 16..32 - 'b' argument
        // 32..48 - 'c' argument
        r.to_translated_circuit(a.concat(b).concat(c).iter())
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circuit = mul_add_circuit();
    let mut builder = CPUBuilder::new(None);
    // Add circuit to builder.
    builder.add_simple("mul_add", circuit);
    let mut execs = builder.build()?;
    // Get input data transformer that converts 96-bit structure into 48-bit circuit input:
    // 0 32-bit word - 'a', 1 32-bit word - 'b', 2 32-bit word - 'c'.
    let mut it = execs[0].input_transformer(
        96,
        &((0..16).chain(32..48).chain(64..80).collect::<Vec<_>>()),
    )?;
    // Get output data transformer that converts 16-bit output into 32-bit array
    // of elements.
    let mut ot = execs[0].output_transformer(32, &((0..16).collect::<Vec<_>>()))?;
    // Prepare empty input for execution.
    let input = execs[0].new_data_from_vec(
        (0..4096u32)
            .map(|x| [(x + 3) & 0xffff, (x + 1489u32) & 0xffff, (5 * x) & 0xffff])
            .flatten()
            .collect::<Vec<_>>(),
    );
    // transform input to internal form.
    let input = it.transform(&input)?;
    // Execute simulation. Set 'b' to b_start and 'c' to c_start by arg input.
    let output = execs[0].execute(&input, 0)?;
    // Transform output to 32-bit array.
    let output = ot.transform(&output)?;
    // Release output data holder - just get its data.
    let output = output.release();
    // Print that data
    for (i, v) in output.into_iter().enumerate() {
        println!("{}: {}", i, v);
    }
    Ok(())
}

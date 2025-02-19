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
    let b_start = 4782u16;
    let c_start = 18941u16;
    let circuit = mul_add_circuit();
    let mut builder = CPUBuilder::new(None);
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            // Assign circuit's inputs 'b' and 'c' to arg input ('c' higher).
            .arg_inputs(Some(&(16..48).collect::<Vec<_>>()))
            // Assign circuit input 'a' to element index.
            .elem_inputs(Some(&(0..16).collect::<Vec<_>>())),
    );
    let mut execs = builder.build()?;
    // Get output data transformer that converts 16-bit output into 32-bit array
    // of elements.
    let mut ot = execs[0].output_transformer(32, &((0..16).collect::<Vec<_>>()))?;
    // Prepare empty input for execution.
    let input = execs[0].new_data(16);
    // Execute simulation. Set 'b' to b_start and 'c' to c_start by arg input.
    let output = execs[0].execute(&input, ((c_start as u64) << 16) | (b_start as u64))?;
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

use gategen::boolvar::*;
use gategen::gatesim::*;
use gategen::intvar::*;
use gatenative::cpu_build_exec::*;
use gatenative::mapper::*;
use gatenative::*;

// generate circuit
fn mul_add_circuit() -> Circuit<u32> {
    call32(|| {
        let a = U10Var32::var();
        let b = U10Var32::var();
        let c = U10Var32::var();
        let r = &a * &b + &c;
        // Circuit has 30-bit input divided into:
        // 0..10 - 'a' argument
        // 10..20 - 'b' argument
        // 20..30 - 'c' argument
        r.to_translated_circuit(a.concat(b).concat(c).iter())
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create builder.
    let builder = CPUBuilder::new(None);
    // Create basic mapper builder.
    let mut builder = CPUParBasicMapperBuilder::new(builder);
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            // Assign circuit's 6 highest bits to arg input.
            .arg_inputs(Some(&(24..30).collect::<Vec<_>>()))
            // Assign circuit 24 lowest bits to element index.
            .elem_inputs(Some(&(0..24).collect::<Vec<_>>())),
    );
    let mut execs = builder.build()?;
    // Get output data transformer that converts 10-bit output into 32-bit array
    // of elements.
    let ot = execs[0].output_transformer(32, &((0..10).collect::<Vec<_>>()))?;
    // Prepare empty input for execution.
    let input = execs[0].new_data(16);
    // Execute simulation for all combinations of inputs.
    let output = execs[0].execute(
        &input,
        0u64,
        // just sum all outputs
        |_, output, arg| {
            eprintln!("Arg Input: {}", arg);
            // Convert to output to 32-bit array. Use clone of output transformer.
            let output = ot.clone().transform(output).unwrap();
            let output = output.release();
            // Make sum of array elements.
            output.into_iter().map(|x| u64::from(x)).sum::<u64>()
        },
        // Join results.
        |a, b| a + b,
        // No stop.
        |_| false,
    )?;
    println!("Sum: {}", output);
    Ok(())
}

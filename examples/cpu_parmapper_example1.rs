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
    // Put transform helpers to code
    builder.transform_helpers();
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            // Assign circuit's 6 highest bits to arg input.
            .arg_inputs(Some(&(24..30).collect::<Vec<_>>()))
            // Assign circuit 24 lowest bits to element index.
            .elem_inputs(Some(&(0..24).collect::<Vec<_>>()))
            // Sets aggr output code that sums all results.
            // Because is possible overflow then uses 16 differents accumulators
            .aggr_output_code(Some(
                r##"{
    size_t i, j;
    // buf are accumulators
    uint32_t* buf = (uint32_t*)output;
    uint32_t temp[TYPE_LEN] TALIGN_ATTR;
    // transform to 32-bit word array.
    OUTPUT_TRANSFORM_B10(temp, o0, o1, o2, o3, o4, o5, o6, o7, o8, o9);
    for (i = 0; i < TYPE_LEN; i += 16)
        for (j = 0; j < 16; j++)
            __sync_fetch_and_add(buf + j, temp[i + j]);
}"##,
            ))
            // Sets aggregated output with length 16 32-bit words
            .aggr_output_len(Some(16)),
    );
    let mut execs = builder.build()?;
    // Prepare empty input for execution.
    let input = execs[0].new_data(16);
    // Execute simulation for all combinations of inputs.
    let output = execs[0].execute_direct(
        &input,
        0u64,
        // just sum all outputs
        |_, output, arg| {
            eprintln!("Arg Input: {}", arg);
            // Make sum of array elements.
            output.into_iter().map(|x| u64::from(*x)).sum::<u64>()
        },
        // Join results.
        |a, b| a + b,
        // No stop.
        |_| false,
    )?;
    println!("Sum: {}", output);
    Ok(())
}

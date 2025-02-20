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
    // Set argument constants.
    let b_start = 4782u16;
    let c_start = 18941u16;
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create circuit.
    let mut builder = CPUBuilder::new(None);
    // Put transform helpers to code
    builder.transform_helpers();
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            // Assign circuit's inputs 'b' and 'c' to arg input ('c' higher).
            .arg_inputs(Some(&(16..48).collect::<Vec<_>>()))
            // Assign circuit input 'a' to element index.
            .elem_inputs(Some(&(0..16).collect::<Vec<_>>()))
            // Sets aggr_output_len to 2**16 32-bit words.
            .aggr_output_len(Some(1 << 16))
            // Sets aggr output code that transforms output.
            // Output will be stored to output data (and that is additional buffer).
            .aggr_output_code(Some(
                r##"{
    uint32_t* output_u32 = (uint32_t*)output;
    OUTPUT_TRANSFORM_B16(output_u32 + TYPE_LEN*idx, o0, o1, o2, o3, o4, o5, o6, o7,
                        o8, o9, o10, o11, o12, o13, o14, o15);
}"##,
            )),
    );
    let mut execs = builder.build()?;
    // Prepare empty input for execution.
    let input = execs[0].new_data(16);
    // Execute simulation. Set 'b' to b_start and 'c' to c_start by arg input.
    let output = execs[0].execute(&input, ((c_start as u64) << 16) | (b_start as u64))?;
    // Release output data holder - just get its data.
    let output = output.release();
    // Print that data
    for (i, v) in output.into_iter().enumerate() {
        println!("{}: {}", i, v);
    }
    Ok(())
}

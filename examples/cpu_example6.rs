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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set argument constants.
    let param_a = 0x4da0u16;
    let param_b = 0x8725u16;
    let param_c = 0xe052u16;
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create builder.
    let mut builder = CPUBuilder::new(None);
    // Put transform helpers to code
    builder.transform_helpers();
    // Add circuit to builder.
    builder.add_with_config(
        "mul_add",
        circuit,
        CodeConfig::new()
            // Sets populating code (pop_input_code) that generates input for 'c' argument.
            // Parameters from additional buffer.
            .pop_input_code(Some(
                r##"{
    // Get paramareters from input
    const uint32_t* params = (const uint32_t*)buffer;
    const uint32_t param_a = params[0];
    const uint32_t param_b = params[1];
    const uint32_t param_c = params[2];
    size_t i;
    uint32_t temp[TYPE_LEN] TALIGN_ATTR;
    // generate values of function: (x xor param_a) + (x and param_b) + (x or param_c).
    for (i = 0; i < TYPE_LEN; i++) {
        const uint32_t id = idx*TYPE_LEN + i;
        temp[i] = ((id^param_a) + (id&param_b) + (id|param_c)) & 0xffff;
    }
    INPUT_TRANSFORM_B16(i32, i33, i34, i35, i36, i37, i38, i39,
                        i40, i41, i42, i43, i44, i45, i46, i47, temp);
}"##,
            ))
            // Generates from pop_input_code from buffer for 'c' (32..48 circuit inputs).
            .pop_from_buffer(Some(&(32..48).collect::<Vec<_>>()))
            // Sets buffer length to 4 32-bit words.
            .pop_input_len(Some(4))
            // Sets aggr_output_code to calculate sum of results c.
            .aggr_output_code(Some(
                r##"{
    size_t i;
    uint32_t* buffer_u32 = (uint32_t*)buffer;
    uint32_t out_table[TYPE_LEN] TALIGN_ATTR;
    OUTPUT_TRANSFORM_B16(out_table, o32, o33, o34, o35, o36, o37, o38, o39,
                        o40, o41, o42, o43, o44, o45, o46, o47);
    for (i = 0; i < TYPE_LEN; i++)
        __sync_fetch_and_add(&(buffer_u32[3]), out_table[i]);
}"##,
            ))
            // Generates from pop_input_code from buffer for 'c' (32..48 circuit inputs).
            .aggr_only_to_buffer(Some(&(32..48).collect::<Vec<_>>()))
            // Sets buffer length to 4 32-bit words.
            .aggr_output_len(Some(4)),
    );
    let mut execs = builder.build()?;
    // Get input data transformer that converts 96-bit structure into 48-bit circuit input:
    // 0 32-bit word - 'a', 1 32-bit word - 'b'.
    let mut it = execs[0].input_transformer(64, &((0..16).chain(32..48).collect::<Vec<_>>()))?;
    // Get output data transformer that converts 48-bit circuit input to 96-bit structure:
    // 0 32-bit word - 'a', 1 32-bit word - 'b'.
    let mut ot = execs[0].output_transformer(64, &((0..16).chain(32..48).collect::<Vec<_>>()))?;
    let input = execs[0].new_data_from_vec(
        (0..16384u32)
            .map(|x| [(x + 3) & 0xffff, (x + 1489u32) & 0xffff])
            .flatten()
            .collect::<Vec<_>>(),
    );
    // Transform input to internal form.
    let input = it.transform(&input)?;
    // Set buffer with parameters. Fourth word is result initially set to 0.
    let mut buffer =
        execs[0].new_data_from_slice(&[param_a.into(), param_b.into(), param_c.into(), 0u32]);
    // Execute simulation with single buffer.
    let output = execs[0].execute_buffer(&input, 0, &mut buffer)?;
    // Transform output to 32-bit array.
    let output = ot.transform(&output)?;
    // Release output data holder - just get its data.
    let output = output.release();
    // Release buffer data holder.
    let buffer = buffer.release();
    // Print that data (3 values per element)
    for (i, v) in output.chunks(2).into_iter().enumerate() {
        println!("{}: {} {}", i, v[0], v[1]);
    }
    // Print result for result c.
    println!("Result: {}", buffer[3]);
    Ok(())
}

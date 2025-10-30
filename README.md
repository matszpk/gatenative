The library allows to execute simulation of the Gate circuit on CPU or GPU (by using OpenCL).
It provides complex configuration of execution including passing data circuit inputs.
This library executes parallel simulation of circuit that runs many circuits.
The execution organized as threads (elements) in single execution. One circuit simulation
mapped to one bit of word of processor. For modern CPU a word can have 64 to 512 bits.
The library uses vector processing instruction to run simulation efficiently on CPU.

Important: CPU builder and CPU executor have been tested only on Linux.

Additional feature is ability to write code to process passing and processing inputs before
calling the simulation and write code to process outputs after calling the simulation.
The code that process inputs called as 'populating code' and the code that process
outputs called as 'aggregating code'.

Next feature is ability to make loop for execution of simulation. Inside that loop
is possible to process inputs and outputs between simulations and use a GPU local memory.
Just it possible to call multiple simulation executions (iterations) under single
kernel call.

This library organizes simulation's execution in two steps: in the first step build code to
execute simulation that native for processor or GPU, in second step just execute built code
to make simulation.

The organization of simulation is shown under image:
```text
+---------------------------------------------------------------------------------+
| SIM(0)....SIM(N-1) | SIM(N)....SIM(2*N-1) | ..... | SIM((K-1)*N)....SIM((K)*N-1)|
+---------------------------------------------------------------------------------+
```
In this image `SIM(X)` is Xth simulation. Simulation are groupped into processor words
with length N bits and all execution contains K*N simulation. If a processor has
256-bit word then can execute 256 simulations under single thread.
A GPU uses only 32-bit words, however simulation also groupped by group size (group length).
that can have even 256 threads. A special option treat whole group as processor's word,
however in many cases is not usable.

Circuit inputs and circuit output data are organized as pack of processor words that
groupped in greater stream. Data can contain more that packs if number of elements
is greater than number of bits of processor word. Later, that form of data going to be
called internal form.

```text
+----------------------------------------------------------------------------------------+
|D(0)(0)B(0)...D(0)(0)B(N)|D(0)(1)B(0)...D(0)(1)B(N)|............|D(0)(1)B0...D(0)(1)B(N)|
+----------------------------------------------------------------------------------------+
|D(1)(0)B(0)...D(1)(0)B(N)|D(1)(1)B(0)...D(1)(1)B(N)|............|D(1)(1)B0...D(1)(1)B(N)|
+----------------------------------------------------------------------------------------+
|........................................................................................|
+----------------------------------------------------------------------------------------+
|D(T)(0)B(0)...D(T)(0)B(N)|D(T)(1)B(0)...D(T)(1)B(N)|............|D(T)(1)B0...D(T)(1)B(N)|
+----------------------------------------------------------------------------------------+
```
D(I)(X)B(Y) - Yth bit in Xth pack element in Ith group. That bit assigned to I*N+Y element
(thread).

By default ith pack element assigned to ith circuit input or ith circuit output.
It can be changed by using input placement or output placement. Number of element in single
execution should be divisible by number of bit of processor word.

Input data or output data organized as bits in processor word. One bit per one
element (thread). If you want convert data organized as packs from/to data organized per
bit you should use data transformer.

In this library it used terminology:
* Builder - object to built code for simulate circuits. Builder can hold many
  simulation configurations for same circuit.
* Code configuration - It holds circuit inputs and outputs configuration,
  populating and aggregating code, loop setup, etc.
* Execution - execution of simulations with specified size that will be run under
  single execution on GPU (as single execution of kernel) or CPU.
* Executor - object to call simulation. Single executor per single circuit.
* MapperBuilder - builder to simplify multiple execution with more elements
  than can have single simulation.
* MapperExecutor - executor that execute multiple simulations.
* Data holder - object that holds data used while simulation
  (as input or output or other data). Data will be in device that will run simulation.
* Data reader - object that allows read data from data holder.
* Data writer - object that allows write data in data holder.
* Populating input code - code in the C language (or OpenCL C) that generate data to
  populate for some specified circuit inputs.
* Aggregating output code - code in the C language (or OpenCL C) that process output
  data from some specified circuit outputs.
* Word - generally is processor's word , however if `group_len` is set then
  it is multipla: group_len*word_len.
* Type in Code - processor's word used while executing simulation. It is used in native code
  and in a populating and an aggregating code.
* Element - single simulation.
* Element index - index of simulation.
* Element input - circuit input that value is element index.
* Argument input - circuit input that obtained from argument from execution call.
* FuncWriter - trait defines object to write native code of function.
* CodeWriter - trait defines object to write native code.
* Data transformer - object to convert input data in internal form to external form.
* Pack element - part of data that assigned to one circuit input or circuit output.

Program should make few steps to run simulation:
1. Create builder.
2. Add circuits and their configurations that includes input and output setup, additional
   code to process input and output data before and after simulation.
3. Built executors.
4. Prepare input data to internal form by using input data transformer if needed.
   Also it can put additional data for populating code in buffers.
5. Execute simulation.
6. Retrieve output data in external form by using output data transformer if needed.
   Also it can get additional data processed by aggregating code.

Usage of data transformer is optional.

The library reads environment variable to get important setup:
* `GATE_SYS_DUMP_SOURCE` - if set to 1 then GateNative prints source code for simulation.
* `GATE_SYS_CC` - path to C compiler that will be used while building code for simulation.
* `GATE_SYS_UNTESTED` - if set to 1 then enables untested features (AVX512 support or other).

Example 1:
```rust
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
    // Create circuit.
    let circuit = mul_add_circuit();
    // Create builder.
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
```

Other examples in 'examples' directory.

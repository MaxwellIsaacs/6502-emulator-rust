
# 6502 emulator in rust

## Description:
This project is a 6502 microprocessor emulator written in Rust. The emulator replicates the behavior of the classic MOS Technology 6502 CPU—the chip powering vintage computers and gaming consoles such as the Apple II, Commodore 64, and the Nintendo Entertainment System. It comes with an assembler to convert 6502 assembly language into machine code and includes a debugger interface to facilitate testing and development.

## Features:
- CPU Emulation:
  Emulates key 6502 registers and flags (A, X, Y, SP, PC, and the status flags) and supports a small range of 6502 instructions.
- Opcode Handling:
  Implements an opcode table with support for various addressing modes (Immediate, Zero Page, Absolute, etc.) and opcode handlers for each instruction.
- Memory Management:
  Simulates the 64KB addressable memory space of the 6502, enabling realistic program execution.
- Assembler:
  Provides functionality to assemble .asm files into binary machine code. The assembler parses assembly instructions, handles operands, and outputs the corresponding opcodes.
- Debugger Integration:
  Includes a basic debugger to step through execution, inspect CPU state, and aid in development and troubleshooting.
- Extensible Architecture:
  The modular design makes it easy to add more instructions or enhance existing functionality. I'm planning on extending it to support basic graphics to render snake.
- Library Facade:
  Provides an `Emulator` type that wraps the CPU and memory, making it simple to reset state, load programs, run for a fixed number of cycles, or execute until a BRK instruction.
- WebAssembly Bindings:
  Optional `wasm-bindgen` exports enable running the emulator in the browser while sharing memory buffers with JavaScript.

  
## Getting Started:
1. Clone the Repository:
   git clone https://github.com/MaxwellIsaacs/6502-emulator-rust.git
   cd 6502-emulator-rust

2. Build the Project:
   ```bash
   cargo build
   ```

3. Running the Emulator:
   The interactive debugger is hidden behind the `debugger` feature flag. Enable it when running the binary and pass the path to the assembly program you want to assemble and execute:
   ```bash
   cargo run --features debugger -- path/to/program.asm
   ```
   If no path is provided, the CLI falls back to `src/example.asm`.

## Implementation Overview:
- CPU and Opcode Table:
  The CPU struct holds registers (a, x, y, sp, pc, and status) and provides methods to execute instructions, set and clear flags, and update the program counter. When the `debugger` feature is enabled, additional debugging helpers are compiled in.
  An OpcodeTable maps opcodes (like LDA, STA, TAX, etc.) to their corresponding handler functions. The handlers are implemented as functions that may or may not require access to memory.

- Memory Management:
  The emulator includes a Memory module to simulate a 64KB address space, providing methods to read and write data at specific addresses.

- Assembler:
  The assembler reads .asm files, removes comments, tokenizes instructions and operands, and uses a lookup table to convert assembly instructions into their corresponding opcode bytes.
  It supports various operand sizes (1 or 2 bytes) and handles little-endian conversion for 16-bit values.

## Debugging and Testing:
The project integrates a debugger (enabled through the `debugger` feature) which allows you to:
- Step through instruction execution.
- Inspect CPU registers and memory.
- Monitor the state of the program counter and flags.

## Library Usage

The `Emulator` type exposed by the crate can be embedded in other applications:

```rust
use rust_6502_emulator::Emulator;

let mut emulator = Emulator::new();
emulator.reset();
emulator.load_program(&program_bytes);
emulator.tick(10); // execute ten instructions
let state = emulator.state();
println!("PC={:#06X}", state.pc);
```

## WebAssembly Build

Enable the `wasm` feature and use `wasm-pack` to build the bindings exposed through `wasm-bindgen`:

```bash
wasm-pack build --target web --features wasm
```

The generated `WasmEmulator` class mirrors the native facade: it exposes `reset`, `load_program`, `tick`, `run_until_break`, and helpers to inspect registers or map the 64KB memory buffer into JavaScript via the returned pointer and length.

## 6502 Opcode Checklist

[✓] ADC  
[✓] AND  
[✓] BCC  
[✓] BCS  
[✓] BEQ  
[✓] BIT  
[✓] BMI  
[✓] BNE  
[✓] BPL  
[✓] BVC  
[✓] BVS  
[✓] CLC  
[✓] CLD  
[✓] CLI  
[✓] CLY  
[✓] CMP  
[✓] CPX  
[✓] CPY  
[✓] EOR  
[✓] JMP  
[✓] JSR  
[✓] LDA  
[✓] LDX  
[✓] LDY  
[✓] ORA  
[✓] PHA  
[✓] PHP  
[✓] PLA  
[✓] PLP  
[✓] RTI  
[✓] RTS  
[✓] SBC  
[✓] STA  
[✓] STX  
[✓] STY  
[✓] TAX  
[✓] TAY  
[✓] TSX  
[✓] TXA  
[✓] TXS  
[✓] TYA  


## Contributing:
Contributions are welcome! If you have suggestions, find bugs, or want to add new features:
- Please open an issue to discuss your ideas.
- Feel free to submit a pull request with your improvements.

## License:
This project is licensed under the MIT License. See the LICENSE file for further details.


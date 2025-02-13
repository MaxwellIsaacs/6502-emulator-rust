====================
6502 Emulator in Rust
====================

# Description:
This project is a 6502 microprocessor emulator written in Rust. The emulator replicates the behavior of the classic MOS Technology 6502 CPU—the heart of vintage computers and gaming consoles such as the Apple II, Commodore 64, and the Nintendo Entertainment System. It comes with an assembler to convert 6502 assembly language into machine code and includes a debugger interface to facilitate testing and development.

Features:
- CPU Emulation:
  Emulates key 6502 registers and flags (A, X, Y, SP, PC, and the status flags) and supports a broad range of 6502 instructions.
- Opcode Handling:
  Implements a comprehensive opcode table with support for various addressing modes (Immediate, Zero Page, Absolute, etc.) and opcode handlers for each instruction.
- Memory Management:
  Simulates the 64KB addressable memory space of the 6502, enabling realistic program execution.
- Assembler:
  Provides functionality to assemble .asm files into binary machine code. The assembler parses assembly instructions, handles operands, and outputs the corresponding opcodes.
- Debugger Integration:
  Includes a basic debugger to step through execution, inspect CPU state, and aid in development and troubleshooting.
- Extensible Architecture:
  The modular design makes it easy to add more instructions or enhance existing functionality.

  
# Getting Started:
1. Clone the Repository:
   git clone https://github.com/MaxwellIsaacs/6502-emulator-rust.git
   cd 6502-emulator-rust

2. Build the Project:
   cargo build --release

3. Running the Emulator:
   To run the emulator with a binary program, either paste your file into example.asm, or change the path in the `main.rs` file

# Implementation Overview:
- CPU and Opcode Table:
  The CPU struct holds registers (a, x, y, sp, pc, and status) and a Debugger instance. It provides methods to execute instructions, set and clear flags, and update the program counter.
  An OpcodeTable maps opcodes (like LDA, STA, TAX, etc.) to their corresponding handler functions. The handlers are implemented as functions that may or may not require access to memory.

- Memory Management:
  The emulator includes a Memory module to simulate a 64KB address space, providing methods to read and write data at specific addresses.

- Assembler:
  The assembler reads .asm files, removes comments, tokenizes instructions and operands, and uses a lookup table to convert assembly instructions into their corresponding opcode bytes.
  It supports various operand sizes (1 or 2 bytes) and handles little-endian conversion for 16-bit values.

# Debugging and Testing:
The project integrates a debugger (accessible via the Debugger struct) which allows you to:
- Step through instruction execution.
- Inspect CPU registers and memory.
- Monitor the state of the program counter and flags.

# Contributing:
Contributions are welcome! If you have suggestions, find bugs, or want to add new features:
- Please open an issue to discuss your ideas.
- Feel free to submit a pull request with your improvements.

License:
This project is licensed under the MIT License. See the LICENSE file for further details.

Note:
This README covers the core functionality and structure of the project—the “meat and bones.” Additional modules and features may be available in the repository.

Happy coding, and enjoy exploring the world of 6502 emulation in Rust!

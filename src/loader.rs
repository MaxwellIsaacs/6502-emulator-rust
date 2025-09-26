use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn assemble<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = file_path.as_ref();

    if let Some(ext) = path.extension() {
        if ext != "asm" {
            return Err(format!("File has wrong extension: {:?}", ext).into());
        }
    } else {
        return Err("File has no extension".into());
    }

    let contents = fs::read_to_string(path)?;

    let opcode_table: HashMap<&str, (u8, usize)> = [
        // Load and Store Instructions
        ("LDA", (0xA9, 1)), // Immediate
        ("LDA", (0xA5, 1)), // Zero Page
        ("LDA", (0xAD, 2)), // Absolute
        ("STA", (0x85, 1)), // Zero Page
        ("STA", (0x8D, 2)), // Absolute
        ("LDX", (0xA2, 1)), // Immediate
        ("LDX", (0xA6, 1)), // Zero Page
        ("LDX", (0xAE, 2)), // Absolute
        ("STX", (0x86, 1)), // Zero Page
        ("STX", (0x8E, 2)), // Absolute
        ("LDY", (0xA0, 1)), // Immediate
        ("LDY", (0xA4, 1)), // Zero Page
        ("LDY", (0xAC, 2)), // Absolute
        ("STY", (0x84, 1)), // Zero Page
        ("STY", (0x8C, 2)), // Absolute
        // Transfer Instructions
        ("TAX", (0xAA, 0)), // Transfer A to X
        ("TXA", (0x8A, 0)), // Transfer X to A
        ("TAY", (0xA8, 0)), // Transfer A to Y
        ("TYA", (0x98, 0)), // Transfer Y to A
        // Stack Instructions
        ("TSX", (0xBA, 0)), // Transfer Stack Pointer to X
        ("TXS", (0x9A, 0)), // Transfer X to Stack Pointer
        ("PHA", (0x48, 0)), // Push A
        ("PLA", (0x68, 0)), // Pull A
        ("PHP", (0x08, 0)), // Push Processor Status
        ("PLP", (0x28, 0)), // Pull Processor Status
        // Arithmetic and Logic Instructions
        ("ADC", (0x69, 1)), // Immediate
        ("ADC", (0x65, 1)), // Zero Page
        ("ADC", (0x6D, 2)), // Absolute
        ("SBC", (0xE9, 1)), // Immediate
        ("SBC", (0xE5, 1)), // Zero Page
        ("SBC", (0xED, 2)), // Absolute
        ("CMP", (0xC9, 1)), // Immediate
        ("CMP", (0xC5, 1)), // Zero Page
        ("CMP", (0xCD, 2)), // Absolute
        ("CPX", (0xE0, 1)), // Immediate
        ("CPX", (0xE4, 1)), // Zero Page
        ("CPX", (0xEC, 2)), // Absolute
        ("CPY", (0xC0, 1)), // Immediate
        ("CPY", (0xC4, 1)), // Zero Page
        ("CPY", (0xCC, 2)), // Absolute
        // Increment and Decrement Instructions
        ("INC", (0xE6, 1)), // Zero Page
        ("INC", (0xEE, 2)), // Absolute
        ("INX", (0xE8, 0)), // Increment X
        ("INY", (0xC8, 0)), // Increment Y
        ("DEC", (0xC6, 1)), // Zero Page
        ("DEC", (0xCE, 2)), // Absolute
        ("DEX", (0xCA, 0)), // Decrement X
        ("DEY", (0x88, 0)), // Decrement Y
        // Shift and Rotate Instructions
        ("ASL", (0x0A, 0)), // Accumulator
        ("ASL", (0x06, 1)), // Zero Page
        ("ASL", (0x0E, 2)), // Absolute
        ("LSR", (0x4A, 0)), // Accumulator
        ("LSR", (0x46, 1)), // Zero Page
        ("LSR", (0x4E, 2)), // Absolute
        ("ROL", (0x2A, 0)), // Accumulator
        ("ROL", (0x26, 1)), // Zero Page
        ("ROL", (0x2E, 2)), // Absolute
        ("ROR", (0x6A, 0)), // Accumulator
        ("ROR", (0x66, 1)), // Zero Page
        ("ROR", (0x6E, 2)), // Absolute
        // Jump and Branch Instructions
        ("JMP", (0x4C, 2)), // Absolute
        ("JMP", (0x6C, 2)), // Indirect
        ("JSR", (0x20, 2)), // Jump to Subroutine
        ("RTS", (0x60, 0)), // Return from Subroutine
        ("RTI", (0x40, 0)), // Return from Interrupt
        ("BCC", (0x90, 1)), // Branch if Carry Clear
        ("BCS", (0xB0, 1)), // Branch if Carry Set
        ("BEQ", (0xF0, 1)), // Branch if Equal
        ("BMI", (0x30, 1)), // Branch if Minus
        ("BNE", (0xD0, 1)), // Branch if Not Equal
        ("BPL", (0x10, 1)), // Branch if Plus
        ("BVC", (0x50, 1)), // Branch if Overflow Clear
        ("BVS", (0x70, 1)), // Branch if Overflow Set
        // Flag Instructions
        ("CLC", (0x18, 0)), // Clear Carry
        ("SEC", (0x38, 0)), // Set Carry
        ("CLI", (0x58, 0)), // Clear Interrupt
        ("SEI", (0x78, 0)), // Set Interrupt
        ("CLV", (0xB8, 0)), // Clear Overflow
        ("CLD", (0xD8, 0)), // Clear Decimal
        ("SED", (0xF8, 0)), // Set Decimal
        // No Operation
        ("NOP", (0xEA, 0)), // No operation
    ]
    .iter()
    .cloned()
    .collect();

    let mut program = Vec::new();

    for line in contents.lines() {
        // Remove comments and trim whitespace
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue; // Skip empty lines
        }

        let mut parts = line.split_whitespace();
        if let Some(instruction) = parts.next() {
            if let Some(&(opcode, operand_size)) = opcode_table.get(instruction) {
                program.push(opcode); // Add the opcode to the program

                for operand in parts.take(operand_size) {
                    let operand = operand.trim_start_matches(&['#', '$'][..]);
                    let value = if operand_size == 1 {
                        u8::from_str_radix(operand, 16)
                            .expect(&format!("Invalid operand: {}", operand))
                            as u16
                    } else {
                        u16::from_str_radix(operand, 16)
                            .expect(&format!("Invalid operand: {}", operand))
                    };

                    // Push the operand bytes (little-endian for 16-bit)
                    if operand_size == 2 {
                        program.push((value & 0xFF) as u8); // Low byte
                        program.push((value >> 8) as u8); // High byte
                    } else {
                        program.push(value as u8); // Single byte
                    }
                }
            } else {
                panic!("Unknown instruction: {}", instruction);
            }
        }
    }

    Ok(program)
}

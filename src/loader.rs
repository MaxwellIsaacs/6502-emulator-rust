use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Accumulator,
    Indirect,
}

fn operand_size(mode: AddressingMode) -> usize {
    match mode {
        AddressingMode::Implied | AddressingMode::Accumulator => 0,
        AddressingMode::Immediate | AddressingMode::ZeroPage | AddressingMode::Relative => 1,
        AddressingMode::Absolute | AddressingMode::Indirect => 2,
    }
}

fn is_branch_instruction(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "BCC" | "BCS" | "BEQ" | "BMI" | "BNE" | "BPL" | "BVC" | "BVS"
    )
}

fn parse_operand(
    mnemonic: &str,
    operand: Option<&str>,
) -> Result<(AddressingMode, Vec<u8>), Box<dyn std::error::Error>> {
    match operand {
        None => Ok((AddressingMode::Implied, Vec::new())),
        Some(raw_operand) => {
            let operand = raw_operand.trim().trim_end_matches(',');

            if operand.is_empty() {
                return Ok((AddressingMode::Implied, Vec::new()));
            }

            if operand.starts_with('#') {
                let value_str = operand
                    .trim_start_matches('#')
                    .trim_start_matches('$')
                    .trim();
                let value = u8::from_str_radix(value_str, 16).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid immediate operand: {}", operand),
                    )
                })?;
                return Ok((AddressingMode::Immediate, vec![value]));
            }

            if operand.eq_ignore_ascii_case("A") {
                return Ok((AddressingMode::Accumulator, Vec::new()));
            }

            if operand.starts_with("($") && operand.ends_with(')') {
                let value_str = &operand[2..operand.len() - 1];
                let value = u16::from_str_radix(value_str, 16).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid indirect operand: {}", operand),
                    )
                })?;
                return Ok((
                    AddressingMode::Indirect,
                    vec![(value & 0xFF) as u8, (value >> 8) as u8],
                ));
            }

            if operand.starts_with('$') {
                let value_str = operand.trim_start_matches('$');
                let value = u16::from_str_radix(value_str, 16).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid operand: {}", operand),
                    )
                })?;

                if is_branch_instruction(mnemonic) {
                    return Ok((AddressingMode::Relative, vec![value as u8]));
                }

                if value_str.len() <= 2 {
                    return Ok((AddressingMode::ZeroPage, vec![value as u8]));
                }

                return Ok((
                    AddressingMode::Absolute,
                    vec![(value & 0xFF) as u8, (value >> 8) as u8],
                ));
            }

            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported operand format: {}", operand),
            )
            .into())
        }
    }
}

pub fn read_file(file_path: String) -> String {
    let path = Path::new(&file_path);

    if let Some(ext) = path.extension() {
        if ext == "asm" {
            println!("File is an ASM file");
        } else {
            panic!("File has wrong extension");
        }
    } else {
        println!("File has no extension");
    }

    let contents = fs::read_to_string(path).expect("Should have read the file, it exists");

    contents
}

pub fn assemble(file_path: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let opcode_entries = [
        // Load and Store Instructions
        ("LDA", AddressingMode::Immediate, 0xA9),
        ("LDA", AddressingMode::ZeroPage, 0xA5),
        ("LDA", AddressingMode::Absolute, 0xAD),
        ("STA", AddressingMode::ZeroPage, 0x85),
        ("STA", AddressingMode::Absolute, 0x8D),
        ("LDX", AddressingMode::Immediate, 0xA2),
        ("LDX", AddressingMode::ZeroPage, 0xA6),
        ("LDX", AddressingMode::Absolute, 0xAE),
        ("STX", AddressingMode::ZeroPage, 0x86),
        ("STX", AddressingMode::Absolute, 0x8E),
        ("LDY", AddressingMode::Immediate, 0xA0),
        ("LDY", AddressingMode::ZeroPage, 0xA4),
        ("LDY", AddressingMode::Absolute, 0xAC),
        ("STY", AddressingMode::ZeroPage, 0x84),
        ("STY", AddressingMode::Absolute, 0x8C),
        // Transfer Instructions
        ("TAX", AddressingMode::Implied, 0xAA),
        ("TXA", AddressingMode::Implied, 0x8A),
        ("TAY", AddressingMode::Implied, 0xA8),
        ("TYA", AddressingMode::Implied, 0x98),
        // Stack Instructions
        ("TSX", AddressingMode::Implied, 0xBA),
        ("TXS", AddressingMode::Implied, 0x9A),
        ("PHA", AddressingMode::Implied, 0x48),
        ("PLA", AddressingMode::Implied, 0x68),
        ("PHP", AddressingMode::Implied, 0x08),
        ("PLP", AddressingMode::Implied, 0x28),
        // Arithmetic and Logic Instructions
        ("ADC", AddressingMode::Immediate, 0x69),
        ("ADC", AddressingMode::ZeroPage, 0x65),
        ("ADC", AddressingMode::Absolute, 0x6D),
        ("SBC", AddressingMode::Immediate, 0xE9),
        ("SBC", AddressingMode::ZeroPage, 0xE5),
        ("SBC", AddressingMode::Absolute, 0xED),
        ("CMP", AddressingMode::Immediate, 0xC9),
        ("CMP", AddressingMode::ZeroPage, 0xC5),
        ("CMP", AddressingMode::Absolute, 0xCD),
        ("CPX", AddressingMode::Immediate, 0xE0),
        ("CPX", AddressingMode::ZeroPage, 0xE4),
        ("CPX", AddressingMode::Absolute, 0xEC),
        ("CPY", AddressingMode::Immediate, 0xC0),
        ("CPY", AddressingMode::ZeroPage, 0xC4),
        ("CPY", AddressingMode::Absolute, 0xCC),
        // Increment and Decrement Instructions
        ("INC", AddressingMode::ZeroPage, 0xE6),
        ("INC", AddressingMode::Absolute, 0xEE),
        ("INX", AddressingMode::Implied, 0xE8),
        ("INY", AddressingMode::Implied, 0xC8),
        ("DEC", AddressingMode::ZeroPage, 0xC6),
        ("DEC", AddressingMode::Absolute, 0xCE),
        ("DEX", AddressingMode::Implied, 0xCA),
        ("DEY", AddressingMode::Implied, 0x88),
        // Shift and Rotate Instructions
        ("ASL", AddressingMode::Accumulator, 0x0A),
        ("ASL", AddressingMode::ZeroPage, 0x06),
        ("ASL", AddressingMode::Absolute, 0x0E),
        ("LSR", AddressingMode::Accumulator, 0x4A),
        ("LSR", AddressingMode::ZeroPage, 0x46),
        ("LSR", AddressingMode::Absolute, 0x4E),
        ("ROL", AddressingMode::Accumulator, 0x2A),
        ("ROL", AddressingMode::ZeroPage, 0x26),
        ("ROL", AddressingMode::Absolute, 0x2E),
        ("ROR", AddressingMode::Accumulator, 0x6A),
        ("ROR", AddressingMode::ZeroPage, 0x66),
        ("ROR", AddressingMode::Absolute, 0x6E),
        // Jump and Branch Instructions
        ("JMP", AddressingMode::Absolute, 0x4C),
        ("JMP", AddressingMode::Indirect, 0x6C),
        ("JSR", AddressingMode::Absolute, 0x20),
        ("RTS", AddressingMode::Implied, 0x60),
        ("RTI", AddressingMode::Implied, 0x40),
        ("BCC", AddressingMode::Relative, 0x90),
        ("BCS", AddressingMode::Relative, 0xB0),
        ("BEQ", AddressingMode::Relative, 0xF0),
        ("BMI", AddressingMode::Relative, 0x30),
        ("BNE", AddressingMode::Relative, 0xD0),
        ("BPL", AddressingMode::Relative, 0x10),
        ("BVC", AddressingMode::Relative, 0x50),
        ("BVS", AddressingMode::Relative, 0x70),
        // Flag Instructions
        ("CLC", AddressingMode::Implied, 0x18),
        ("SEC", AddressingMode::Implied, 0x38),
        ("CLI", AddressingMode::Implied, 0x58),
        ("SEI", AddressingMode::Implied, 0x78),
        ("CLV", AddressingMode::Implied, 0xB8),
        ("CLD", AddressingMode::Implied, 0xD8),
        ("SED", AddressingMode::Implied, 0xF8),
        // No Operation
        ("NOP", AddressingMode::Implied, 0xEA),
    ];

    let mut opcode_table: HashMap<(String, AddressingMode), u8> = HashMap::new();
    for (mnemonic, mode, opcode) in opcode_entries.into_iter() {
        opcode_table.insert((mnemonic.to_string(), mode), opcode);
    }

    let mut program = Vec::new();
    let contents = read_file(file_path);

    for line in contents.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, |c: char| c.is_whitespace());
        if let Some(instruction) = parts.next() {
            let mnemonic = instruction.to_ascii_uppercase();
            let operand_str = parts.next().map(str::trim);
            let (addressing_mode, operand_bytes) = parse_operand(&mnemonic, operand_str)?;

            let expected_operand_size = operand_size(addressing_mode);
            if operand_bytes.len() != expected_operand_size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Operand width mismatch for {} {:?}: expected {} bytes, got {} bytes",
                        mnemonic,
                        addressing_mode,
                        expected_operand_size,
                        operand_bytes.len()
                    ),
                )
                .into());
            }

            if let Some(&opcode) = opcode_table.get(&(mnemonic.clone(), addressing_mode)) {
                program.push(opcode);
                program.extend_from_slice(&operand_bytes);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Unknown instruction or addressing mode: {} {:?}",
                        mnemonic, addressing_mode
                    ),
                )
                .into());
            }
        }
    }

    Ok(program)
}

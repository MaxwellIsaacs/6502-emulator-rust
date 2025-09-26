use std::env;
use std::io::{self, Write};
use std::path::Path;

use rust_6502_emulator::{load_program, Debugger, Memory, ProgramSource, CPU};

fn run_step_mode(cpu: &mut CPU, memory: &mut Memory) {
    loop {
        // Print a snapshot of the CPU state
        println!("--- CPU State ---");
        println!("PC:     {:#06X}", cpu.pc);
        println!("A:      {:#04X}", cpu.a);
        println!("X:      {:#04X}", cpu.x);
        println!("Y:      {:#04X}", cpu.y);
        println!("SP:     {:#04X}", cpu.sp);
        println!("Status: {:#04X}", cpu.status);
        println!("-----------------");

        // Prompt the user for a command
        print!("Enter command (n: next, s: stop, m <addr>: dump memory): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "n" {
            // Execute the next opcode
            cpu.execute(memory);
        } else if input == "s" {
            println!("Stopping execution.");
            cpu.pc = 0x2000;
            break;
        } else if input.starts_with("m") {
            // Expecting a command like "m -8F" or "m 8F"
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() < 2 {
                println!("Usage: m <address in hex>");
            } else {
                // Remove any '-' that might be present
                let hex_str = parts[1].trim_start_matches('-');
                match u16::from_str_radix(hex_str, 16) {
                    Ok(addr) => {
                        let value = memory.read(addr);
                        println!("Memory at {:#04X}: {:#04X}", addr, value);
                    }
                    Err(e) => println!("Invalid hex address: {}", e),
                }
            }
        } else {
            println!("Unknown command. Use 'n', 's', or 'm <address>'");
        }
    }
}

fn run_program(cpu: &mut CPU, memory: &mut Memory, _debugger: &mut Debugger) {
    println!("Starting program execution...\n");

    loop {
        // Execute the opcode
        run_step_mode(cpu, memory);

        // Break if PC reaches the end of the program
        if cpu.pc == 0x2000 {
            break;
        }
    }
}

fn main() {
    let mut cpu: CPU = CPU::new();
    let mut memory: Memory = Memory::new();
    let mut debugger: Debugger = Debugger::new();

    cpu.reset();

    let args: Vec<String> = env::args().collect();
    let program_arg = args.get(1).map(String::as_str).unwrap_or("src/example.asm");
    let program_path = Path::new(program_arg);

    if let Err(e) = load_program(&mut cpu, &mut memory, ProgramSource::Path(program_path)) {
        eprintln!("failed to load program {}", e);
        return;
    }

    run_program(&mut cpu, &mut memory, &mut debugger);
}

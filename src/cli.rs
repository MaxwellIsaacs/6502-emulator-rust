#![cfg(feature = "debugger")]

use crate::loader::assemble;
use crate::Emulator;
use std::error::Error;
use std::io::{self, Write};

pub fn run_debugger() -> Result<(), Box<dyn Error>> {
    let program_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("src/example.asm"));

    let mut emulator = Emulator::default();
    emulator.reset();

    let program = assemble(program_path)?;
    emulator.load_program(&program);

    interactive_loop(&mut emulator)?;
    Ok(())
}

fn interactive_loop(emulator: &mut Emulator) -> io::Result<()> {
    loop {
        let state = emulator.state();
        println!("--- CPU State ---");
        println!("PC:     {:#06X}", state.pc);
        println!("A:      {:#04X}", state.a);
        println!("X:      {:#04X}", state.x);
        println!("Y:      {:#04X}", state.y);
        println!("SP:     {:#04X}", state.sp);
        println!("Status: {:#04X}", state.status);
        println!("-----------------");

        print!("Enter command (n: next, s: stop, m <addr>: dump memory, c: continue): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "n" {
            emulator.tick(1);
        } else if input == "s" {
            println!("Stopping execution.");
            break;
        } else if input == "c" {
            emulator.run_until_break();
        } else if let Some(addr_str) = input
            .strip_prefix('m')
            .and_then(|s| s.trim().split_whitespace().next())
        {
            match u16::from_str_radix(addr_str.trim_start_matches('-'), 16) {
                Ok(addr) => {
                    let value = emulator.memory().read(addr);
                    println!("Memory at {:#04X}: {:#04X}", addr, value);
                }
                Err(e) => println!("Invalid hex address: {}", e),
            }
        } else {
            println!("Unknown command. Use 'n', 's', 'c', or 'm <address>'");
        }
    }

    Ok(())
}

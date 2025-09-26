pub mod cpu;
pub mod debugger;
pub mod loader;
pub mod memory;
pub mod op_code;

pub use cpu::CPU;
pub use debugger::Debugger;
pub use memory::Memory;

use std::path::{Path, PathBuf};

pub enum ProgramSource<'a> {
    Bytes(&'a [u8]),
    Path(&'a Path),
}

pub fn load_program(
    cpu: &mut CPU,
    memory: &mut Memory,
    source: ProgramSource,
) -> Result<(), Box<dyn std::error::Error>> {
    let program = match source {
        ProgramSource::Bytes(bytes) => bytes.to_vec(),
        ProgramSource::Path(path) => {
            let resolved_path: PathBuf = if path.is_relative() {
                Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
            } else {
                path.to_path_buf()
            };

            loader::assemble(&resolved_path)?
        }
    };

    let start_addr = 0x0600u16;
    for (i, &byte) in program.iter().enumerate() {
        memory.write(start_addr + i as u16, byte);
    }

    cpu.pc = start_addr;
    Ok(())
}

use std::borrow::Cow;

pub mod cpu;
pub mod debugger;
pub mod loader;
pub mod memory;
pub mod op_code;

pub use cpu::CPU;
pub use memory::Memory;

pub type ProgramSource<'a> = Cow<'a, [u8]>;

pub fn load_program<'a>(cpu: &mut CPU, memory: &mut Memory, source: ProgramSource<'a>) {
    const START_ADDR: u16 = 0x0600;

    for (offset, &byte) in source.iter().enumerate() {
        memory.write(START_ADDR + offset as u16, byte);
    }

    cpu.pc = START_ADDR;
}

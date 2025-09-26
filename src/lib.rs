pub mod cpu;
pub mod memory;
pub mod op_code;

#[cfg(feature = "debugger")]
pub mod debugger;

#[cfg(feature = "debugger")]
mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub mod loader;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod wasm;

pub use cpu::CPU;
pub use memory::Memory;

pub const DEFAULT_LOAD_ADDRESS: u16 = 0x0600;

#[derive(Debug, Clone, Copy)]
pub struct EmulatorState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,
}

pub struct Emulator {
    cpu: CPU,
    memory: Memory,
    load_address: u16,
}

impl Emulator {
    pub fn new() -> Self {
        let mut emulator = Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            load_address: DEFAULT_LOAD_ADDRESS,
        };
        emulator.reset();
        emulator
    }

    pub fn with_load_address(load_address: u16) -> Self {
        let mut emulator = Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            load_address,
        };
        emulator.reset();
        emulator
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.clear();
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let start = self.load_address as usize;
        let end = start
            .checked_add(program.len())
            .expect("program exceeds addressable memory");

        if end > self.memory.as_slice().len() {
            panic!("program exceeds addressable memory");
        }

        for (offset, byte) in program.iter().enumerate() {
            let address = self.load_address + offset as u16;
            self.memory.write(address, *byte);
        }

        self.cpu.pc = self.load_address;
    }

    pub fn tick(&mut self, cycles: usize) -> usize {
        let mut executed = 0;
        for _ in 0..cycles {
            self.cpu.execute(&mut self.memory);
            executed += 1;
        }
        executed
    }

    pub fn run_until_break(&mut self) -> usize {
        let mut executed = 0;
        loop {
            let opcode = self.memory.read(self.cpu.pc);
            self.cpu.execute(&mut self.memory);
            executed += 1;
            if opcode == 0x00 {
                break;
            }
        }
        executed
    }

    pub fn state(&self) -> EmulatorState {
        EmulatorState {
            a: self.cpu.a,
            x: self.cpu.x,
            y: self.cpu.y,
            sp: self.cpu.sp,
            pc: self.cpu.pc,
            status: self.cpu.status,
        }
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn load_address(&self) -> u16 {
        self.load_address
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "debugger")]
pub use cli::run_debugger;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm::WasmEmulator;

#![allow(dead_code)]

use crate::cpu::CPU;
use crate::op_code::OpCodeHandler;
use crate::op_code::OpcodeTable;
use crate::Memory;

// same as the CPU struct, just without opcodes. Used to save space on the stack. Min means minimal
pub struct MinCPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,
}

impl MinCPU {
    pub fn new() -> Self {
        MinCPU {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xff,
            pc: 0x0000,
            status: 0,
        }
    }

    pub fn copy_cpu(&mut self, cpu: &CPU) {
        self.a = cpu.a;
        self.x = cpu.x;
        self.y = cpu.y;
        self.sp = cpu.sp;
        self.pc = cpu.pc;
    }
}

pub struct CPUStack {
    elements: Vec<MinCPU>,
}

impl CPUStack {
    pub fn new() -> Self {
        CPUStack {
            elements: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<MinCPU> {
        self.elements.pop()
    }

    pub fn push(&mut self, x: MinCPU) {
        self.elements.push(x);
    }

    pub fn peek(&self) -> Option<&MinCPU> {
        self.elements.last()
    }

    pub fn size(self) -> usize {
        self.elements.len()
    }
}

pub struct Debugger {
    pub cpu_stack: CPUStack,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            cpu_stack: CPUStack::new(),
        }
    }

    pub fn print(self, temp: MinCPU) {
        println!(
            "PC: {:#X} | A: {:#X}, X: {:#X}, Y: {:#X}, SP: {:#X}, Status: {:#b}",
            temp.pc, temp.a, temp.x, temp.y, temp.sp, temp.status
        );
    }

    pub fn push(&mut self, cpu: &mut CPU) {
        let mut temp = MinCPU::new();
        temp.copy_cpu(&cpu);
        self.cpu_stack.push(temp);
    }
}

#![cfg(all(feature = "wasm", target_arch = "wasm32"))]

use wasm_bindgen::prelude::*;

use crate::Emulator;

#[wasm_bindgen]
pub struct WasmEmulator {
    inner: Emulator,
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmulator {
        WasmEmulator {
            inner: Emulator::new(),
        }
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.inner.load_program(program);
    }

    pub fn tick(&mut self, cycles: usize) -> usize {
        self.inner.tick(cycles)
    }

    pub fn run_until_break(&mut self) -> usize {
        self.inner.run_until_break()
    }

    pub fn memory_ptr(&mut self) -> *mut u8 {
        self.inner.memory_mut().as_mut_slice().as_mut_ptr()
    }

    pub fn memory_len(&self) -> usize {
        self.inner.memory().as_slice().len()
    }

    pub fn pc(&self) -> u16 {
        self.inner.state().pc
    }

    pub fn a(&self) -> u8 {
        self.inner.state().a
    }

    pub fn x(&self) -> u8 {
        self.inner.state().x
    }

    pub fn y(&self) -> u8 {
        self.inner.state().y
    }

    pub fn status(&self) -> u8 {
        self.inner.state().status
    }

    pub fn sp(&self) -> u8 {
        self.inner.state().sp
    }
}

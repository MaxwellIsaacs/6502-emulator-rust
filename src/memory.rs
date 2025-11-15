pub struct Memory {
    data: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {data: [0; 0x10000] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}

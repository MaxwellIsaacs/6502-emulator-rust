use crate::Memory;
use crate::op_code::OpCodeHandler;
use crate::op_code::OpcodeTable;
use crate::debugger::Debugger;


pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,
    pub debugger: Debugger,
    opcode_table: OpcodeTable,
}


pub const NEGATIVE_FLAG: u8 = 0b1000_0000; // Bit 7
pub const OVERFLOW_FLAG: u8 = 0b0100_0000; // Bit 6
pub const BREAK_FLAG: u8 = 0b0001_0000;    // Bit 4
pub const DECIMAL_FLAG: u8 = 0b0000_1000;  // Bit 3
pub const INTERRUPT_FLAG: u8 = 0b0000_0100; // Bit 2
pub const ZERO_FLAG: u8 = 0b0000_0010;     // Bit 1
pub const CARRY_FLAG: u8 = 0b0000_0001;    // Bit 0

// set and clear status flags
impl CPU {
    pub fn set_flag (&mut self, flag: u8, value: bool) {
        match flag {
            NEGATIVE_FLAG => {
                if value {
                    self.status |= NEGATIVE_FLAG;
                } else {
                    self.status &= !NEGATIVE_FLAG;
                }
            }
            OVERFLOW_FLAG => {
                if value {
                    self.status |= OVERFLOW_FLAG;
                } else {
                    self.status &= !OVERFLOW_FLAG;
                }
            }
            BREAK_FLAG => {
                if value {
                    self.status |= BREAK_FLAG;
                } else {
                    self.status &= !BREAK_FLAG;
                }
            }
            DECIMAL_FLAG => {
                if value {
                    self.status |= DECIMAL_FLAG;
                } else {
                    self.status &= !DECIMAL_FLAG;
                }
            }
            INTERRUPT_FLAG => {
                if value {
                    self.status |= INTERRUPT_FLAG;
                } else {
                    self.status &= !INTERRUPT_FLAG;
                }
            }
            ZERO_FLAG => {
                if value {
                    self.status |= ZERO_FLAG;
                } else {
                    self.status &= !ZERO_FLAG;
                }
            }
            CARRY_FLAG => {
                if value {
                    self.status |= CARRY_FLAG;
                } else {
                    self.status &= !CARRY_FLAG;
                }
            }
            _ => {
                panic!("Unknown fag: {:#X}", flag);
            }
        }
    }

    pub fn clear_flag (&mut self, flag: u8) {
        self.set_flag (flag, false);
    }

    pub fn grab_flag (self, flag: u8) -> bool {
        (self.status & flag) != 0
    }
}
/*
impl CPU {
    pub fn get_operand_addr (&mut self, )
}

*/

// constructor for cpu
impl CPU {
    pub fn new () -> Self {
        let opcode_table: OpcodeTable = OpcodeTable::new ();
        let debugger: Debugger = Debugger::new();
        CPU {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xff,
            pc: 0x0000,
            status: 0,
            debugger,
            opcode_table,
        }
    }

    pub fn reset (&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xff;
        self.pc = 0x0000;
        self.status = 0;
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        // Fetch the opcode from memory
        let op_code = memory.read(self.pc);

        // Retrieve the OpcodeEntry from the table
        let entry = self.opcode_table.table[op_code as usize].clone();

        if let Some(handler) = &entry.handler {
            // Execute the handler based on its type
            match handler {
                OpCodeHandler::WithMem(func) => func(self, memory),
                OpCodeHandler::NoMem(func) => func(self),
            }
        } else {
            panic!("Invalid opcode: [{:#X}] at address: [{:#X}]", op_code, self.pc);
        }
    }

}



/*
 * LOADING
 * <LDA, LDX, LDY>
 */

impl CPU {

    fn set_ld_flags (&mut self, reg: u8) {
        self.set_flag(ZERO_FLAG, reg == 0);
        self.set_flag(NEGATIVE_FLAG, (reg & 0b1000_0000) != 0);
    }

    // LDA
    pub fn lda_immediate(&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1);

        self.a = value;
        self.set_ld_flags (self.a);
        self.pc += 2;
    }


    pub fn lda_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);

        self.a = value;
        self.set_ld_flags (self.a);
        self.pc += 2;
    }

    pub fn lda_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = memory.read(addr);

        self.a = value;
        self.set_ld_flags (self.a);
        self.pc += 3;
    }

    // LDX
    pub fn ldx_immediate(&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1);

        self.x = value;
        self.set_ld_flags (self.x);

        self.pc += 2;
    }

    pub fn ldx_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);

        self.x = value;
        self.set_ld_flags (self.x);

        self.pc += 2;
    }

    pub fn ldx_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = memory.read(addr);

        self.x = value;
        self.set_ld_flags (self.x);

        self.pc += 3;
    }

    // LDY
    pub fn ldy_immediate(&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1);


        self.y = value;
        self.set_ld_flags (self.y);
        self.pc += 2;
    }

    pub fn ldy_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);

        self.y = value;
        self.set_ld_flags (self.y);
        self.pc += 2;
    }

    pub fn ldy_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = memory.read(addr);

        self.y = value;
        self.set_ld_flags (self.y);
        self.pc += 3;
    }
}

// * STORING:
//  * <STA, STX, STY>


impl CPU {
    fn st_(&mut self, addr: u16, memory: &mut Memory, value: u8) {
        memory.write(addr, value);
    }

    pub fn sta_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = self.a; // read the CPU register
        self.st_(addr, memory, value);

        self.pc += 2;
    }

    pub fn sta_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = self.a;
        self.st_(addr, memory, value);

        self.pc += 3;
    }

    pub fn stx_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = self.x;
        self.st_(addr, memory, value);
        self.pc += 2;
    }

    pub fn stx_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = self.x;
        self.st_(addr, memory, value);

        self.pc += 3;
    }

    pub fn sty_zero_page(&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = self.y;
        self.st_(addr, memory, value);

        self.pc += 2;
    }

    pub fn sty_absolute(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1) as u16;
        let hi = memory.read(self.pc + 2) as u16;
        let addr = (hi << 8) | lo;
        let value = self.y;
        self.st_(addr, memory, value);

        self.pc += 3;
    }
}

/*
 * TRANSFER:
 * <TAX, TAY, TSX, TXA, TXS, TYA>
*/

impl CPU {
    pub fn tax(&mut self) {
        self.x = self.a;
        self.set_flag(ZERO_FLAG, self.x == 0);
        self.set_flag(NEGATIVE_FLAG, self.x & 0b1000_0000 != 0);

        self.pc += 1;
    }

    pub fn tay(&mut self) {
        self.y = self.a;
        self.set_flag(ZERO_FLAG, self.y == 0);
        self.set_flag(NEGATIVE_FLAG, self.y & 0b1000_0000 != 0);

        self.pc += 1;
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.set_flag(ZERO_FLAG, self.x == 0);
        self.set_flag(NEGATIVE_FLAG, self.x & 0b1000_0000 != 0);

        self.pc += 1;
    }

    pub fn txa(&mut self) {
        self.a = self.x;
        self.set_flag(ZERO_FLAG, self.a == 0);
        self.set_flag(NEGATIVE_FLAG, self.a & 0b1000_0000 != 0);

        self.pc += 1;
    }

    pub fn txs(&mut self) {
        self.sp = self.x;

        self.pc += 1;
    }

    pub fn tya(&mut self) {
        self.a = self.y;
        self.set_flag(ZERO_FLAG, self.a == 0);
        self.set_flag(NEGATIVE_FLAG, self.a & 0b1000_0000 != 0);

        self.pc += 1;
    }
}


/*
 * BRANCHING
 * <JMP, JNE, JSR, RTS, RTI, BCC, BCS, BEQ, BMI, BPL, BVC, BVS>
*/

impl CPU {
    pub fn jmp_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);

        let addr = ((hi as u16) << 8) | lo as u16;
        self.pc = addr;
    }

    pub fn jmp_indirect (&mut self, memory: &mut Memory) {
        let pointer_hi = memory.read (self.pc + 1);
        let pointer_lo = memory.read (self.pc + 2);
        let pointer = ((pointer_hi as u16) << 8) | pointer_lo as u16;

        let target_hi = memory.read (pointer);
        let target_lo = memory.read ((pointer & 0xFF00) | (pointer + 1 & 0x00FF));
        let target = ((target_hi as u16) << 8) | target_lo as u16;

        self.pc = target;
    }

    pub fn jsr(&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let target = ((hi as u16) << 8) | lo as u16;

        let return_addr = self.pc + 2;

        memory.write(0x0100 + self.sp as u16, (return_addr >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);

        memory.write(0x0100 + self.sp as u16, (return_addr & 0xFF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.pc = target;
    }

    pub fn bcs (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0000_0001 != 0 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bcc (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0000_0001 == 0 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn beq (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0000_0010 == 1 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bmi (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b1000_0000 == 1 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bne (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0000_0010 == 0 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bpl (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b1000_0000 == 0 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bvc (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0100_0000 == 0 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }

    pub fn bvs (&mut self, memory: &mut Memory) {
        let value: i8 = memory.read(self.pc + 1) as i8;

        if self.status & 0b0100_0000 == 1 {
            let new_pc = (self.pc as i16) + (value as i16);
            self.pc = new_pc as u16;
        } else {
            self.pc += 2;
        }
    }
}


impl CPU {
    pub fn cmp_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr) as u16;
        let result = self.a.wrapping_sub(value as u8);
        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.a as u16) >= value);           // Carry flag if A >= value

        self.pc += 3;
   }

    pub fn cmp_immediate (&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1) as u16;
        let result = self.a.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.a as u16) >= value);           // Carry flag if A >= value

        self.pc += 3;
   }


    pub fn cmp_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        let result = self.a.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.a as u16) >= value);           // Carry flag if A >= value

        self.pc += 3;
   }

    pub fn cpy_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);
        let result = self.y.wrapping_sub(value);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.y as u16) >= value as u16); // Set carry flag if Y >= value

        self.pc += 3;
    }

    pub fn cpy_immediate (&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1);
        let result = self.y.wrapping_sub(value);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.y as u16) >= value as u16); // Set carry flag if Y >= value

        self.pc += 2;
    }


    pub fn cpy_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        let result = self.y.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.y as u16) >= value as u16); // Set carry flag if Y >= value

        self.pc += 3;
    }


    pub fn cpx_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);


        let result = self.x.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.x as u16) >= value as u16); // Set carry flag if Y >= value

        self.pc += 2;
    }

    pub fn cpx_immediate (&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc + 1);

        let result = self.x.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.x as u16) >= value as u16); // Set carry flag if Y >= value


        self.pc += 2;
    }


    pub fn cpx_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        let result = self.x.wrapping_sub(value as u8);

        self.set_flag(ZERO_FLAG, result == 0);                // Zero flag if A == value
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);   // Negative flag if result is negative
        self.set_flag(CARRY_FLAG, (self.x as u16) >= value as u16); // Set carry flag if Y >= value

        self.pc += 3;
    }
}

// CLEAR FLAGS:
// <CLC, CLD, CLI, CLV>

impl CPU {
   pub fn clc (&mut self) {
       self.clear_flag(CARRY_FLAG);
   }
    pub fn cld (&mut self) {
       self.clear_flag(DECIMAL_FLAG);
   }
    pub fn cli (&mut self) {
       self.clear_flag(INTERRUPT_FLAG);
   }
    pub fn cly (&mut self) {
       self.clear_flag(OVERFLOW_FLAG);
   }
}

//
//
impl CPU {

    // and

    pub fn and_immediate (&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc+1);
        self.a = self.a & value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
    }

    pub fn and_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);
        self.a = self.a & value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
    }

    pub fn and_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        self.a = self.a & (value as u8);

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 3;
    }

    // ora

    pub fn ora_immediate (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);
        self.a = self.a & value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
    }

    pub fn ora_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);
        self.a = self.a & value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
     }

    pub fn ora_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        self.a = self.a | (value as u8);

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 3;
    }

    // eor

    pub fn eor_immediate (&mut self, memory: &mut Memory) {
        let value = memory.read(self.pc+1);
        self.a = self.a ^ value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
    }

    pub fn eor_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);
        self.a = self.a ^ value;

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 2;
    }

    pub fn eor_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        self.a = self.a ^ (value as u8);

        self.set_flag (ZERO_FLAG, value == 0);
        self.set_flag (NEGATIVE_FLAG, (self.a & 0b1000_000) != 0);
        self.pc += 3;
     }

    // bit

    pub fn bit_zero_page (&mut self, memory: &mut Memory) {
        let addr = memory.read(self.pc + 1) as u16;
        let value = memory.read(addr);

        self.set_flag (ZERO_FLAG, value as u8 & self.a == 0);
        self.set_flag (NEGATIVE_FLAG, value & 0b1000_000 != 0);
        self.set_flag(OVERFLOW_FLAG, value & 0b0100_0000 != 0);

        self.pc += 2;
    }

    pub fn bit_absolute (&mut self, memory: &mut Memory) {
        let lo = memory.read(self.pc + 1);
        let hi = memory.read(self.pc + 2);
        let value = ((hi as u16) << 8) | lo as u16;

        self.set_flag (ZERO_FLAG, (value as u8 & self.a) == 0);
        self.set_flag (NEGATIVE_FLAG, (value as u8 & 0b1000_000) != 0);
        self.set_flag(OVERFLOW_FLAG, (value as u8) & 0b0100_0000 != 0);

        self.pc += 3;
    }
}

// STACK INSTRUCTIONS
// < PHP, PLP, >

impl CPU {

    const START_STACK: u16 = 0b0000_0001_0000_0000;  // 256 in decimal
    const END_STACK: u16 = 0b0000_0001_1111_1111;  // 511 in decimal

    fn push (&mut self, memory: &mut Memory, value: u8) {
        memory.write(Self::START_STACK + self.sp as u16, value);
        self.sp -= 1;
    }

    fn pop (&mut self, memory: &mut Memory) -> u8 {
        self.sp += 1;
        memory.read(Self::START_STACK + self.sp as u16 + 1)
    }

    pub fn php (&mut self, memory: &mut Memory) {
        self.push (memory, self.status);
    }

    pub fn plp (&mut self, memory: &mut Memory) {
        self.status = self.pop();
    }

    pub fn pha (&mut self, memory: &mut Memory) {
        self.push (memory, self.a);
    }

    pub fn pla (&mut self, memory: &mut Memory) {
        self.a = self.pop();
    }

    pub fn rts(&mut self, memory: &mut Memory) {
        let lo = self.pop(memory) as u16;
        let hi = self.pop(memory) as u16;
        self.pc = ((hi << 8) | lo) + 1;
    }

    pub fn rti(&mut self, memory: &mut Memory) {
        self.status = self.pop(memory);
        let lo = self.pop(memory) as u16;
        let hi = self.pop(memory) as u16;
        self.pc = (hi << 8) | lo;
    }
}

// ARTHIMATIC
// < ADC, SBC,

impl CPU {
    pub fn adc (&mut self, memory: &mut Memory)  {
        let value = memory.read (self.pc) as u16;
        let carry: u16 = if (self.status & CARRY_FLAG) != 0 {1} else {0} ;
        self.a = (value + carry) as u8;
        self.set_flag(ZERO_FLAG, result == 0);
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);
        let overflow = (!((self.a as u16) ^ (operand as u16)) & ((self.a as u16) ^ result as u16) & 0x80) != 0;
        self.set_flag(OVERFLOW_FLAG, overflow);
    }

     pub fn sbc (&mut self, memory: &mut Memory)  {
        let result = memory.read (self.pc) as u16;
        let carry: u16 = if (self.status & CARRY_FLAG) != 0 {1} else {0} ;
        self.a = (result - (1 - carry)) as u8;
        self.set_flag(ZERO_FLAG, result == 0);
        self.set_flag(NEGATIVE_FLAG, (result & 0x80) != 0);
        let overflow = (!((self.a as u16) ^ (operand as u16)) & ((self.a as u16) ^ result as u16) & 0x80) != 0;
        self.set_flag(OVERFLOW_FLAG, overflow);
    }


}

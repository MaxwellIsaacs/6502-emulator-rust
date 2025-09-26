use crate::{Memory, CPU};

/*
 *  Custom enum for opcodes, vary based on the parameters.
 *  @params cpu struct (constant), memory (sometimes), addr (sometimes)
 */

type OpCodeHandlerWithMem = fn(&mut CPU, &mut Memory);
type OpCodeHandlerNoMem = fn(&mut CPU);
type OpCodeHandlerWithAddr = fn(&mut CPU, &mut Memory, u8);

#[derive(Clone)]
pub enum OpCodeHandler {
    WithMem(OpCodeHandlerWithMem),
    NoMem(OpCodeHandlerNoMem),
    // WithAddr(OpCodeHandlerWithAddr)
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    Implied,
}

pub const OPCODE_DEFINITIONS: [(u8, OpCodeHandler); 64] = [
    // -- LDA --
    (0xA9, OpCodeHandler::WithMem(CPU::lda_immediate)), // Immediate
    (0xA5, OpCodeHandler::WithMem(CPU::lda_zero_page)), // Zero Page
    (0xAD, OpCodeHandler::WithMem(CPU::lda_absolute)),  // Absolute
    // -- STA --
    (0x85, OpCodeHandler::WithMem(CPU::sta_zero_page)), // Zero Page
    (0x8D, OpCodeHandler::WithMem(CPU::sta_absolute)),  // Absolute
    // -- LDX --
    (0xA2, OpCodeHandler::WithMem(CPU::ldx_immediate)), // Immediate
    (0xA6, OpCodeHandler::WithMem(CPU::ldx_zero_page)), // Zero Page
    (0xAE, OpCodeHandler::WithMem(CPU::ldx_absolute)),  // Absolute
    // -- STX --
    (0x86, OpCodeHandler::WithMem(CPU::stx_zero_page)), // Zero Page
    (0x8E, OpCodeHandler::WithMem(CPU::stx_absolute)),  // Absolute
    // -- LDY --
    (0xA0, OpCodeHandler::WithMem(CPU::ldy_immediate)), // Immediate
    (0xA4, OpCodeHandler::WithMem(CPU::ldy_zero_page)), // Zero Page
    (0xAC, OpCodeHandler::WithMem(CPU::ldy_absolute)),  // Absolute
    // -- STY --
    (0x84, OpCodeHandler::WithMem(CPU::sty_zero_page)), // Zero Page
    (0x8C, OpCodeHandler::WithMem(CPU::sty_absolute)),  // Absolute
    // -- Transfer Instructions --
    (0xAA, OpCodeHandler::NoMem(CPU::tax)), // Implied
    (0xA8, OpCodeHandler::NoMem(CPU::tay)), // Implied
    (0xBA, OpCodeHandler::NoMem(CPU::tsx)), // Implied
    (0x8A, OpCodeHandler::NoMem(CPU::txa)), // Implied
    (0x9A, OpCodeHandler::NoMem(CPU::txs)), // Implied
    (0x98, OpCodeHandler::NoMem(CPU::tya)), // Implied
    // -- Jump Instructions --
    (0x4C, OpCodeHandler::WithMem(CPU::jmp_absolute)), // Absolute
    (0x6C, OpCodeHandler::WithMem(CPU::jmp_indirect)), // Indirect
    (0x20, OpCodeHandler::WithMem(CPU::jsr)),          // Absolute
    // -- CMP --
    (0xCD, OpCodeHandler::WithMem(CPU::cmp_immediate)), // Immediate
    (0xC5, OpCodeHandler::WithMem(CPU::cmp_zero_page)), // Zero Page
    (0xC9, OpCodeHandler::WithMem(CPU::cmp_absolute)),  // Absolute
    // -- CPX --
    (0xE0, OpCodeHandler::WithMem(CPU::cpx_immediate)), // Immediate
    (0xE4, OpCodeHandler::WithMem(CPU::cpx_zero_page)), // Zero Page
    (0xEC, OpCodeHandler::WithMem(CPU::cpx_absolute)),  // Absolute
    // -- CPY --
    (0xC0, OpCodeHandler::WithMem(CPU::cpy_immediate)), // Immediate
    (0xC4, OpCodeHandler::WithMem(CPU::cpy_zero_page)), // Zero Page
    (0xCC, OpCodeHandler::WithMem(CPU::cpy_absolute)),  // Absolute
    // -- Clear Flag Instructions --
    (0x18, OpCodeHandler::NoMem(CPU::clc)), // Implied
    (0xD8, OpCodeHandler::NoMem(CPU::cld)), // Implied
    (0x58, OpCodeHandler::NoMem(CPU::cli)), // Implied
    (0xB8, OpCodeHandler::NoMem(CPU::cly)), // Implied
    // -- Bitwise Operations --
    // * AND *
    (0x29, OpCodeHandler::WithMem(CPU::and_immediate)), // Absolute
    (0x25, OpCodeHandler::WithMem(CPU::and_zero_page)), // Indirect
    (0x2D, OpCodeHandler::WithMem(CPU::and_absolute)),  // Absolute
    // * ORA *
    (0x09, OpCodeHandler::WithMem(CPU::ora_immediate)), // Absolute
    (0x05, OpCodeHandler::WithMem(CPU::ora_zero_page)), // Indirect
    (0x0D, OpCodeHandler::WithMem(CPU::ora_absolute)),  // Absolute
    // * EOR *
    (0x49, OpCodeHandler::WithMem(CPU::eor_immediate)), // Absolute
    (0x45, OpCodeHandler::WithMem(CPU::eor_zero_page)), // Indirect
    (0x4D, OpCodeHandler::WithMem(CPU::eor_absolute)),  // Absolute
    // * BIT *
    (0x24, OpCodeHandler::WithMem(CPU::bit_zero_page)), // Indirect
    (0x2C, OpCodeHandler::WithMem(CPU::bit_absolute)),  // Absolute
    // -- Branch Instructions --
    (0x90, OpCodeHandler::WithMem(CPU::bcc)), // Implied
    (0xB0, OpCodeHandler::WithMem(CPU::bcs)), // Implied
    (0xF0, OpCodeHandler::WithMem(CPU::beq)), // Implied
    (0x30, OpCodeHandler::WithMem(CPU::bmi)), // Implied
    (0xD0, OpCodeHandler::WithMem(CPU::bne)), // Implied
    (0x10, OpCodeHandler::WithMem(CPU::bpl)), // Implied
    (0x50, OpCodeHandler::WithMem(CPU::bvc)), // Implied
    (0x70, OpCodeHandler::WithMem(CPU::bvs)), // Implied
    // -- Stack Instructions --
    (0x08, OpCodeHandler::WithMem(CPU::php)), // PHP - Push Processor Status
    (0x28, OpCodeHandler::WithMem(CPU::plp)), // PLP - Pull Processor Status
    (0x48, OpCodeHandler::WithMem(CPU::pha)), // PHA - Push Accumulator
    (0x68, OpCodeHandler::WithMem(CPU::pla)), // PLA - Pull Accumulator
    (0x60, OpCodeHandler::WithMem(CPU::rts)), // RTS - Return from Subroutine
    (0x40, OpCodeHandler::WithMem(CPU::rti)), // RTI - Return from Interrupt
    // -- ADC (Add with Carry) --
    (0x69, OpCodeHandler::WithMem(CPU::adc)), // Immediate
    // (0x65, OpCodeHandler::WithMem(CPU::adc)), // Zero Page
    // (0x75, OpCodeHandler::WithMem(CPU::adc)), // Zero Page,X
    // (0x6D, OpCodeHandler::WithMem(CPU::adc)), // Absolute
    // (0x7D, OpCodeHandler::WithMem(CPU::adc)), // Absolute,X
    // (0x79, OpCodeHandler::WithMem(CPU::adc)), // Absolute,Y
    // (0x61, OpCodeHandler::WithMem(CPU::adc)), // (Indirect,X)
    // (0x71, OpCodeHandler::WithMem(CPU::adc)), // (Indirect),Y

    // -- SBC (Subtract with Carry) --
    (0xE9, OpCodeHandler::WithMem(CPU::sbc)), // Immediate
                                              //(0xE5, OpCodeHandler::WithMem(CPU::sbc)), // // Zero Page
                                              // (0xF5, OpCodeHandler::WithMem(CPU::sbc)), // Zero Page,X
                                              // (0xED, OpCodeHandler::WithMem(CPU::sbc)), // Absolute
                                              // (0xFD, OpCodeHandler::WithMem(CPU::sbc)), // Absolute,X
                                              // (0xF9, OpCodeHandler::WithMem(CPU::sbc)), // Absolute,Y
                                              // (0xE1, OpCodeHandler::WithMem(CPU::sbc)), // (Indirect,X)
                                              // (0xF1, OpCodeHandler::WithMem(CPU::sbc)), // (Indirect),
];

#[derive(Clone, Default)]
pub struct OpcodeEntry {
    pub handler: Option<OpCodeHandler>,
}

pub struct OpcodeTable {
    pub table: [OpcodeEntry; 256],
}

impl OpcodeTable {
    pub fn new() -> Self {
        let mut table = core::array::from_fn(|_| OpcodeEntry::default());

        for (op_code, handler) in OPCODE_DEFINITIONS.iter() {
            table[*op_code as usize] = OpcodeEntry {
                handler: Some((*handler).clone()),
            };
        }
        OpcodeTable { table }
    }
}

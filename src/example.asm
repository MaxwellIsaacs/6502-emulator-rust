
; Simple 6502 program using opcodes from loader.rs

    LDA #$32  ; Load 0x42 into A (Immediate addressing)
    STA $10   ; Store A at memory location $10 (Zero-page)

    JMP $0600

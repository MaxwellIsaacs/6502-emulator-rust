use rust_6502_emulator::loader::assemble;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn write_asm(contents: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("assembler_test_{}.asm", timestamp));
    fs::write(&path, contents).expect("Failed to write assembly file");
    path
}

#[test]
fn assemble_handles_lda_addressing_modes() {
    let path = write_asm("LDA #$01\nLDA $10\nLDA $1234\n");

    let program = assemble(path.to_string_lossy().into_owned()).expect("Failed to assemble");
    fs::remove_file(&path).ok();

    assert_eq!(program, vec![0xA9, 0x01, 0xA5, 0x10, 0xAD, 0x34, 0x12]);
}

#[test]
fn assemble_handles_sta_addressing_modes() {
    let path = write_asm("STA $20\nSTA $5678\n");

    let program = assemble(path.to_string_lossy().into_owned()).expect("Failed to assemble");
    fs::remove_file(&path).ok();

    assert_eq!(program, vec![0x85, 0x20, 0x8D, 0x78, 0x56]);
}

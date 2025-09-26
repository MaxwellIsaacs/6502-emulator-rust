use std::fs;
use std::path::PathBuf;
use std::process;

use rust_6502_emulator::{load_program, loader, Memory, ProgramSource, CPU};

#[test]
fn loads_program_bytes_and_paths() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp_dir = manifest_dir
        .join("tests")
        .join(format!("tmp_program_loading_{}", process::id()));
    fs::create_dir_all(&temp_dir).expect("create temp dir");
    let asm_path = temp_dir.join("snippet.asm");
    let asm_source = "    LDA #$01\n    STA $0200\n    JMP $0600\n";
    fs::write(&asm_path, asm_source).expect("write asm file");

    let assembled = loader::assemble(&asm_path).expect("assemble snippet");

    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    cpu.reset();

    load_program(&mut cpu, &mut memory, ProgramSource::Bytes(&assembled)).expect("load from bytes");

    assert_eq!(cpu.pc, 0x0600);
    let loaded_bytes: Vec<u8> = (0..assembled.len())
        .map(|offset| memory.read(0x0600 + offset as u16))
        .collect();
    assert_eq!(loaded_bytes, assembled);

    let mut cpu_from_path = CPU::new();
    let mut memory_from_path = Memory::new();
    cpu_from_path.reset();

    let relative_path = asm_path.strip_prefix(&manifest_dir).unwrap().to_path_buf();

    load_program(
        &mut cpu_from_path,
        &mut memory_from_path,
        ProgramSource::Path(relative_path.as_path()),
    )
    .expect("load from relative path");

    assert_eq!(cpu_from_path.pc, 0x0600);
    let loaded_from_path: Vec<u8> = (0..assembled.len())
        .map(|offset| memory_from_path.read(0x0600 + offset as u16))
        .collect();
    assert_eq!(loaded_from_path, assembled);

    fs::remove_file(&asm_path).ok();
    fs::remove_dir(&temp_dir).ok();
}

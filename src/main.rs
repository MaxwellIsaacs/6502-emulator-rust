#[cfg(feature = "debugger")]
fn main() {
    if let Err(error) = rust_6502_emulator::run_debugger() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

#[cfg(not(feature = "debugger"))]
fn main() {
    eprintln!("The interactive CLI is disabled. Rebuild with `--features debugger` to enable it.");
}

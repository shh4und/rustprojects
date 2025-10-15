mod opcodes;
mod utils;
mod chip8;
use std::env;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Use: cargo run -- <file.ch8>");

        return Ok(());
    }

    let filepath = &args[1];
    let rom =
        fs::read(filepath)
            .map_err(|e| format!("Error at handling file {}: {}", filepath, e))?;
    println!("Read ROM at: {} ({} bytes)", filepath, rom.len());

    let mut cpu = chip8::Chip8::new();
    cpu.load_rom(&rom)
        .map_err(|e| format!("Error at loading rom into Chip8 memory: {}", e))?;


    let max_cycles = 500;
    let mut cycle_count = 0;

    loop {
        cpu.cycle();
        cycle_count += 1;
        println!("#{} cycle", cycle_count);
        
        if cycle_count >= max_cycles{
            println!("\nReached {} cycles, stopping...", max_cycles);
            break;
        }
    }

    Ok(())
}

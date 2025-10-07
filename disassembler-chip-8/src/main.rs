mod disassembler;
mod utils;
use std::env;
use std::error::Error;
use std::fs;
// use std::io;

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
    println!("Disassembling {} ({} bytes)", filepath, rom.len());

    disassembler::run(&rom);

    Ok(())
}

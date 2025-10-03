mod utils;
mod disassembler;
use std::env;
use std::fs;
// use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();    
    if args.len() < 2 {
        eprintln!("Use: cargo run -- <file.ch8>");
        
        return;
    }

    let filepath = &args[1];
    let rom = fs::read(filepath).expect("Error at handling file {filepath}");
    println!("Disassembling {} ({} bytes)", filepath, rom.len());

    disassembler::run(rom);

}

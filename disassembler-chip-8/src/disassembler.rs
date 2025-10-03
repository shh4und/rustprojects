use std::u16;

use crate::utils;

pub fn run(rom: Vec<u8>) {
    let start_addr = 0x200usize;
    for (idx, chunk) in rom.chunks_exact(2).enumerate() {
        let opcode = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        let addr = start_addr + idx * 2;

        let instr = decode(opcode);

        println!("{:03X}: {:04X} {}", addr, opcode, instr);
    }

    // checks if rom has reminder bytes <=> rom.len() is odd
    let rem = rom.chunks_exact(2).remainder();
    if !rem.is_empty() {
        eprintln!("Warning: ROM has reminder byte: {:?}", rem);
    }
}

fn decode(opcode: u16) -> String {
    match opcode & 0xF000 {
        // 0nnn instrs
        0x0000 => match opcode {
            0x00E0 => "CLS".to_string(),
            0x00EE => "RET".to_string(),
            _ => format!("SYS ${:03X}", utils::nnn(opcode)),
        },
        // 1nnn: JP addr (Jump to location nnn)
        0x1000 => format!("JMP ${:03X}", utils::nnn(opcode)),
        // 2nnn: CALL addr (Call subroutine at nnn)
        0x2000 => format!("CALL ${:03X}", utils::nnn(opcode)),
        // 3xkk: SE Vx, byte (Skip next instruction if Vx = kk)
        0x3000 => format!("SE V{:01X}, {:02X}", utils::x(opcode), utils::kk(opcode)),  
        // 4xkk: SNE Vx, byte (Skip next instruction if Vx != kk)
        0x4000 => format!("SNE V{:01X}, {:02X}", utils::x(opcode), utils::kk(opcode)),  
        // 5xy0: SE Vx, Vy (Skip next instruction if Vx = Vy)
        0x5000 => format!("SE V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),  
        // 6xkk: LD Vx, byte (Set Vx = kk)
        0x6000 => format!("LD V{:01X}, {:02X}", utils::x(opcode), utils::kk(opcode)),  
        // 7xkk: ADD Vx, byte (Set Vx = Vx + kk)
        0x7000 => format!("ADD V{:01X}, {:02X}", utils::x(opcode), utils::kk(opcode)),  
        // 8xyn instrs
        0x8000 => match opcode & 0x800F {
            0x8000 => format!("LD V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8001 => format!("OR V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8002 => format!("AND V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8003 => format!("XOR V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8004 => format!("ADD V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8005 => format!("SUB V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x8006 => format!("SHR V{:01X},", utils::x(opcode)),
            0x8007 => format!("SUBN V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            0x800E => format!("SHL V{:01X} {{ ,V{:01X}}}", utils::x(opcode), utils::y(opcode)),

            _ => format!("0x8???"),
        },
        0x9000 => match opcode & 0x900F{
            0x9000 => format!("SNE V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
            _ => format!("0x9???"),

        }
        _ => format!("???"),
    }
}

use std::u16;

use crate::utils;

pub fn run(rom: &[u8]) {

    // checks if rom has reminder bytes <=> rom.len() is odd
    let rem = rom.chunks_exact(2).remainder();
    if !rem.is_empty() {
        eprintln!("Warning: ROM has reminder byte: {:?}", rem);
    }
    
    let start_addr = 0x200usize;
    println!("0xaddr: 0xopcode instr");
    for (idx, chunk) in rom.chunks_exact(2).enumerate() {
        let opcode = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        let addr = start_addr + idx * 2;

        let instr = decode(opcode);

        println!("0x{:04X}: 0x{:04X} {}", addr, opcode, instr);
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
        // 9xy0: SNE Vx, Vy (Skip next instruction if Vx != Vy)
        0x9000 =>  format!("SNE V{:01X}, V{:01X}", utils::x(opcode), utils::y(opcode)),
        // Annn: LD I, addr (Set I = nnn)
        0xA000 => format!("LD I, ${:03X}", utils::nnn(opcode)),
        // Bnnn: JP V0, addr (Jump to location nnn + V0)
        0xB000 => format!("JMP V0, ${:03X}", utils::nnn(opcode)),
        // Cxkk: RND Vx, byte (Set Vx = random byte AND kk)
        0xC000 => format!("RND V{:01X}, {:02X}", utils::x(opcode), utils::kk(opcode)),
        // Dxyn: DRW Vx, Vy, nibble (Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision)
        0xD000 => format!("DRW V{:01X}, V{:01X}, {:01X}", utils::x(opcode), utils::y(opcode), utils::k(opcode)),
        
        0xE000 => match opcode & 0xE0FF {
            // Ex9E: SKP Vx (Skip next instruction if key with the value of Vx is pressed)
            0xE09E => format!("SKP V{:01X}", utils::x(opcode)),
            // Skip next instruction if key with the value of Vx is not pressed
            0xE0A1 => format!("SKNP V{:01X}", utils::x(opcode)),
            _ => format!("0xEX??"),
        },
        
        0xF000 => match opcode & 0xF0FF {
            // Fx07: LD Vx, DT (Set Vx = delay timer value)
            0xF007 => format!("LD, V{:01X} DT", utils::x(opcode)),
            // Fx0A - LD Vx, K (Wait for a key press, store the value of the key in Vx)
            0xF00A => format!("LD V{:01X}, K", utils::x(opcode)),
            // Fx15 - LD DT, Vx (Set delay timer = Vx)
            0xF015 => format!("LD DT, V{:01X}", utils::x(opcode)),
            // Fx18 - LD ST, Vx (Set sound timer = Vx)
            0xF018 => format!("LD ST, V{:01X}", utils::x(opcode)),
            // Fx1E - ADD I, Vx (Set I = I + Vx)
            0xF01E => format!("ADD I, V{:01X}", utils::x(opcode)),
            // Fx29 - LD F, Vx (Set I = location of sprite for digit Vx)
            0xF029 => format!("LD F, V{:01X}", utils::x(opcode)),
            // Fx33 - LD B, Vx (Store BCD representation of Vx in memory locations I, I+1, and I+2)
            0xF033 => format!("LD B, V{:01X}", utils::x(opcode)),
            // Fx55 - LD [I], Vx (Store registers V0 through Vx in memory starting at location I)
            0xF055 => format!("LD [I], V{:01X}", utils::x(opcode)),
            // Fx65 - LD Vx, [I] (Read registers V0 through Vx from memory starting at location I)
            0xF065 => format!("LD V{:01X}, [I]", utils::x(opcode)),
            _ => format!("0xF???"),
        },
        _ => format!("???"),
    }
}

// use crate::utils;
// use std::fmt;

pub enum Instruction {
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEVxImm { x: u8, imm: u8 },
    SNEVxImm { x: u8, imm: u8 },
    SEVxVy { x: u8, y: u8 },
    LDVxImm { x: u8, imm: u8 },
    ADDVxImm { x: u8, imm: u8 },

    Unknown(u16),
}





// pub fn run(rom: &[u8]) {
//     let start_addr = 0x200usize;
//     for (idx, chunk) in rom.chunks_exact(2).enumerate() {
//         let opcode = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
//         let addr = start_addr + idx * 2;
//         let instr = decode(opcode);
//         println!("{:03X}: {:04X}  {}", addr, opcode, instr);
//     }

//     let rem = rom.chunks_exact(2).remainder();
//     if !rem.is_empty() {
//         eprintln!("Warning: ROM has remainder bytes: {:?}", rem);
//     }
// }

// // Opcional: enum Instruction para facilitar evolução
// #[derive(Debug)]


// impl fmt::Display for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Instruction::CLS => write!(f, "CLS"),
//             Instruction::RET => write!(f, "RET"),
//             Instruction::JP(addr) => write!(f, "JP {:03X}", addr),
//             Instruction::LDVxImm { x, imm } => write!(f, "LD V{:X}, {:02X}", x, imm),
//             Instruction::Unknown(op) => write!(f, "???: {:04X}", op),
//         }
//     }
// }

fn decode(opcode: u16) -> Instruction {
    match (opcode & 0xF000) >> 12{
        0x0 => match opcode {
            0x00E0 => Instruction::CLS,
            0x00EE => Instruction::RET,
            _ => Instruction::Unknown(opcode),
        },
        0x1 => Instruction::JP(nnn(opcode)),
        0x2 => Instruction::CALL(nnn(opcode)),
        0x3 => Instruction::SEVxImm { x: x(opcode), imm: kk(opcode) },
        0x4 => Instruction::SNEVxImm { x: x(opcode), imm: kk(opcode) },
        0x5 => Instruction::SEVxVy { x: x(opcode), y: y(opcode) },
        0x6 => Instruction::LDVxImm { x: x(opcode), imm: kk(opcode) },
        0x7 => Instruction::ADDVxImm { x: x(opcode), imm: kk(opcode) },
        _ => Instruction::Unknown(opcode),
    }
}

/// Retorna os 12 bits menos significativos (NNN).
pub fn nnn(opcode: u16) -> u16 { opcode & 0x0FFF }

/// Retorna o byte menos significativo (KK).
pub fn kk(opcode: u16) -> u8 { (opcode & 0x00FF) as u8 }

/// Retorna nibble menos significativo (N).
pub fn n(opcode: u16) -> u8 { (opcode & 0x000F) as u8 }

/// Retorna o nibble X (bits 8..11).
pub fn x(opcode: u16) -> u8 { ((opcode & 0x0F00) >> 8) as u8 }

/// Retorna o nibble Y (bits 4..7).
pub fn y(opcode: u16) -> u8 { ((opcode & 0x00F0) >> 4) as u8 }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_extract() {
//         let op = 0x6A3C;
//         assert_eq!(x(op), 0xA);
//         assert_eq!(kk(op), 0x3C);
//         assert_eq!(nnn(op), 0xA3C);
//         assert_eq!(k(op), 0xC);
//     }
// }

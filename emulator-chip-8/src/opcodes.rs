// use crate::utils;
// use std::fmt;

pub enum Instruction {
    CLS,
    RET,
    SYS(u16),
    JP(u16),
    CALL(u16),
    SEVxImm { x: u8, imm: u8 },
    SNEVxImm { x: u8, imm: u8 },
    SEVxVy { x: u8, y: u8 },
    LDVxImm { x: u8, imm: u8 },
    ADDVxImm { x: u8, imm: u8 },
    LDVxVy { x: u8, y: u8 },
    ORVxVy { x: u8, y: u8 },
    ANDVxVy { x: u8, y: u8 },
    XORVxVy { x: u8, y: u8 },
    ADDVxVy { x: u8, y: u8 },
    SUBVxVy { x: u8, y: u8 },
    SHRVxVy { x: u8, y: u8 },
    SUBNVxVy { x: u8, y: u8 },
    SHLVxVy { x: u8, y: u8 },
    SNEVxVy { x: u8, y: u8 }, // SNE Vx, Vy (Skip next instruction if Vx != Vy)
    LDI(u16), // LD I, addr (Set I = nnn)
    JPV0(u16), // JP V0, addr (Jump to location nnn + V0)
    RNDVxImm { x: u8, imm: u8},
    DRWVxVyn {x: u8, y: u8, n: u8}, // DRW Vx, Vy, nibble (Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision)
    DRWVxVy0 { x: u8, y: u8 },
    SKPVx(u8),
    SKNPVx(u8),
    LDVxDT(u8),
    LDVxK(u8),
    LDDTVx(u8),
    LDSTVx(u8),
    ADDIVx(u8),
    LDFVx(u8),
    LDBVx(u8),
    LDIVx(u8),
    LDVxI(u8),
    LDHFVx(u8),
    LDRV(u8),
    LDVxR(u8),
    SCD(u8),
    SCR,
    SCL,
    EXIT,
    LOW,
    HIGH,

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

pub fn decode(opcode: u16) -> Instruction {
    match (opcode & 0xF000) >> 12 {
        0x0 => match opcode & 0x00FF {
            0xE0 => Instruction::CLS,
            0xEE => Instruction::RET,
            0xFB => Instruction::SCR,
            0xFC => Instruction::SCL,
            0xFD => Instruction::EXIT,
            0xFE => Instruction::LOW,
            0xFF => Instruction::HIGH,
            0xC0..=0xCF => Instruction::SCD(n(opcode)),
            _ => Instruction::SYS(nnn(opcode)),
        },
        0x1 => Instruction::JP(nnn(opcode)),
        0x2 => Instruction::CALL(nnn(opcode)),
        0x3 => Instruction::SEVxImm { x: x(opcode), imm: kk(opcode) },
        0x4 => Instruction::SNEVxImm { x: x(opcode), imm: kk(opcode) },
        0x5 => match opcode & 0x000F {
            0x0 => Instruction::SEVxVy { x: x(opcode), y: y(opcode) },
            _ => Instruction::Unknown(opcode),
        },
        0x6 => Instruction::LDVxImm { x: x(opcode), imm: kk(opcode) },
        0x7 => Instruction::ADDVxImm { x: x(opcode), imm: kk(opcode) },
        0x8 => match opcode & 0x000F {
            0x0 => Instruction::LDVxVy { x: x(opcode), y: y(opcode) },
            0x1 => Instruction::ORVxVy { x: x(opcode), y: y(opcode) },
            0x2 => Instruction::ANDVxVy { x: x(opcode), y: y(opcode) },
            0x3 => Instruction::XORVxVy { x: x(opcode), y: y(opcode) },
            0x4 => Instruction::ADDVxVy { x: x(opcode), y: y(opcode) },
            0x5 => Instruction::SUBVxVy { x: x(opcode), y: y(opcode) },
            0x6 => Instruction::SHRVxVy { x: x(opcode), y: y(opcode) },
            0x7 => Instruction::SUBNVxVy { x: x(opcode), y: y(opcode) },
            0xE => Instruction::SHLVxVy { x: x(opcode), y: y(opcode) },
            _ => Instruction::Unknown(opcode),

        },
        0x9 => match opcode & 0x000F {
            0x0 => Instruction::SNEVxVy { x: x(opcode), y: y(opcode) },
            _ => Instruction::Unknown(opcode),
        },
        0xA => Instruction::LDI(nnn(opcode)),
        0xB => Instruction::JPV0(nnn(opcode)),
        0xC => Instruction::RNDVxImm { x: x(opcode), imm: kk(opcode) },
        0xD => match n(opcode) {
            0 => Instruction::DRWVxVy0 { x: x(opcode), y: y(opcode) },
            nibble => Instruction::DRWVxVyn { x: x(opcode), y: y(opcode), n: nibble },
        },
        0xE => match opcode & 0x00FF {
            0x9E => Instruction::SKPVx(x(opcode)),
            0xA1 => Instruction::SKNPVx(x(opcode)),
            _ => Instruction::Unknown(opcode),
        },
        0xF => match opcode & 0x00FF {
            0x07 => Instruction::LDVxDT(x(opcode)),
            0x0A => Instruction::LDVxK(x(opcode)),
            0x15 => Instruction::LDDTVx(x(opcode)),
            0x18 => Instruction::LDSTVx(x(opcode)),
            0x1E => Instruction::ADDIVx(x(opcode)),
            0x29 => Instruction::LDFVx(x(opcode)),
            0x30 => Instruction::LDHFVx(x(opcode)),
            0x33 => Instruction::LDBVx(x(opcode)),
            0x55 => Instruction::LDIVx(x(opcode)),
            0x65 => Instruction::LDVxI(x(opcode)),
            0x75 => Instruction::LDRV(x(opcode)),
            0x85 => Instruction::LDVxR(x(opcode)),
            _ => Instruction::Unknown(opcode),
        },
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

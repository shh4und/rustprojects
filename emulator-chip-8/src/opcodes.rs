// use crate::utils;
// use std::fmt;

pub enum Instruction {
    /// Clear the display.
    CLS, 
    /// Return from a subroutine.
    RET, 
    /// Jump to a machine code routine at nnn. (Ignored by modern interpreters.)
    SYS(u16), 
    /// Jump to location nnn.
    JP(u16), 
    /// Call subroutine at nnn.
    CALL(u16), 
    /// Skip next instruction if Vx == kk.
    SEVxImm { x: u8, imm: u8 }, 
    /// Skip next instruction if Vx != kk.
    SNEVxImm { x: u8, imm: u8 }, 
    /// Skip next instruction if Vx == Vy.
    SEVxVy { x: u8, y: u8 }, 
    /// Set Vx = kk.
    LDVxImm { x: u8, imm: u8 }, 
    /// Set Vx = Vx + kk.
    ADDVxImm { x: u8, imm: u8 }, 
    /// Set Vx = Vy.
    LDVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx | Vy (bitwise OR).
    ORVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx & Vy (bitwise AND).
    ANDVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx ^ Vy (bitwise XOR).
    XORVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx + Vy, set VF = carry.
    ADDVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    SUBVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx >> 1, set VF = LSB.
    SHRVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    SUBNVxVy { x: u8, y: u8 }, 
    /// Set Vx = Vx << 1, set VF = MSB.
    SHLVxVy { x: u8, y: u8 }, 
    /// Skip next instruction if Vx != Vy.
    SNEVxVy { x: u8, y: u8 }, 
    /// Set I = nnn.
    LDI(u16), 
    /// Jump to location nnn + V0.
    JPV0(u16), 
    /// Set Vx = random byte & kk.
    RNDVxImm { x: u8, imm: u8}, 
    /// Display n-byte sprite at (Vx, Vy), set VF = collision.
    DRWVxVyn {x: u8, y: u8, n: u8}, 
    /// Display 16x16 sprite at (Vx, Vy), set VF = collision. (Super Chip-48)
    DRWVxVy0 { x: u8, y: u8 }, 
    /// Skip next instruction if key Vx is pressed.
    SKPVx(u8), 
    /// Skip next instruction if key Vx is not pressed.
    SKNPVx(u8), 
    /// Set Vx = delay timer value.
    LDVxDT(u8), 
    /// Wait for key press, store in Vx.
    LDVxK(u8), 
    /// Set delay timer = Vx.
    LDDTVx(u8), 
    /// Set sound timer = Vx.
    LDSTVx(u8), 
    /// Set I = I + Vx.
    ADDIVx(u8), 
    /// Set I = location of sprite for digit Vx.
    LDFVx(u8), 
    /// Store BCD of Vx in memory at I, I+1, I+2.
    LDBVx(u8), 
    /// Store registers V0 through Vx in memory starting at I.
    LDIVx(u8), 
    /// Read registers V0 through Vx from memory starting at I.
    LDVxI(u8), 
    /// Set I = location of high-res sprite for digit Vx. (Super Chip-48)
    LDHFVx(u8), 
    /// Store Vx in RPL user flags. (Super Chip-48)
    LDRV(u8), 
    /// Read Vx from RPL user flags. (Super Chip-48)
    LDVxR(u8), 
    /// Scroll display down by n lines. (Super Chip-48)
    SCD(u8), 
    /// Scroll display right by 4 pixels. (Super Chip-48)
    SCR, 
    /// Scroll display left by 4 pixels. (Super Chip-48)
    SCL, 
    /// Exit the interpreter. (Super Chip-48)
    EXIT, 
    /// Set display to low resolution (64x32). (Super Chip-48)
    LOW, 
    /// Set display to high resolution (128x64). (Super Chip-48)
    HIGH, 
    /// Unknown opcode.
    Unknown(u16), 
}

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

/// Returns the 12 least significant bits (NNN).
fn nnn(opcode: u16) -> u16 { opcode & 0x0FFF }

/// Returns the least significant byte (KK).
fn kk(opcode: u16) -> u8 { (opcode & 0x00FF) as u8 }

/// Returns the least significant nibble (N).
fn n(opcode: u16) -> u8 { (opcode & 0x000F) as u8 }

/// Returns the X nibble (bits 8..11).
fn x(opcode: u16) -> u8 { ((opcode & 0x0F00) >> 8) as u8 }

/// Returns the Y nibble (bits 4..7).
fn y(opcode: u16) -> u8 { ((opcode & 0x00F0) >> 4) as u8 }


pub fn nnn(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

pub fn x(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

pub fn y(opcode: u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

pub fn kk(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}

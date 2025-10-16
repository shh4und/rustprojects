use crate::opcodes::{Instruction, decode};
use rand::Rng;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const ROM_START_ADDR: usize = 0x200;
const FONT_START_ADDR: usize = 0x000;

/// The built-in font set for hexadecimal digits 0-F (5 bytes per digit).
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// The Chip-8 emulator core, containing all state including RAM, registers, stack, keypad, and display.
pub struct Chip8 {
    /// 4KB (4,096 bytes) of RAM
    ram: [u8; RAM_SIZE],

    /// 16 8-bit general purpose registers, V0 through VF (VF is used for carry/borrow flags)
    reg_v: [u8; 16],

    /// Special register I, usually used to store memory addresses
    reg_i: u16,

    /// Delay timer register
    reg_dt: u8,

    /// Sound timer register
    reg_st: u8,

    /// Program counter register
    reg_pc: u16,

    /// Stack pointer register
    reg_sp: u8,

    /// Stack for subroutine calls (16 levels)
    stack: [u16; STACK_SIZE],

    /// Keypad state (16 keys)
    keypad: [u8; 16],

    /// Display buffer (64x32 pixels)
    display: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Chip8 {
    /// Creates a new Chip8 instance with initialized font set in RAM.
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            ram: [0; RAM_SIZE],
            reg_v: [0; 16],
            reg_i: 0,
            reg_dt: 0,
            reg_st: 0,
            reg_pc: 0,
            reg_sp: 0,
            stack: [0; STACK_SIZE],
            keypad: [0; 16],
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        };

        cpu.ram[FONT_START_ADDR..FONT_SET.len()].copy_from_slice(&FONT_SET);

        return cpu;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn get_display(&self) -> &[u32] {
        &self.display
    }

    /// Loads a ROM into RAM starting at address 0x200. Returns an error if the ROM is too large.
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str> {
        let end = ROM_START_ADDR + rom.len();
        if end > self.ram.len() {
            return Err("ROM longer than available memory");
        }

        self.ram[ROM_START_ADDR..end].copy_from_slice(rom);

        self.reg_pc = ROM_START_ADDR as u16;

        //println!("ROM loaded into RAM.\nPC initialized: {:x}",self.reg_pc);
        Ok(())
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();

        self.execute(opcode);
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.reg_pc as usize;
        let high_byte = self.ram[pc] as u16;
        let low_byte = self.ram[pc + 1] as u16;
        let opcode = (high_byte << 8) | low_byte;

        self.reg_pc += 2;

        opcode
    }

    fn execute(&mut self, opcode: u16) {
        let instr = decode(opcode);

        match instr {
            Instruction::CLS => self.op_cls(),
            Instruction::RET => self.op_ret(),
            Instruction::SYS(addr) => self.op_sys(addr),
            Instruction::JP(addr) => self.op_jp(addr),
            Instruction::CALL(addr) => self.op_call(addr),
            Instruction::SEVxImm { x, imm } => self.op_sevx_imm(x, imm),
            Instruction::SNEVxImm { x, imm } => self.op_snevx_imm(x, imm),
            Instruction::SEVxVy { x, y } => self.op_sevx_vy(x, y),
            Instruction::LDVxImm { x, imm } => self.op_ldvx_imm(x, imm),
            Instruction::ADDVxImm { x, imm } => self.op_addvx_imm(x, imm),
            Instruction::LDVxVy { x, y } => self.op_ldvx_vy(x, y),
            Instruction::ORVxVy { x, y } => self.op_orvx_vy(x, y),
            Instruction::ANDVxVy { x, y } => self.op_andvx_vy(x, y),
            Instruction::XORVxVy { x, y } => self.op_xorvx_vy(x, y),
            Instruction::ADDVxVy { x, y } => self.op_addvx_vy(x, y),
            Instruction::SUBVxVy { x, y } => self.op_subvx_vy(x, y),
            Instruction::SHRVxVy { x, y } => self.op_shrvx(x),
            Instruction::SUBNVxVy { x, y } => self.op_subnvx_vy(x, y),
            Instruction::SHLVxVy { x, y } => self.op_shlvx(x),
            Instruction::SNEVxVy { x, y } => self.op_snevx_vy(x, y),
            Instruction::LDI(addr) => self.op_ldi_addr(addr),
            Instruction::JPV0(addr) => self.op_jpv0_addr(addr),
            Instruction::RNDVxImm { x, imm} => self.op_rndvx_imm(x, imm),
            Instruction::DRWVxVyn {x, y, n} => ,
            Instruction::DRWVxVy0 { x, y } => ,
            Instruction::SKPVx(addr) => ,
            Instruction::SKNPVx(addr) => ,
            Instruction::LDVxDT(addr) => ,
            Instruction::LDVxK(addr) => ,
            Instruction::LDDTVx(addr) => ,
            Instruction::LDSTVx(addr) => ,
            Instruction::ADDIVx(addr) => ,
            Instruction::LDFVx(addr) => ,
            Instruction::LDBVx(addr) => ,
            Instruction::LDIVx(addr) => ,
            Instruction::LDVxI(addr) => ,
            Instruction::LDHFVx(addr) => ,
            Instruction::LDRV(addr) => ,
            Instruction::LDVxR(addr) => ,
            Instruction::SCD(addr) => ,
            Instruction::SCR => ,
            Instruction::SCL => ,
            Instruction::EXIT => ,
            Instruction::LOW => ,
            Instruction::HIGH => ,


            _ => self.op_unknown(),
        }
    }

    fn op_cls(&mut self) {
        self.display.fill(0);
    }

    fn op_ret(&mut self) {
        self.reg_sp -= 1;
        self.reg_pc = self.stack[self.reg_sp as usize]
    }

    fn op_sys(&mut self, addr: u16) {
        self.reg_pc = addr; // TO CHECK IF SYS IS IGNORED, IF SO, PC+=2
    }

    fn op_jp(&mut self, addr: u16) {
        self.reg_pc = addr;
    }

    fn op_call(&mut self, addr: u16) {
        self.stack[self.reg_sp as usize] = self.reg_pc;
        self.reg_sp += 1;
        self.reg_pc = addr;
    }

    fn op_sevx_imm(&mut self, x: u8, imm: u8) {
        if self.reg_v[x as usize] == imm {
            self.reg_pc += 2;
        }
    }

    fn op_snevx_imm(&mut self, x: u8, imm: u8) {
        if self.reg_v[x as usize] != imm {
            self.reg_pc += 2;
        }
    }

    fn op_sevx_vy(&mut self, x: u8, y: u8) {
        if self.reg_v[x as usize] == self.reg_v[y as usize] {
            self.reg_pc += 2;
        }
    }

    fn op_ldvx_imm(&mut self, x: u8, imm: u8) {
        self.reg_v[x as usize] = imm;
    }

    fn op_addvx_imm(&mut self, x: u8, imm: u8) {
        self.reg_v[x as usize] += imm;
    }

    fn op_ldvx_vy(&mut self, x: u8, y: u8) {
        self.reg_v[x as usize] = self.reg_v[y as usize];
    }

    fn op_orvx_vy(&mut self, x: u8, y: u8) {
        self.reg_v[x as usize] =  self.reg_v[x as usize] | self.reg_v[y as usize];
    }

    fn op_andvx_vy(&mut self, x: u8, y: u8) {
        self.reg_v[x as usize] =  self.reg_v[x as usize] & self.reg_v[y as usize];
    }

    fn op_xorvx_vy(&mut self, x: u8, y: u8) {
        self.reg_v[x as usize] =  self.reg_v[x as usize] ^ self.reg_v[y as usize];
    }

    fn op_addvx_vy(&mut self, x: u8, y: u8) {
        let sum: u8 = self.reg_v[x as usize] + self.reg_v[y as usize];
        if sum > 0xFF{
            self.reg_v[0xF] = 1;
        }else {
            self.reg_v[0xF] = 0;
        }

        self.reg_v[x as usize] = sum;
    }

    fn op_subvx_vy(&mut self, x: u8, y: u8) {
        if self.reg_v[x as usize] > self.reg_v[y as usize]{
            self.reg_v[0xF] = 1;
        }else {
            self.reg_v[0xF] = 0;
        }
        
        self.reg_v[x as usize] -= self.reg_v[y as usize];
    }

    fn op_shrvx(&mut self, x: u8) {
        self.reg_v[0xF] = self.reg_v[x as usize] & 0x1;

        self.reg_v[x as usize] >>= 1;
    }

    fn op_subnvx_vy(&mut self, x: u8, y: u8) {
        if self.reg_v[y as usize] > self.reg_v[x as usize]{
            self.reg_v[0xF] = 1;
        }else {
            self.reg_v[0xF] = 0;
        }
        
        self.reg_v[x as usize] = self.reg_v[y as usize] - self.reg_v[x as usize];
    }

    fn op_shlvx(&mut self, x: u8) {
        self.reg_v[0xF] = (self.reg_v[x as usize] & 0x80) >> 7;

        self.reg_v[x as usize] <<= 1;
    }

    fn op_snevx_vy(&mut self, x: u8, y: u8) {
        if self.reg_v[x as usize] != self.reg_v[y as usize] {
            self.reg_pc += 2;
        }
        println!("SNE V{:01X}, V{:01X}", x, y);
    }

    fn op_ldi_addr(&mut self, addr: u16) {
        self.reg_i = addr;
    }

    fn op_jpv0_addr(&mut self, addr: u16) {
        self.reg_pc = (self.reg_v[0x0] as u16) + addr;
    }

    fn op_rndvx_imm(&mut self, x: u8, imm: u8) {
        let mut rng = rand::rng();
        let randByte: u8 = rng.random_range(0x0..0xFF);

        self.reg_v[x as usize] = randByte & imm;
        
    }

    fn op_unknown(&mut self) {
        eprintln!("Unknown or not decoded instruction");
    }
}

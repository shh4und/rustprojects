use crate::opcodes::{decode, Instruction};

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

    pub fn cycle(&mut self){
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

    fn execute(&mut self, opcode: u16){
        let instr = decode(opcode);

        match instr {
            Instruction::CLS => self.op_cls(),
            Instruction::RET => self.op_ret(),
            Instruction::JP(addr) => self.op_jp(addr),  
            _ => self.reset(),
        }
    }

    fn op_cls(&mut self) {
        self.display.fill(0);
    }

    fn op_ret(&mut self){
        self.reg_sp -= 1;
        self.reg_pc = self.stack[self.reg_sp as usize]
    }

    fn op_jp(&mut self, addr: u16){
        self.reg_pc = addr;
    }
    
}

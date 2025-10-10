use crate::opcodes;

/// The Chip-8 emulator core, containing all state including RAM, registers, stack, keypad, and display.
pub struct Chip8 {
    /// 4KB (4,096 bytes) of RAM
    pub ram: [u8; 4096],

    /// 16 8-bit general purpose registers, V0 through VF (VF is used for carry/borrow flags)
    pub reg_v: [u8; 16],

    /// Special register I, usually used to store memory addresses
    pub reg_i: u16,

    /// Delay timer register
    pub reg_dt: u8,

    /// Sound timer register
    pub reg_st: u8,

    /// Program counter register
    pub reg_pc: u16,

    /// Stack pointer register
    pub reg_sp: u8,

    /// Stack for subroutine calls (16 levels)
    pub stack: [u16; 16],

    /// Keypad state (16 keys)
    pub keypad: [u8; 16],

    /// Display buffer (64x32 pixels)
    pub display: [u32; 64 * 32],
}

impl Chip8 {
    /// Creates a new Chip8 instance with initialized font set in RAM.
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            ram: [0; 4096],
            reg_v: [0; 16],
            reg_i: 0,
            reg_dt: 0,
            reg_st: 0,
            reg_pc: 0x200,
            reg_sp: 0,
            stack: [0; 16],
            keypad: [0; 16],
            display: [0; 64 * 32],
        };

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

        cpu.ram[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        return cpu;
    }

    /// Loads a ROM into RAM starting at address 0x200. Returns an error if the ROM is too large.
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str> {
        const START_ADDR: usize = 0x200;
        let end = START_ADDR + rom.len();
        if end > self.ram.len() {
            return Err("ROM longer than available memory");
        }

        self.ram[START_ADDR..end].copy_from_slice(rom);

        self.reg_pc = START_ADDR as u16;
        
        //println!("ROM loaded into RAM.\nPC initialized: {:x}",self.reg_pc);
        Ok(())
    }
}
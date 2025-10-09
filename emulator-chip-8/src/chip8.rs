use crate::opcodes;

pub struct Chip8 {
    pub ram: [u8; 4096], // 4KB (4,096 bytes) of RAM

    pub v: [u8; 16], // 16 8-bits general porpuse registers, V0 through VF (VF is a special case though)
    pub reg_i: u16,  // special register I, usually used to store memory address
    pub reg_delay_timer: u8,
    pub reg_sound_timer: u8,
    pub reg_pc: u16,
    pub reg_sp: u8,

    pub stack: [u16; 16],

    pub keypad: [u8; 16],

    pub display: [u32; 64 * 32],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            ram: [0; 4096],
            v: [0x000; 16],
            reg_i: 0,
            reg_delay_timer: 0,
            reg_sound_timer: 0,
            reg_pc: 0x200,
            reg_sp: 0,
            stack: [0; 16],
            keypad: [0; 16],
            display: [0; 64 * 32],
        };

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

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str>{
        // checks if rom has reminder bytes <=> rom.len() is odd
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

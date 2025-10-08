


pub struct Chip8{
    pub RAM: [u8;4096], // 4KB (4,096 bytes) of RAM

    pub v: [u8;16], // 16 8-bits general porpuse registers, V0 through VF (VF is a special case though)
    pub reg_I: u16, // special register I, usually used to store memory address
    pub reg_delay_timer: u8,
    pub reg_sound_timer: u8,
    pub reg_PC: u16,
    pub reg_SP: u8,

    pub stack: [u16; 16],

    pub keyboard: [u8; 16],

    //pub display: []

}
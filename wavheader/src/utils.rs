use std::fs;
use std::io;

pub fn read_file_bytes(filepath: &str) -> io::Result<Vec<u8>> {
    fs::read(filepath)
}

pub struct WAVHeader {
    pub chunk_id: u32,
    pub chunk_size: u32,
    pub format: u32,
    pub sub_chunk_1_id: u32,
    pub sub_chunk_1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub sub_chunk_2_id: u32,
    pub sub_chunk_2_size: u32,
}

impl WAVHeader {
    pub fn new(
        chunk_id: u32,
        chunk_size: u32,
        format: u32,
        sub_chunk_1_id: u32,
        sub_chunk_1_size: u32,
        audio_format: u16,
        num_channels: u16,
        sample_rate: u32,
        byte_rate: u32,
        block_align: u16,
        bits_per_sample: u16,
        sub_chunk_2_id: u32,
        sub_chunk_2_size: u32,
    ) -> Self {
        WAVHeader {
            chunk_id,
            chunk_size,
            format,
            sub_chunk_1_id,
            sub_chunk_1_size,
            audio_format,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            sub_chunk_2_id,
            sub_chunk_2_size,
        }
    }
    
}

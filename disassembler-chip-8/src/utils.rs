use std::fs;
use std::io;

pub fn read_file_bytes(filepath: &str) -> io::Result<Vec<u8>> {
    fs::read(filepath)
}



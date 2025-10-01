mod utils;
use std::convert::TryInto;

fn main() {
    let filepath = "../samples/BAK.wav";
    let mut header: Vec<u8> = Vec::with_capacity(44);
    match utils::read_file_bytes(filepath) {
        Ok(bytes) => {
            println!("Successfully read {} bytes from {}", bytes.len(), filepath);
            header = bytes[..44].to_vec();
        }
        Err(e) => {
            eprintln!("Error at handling file: {}", e)
        }
    }

    let wav_header: utils::WAVHeader = utils::WAVHeader::new(
        u32::from_be_bytes(header[0..4].try_into().unwrap()),
        u32::from_le_bytes(header[4..8].try_into().unwrap()),
        u32::from_be_bytes(header[8..12].try_into().unwrap()),
        u32::from_be_bytes(header[12..16].try_into().unwrap()),
        u32::from_le_bytes(header[16..20].try_into().unwrap()),
        u16::from_le_bytes(header[20..22].try_into().unwrap()),
        u16::from_le_bytes(header[22..24].try_into().unwrap()),
        u32::from_le_bytes(header[24..28].try_into().unwrap()),
        u32::from_le_bytes(header[28..32].try_into().unwrap()),
        u16::from_le_bytes(header[32..34].try_into().unwrap()),
        u16::from_le_bytes(header[34..36].try_into().unwrap()),
        u32::from_be_bytes(header[36..40].try_into().unwrap()),
        u32::from_le_bytes(header[40..44].try_into().unwrap()),
    );

    let chunk_id_str = String::from_utf8(wav_header.chunk_id.to_be_bytes().to_vec()).unwrap_or_else(|_| "Invalid".to_string());
    let format_str = String::from_utf8(wav_header.format.to_be_bytes().to_vec()).unwrap_or_else(|_| "Invalid".to_string());

    println!(
        ".WAV Header extracted {{\n\tChunkID: {}\n\tChunkSize: {}\n\tFormat: {}\n\tAudioFormat: {}\n\tNumChannels: {}\n\tSampleRate: {}\n\tByteRate: {}\n\tBitsPerSample: {}\n}}",
        chunk_id_str,
        wav_header.chunk_size.to_string(),
        format_str,
        wav_header.audio_format.to_string(),
        wav_header.num_channels.to_string(),
        wav_header.sample_rate.to_string(),
        wav_header.byte_rate.to_string(),
        wav_header.bits_per_sample.to_string()
    );
}

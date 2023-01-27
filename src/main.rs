use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[path = "../src/aes.rs"]
mod aes;

#[path = "../src/white_box.rs"]
mod white_box;

fn enc_buf(buf: &[u8], enc_type: &str, key: &[u8; 16]) -> Result<[u8; 16], & 'static  str> {
    let mut buffer = [0u8; 16];
    for i in 0..buf.len() {
        buffer[i] = buf[i];
    }

    match enc_type {
        "aes" => Ok(crate::aes::encryption_block(key, &buffer)),
        "white_box" => Ok(crate::white_box::encryption_block(&buffer)),
        _ => Err("Unkown encryption type use aes or white_box "),
    }
}

fn read_file_buffer_enc(
    filepath: &str,
    filepathenc: &str,
    enc_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 16;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;
    let mut file_enc = File::create(&filepathenc)?;

    loop {
        let read_count = file.read(&mut buffer)?;

        let buf = enc_buf(&buffer[..read_count], enc_type , &crate::white_box::key)?;
        file_enc.write_all(&buf)?;
        if read_count != BUFFER_LEN {
            break;
        }   
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let enc_type = &args[1];
    let file_path = &args[2];
    let file_path_enc: String = String::from(file_path);
    file_path_enc.push_str(".enc");
    let path = Path::new(file_path);
    let path_enc = Path::new(&file_path_enc);


    match read_file_buffer_enc(&file_path, &file_path_enc, enc_type) {
        Ok(_) => {},
        Err(a) => {}, 
    };
}

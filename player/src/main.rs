use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{Parser , ValueEnum};


mod api;
use aes_lib ; 

#[derive(ValueEnum, Clone , Debug)]
enum Mode {
    white_box,
    classic
    
}
#[derive(ValueEnum, Clone , Debug)]
enum Action {
    encrypt,
    decrypt
    
}


#[derive(Parser, Debug)]
#[clap(author="aldu", version="1.0", about="White box AES-128 encryption implementation from Chow scheme", long_about = "White box AES-128 encryption implementation from Chow scheme. This is a univeristy project soo watch out to bugss :p")]
struct Args {
   /// Mode of the operation 
   #[arg(short,long,required(true), value_enum)]
   mode: Mode,
   /// Action to perform   
   #[arg(short, long, required(true), value_enum)]
   action: Action,

    /// file to operate
   #[arg(short, long , required(true))]
   file: String,

   
}


fn dec_buf(buf: &[u8], key: &[u8; 16]) -> Result<[u8; 16], & 'static  str> {
    let mut buffer = [0u8; 16];
    for i in 0..buf.len() {
        buffer[i] = buf[i];
    }

     Ok(aes_lib::decryption_block(key ,&buffer))
    
}

fn enc_buf(buf: &[u8], enc_type: &Mode, key: &[u8; 16]) -> Result<[u8; 16], & 'static  str> {
    let mut buffer = [0u8; 16];
    for i in 0..buf.len() {
        buffer[i] = buf[i];
    }

    match enc_type {
        Mode::classic => Ok(aes_lib::encryption_block(key, &buffer)),
        Mode::white_box => Ok(api::encryption_block(&buffer)),
        _ => Err("Unkown encryption type use aes or white_box "),
    }
}

fn read_file_buffer_enc(
    filepath: &str,
    filepathenc: &str,
    enc_type: Mode,
) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 16;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;
    let mut file_enc = File::create(&filepathenc)?;

    loop {
        let read_count = file.read(&mut buffer)?;

        let buf = enc_buf(&buffer[..read_count], &enc_type , &crate::api::key)?;
        file_enc.write_all(&buf)?;
        if read_count != BUFFER_LEN {
            break;
        }   
    }
    Ok(())
}



fn read_file_buffer_dec(
    filepath: &str,
    filepathdec: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 16;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;
    let mut file_dec = File::create(&filepathdec)?;

    loop {
        let read_count = file.read(&mut buffer)?;

        let buf = dec_buf(&buffer[..read_count] , &crate::api::key)?;
        file_dec.write_all(&buf)?;
        if read_count != BUFFER_LEN {
            break;
        }   
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    

    let action = args.action;
    let enc_type = args.mode;
    let file_path = args.file;

    let mut file_path_enc: String = String::from(&file_path);
    file_path_enc.push_str(".enc");
    let mut file_path_dec: String = String::from(&file_path);
    file_path_dec.push_str(".dec");
    

    match action {
        Action::encrypt => {read_file_buffer_enc(&file_path, &file_path_enc, enc_type).unwrap() },
        Action::decrypt => {read_file_buffer_dec(&file_path, &file_path_dec).unwrap()},
        _ => {panic!("Unkown action type use enc or dec ") },
    }

}

#[cfg(test)]
mod test{
    use crate::api::{key ,encryption_block } ;
    use aes_lib::{decryption_block};

    #[test]
    fn test_encryption_block_white_box() {
        
        let message: [u8; 16] = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
            0x2e,
        ];
    
        let encr = encryption_block( &message);
        let decrypt = decryption_block(&key, &encr);
    
        assert!(
            message.iter().zip(decrypt.iter()).all(|(a, b)| a == b),
            "Arrays are not equal"
        );
    }

}
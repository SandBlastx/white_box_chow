
use std::{env, fs, path::Path};


use white_box_lib::{mixing_bijection_l , mixing_bijection_m ,mixing_bijection_m_l , tylm_tables , xor_tables,t_boxes } ; 

fn main(){


    // generate tables : 
    let key: [u8; 16] = [
        0x47, 0x47, 0xf0, 0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e,
        0x3f,
    ];
    let mut tl_boxes = t_boxes(&key);


    let l_box = mixing_bijection_l();
    let m_box = mixing_bijection_m();
    let ml_box = mixing_bijection_m_l(&m_box, &l_box);

    let tylm_boxes = tylm_tables(&mut tl_boxes, &l_box, &m_box);

    let xor_tables = xor_tables();



    let mut key_string = String::from("pub static key:[u8; 16 ] = ");
    let mut tylm_string = String::from("static tylm_boxes : [[[[[u8; 4]; 256]; 4]; 4]; 9] = ");
    let mut tl_string = String::from("static tl_boxes : [[[u8; 256]; 16]; 10] = ");
    let mut ml_string = String::from("static ml_box : [[[[[u8; 4]; 4]; 256]; 4]; 9] = ");
    let mut xor_ml_string = String::from("static xor_ml_tables : [[[[u8; 16]; 16]; 96]; 9] = ");
    let mut xor_string = String::from("static xor_table : [[[[u8; 16]; 16]; 96]; 9] = ");



    
   
    key_string.push_str(&format!("{:?}", key));
    key_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("keys.rs");
    fs::write(&dest_path, key_string).unwrap();


    tl_string.push_str(&format!("{:?}", tl_boxes));
    tl_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tl_boxes.rs");
    fs::write(&dest_path, tl_string).unwrap();


    tylm_string.push_str(&format!("{:?}", tylm_boxes));
    tylm_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tylm_boxes.rs");
    fs::write(&dest_path, tylm_string).unwrap();



    ml_string.push_str(&format!("{:?}", ml_box));
    ml_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ml_box.rs");
    fs::write(&dest_path, ml_string).unwrap();



    xor_string.push_str(&format!("{:?}", xor_tables));
    xor_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("xor_table.rs");
    fs::write(&dest_path, xor_string).unwrap();


    xor_ml_string.push_str(&format!("{:?}", xor_tables));
    xor_ml_string.push_str(";\r\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("xor_ml_tables.rs");
    fs::write(&dest_path, xor_ml_string).unwrap();



}








// static mut tl_boxes  : [[[u8; 256]; 16]; 10]= t_boxes(&key);


// const l_box :[[[[[u8; 8]; 8]; 16]; 9]; 2] = mixing_bijection_l();
// static m_box  : [[[[[u8; 32]; 32]; 4]; 9]; 2] = mixing_bijection_m();
// static ml_box  : [[[[[u8; 4]; 4]; 256]; 4]; 9] = mixing_bijection_m_l(&m_box, &l_box);

// static tylm_boxes  : [[[[[u8; 4]; 256]; 4]; 4]; 9] = tylm_tables(&mut tl_boxes, &l_box, &m_box);

// static xor_ml_tables  :[[[[u8; 16]; 16]; 96]; 9]  = xor_tables();
// static xor_table : [[[[u8; 16]; 16]; 96]; 9] = xor_tables();


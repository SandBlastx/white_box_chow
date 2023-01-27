

#[path = "../src/white_box.rs"]
mod white_box;

fn main(){


    // generate tables : 
    let mut tl_boxes = white_box::t_boxes(&key);


    let l_box = mixing_bijection_l();
    let m_box = mixing_bijection_m();
    let ml_box = mixing_bijection_m_l(&m_box, &l_box);

    let tylm_boxes = tylm_tables(&mut tl_boxes, &l_box, &m_box);

    let xor_ml_tables = xor_tables();
    let xor_tables = xor_tables();
}




// primarily used for writing the file
use std::{env, fs, path::Path};



static mut tl_boxes  : [[[u8; 256]; 16]; 10]= t_boxes(&key);


const l_box :[[[[[u8; 8]; 8]; 16]; 9]; 2] = mixing_bijection_l();
static m_box  : [[[[[u8; 32]; 32]; 4]; 9]; 2] = mixing_bijection_m();
static ml_box  : [[[[[u8; 4]; 4]; 256]; 4]; 9] = mixing_bijection_m_l(&m_box, &l_box);

static tylm_boxes  : [[[[[u8; 4]; 256]; 4]; 4]; 9] = tylm_tables(&mut tl_boxes, &l_box, &m_box);

static xor_ml_tables  :[[[[u8; 16]; 16]; 96]; 9]  = xor_tables();
static xor_table : [[[[u8; 16]; 16]; 96]; 9] = xor_tables();

fn main() {



    // I need a way to get the dimension during compile time.
    // Here, for simplicity reasons, I hard-coded it to be 2.
    // See below, how to get the dimension during compile time.
    let dimension: usize = 2;

    // creating a string with the alpha values
    let mut array_string = String::from("static ALPHAS:[f64; ");
    array_string.push_str(dimension.to_string().as_str());
    array_string.push_str("] = [\r\n");
    let alphas = create_alphas(dimension);
    for alpha in &alphas {
        // a little bit of formatting is happening as well
        array_string.push_str("\u{20}\u{20}\u{20}\u{20}");
        array_string.push_str(alpha.to_string().as_str());
        array_string.push_str(",\r\n");
    }
    array_string.push_str("];\r\n");

    // write the string to a file. OUT_DIR environment variable is defined by cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("alphas.rs");
    fs::write(&dest_path, array_string).unwrap();
}
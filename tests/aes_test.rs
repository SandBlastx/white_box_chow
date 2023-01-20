

#[path = "../src/aes.rs"]
mod aes;

#[path = "../src/white_box.rs"]
mod white_box;
#[test]
fn test_galois_mul() {
    assert_eq!(crate::aes::galois_mul(3, 7), 9 as u8);
}

#[test]
fn test_sub_bytes() {
    let mut state: [u8; 16] = [1; 16];
    crate::aes::sub_bytes(&mut state);
    crate::aes::inv_sub_bytes(&mut state);

    assert!(
        state.iter().zip([1; 16].iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );
}

#[test]
fn test_shift_rows() {
    let mut state: [u8; 16] = [1; 16];
    crate::aes::shift_rows(&mut state);
    crate::aes::inv_shift_rows(&mut state);
    assert!(
        state.iter().zip([1; 16].iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );
}

#[test]
fn test_mix_columns() {
    let mut state: [u8; 16] = [1; 16];
    crate::aes::mix_columns(&mut state);
    crate::aes::inv_mix_columns(&mut state);
    assert!(
        state.iter().zip([1; 16].iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );
}

#[test]
fn test_expend_key() {
    let mut key: [u8; 176] = [0xff; 176];
    let key_: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ];
    let mut list_key: [u8; 176] = [0; 176];
    let result: [u8; 176] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16,
        0x16, 0x16, 0xad, 0xae, 0xae, 0x19, 0xba, 0xb8, 0xb8, 0x0f, 0x52, 0x51, 0x51, 0xe6, 0x45,
        0x47, 0x47, 0xf0, 0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e,
        0xa4, 0xa0, 0x8c, 0x6e, 0xe1, 0x6a, 0xbd, 0x3e, 0x52, 0xdc, 0x27, 0x46, 0xb3, 0x3b, 0xec,
        0xd8, 0x17, 0x9b, 0x60, 0xb6, 0xe5, 0xba, 0xf3, 0xce, 0xb7, 0x66, 0xd4, 0x88, 0x04, 0x5d,
        0x38, 0x50, 0x13, 0xc6, 0x58, 0xe6, 0x71, 0xd0, 0x7d, 0xb3, 0xc6, 0xb6, 0xa9, 0x3b, 0xc2,
        0xeb, 0x91, 0x6b, 0xd1, 0x2d, 0xc9, 0x8d, 0xe9, 0x0d, 0x20, 0x8d, 0x2f, 0xbb, 0x89, 0xb6,
        0xed, 0x50, 0x18, 0xdd, 0x3c, 0x7d, 0xd1, 0x50, 0x96, 0x33, 0x73, 0x66, 0xb9, 0x88, 0xfa,
        0xd0, 0x54, 0xd8, 0xe2, 0x0d, 0x68, 0xa5, 0x33, 0x5d, 0x8b, 0xf0, 0x3f, 0x23, 0x32, 0x78,
        0xc5, 0xf3, 0x66, 0xa0, 0x27, 0xfe, 0x0e, 0x05, 0x14, 0xa3, 0xd6, 0x0a, 0x35, 0x88, 0xe4,
        0x72, 0xf0, 0x7b, 0x82, 0xd2, 0xd7, 0x85, 0x8c, 0xd7, 0xc3, 0x26,
    ];
    crate::white_box::expend_key(&mut list_key, &key_);

    crate::aes::expend_key(&mut key);
    assert!(
        key.iter().zip(result.iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );
    assert!(
        list_key.iter().zip(list_key.iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );
}

#[test]
fn test_encryption_block() {



    let key: [u8; 16] = [
        0x47, 0x47, 0xf0, 0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e,
        0x3f,
    ];
    let message: [u8; 16] = [
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];

    let encr = crate::aes::encryption_block(&key, &message);
    let decrypt = crate::aes::decryption_block(&key, &encr);

    assert!(
        message.iter().zip(decrypt.iter()).all(|(a, b)| a == b),
        "Arrays are not equal"
    );



//     let key_ = GenericArray::from(key);
//     let mut block = GenericArray::from(message);

// // Initialize cipher
//     let cipher = Aes128::new(&key);

//     let block_copy = block.clone();

// // Encrypt block in-place
// cipher.encrypt_block(&mut block);

// // And decrypt it back
// cipher.decrypt_block(&mut block);
// assert_eq!(block, block_copy);

}



#[test]
fn test_gf2_invertible_matrix_() {
    //test mul

    let inv_1: [[u8; 8]; 8] = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
    ];
    let mut inv_2: [[u8; 32]; 32] = [[0; 32]; 32];
    for i in 0..32 {
        inv_2[i][i] = 1;
    }

    let res_1 = crate::white_box::mul_l_l(&inv_1, &inv_1);
    let res_2 = crate::white_box::mul_m_m(&inv_2, &inv_2);

    assert!(
        res_1.iter().zip(inv_1.iter()).all(|(a, b)| a == b),
        "matrix are not equal"
    );

    assert!(
        res_2.iter().zip(inv_2.iter()).all(|(a, b)| a == b),
        "matrix are not equal"
    );

    let (a, b) = crate::white_box::gf2_invertible_matrix_8();
    let (c, d) = crate::white_box::gf2_invertible_matrix_32();

    let res_1 = crate::white_box::mul_l_l(&a, &b);
    let res_2 = crate::white_box::mul_m_m(&c, &d);

    assert!(
        res_1.iter().zip(inv_1.iter()).all(|(a, b)| a == b),
        "matrix inverse are not equal"
    );

    assert!(
        res_2.iter().zip(inv_2.iter()).all(|(a, b)| a == b),
        "matrix inverse are not equal"
    );
}

#[test]
fn test_mul() {
    let a: u8 = 0b11100011;
    let b: [[u8; 8]; 8] = [
        [0, 1, 1, 0, 0, 1, 1, 0],
        [1, 1, 0, 1, 0, 0, 0, 0],
        [1, 1, 0, 0, 1, 1, 1, 0],
        [0, 0, 0, 1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 1, 1],
        [0, 1, 0, 1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0, 1, 1, 1],
        [0, 1, 1, 0, 1, 0, 0, 0],
    ];
    let c: u8 = 0b11101001;

    let res_1 = crate::white_box::mul_l(a, &b);

    assert_eq!(res_1, c);

    let a: [u8; 4] = [0b10011011, 0b01011110u8, 0b11001011u8, 0b00011010u8];
    let b: [[u8; 32]; 32] = [
        [
            0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0,
            0, 1, 1,
        ],
        [
            1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1,
            1, 0, 1,
        ],
        [
            0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1,
            0, 1, 0,
        ],
        [
            0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0,
            0, 0, 1,
        ],
        [
            0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 0,
        ],
        [
            1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0,
            0, 1, 1,
        ],
        [
            1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1,
            0, 1, 1,
        ],
        [
            1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 1, 1,
        ],
        [
            1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1,
            1, 0, 1,
        ],
        [
            0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1,
            1, 0, 0,
        ],
        [
            0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ],
        [
            1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0,
            0, 1, 0,
        ],
        [
            1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0,
            0, 0, 1,
        ],
        [
            1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 0,
        ],
        [
            0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0,
            0, 0, 0,
        ],
        [
            0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1,
            0, 0, 1,
        ],
        [
            0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1,
            0, 1, 0,
        ],
        [
            0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1,
            1, 0, 1,
        ],
        [
            0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0,
            1, 1, 0,
        ],
        [
            1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0,
            1, 1, 1,
        ],
        [
            1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1,
            1, 1, 1,
        ],
        [
            0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1,
            1, 0, 1,
        ],
        [
            1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1,
            0, 0, 1,
        ],
        [
            1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1,
            0, 1, 0,
        ],
        [
            1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1,
            1, 0, 1,
        ],
        [
            0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0,
            1, 1, 1,
        ],
        [
            1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1,
            0, 1, 1,
        ],
        [
            1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0,
            1, 0, 1,
        ],
        [
            0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0,
        ],
        [
            1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0,
            1, 0, 1,
        ],
        [
            0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1,
            1, 1, 0,
        ],
    ];

    let c: [u8; 4] = [0b01011111u8, 0b01110101u8, 0b01101001u8, 0b10001010u8];

    let res_2 = crate::white_box::mul_m(&a, &b);

    assert!(
        res_2.iter().zip(c.iter()).all(|(a, b)| a == b),
        "matrix are not equal"
    );

    //convert
    let convert_1: [u8; 8] = [0, 1, 1, 0, 0, 1, 1, 0];
    assert_eq!(
        crate::white_box::convert_from_32b(&crate::white_box::convert_to_32b(&convert_1)),
        convert_1
    );
}

#[test]
fn test_xor() {
    let xor_tables = crate::white_box::xor_tables();
    for k in 0..16 {
        for l in 0..16 {
            assert_eq!(xor_tables[2][4][k][l], (k as u8) ^ (l as u8));
        }
    }

    let a = [0b00011010u8, 0b01011111u8, 0b01101001u8, 0b10001010u8];
    let b = [0b00011010u8, 0b01011111u8, 0b10001010u8, 0b01101001u8];
    let c = [0, 0, 0b11100011, 0b11100011];
    let res = crate::white_box::xor_32b(1, 2, &xor_tables, &a, &b);
    let res_2 = crate::white_box::xor_32b(2, 4, &xor_tables, &a, &b);
    assert!(
        res.iter().zip(c.iter()).all(|(a, b)| a == b),
        "matrix are not equal"
    );
    assert!(
        res_2.iter().zip(c.iter()).all(|(a, b)| a == b),
        "matrix are not equal"
    );
}

#[test]
fn test_tbox() {
    let key: [u8; 16] = [
        0x8, 0x65, 0x3, 0x6c, 0x6f, 0x2, 0x57, 0xf, 0x7, 0xc, 0x4, 0x0, 0x3, 0x9, 0x0, 0xe,
    ];
    let message_1: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];
    let message_2: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];

    let mut state: [u8; 16] = message_1;
    let mut state_2: [u8; 16] = message_2;
    let mut list_key: [u8; 176] = [0; 176];

    //init tables:
    let t_boxes = crate::white_box::t_boxes(&key);

    crate::white_box::expend_key(&mut list_key, &key);

    for i in 0..9 {
        //classic
        let mut prev_key_round = list_key[i * 16..(i + 1) * 16]
            .try_into()
            .expect("slice with incorrect length");

        crate::white_box::shift_rows(&mut prev_key_round);
        crate::white_box::shift_rows(&mut state);
        crate::white_box::add_round_key(&mut state, &prev_key_round);
        crate::white_box::sub_bytes(&mut state);

        //tboxes
        crate::white_box::shift_rows(&mut state_2);
        for j in 0..16 {
            state_2[j] = t_boxes[i][j][state_2[j] as usize];
        }

        assert!(
            state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
            "Arrays are not equal in round {}",
            i
        );
    }

    let mut key_round_9: [u8; 16] = list_key[144..160]
        .try_into()
        .expect("slice with incorrect length for key round 9");

    let key_round_10: [u8; 16] = list_key[160..176]
        .try_into()
        .expect("slice with incorrect length for key round 10");
    //classic
    crate::white_box::shift_rows(&mut state);
    crate::white_box::shift_rows(&mut key_round_9);
    crate::white_box::add_round_key(&mut state, &key_round_9);
    crate::white_box::sub_bytes(&mut state);
    crate::white_box::add_round_key(&mut state, &key_round_10);

    //white_box

    crate::white_box::shift_rows(&mut state_2);
    // last Tboxe
    for i in 0..16 {
        state_2[i] = t_boxes[9][i][state_2[i] as usize];
    }

    assert!(
        state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
}

#[test]
fn test_tybox() {
    //input
    let key: [u8; 16] = [
        0x8, 0x65, 0x3, 0x6c, 0x6f, 0x2, 0x57, 0xf, 0x7, 0xc, 0x4, 0x0, 0x3, 0x9, 0x0, 0xe,
    ];
    let message_1: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];
    let message_2: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];

    let mut state: [u8; 16] = message_1;
    let mut state_2: [u8; 16] = message_2;
    let mut list_key: [u8; 176] = [0; 176];

    //init tables:
    let t_boxes = crate::white_box::t_boxes(&key);
    let ty_boxes = crate::white_box::ty_tables(&t_boxes);
    let xor_tables = crate::white_box::xor_tables();

    crate::white_box::expend_key(&mut list_key, &key);

    for i in 0..9 {
        //classic
        let mut prev_key_round = list_key[i * 16..(i + 1) * 16]
            .try_into()
            .expect("slice with incorrect length");

        crate::white_box::shift_rows(&mut prev_key_round);
        crate::white_box::shift_rows(&mut state);
        crate::white_box::add_round_key(&mut state, &prev_key_round);
        crate::white_box::sub_bytes(&mut state);
        crate::white_box::mix_columns(&mut state);

        //tboxes
        crate::white_box::shift_rows(&mut state_2);

        for g in 0..4 {
            // group

            //calculate ty_i
            let mut ty_i: [[u8; 4]; 4] = [[0; 4]; 4];
            for x in 0..4 {
                // tyi
                for b in 0..4 {
                    // 32 bit in 4 bytes
                    ty_i[x][b] = ty_boxes[i][g][x][state_2[g * 4 + x] as usize][b];
                }
            }

            // xor ty_i
            let xor_tmp_0 = crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i[0], &ty_i[1]);
            let xor_tmp_1 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i[2], &ty_i[3]);
            let xor_group_result =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);

            for res_xor in 0..4 {
                state_2[g * 4 + res_xor] = xor_group_result[res_xor];
            }
        }

        //test
        assert!(
            state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
            "Arrays are not equal in round {}",
            i
        );
    }

    let mut key_round_9: [u8; 16] = list_key[144..160]
        .try_into()
        .expect("slice with incorrect length for key round 9");

    let key_round_10: [u8; 16] = list_key[160..176]
        .try_into()
        .expect("slice with incorrect length for key round 10");

    //classic
    crate::white_box::shift_rows(&mut state);
    crate::white_box::shift_rows(&mut key_round_9);
    crate::white_box::add_round_key(&mut state, &key_round_9);
    crate::white_box::sub_bytes(&mut state);
    crate::white_box::add_round_key(&mut state, &key_round_10);

    //white_box

    crate::white_box::shift_rows(&mut state_2);
    // last Tboxe
    for i in 0..16 {
        state_2[i] = t_boxes[9][i][state_2[i] as usize];
    }

    assert!(
        state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
    println!("state {:?}", state);
    println!("state2 {:?}", state_2);
}

#[test]
fn test_lbox() {
    //input
    let key: [u8; 16] = [
        0x8, 0x65, 0x3, 0x6c, 0x6f, 0x2, 0x57, 0xf, 0x7, 0xc, 0x4, 0x0, 0x3, 0x9, 0x0, 0xe,
    ];
    let message_1: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];
    let message_2: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];

    let message_3: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];

    let shift = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];

    let mut state: [u8; 16] = message_1;
    let mut state_2: [u8; 16] = message_2;
    let mut state_3: [u8; 16] = message_3;
    let mut list_key: [u8; 176] = [0; 176];

    //init tables:
    let t_boxes = crate::white_box::t_boxes(&key);
    let mut tl_boxes = crate::white_box::t_boxes(&key);

    let l_box = crate::white_box::mixing_bijection_l();

    let ty_boxes = crate::white_box::ty_tables(&t_boxes);
    let tyl_boxes = crate::white_box::tyl_tables(&mut tl_boxes, &l_box);

    let xor_tables = crate::white_box::xor_tables();

    crate::white_box::expend_key(&mut list_key, &key);

    for i in 0..9 {
        //classic
        let mut prev_key_round = list_key[i * 16..(i + 1) * 16]
            .try_into()
            .expect("slice with incorrect length");

        crate::white_box::shift_rows(&mut prev_key_round);
        crate::white_box::shift_rows(&mut state);
        crate::white_box::add_round_key(&mut state, &prev_key_round);
        crate::white_box::sub_bytes(&mut state);
        crate::white_box::mix_columns(&mut state);

        //tboxes
        crate::white_box::shift_rows(&mut state_2);
        crate::white_box::shift_rows(&mut state_3);

        for g in 0..4 {
          
            let mut ty_i: [[u8; 4]; 4] = [[0; 4]; 4];
            let mut ty_i_3: [[u8; 4]; 4] = [[0; 4]; 4];

            for x in 0..4 {
                if i > 0 {
                    state_2[g * 4 + x] =
                        crate::white_box::mul_l(state_2[g * 4 + x], &l_box[1][i - 1][g * 4 + x]);
                }
                // tyi
                
                    // 32 bit in 4 bytes

                    ty_i[x] = ty_boxes[i][g][x][state_2[g * 4 + x] as usize];
                    ty_i_3[x] = tyl_boxes[i][g][x][state_3[g * 4 + x] as usize];
                
            }

            // xor ty_i
            let xor_tmp_0 = crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i[0], &ty_i[1]);
            let xor_tmp_1 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i[2], &ty_i[3]);
            let xor_group_result =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);

            let xor_tmp_0 =
                crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i_3[0], &ty_i_3[1]);
            let xor_tmp_1 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i_3[2], &ty_i_3[3]);
            let xor_group_result_3 =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);

            for res_xor in 0..4 {
                state_2[g * 4 + res_xor] = xor_group_result[res_xor];
                state_3[g * 4 + res_xor] = xor_group_result_3[res_xor];
            }
        }

        // apply L

        for l in 0..16 {
            state_2[l] = crate::white_box::mul_l(state_2[l], &l_box[0][i][shift[l]]);
            state_3[l] = crate::white_box::mul_l(state_3[l], &l_box[0][i][shift[l]]);
        }

        //test

        assert!(
            state_2.iter().zip(state_3.iter()).all(|(a, b)| a == b),
            "Arrays are not equal in round {}",
            i
        );
    }

    let mut key_round_9: [u8; 16] = list_key[144..160]
        .try_into()
        .expect("slice with incorrect length for key round 9");

    let key_round_10: [u8; 16] = list_key[160..176]
        .try_into()
        .expect("slice with incorrect length for key round 10");

    //classic
    crate::white_box::shift_rows(&mut state);
    crate::white_box::shift_rows(&mut key_round_9);
    crate::white_box::add_round_key(&mut state, &key_round_9);
    crate::white_box::sub_bytes(&mut state);
    crate::white_box::add_round_key(&mut state, &key_round_10);

    //white_box

    crate::white_box::shift_rows(&mut state_2);
    crate::white_box::shift_rows(&mut state_3);

    // last Tboxe
    for i in 0..16 {
        state_2[i] = crate::white_box::mul_l(state_2[i], &l_box[1][8][i]);
        // state_3[i] = crate::white_box::mul_l(state_3[i], &l_box[1][8][i]);


        state_2[i] = t_boxes[9][i][state_2[i] as usize];
        state_3[i] = tl_boxes[9][i][state_3[i] as usize];
    }

    println!("state {:?}", state);
    println!("state_2 {:?}", state_2);
    println!("state_3 {:?}", state_3);

    assert!(
        state_2.iter().zip(state_3.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
    assert!(
        state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
}

#[test]
fn test_lmbox() {
    //input
    let key: [u8; 16] = [
        0x8, 0x65, 0x3, 0x6c, 0x6f, 0x2, 0x57, 0xf, 0x7, 0xc, 0x4, 0x0, 0x3, 0x9, 0x0, 0xe,
    ];
    let message_1: [u8; 16] = [
        0x48, 0x5, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
        0x2e,
    ];


    let shift = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];

    let mut state: [u8; 16] = message_1;
    let mut state_2: [u8; 16] = message_1;
    let mut state_3: [u8; 16] = message_1;
    let mut state_4 = message_1;
    let mut list_key: [u8; 176] = [0; 176];

    //init tables:
    let t_boxes = crate::white_box::t_boxes(&key);

    let mut tl_boxes = crate::white_box::t_boxes(&key);
    let mut tlm_boxes = crate::white_box::t_boxes(&key);

    let l_box = crate::white_box::mixing_bijection_l();
    let m_box = crate::white_box::mixing_bijection_m();
    let ml_box = crate::white_box::mixing_bijection_m_l(&m_box, & l_box);

    let ty_boxes = crate::white_box::ty_tables(&t_boxes);
    let tyl_boxes = crate::white_box::tyl_tables(&mut tl_boxes, &l_box);
    let tylm_boxes = crate::white_box::tylm_tables(&mut tlm_boxes, &l_box, &m_box);

    let xor_tables = crate::white_box::xor_tables();
    let xor_tables_ml = crate::white_box::xor_tables();

    crate::white_box::expend_key(&mut list_key, &key);

    for i in 0..9 {
        //classic
        let mut prev_key_round = list_key[i * 16..(i + 1) * 16]
            .try_into()
            .expect("slice with incorrect length");

        crate::white_box::shift_rows(&mut prev_key_round);
        crate::white_box::shift_rows(&mut state);
        crate::white_box::add_round_key(&mut state, &prev_key_round);
        crate::white_box::sub_bytes(&mut state);
        crate::white_box::mix_columns(&mut state);

        //tboxes
        crate::white_box::shift_rows(&mut state_2);
        crate::white_box::shift_rows(&mut state_3);
        crate::white_box::shift_rows(&mut state_4);


        for g in 0..4 {
          
            let mut ty_i: [[u8; 4]; 4] = [[0; 4]; 4];
            let mut ty_i_3: [[u8; 4]; 4] = [[0; 4]; 4];
            let mut ty_i_4: [[u8; 4]; 4] = [[0; 4]; 4];
            let mut tmp: [[u8; 4];4] = [[0; 4]; 4];


            for x in 0..4 {
                if i > 0 {
                    state_2[g * 4 + x] =
                        crate::white_box::mul_l(state_2[g * 4 + x], &l_box[1][i - 1][g * 4 + x]);
                }
                // tyi
                
                    // 32 bit in 4 bytes

                    ty_i[x] = ty_boxes[i][g][x][state_2[g * 4 + x] as usize];
                    ty_i_3[x] = tyl_boxes[i][g][x][state_3[g * 4 + x] as usize];
                    ty_i_4[x] = tylm_boxes[i][g][x][state_4[g * 4 + x] as usize];


            }

            // xor ty_i
            let xor_tmp_0 = crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i[0], &ty_i[1]);
            let xor_tmp_1 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i[2], &ty_i[3]);
            let xor_group_result =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);

            let xor_tmp_0_3 =
                crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i_3[0], &ty_i_3[1]);
            let xor_tmp_1_3 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i_3[2], &ty_i_3[3]);
            let xor_group_result_3 =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_1_3, &xor_tmp_0_3);


                let xor_tmp_0_4 =
                crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i_4[0], &ty_i_4[1]);
            let xor_tmp_1_4 =

                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i_4[2], &ty_i_4[3]);
            let xor_group_result_4 =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_1_4, &xor_tmp_0_4);


                for r in 0..4{
                    tmp[r] = ml_box[i][g][xor_group_result_4[r] as usize][r];
                }

                let xor_tmp_0_4_4 =
                crate::white_box::xor_32b(i, g * 3, &xor_tables_ml, &tmp[0], &tmp[1]);
            let xor_tmp_1_4_4 =
                crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables_ml, &tmp[2], &tmp[3]);
            let xor_group_result_4_4 =
                crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables_ml, &xor_tmp_0_4_4, &xor_tmp_1_4_4);

            for res_xor in 0..4 {
                state_2[g * 4 + res_xor] = xor_group_result[res_xor];
                state_3[g * 4 + res_xor] = xor_group_result_3[res_xor];
                state_4[g * 4 + res_xor] = xor_group_result_4_4[res_xor];

            }
        }

        // apply L

        for l in 0..16 {
            state_2[l] = crate::white_box::mul_l(state_2[l], &l_box[0][i][shift[l]]);
            state_3[l] = crate::white_box::mul_l(state_3[l], &l_box[0][i][shift[l]]);
        }

        //test

        assert!(
            state_2.iter().zip(state_3.iter()).all(|(a, b)| a == b),
            "Arrays are not equal in round {}",
            i
        );
    }

    let mut key_round_9: [u8; 16] = list_key[144..160]
        .try_into()
        .expect("slice with incorrect length for key round 9");

    let key_round_10: [u8; 16] = list_key[160..176]
        .try_into()
        .expect("slice with incorrect length for key round 10");

    //classic
    crate::white_box::shift_rows(&mut state);
    crate::white_box::shift_rows(&mut key_round_9);
    crate::white_box::add_round_key(&mut state, &key_round_9);
    crate::white_box::sub_bytes(&mut state);
    crate::white_box::add_round_key(&mut state, &key_round_10);

    //white_box

    crate::white_box::shift_rows(&mut state_2);
    crate::white_box::shift_rows(&mut state_3);
    crate::white_box::shift_rows(&mut state_4);

    // last Tboxe
    for i in 0..16 {
        state_2[i] = crate::white_box::mul_l(state_2[i], &l_box[1][8][i]);
        // state_3[i] = crate::white_box::mul_l(state_3[i], &l_box[1][8][i]);


        state_2[i] = t_boxes[9][i][state_2[i] as usize];
        state_3[i] = tl_boxes[9][i][state_3[i] as usize];
        state_4[i] = tl_boxes[9][i][state_4[i] as usize];

    }

    println!("state {:?}", state);
    println!("state_2 {:?}", state_2);
    println!("state_3 {:?}", state_3);
    println!("state_4 {:?}", state_4);

    assert!(
        state_2.iter().zip(state_3.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
    assert!(
        state_2.iter().zip(state.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );

    assert!(
        state_2.iter().zip(state_4.iter()).all(|(a, b)| a == b),
        "Arrays are not equal in round 10"
    );
}

    #[test]
    fn test_xor_mulm() {
    
    let xor_tables = crate::white_box::xor_tables();
    let m_box = crate::white_box::mixing_bijection_m();
    
    let mut ty_i_3: [[u8; 4]; 4] = [[1,3,4,2] , [3,4,5,6] , [33,43,34,2] , [4,0,4,22]];
    let mut tmp: [[u8; 4];4] = [[0;4]; 4];
    let g = 2;
    let i = 3 ;
    
    for r in 0..4 {
    ty_i_3[r] = crate::white_box::mul_m(&ty_i_3[r],&m_box[0][i][g]);
    }
    println!("ty_i:{:?}", ty_i_3);
    let xor_tmp_0 = crate::white_box::xor_32b(i, g * 3, &xor_tables, &ty_i_3[0], &ty_i_3[1]);
    let xor_tmp_1 = crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &ty_i_3[2], &ty_i_3[3]);
    let  xor_group_result = crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);
    let mut xor_res_3 = xor_group_result;

    //
    println!("xor_res_3:{:?}", xor_res_3);

    tmp[0] = [xor_group_result[0], 0, 0, 0];
    tmp[1] =  [0,xor_group_result[1], 0, 0] ;
    tmp[2] = [0,0,xor_group_result[2],0] ;
    tmp[3] = [0,0,0,xor_group_result[3]] ;

    for  r in 0..4{
        tmp[r] =  crate::white_box::mul_m(&tmp[r],&m_box[1][i][g]);
    }


        

    let xor_tmp_0 = crate::white_box::xor_32b(i, g * 3, &xor_tables, &tmp[0], &tmp[1]);
    let xor_tmp_1 =
        crate::white_box::xor_32b(i, g * 3 + 1, &xor_tables, &tmp[2], &tmp[3]);
    let   xor_group_result_tmp =
        crate::white_box::xor_32b(i, g * 3 + 2, &xor_tables, &xor_tmp_0, &xor_tmp_1);


        xor_res_3 = crate::white_box::mul_m(& xor_res_3,&m_box[1][i][g]);
    
        //

        println!("tmp {:?}", xor_group_result_tmp );
        println!("xor {:?}", xor_res_3);


    }


    #[test]
    fn test_encryption_block_white_box() {
        let key: [u8; 16] = [
            0x47, 0x47, 0xf0, 0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e,
            0x3f,
        ];
        let message: [u8; 16] = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x3a, 0x29, 0x20,
            0x2e,
        ];
    
        let encr = crate::white_box::encryption_block(&key, &message);
        let decrypt = crate::aes::decryption_block(&key, &encr);
    
        assert!(
            message.iter().zip(decrypt.iter()).all(|(a, b)| a == b),
            "Arrays are not equal"
        );
    }
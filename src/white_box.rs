

mod intern {

    #![allow(dead_code)]
    use rand::Rng;
    


pub const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

pub const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
];

pub fn mul_l(a: u8, b: &[[u8; 8]; 8]) -> u8 {
    let mut res: u8 = 0;
    for i in 0..8 {
        let mut tmp = 0;

        for j in 0..8 {
            tmp ^= b[i][j] & ((a >> j) & 0b00000001);
        }
        res |= tmp << i;
    }
    return res;
}

pub fn mul_m(a: &[u8; 4], b: &[[u8; 32]; 32]) -> [u8; 4] {
    let mut r: [u8; 4] = [0; 4];
    for i in 0..32 {
        let mut tmp = 0;
        for j in 0..32 {
            tmp ^= b[i][j] & ((a[j / 8] >> (j % 8)) & 0b00000001);
        }
        r[i / 8] |= tmp << i % 8;
    }
    return r;
}

pub fn convert_to_32b(a: &[u8; 8]) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    for i in 0..4 {
        result[i] = a[2 * i];
        result[i] |= a[2 * i + 1] << 4;
    }
    return result;
}
pub fn convert_from_32b(a: &[u8; 4]) -> [u8; 8] {
    let mut result: [u8; 8] = [0; 8];
    for i in 0..4 {
        result[2 * i] = a[i] & 0b00001111;
        result[2 * i + 1] = a[i] >> 4;
    }
    return result;
}

pub fn xor_32b(
    r: usize,
    i: usize,
    xor_table: &[[[[u8; 16]; 16]; 96]; 9],
    a: &[u8; 4],
    b: &[u8; 4],
) -> [u8; 4] {
    let mut xor_result: [u8; 8] = [0; 8];

    let a_tmp = convert_from_32b(&a);
    let b_tmp = convert_from_32b(&b);

    for k in 0..8 {
        xor_result[k] = xor_table[r][8 * i + k][a_tmp[k] as usize][b_tmp[k] as usize];
    }

    return convert_to_32b(&xor_result);
}
fn check_column(m: &[[u8; 8]; 8], c: usize) -> (bool, usize) {
    for i in c..8 {
        if m[i][c] == 1 {
            return (true, i);
        }
    }
    return (false, 0);
}

fn check_column_32(m: &[[u8; 32]; 32], c: usize) -> (bool, usize) {
    for i in c..32 {
        if m[i][c] == 1 {
            return (true, i);
        }
    }
    return (false, 0);
}

fn add_row_gf2(m: &mut [[u8; 8]; 8], a: usize, b: usize) {
    for j in 0..8 {
        m[a][j] ^= m[b][j];
    }
}

fn add_row_gf2_32(m: &mut [[u8; 32]; 32], a: usize, b: usize) {
    for j in 0..32 {
        m[a][j] ^= m[b][j];
    }
}
fn swap_row_gf2(m: &mut [[u8; 8]; 8], a: usize, b: usize) {
    for j in 0..8 {
        let tmp = m[b][j];
        m[b][j] = m[a][j];
        m[a][j] = tmp;
    }
}

fn swap_row_gf2_32(m: &mut [[u8; 32]; 32], a: usize, b: usize) {
    for j in 0..32 {
        let tmp = m[b][j];
        m[b][j] = m[a][j];
        m[a][j] = tmp;
    }
}
pub fn mul_l_l(a: &[[u8; 8]; 8], b: &[[u8; 8]; 8]) -> [[u8; 8]; 8] {
    let mut res: [[u8; 8]; 8] = [[0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            let mut tmp = 0;
            for l in 0..8 {
                tmp ^= a[i][l] & b[l][j];
            }
            res[i][j] = tmp;
        }
    }
    return res;
}

pub fn mul_m_m(a: &[[u8; 32]; 32], b: &[[u8; 32]; 32]) -> [[u8; 32]; 32] {
    let mut res: [[u8; 32]; 32] = [[0; 32]; 32];
    for i in 0..32 {
        for j in 0..32 {
            let mut tmp = 0;
            for l in 0..32 {
                tmp ^= a[i][l] & b[l][j];
            }
            res[i][j] = tmp;
        }
    }
    return res;
}

pub fn gaussian_reduction_32(a: &mut [[u8; 32]; 32]) -> (bool, [[u8; 32]; 32]) {
    let mut tmp: [[u8; 32]; 32] = a.clone();
    let mut inv: [[u8; 32]; 32] = [[0; 32]; 32];
    for i in 0..32 {
        inv[i][i] = 1;
    }
    for i in 0..32 {
        let pivot = check_column_32(&tmp, i);
        if !pivot.0 {
            return (false, inv);
        }
        if pivot.1 != i {
            swap_row_gf2_32(&mut tmp, pivot.1, i);
            swap_row_gf2_32(&mut inv, pivot.1, i);
        }
        for j in 0..32 {
            if tmp[j][i] == 1 && j != i {
                add_row_gf2_32(&mut tmp, j, i);
                add_row_gf2_32(&mut inv, j, i);
            }
        }
    }
    return (true, inv);
}
pub fn gaussian_reduction_8(a: &mut [[u8; 8]; 8]) -> (bool, [[u8; 8]; 8]) {
    let mut tmp: [[u8; 8]; 8] = a.clone();
    let mut inv: [[u8; 8]; 8] = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
    ];
    for i in 0..8 {
        let pivot = check_column(&tmp, i);
        if !pivot.0 {
            return (false, inv);
        }
        if pivot.1 != i {
            swap_row_gf2(&mut tmp, pivot.1, i);
            swap_row_gf2(&mut inv, pivot.1, i);
        }
        for j in 0..8 {
            if tmp[j][i] == 1 && j != i {
                add_row_gf2(&mut tmp, j, i);
                add_row_gf2(&mut inv, j, i);
            }
        }
    }
    return (true, inv);
}
pub fn gf2_invertible_matrix_8() -> ([[u8; 8]; 8], [[u8; 8]; 8]) {
    // Make fn type variable
    let mut rng = rand::thread_rng();
    let mut m: [[u8; 8]; 8] = [[0; 8]; 8];
    let mut invertible = false;
    let mut decomp = (false, [[0; 8]; 8]);
    while !invertible {
        for i in 0..8 {
            for j in 0..8 {
                m[i][j] = rng.gen_range(0..2);
            }
        }
        decomp = gaussian_reduction_8(&mut m);
        invertible = decomp.0;
    }
    let inverse = decomp.1;
    return (m, inverse);
}

pub fn gf2_invertible_matrix_32() -> ([[u8; 32]; 32], [[u8; 32]; 32]) {
    let mut rng = rand::thread_rng();
    let mut m: [[u8; 32]; 32] = [[0; 32]; 32];
    let mut invertible = false;
    let mut decomp = (false, [[0; 32]; 32]);
    while !invertible {
        for i in 0..32 {
            for j in 0..32 {
                m[i][j] = rng.gen_range(0..2);
            }
        }
        decomp = gaussian_reduction_32(&mut m);
        invertible = decomp.0;
    }
    let inverse = decomp.1;
    return (m, inverse);
}

 pub fn mixing_bijection_l() -> [[[[[u8; 8]; 8]; 16]; 9]; 2] {
    let mut l: [[[[[u8; 8]; 8]; 16]; 9]; 2] = [[[[[0; 8]; 8]; 16]; 9]; 2];
    for r in 0..9 {
        for i in 0..16 {
            let tmp: ([[u8; 8]; 8], [[u8; 8]; 8]) = gf2_invertible_matrix_8();
            l[0][r][i] = tmp.0;
            l[1][r][i] = tmp.1; //inverse
        }
    }
    return l;
}

 pub fn mixing_bijection_m() -> [[[[[u8; 32]; 32]; 4]; 9]; 2] {
    let mut m: [[[[[u8; 32]; 32]; 4]; 9]; 2] = [[[[[0; 32]; 32]; 4]; 9]; 2];
    for r in 0..9 {
        for i in 0..4 {
            let tmp = gf2_invertible_matrix_32();
            m[0][r][i] = tmp.0;
            m[1][r][i] = tmp.1; //inverse
        }
    }
    return m;
}

 pub fn mixing_bijection_m_l(
    mixing_bijection_m: &[[[[[u8; 32]; 32]; 4]; 9]; 2],
    mixing_bijection_l: &[[[[[u8; 8]; 8]; 16]; 9]; 2],
) -> [[[[[u8; 4]; 4]; 256]; 4]; 9] {
    let mut ml_tables: [[[[[u8; 4]; 4]; 256]; 4]; 9] = [[[[[0; 4]; 4]; 256]; 4]; 9];
    let shift = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];
    let mut l: [[u8; 32]; 32] = [[0; 32]; 32];
    // m_ bijection round: (1 ..9); l_: (2..10) ; ml :(1..9)
    for r in 0..9 {
        for j in 0..4 {
            for i in 0..4 {
                // need next round l bijection ,/need to be diagonal matrix with L  = L0 || L13||L10||L7
                for k in 0..8 {
                    for m in 0..8 {
                        l[8 * i + k][8 * i + m] = mixing_bijection_l[0][r][shift[j * 4 + i]][k][m];
                    }
                }
            }
            for x in 0..=255u8 {
                let x_indice = x as usize;
                let mut tmp = [[8u8; 4]; 4];

                tmp[0] = [x, 0, 0, 0];
                tmp[1] = [0, x, 0, 0];
                tmp[2] = [0, 0, x, 0];
                tmp[3] = [0, 0, 0, x];

                for k in 0..4 {
                    tmp[k] = mul_m(&tmp[k], &mixing_bijection_m[1][r][j]);
                    ml_tables[r][j][x_indice][k] = mul_m(&tmp[k], &l);
                }
            }
        }
    }
    return ml_tables;
}

pub fn t_boxes(key: &[u8; 16]) -> [[[u8; 256]; 16]; 10] {
    let mut tboxes: [[[u8; 256]; 16]; 10] = [[[0; 256]; 16]; 10];

    let mut list_key: [u8; 176] = [0; 176];
    let mut key_round: [u8; 16] = [0; 16];
    let mut key_pr: [u8; 16] = [0; 16];

    expend_key(&mut list_key, key);

    for r in 0..10 {
        key_round = list_key[r * 16..(r + 1) * 16]
            .try_into()
            .expect("slice with incorrect length getting key round ");

        if r == 9 {
            key_pr = list_key[(r + 1) * 16..(r + 2) * 16]
                .try_into()
                .expect("slice with incorrect length getting last key round ");
        }
        shift_rows(&mut key_round);

        for i in 0..16 {
            for x in 0..=255u8 {
                if r == 9 {
                    let tmp: u8 = x ^ key_round[i];
                    tboxes[r][i][x as usize] = SBOX[tmp as usize] ^ key_pr[i];
                } else {
                    let tmp: u8 = x ^ key_round[i];
                    tboxes[r][i][x as usize] = SBOX[tmp as usize];
                }
            }
        }
    }
    return tboxes;
}

pub fn ty_tables(tboxes: &[[[u8; 256]; 16]; 10]) -> [[[[[u8; 4]; 256]; 4]; 4]; 9] {
    let mut ty_tables: [[[[[u8; 4]; 256]; 4]; 4]; 9] = [[[[[0; 4]; 256]; 4]; 4]; 9];

    for r in 0..9 {
        for j in 0..4 {
            for x in 0..=255u8 {
                let x_i = x as usize;
                let k = tboxes[r][j * 4 + 0][x_i];
                ty_tables[r][j][0][x_i][0] = galois_mul(k, 2);
                ty_tables[r][j][0][x_i][1] = k;
                ty_tables[r][j][0][x_i][2] = k;
                ty_tables[r][j][0][x_i][3] = galois_mul(k, 3);

                let k = tboxes[r][j * 4 + 1][x_i];
                ty_tables[r][j][1][x_i][0] = galois_mul(k, 3);
                ty_tables[r][j][1][x_i][1] = galois_mul(k, 2);
                ty_tables[r][j][1][x_i][2] = k;
                ty_tables[r][j][1][x_i][3] = k;

                let k = tboxes[r][j * 4 + 2][x_i];
                ty_tables[r][j][2][x_i][0] = k;
                ty_tables[r][j][2][x_i][1] = galois_mul(k, 3);
                ty_tables[r][j][2][x_i][2] = galois_mul(k, 2);
                ty_tables[r][j][2][x_i][3] = k;

                let k = tboxes[r][j * 4 + 3][x_i];
                ty_tables[r][j][3][x_i][0] = k;
                ty_tables[r][j][3][x_i][1] = k;
                ty_tables[r][j][3][x_i][2] = galois_mul(k, 3);
                ty_tables[r][j][3][x_i][3] = galois_mul(k, 2);
            }
        }
    }
    return ty_tables;
}

 pub fn xor_tables() -> [[[[u8; 16]; 16]; 96]; 9] {
    let mut xor_table: [[[[u8; 16]; 16]; 96]; 9] = [[[[0; 16]; 16]; 96]; 9];
    for i in 0..9 {
        for j in 0..96 {
            for k in 0..16u8 {
                for l in 0..16u8 {
                    xor_table[i][j][k as usize][l as usize] = k ^ l;
                }
            }
        }
    }
    return xor_table;
}

pub fn tyl_tables(
    tboxes: &mut [[[u8; 256]; 16]; 10],
    mixing_bijection_l: &[[[[[u8; 8]; 8]; 16]; 9]; 2],
) -> [[[[[u8; 4]; 256]; 4]; 4]; 9] {
    let t_boxe_clone = tboxes.clone();
    let mut ty_tables: [[[[[u8; 4]; 256]; 4]; 4]; 9] = [[[[[0; 4]; 256]; 4]; 4]; 9];

    for r in 0..9 {
        for j in 0..4 {
            for x in 0..=255u8 {
                if r > 0 {
                    let x_i = x as usize;
                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4]) as usize;
                    let k = tboxes[r][j * 4 + 0][l];
                    ty_tables[r][j][0][x_i][0] = galois_mul(k, 2);
                    ty_tables[r][j][0][x_i][1] = k;
                    ty_tables[r][j][0][x_i][2] = k;
                    ty_tables[r][j][0][x_i][3] = galois_mul(k, 3);

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 1]) as usize;
                    let k = tboxes[r][j * 4 + 1][l];
                    ty_tables[r][j][1][x_i][0] = galois_mul(k, 3);
                    ty_tables[r][j][1][x_i][1] = galois_mul(k, 2);
                    ty_tables[r][j][1][x_i][2] = k;
                    ty_tables[r][j][1][x_i][3] = k;

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 2]) as usize;
                    let k = tboxes[r][j * 4 + 2][l];
                    ty_tables[r][j][2][x_i][0] = k;
                    ty_tables[r][j][2][x_i][1] = galois_mul(k, 3);
                    ty_tables[r][j][2][x_i][2] = galois_mul(k, 2);
                    ty_tables[r][j][2][x_i][3] = k;

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 3]) as usize;

                    let k = tboxes[r][j * 4 + 3][l];
                    ty_tables[r][j][3][x_i][0] = k;
                    ty_tables[r][j][3][x_i][1] = k;
                    ty_tables[r][j][3][x_i][2] = galois_mul(k, 3);
                    ty_tables[r][j][3][x_i][3] = galois_mul(k, 2);
                } else {
                    let x_i = x as usize;
                    let k = tboxes[r][j * 4 + 0][x_i];
                    ty_tables[r][j][0][x_i][0] = galois_mul(k, 2);
                    ty_tables[r][j][0][x_i][1] = k;
                    ty_tables[r][j][0][x_i][2] = k;
                    ty_tables[r][j][0][x_i][3] = galois_mul(k, 3);

                    let k = tboxes[r][j * 4 + 1][x_i];
                    ty_tables[r][j][1][x_i][0] = galois_mul(k, 3);
                    ty_tables[r][j][1][x_i][1] = galois_mul(k, 2);
                    ty_tables[r][j][1][x_i][2] = k;
                    ty_tables[r][j][1][x_i][3] = k;

                    let k = tboxes[r][j * 4 + 2][x_i];
                    ty_tables[r][j][2][x_i][0] = k;
                    ty_tables[r][j][2][x_i][1] = galois_mul(k, 3);
                    ty_tables[r][j][2][x_i][2] = galois_mul(k, 2);
                    ty_tables[r][j][2][x_i][3] = k;

                    let k = tboxes[r][j * 4 + 3][x_i];
                    ty_tables[r][j][3][x_i][0] = k;
                    ty_tables[r][j][3][x_i][1] = k;
                    ty_tables[r][j][3][x_i][2] = galois_mul(k, 3);
                    ty_tables[r][j][3][x_i][3] = galois_mul(k, 2);
                }
            }
        }
    }
    for j in 0..16 {
        for x in 0..=255u8 {
            let x_indice = x as usize;
            let l = mul_l(x, &mixing_bijection_l[1][8][j]) as usize;
            tboxes[9][j][x_indice] = t_boxe_clone[9][j][l];
        }
    }
    return ty_tables;
}

 pub fn tylm_tables(
    tboxes: &mut [[[u8; 256]; 16]; 10],
    mixing_bijection_l: &[[[[[u8; 8]; 8]; 16]; 9]; 2],
    mixing_bijection_m: &[[[[[u8; 32]; 32]; 4]; 9]; 2],
) -> [[[[[u8; 4]; 256]; 4]; 4]; 9] {
    let mut ty_tables: [[[[[u8; 4]; 256]; 4]; 4]; 9] = [[[[[0; 4]; 256]; 4]; 4]; 9];
    let t_boxe_clone = tboxes.clone();

    for r in 0..9 {
        for j in 0..4 {
            for x in 0..=255u8 {
                if r > 0 {
                    let x_i = x as usize;
                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4]) as usize;
                    let k = tboxes[r][j * 4 + 0][l];
                    ty_tables[r][j][0][x_i][0] = galois_mul(k, 2);
                    ty_tables[r][j][0][x_i][1] = k;
                    ty_tables[r][j][0][x_i][2] = k;
                    ty_tables[r][j][0][x_i][3] = galois_mul(k, 3);
                    ty_tables[r][j][0][x_i] =
                        mul_m(&ty_tables[r][j][0][x_i], &mixing_bijection_m[0][r][j]);

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 1]) as usize;
                    let k = tboxes[r][j * 4 + 1][l];
                    ty_tables[r][j][1][x_i][0] = galois_mul(k, 3);
                    ty_tables[r][j][1][x_i][1] = galois_mul(k, 2);
                    ty_tables[r][j][1][x_i][2] = k;
                    ty_tables[r][j][1][x_i][3] = k;
                    ty_tables[r][j][1][x_i] =
                        mul_m(&ty_tables[r][j][1][x_i], &mixing_bijection_m[0][r][j]);

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 2]) as usize;
                    let k = tboxes[r][j * 4 + 2][l];
                    ty_tables[r][j][2][x_i][0] = k;
                    ty_tables[r][j][2][x_i][1] = galois_mul(k, 3);
                    ty_tables[r][j][2][x_i][2] = galois_mul(k, 2);
                    ty_tables[r][j][2][x_i][3] = k;
                    ty_tables[r][j][2][x_i] =
                        mul_m(&ty_tables[r][j][2][x_i], &mixing_bijection_m[0][r][j]);

                    let l = mul_l(x, &mixing_bijection_l[1][r - 1][j * 4 + 3]) as usize;

                    let k = tboxes[r][j * 4 + 3][l];
                    ty_tables[r][j][3][x_i][0] = k;
                    ty_tables[r][j][3][x_i][1] = k;
                    ty_tables[r][j][3][x_i][2] = galois_mul(k, 3);
                    ty_tables[r][j][3][x_i][3] = galois_mul(k, 2);
                    ty_tables[r][j][3][x_i] =
                        mul_m(&ty_tables[r][j][3][x_i], &mixing_bijection_m[0][r][j]);
                } else {
                    let x_i = x as usize;
                    let k = tboxes[r][j * 4 + 0][x_i];
                    ty_tables[r][j][0][x_i][0] = galois_mul(k, 2);
                    ty_tables[r][j][0][x_i][1] = k;
                    ty_tables[r][j][0][x_i][2] = k;
                    ty_tables[r][j][0][x_i][3] = galois_mul(k, 3);
                    ty_tables[r][j][0][x_i] =
                        mul_m(&ty_tables[r][j][0][x_i], &mixing_bijection_m[0][r][j]);

                    let k = tboxes[r][j * 4 + 1][x_i];
                    ty_tables[r][j][1][x_i][0] = galois_mul(k, 3);
                    ty_tables[r][j][1][x_i][1] = galois_mul(k, 2);
                    ty_tables[r][j][1][x_i][2] = k;
                    ty_tables[r][j][1][x_i][3] = k;
                    ty_tables[r][j][1][x_i] =
                        mul_m(&ty_tables[r][j][1][x_i], &mixing_bijection_m[0][r][j]);

                    let k = tboxes[r][j * 4 + 2][x_i];
                    ty_tables[r][j][2][x_i][0] = k;
                    ty_tables[r][j][2][x_i][1] = galois_mul(k, 3);
                    ty_tables[r][j][2][x_i][2] = galois_mul(k, 2);
                    ty_tables[r][j][2][x_i][3] = k;
                    ty_tables[r][j][2][x_i] =
                        mul_m(&ty_tables[r][j][2][x_i], &mixing_bijection_m[0][r][j]);

                    let k = tboxes[r][j * 4 + 3][x_i];
                    ty_tables[r][j][3][x_i][0] = k;
                    ty_tables[r][j][3][x_i][1] = k;
                    ty_tables[r][j][3][x_i][2] = galois_mul(k, 3);
                    ty_tables[r][j][3][x_i][3] = galois_mul(k, 2);
                    ty_tables[r][j][3][x_i] =
                        mul_m(&ty_tables[r][j][3][x_i], &mixing_bijection_m[0][r][j]);
                }
            }
        }
    }
    for j in 0..16 {
        for x in 0..=255u8 {
            let x_indice = x as usize;
            let l = mul_l(x, &mixing_bijection_l[1][8][j]) as usize;
            tboxes[9][j][x_indice] = t_boxe_clone[9][j][l];
        }
    }
    return ty_tables;
}
pub fn shift_rows(state: &mut [u8; 16]) {
    let tmp: [u8; 16] = state.clone();
    let init = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut permutation = [0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];
    for i in init.iter().zip(permutation.iter_mut()) {
        state[*i.0] = tmp[*i.1];
    }
}

pub fn inv_shift_rows(state: &mut [u8; 16]) {
    let tmp: [u8; 16] = state.clone();
    let init = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut permutation = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];
    for i in init.iter().zip(permutation.iter_mut()) {
        state[*i.0] = tmp[*i.1];
    }
}
pub fn add_round_key(state: &mut [u8; 16], key_round: &[u8]) {
    for i in 0..16 {
        state[i] = state[i] ^ key_round[i];
    }
}
pub fn sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = SBOX[state[i] as usize];
    }
}
pub fn inv_sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = INV_SBOX[state[i] as usize];
    }
}
// we need to impl galois mul for mix columns https://www.samiam.org/galois.html
pub fn galois_mul(a_: u8, b_: u8) -> u8 {
    let mut p: u8 = 0;
    let mut a = a_;
    let mut b = b_;
    let mut hi_bit: u8 = 0;
    for _i in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        hi_bit = a & 0x80;
        a <<= 1;
        if hi_bit == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

pub fn mix_columns(state: &mut [u8; 16]) {
    let tmp: [u8; 16] = state.clone(); // cpoy from slice or clone
    for i in 0..4 {
        state[i * 4] = galois_mul(tmp[i * 4], 2)
            ^ galois_mul(tmp[i * 4 + 1], 3)
            ^ galois_mul(tmp[i * 4 + 2], 1)
            ^ galois_mul(tmp[i * 4 + 3], 1);
        state[i * 4 + 1] = galois_mul(tmp[i * 4], 1)
            ^ galois_mul(tmp[i * 4 + 1], 2)
            ^ galois_mul(tmp[i * 4 + 2], 3)
            ^ galois_mul(tmp[i * 4 + 3], 1);
        state[i * 4 + 2] = galois_mul(tmp[i * 4], 1)
            ^ galois_mul(tmp[i * 4 + 1], 1)
            ^ galois_mul(tmp[i * 4 + 2], 2)
            ^ galois_mul(tmp[i * 4 + 3], 3);
        state[i * 4 + 3] = galois_mul(tmp[i * 4], 3)
            ^ galois_mul(tmp[i * 4 + 1], 1)
            ^ galois_mul(tmp[i * 4 + 2], 1)
            ^ galois_mul(tmp[i * 4 + 3], 2);
    }
}

pub fn inv_mix_columns(state: &mut [u8; 16]) {
    let tmp: [u8; 16] = state.clone(); // cpoy from slice or clone
    for i in 0..4 {
        state[i * 4] = galois_mul(tmp[i * 4], 14)
            ^ galois_mul(tmp[i * 4 + 1], 11)
            ^ galois_mul(tmp[i * 4 + 2], 13)
            ^ galois_mul(tmp[i * 4 + 3], 9);
        state[i * 4 + 1] = galois_mul(tmp[i * 4], 9)
            ^ galois_mul(tmp[i * 4 + 1], 14)
            ^ galois_mul(tmp[i * 4 + 2], 11)
            ^ galois_mul(tmp[i * 4 + 3], 13);
        state[i * 4 + 2] = galois_mul(tmp[i * 4], 13)
            ^ galois_mul(tmp[i * 4 + 1], 9)
            ^ galois_mul(tmp[i * 4 + 2], 14)
            ^ galois_mul(tmp[i * 4 + 3], 11);
        state[i * 4 + 3] = galois_mul(tmp[i * 4], 11)
            ^ galois_mul(tmp[i * 4 + 1], 13)
            ^ galois_mul(tmp[i * 4 + 2], 9)
            ^ galois_mul(tmp[i * 4 + 3], 14);
    }
}

pub fn rcon(mut n: u8) -> u8 {
    let mut c: u8 = 1;
    if n == 0 {
        return 0;
    }
    while n != 1 {
        c = galois_mul(c, 2);
        n -= 1;
    }
    return c;
}

pub fn rotate_galois(state: &mut [u8; 4]) {
    let tmp: u8 = state[0];
    for i in 0..3 {
        state[i] = state[i + 1];
    }
    state[3] = tmp;
}

pub fn schedule_core(input: &mut [u8; 4], n: u8) {
    rotate_galois(input);
    for i in 0..4 {
        input[i] = SBOX[input[i] as usize];
    }
    input[0] ^= rcon(n);
}
pub fn expend_key(input: &mut [u8; 176], key: &[u8; 16]) {
    // https://www.samiam.org/key-schedule.html

    let mut t: [u8; 4] = [0; 4];
    let mut c: u8 = 16;
    let mut n: u8 = 1;

    for i in 0..16 {
        input[i] = key[i];
    }

    while c < 176 {
        for i in 0..4 {
            t[i] = input[i + (c as usize) - 4]
        }
        if c % 16 == 0 {
            schedule_core(&mut t, n);
            n += 1;
        }
        for i in 0..4 {
            input[c as usize] = input[(c as usize) - 16] ^ t[i];
            c += 1;
        }
    }
}

}


pub use intern::{*}; // to modify ... 


include!(concat!(env!("OUT_DIR"), "/keys.rs"));
include!(concat!(env!("OUT_DIR"), "/tl_boxes.rs"));
include!(concat!(env!("OUT_DIR"), "/tylm_boxes.rs"));
include!(concat!(env!("OUT_DIR"), "/ml_box.rs"));
include!(concat!(env!("OUT_DIR"), "/xor_table.rs"));
include!(concat!(env!("OUT_DIR"), "/xor_ml_tables.rs"));


// pub static  key: [u8; 16] = [
//     0x47, 0x47, 0xf0, 0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e,
//     0x3f,
// ];

// static mut tl_boxes  : [[[u8; 256]; 16]; 10]= t_boxes(&key);



// const l_box :[[[[[u8; 8]; 8]; 16]; 9]; 2] = mixing_bijection_l();
// static m_box  : [[[[[u8; 32]; 32]; 4]; 9]; 2] = mixing_bijection_m();
// static ml_box  : [[[[[u8; 4]; 4]; 256]; 4]; 9] = mixing_bijection_m_l(&m_box, &l_box);

// static tylm_boxes  : [[[[[u8; 4]; 256]; 4]; 4]; 9] = tylm_tables(&mut tl_boxes, &l_box, &m_box);

// static xor_ml_tables  :[[[[u8; 16]; 16]; 96]; 9]  = xor_tables();
// static xor_table : [[[[u8; 16]; 16]; 96]; 9] = xor_tables();



pub fn encryption_block( bytes: &[u8; 16]) -> [u8; 16] {
    
    let mut state: [u8; 16] = *bytes;





    for i in 0..9 {
        shift_rows(&mut state);

        for g in 0..4 {
            let mut ty_i: [[u8; 4]; 4] = [[0; 4]; 4];
            let mut tmp: [[u8; 4]; 4] = [[0; 4]; 4];

            for x in 0..4 {
                ty_i[x] = tylm_boxes[i][g][x][state[g * 4 + x] as usize];
            }

            let xor_ty_i_0 = xor_32b(i, g * 3, &xor_table, &ty_i[0], &ty_i[1]);
            let xor_ty_i_1 = xor_32b(i, g * 3 + 1, &xor_table, &ty_i[2], &ty_i[3]);
            let xor_result_ty_i = xor_32b(i, g * 3 + 2, &xor_table, &xor_ty_i_0, &xor_ty_i_1);

            for r in 0..4 {
                tmp[r] = ml_box[i][g][xor_result_ty_i[r] as usize][r];
            }

            let xor_tmp_0 = xor_32b(i, g * 3, &xor_ml_tables, &tmp[0], &tmp[1]);
            let xor_tmp_1 = xor_32b(i, g * 3 + 1, &xor_ml_tables, &tmp[2], &tmp[3]);
            let xor_result_tmp = xor_32b(i, g * 3 + 2, &xor_ml_tables, &xor_tmp_1, &xor_tmp_0);

            for res_xor in 0..4 {
                state[g * 4 + res_xor] = xor_result_tmp[res_xor];
            }
        }
    }

    shift_rows(&mut state);

    for i in 0..16 {
        state[i] = tl_boxes[9][i][state[i] as usize];
    }
    return  state;

}

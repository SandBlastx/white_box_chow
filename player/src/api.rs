use white_box_lib::{shift_rows, xor_32b};

include!(concat!(env!("OUT_DIR"), "/keys.rs"));
include!(concat!(env!("OUT_DIR"), "/tl_boxes.rs"));
include!(concat!(env!("OUT_DIR"), "/tylm_boxes.rs"));
include!(concat!(env!("OUT_DIR"), "/ml_box.rs"));
include!(concat!(env!("OUT_DIR"), "/xor_table.rs"));
include!(concat!(env!("OUT_DIR"), "/xor_ml_tables.rs"));

pub fn encryption_block(bytes: &[u8; 16]) -> [u8; 16] {
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
    return state;
}

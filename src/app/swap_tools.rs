#[derive(Debug, PartialEq)]
pub enum TrimResult {
    Nothing,
    Left,
    Top,
    Both,
}

pub fn trim_left(b: &mut [u8; 16]) {
    for y in 0..4 {
        for x in 0..3 {
            b[(y * 4) + x] = b[(y * 4) + x + 1];
        }
    }

    b[3] = 0;
    b[7] = 0;
    b[11] = 0;
    b[15] = 0;
}

pub fn trim_top(b: &mut [u8; 16]) {
    for y in 0..3 {
        for x in 0..4 {
            b[(y * 4) + x] = b[((y + 1) * 4) + x];
        }
    }

    b[12] = 0;
    b[13] = 0;
    b[14] = 0;
    b[15] = 0;
}

pub fn need_trim(b: &[u8; 16]) -> TrimResult {
    let _r = (b[0], b[1], b[2], b[3], b[4], b[8], b[12]);
    match (b[0], b[1], b[2], b[3], b[4], b[8], b[12]) {
        (0, 0, 0, 0, 0, 0, 0) => TrimResult::Both,
        (0, 0, 0, 0, _, _, _) => TrimResult::Top,
        (0, _, _, _, 0, 0, 0) => TrimResult::Left,
        _ => TrimResult::Nothing,
    }
}


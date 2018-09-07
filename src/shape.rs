use rand::prelude::*;
use super::rand;

#[derive(Debug)]
pub enum BlockShape {
    ShapeL,
    ShapeRevL,
    ShapeI,
    Shape4,
    ShapeRev4,
    ShapeRect,
}

pub fn fill_block(buffer: &mut [u8; 252], start: usize, shape: BlockShape) {
    match shape {
        BlockShape::ShapeL => {
            // 1 0 0 0
            // 1 0 0 0
            // 1 1 0 0
            // 0 0 0 0
            buffer[start + 0] = 1;
            buffer[start + 1] = 0;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 1;
            buffer[start + 5] = 0;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 1;
            buffer[start + 9] = 1;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 0;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
        BlockShape::ShapeRevL => {
            // 0 1 0 0
            // 0 1 0 0
            // 1 1 0 0
            // 0 0 0 0
            buffer[start + 0] = 0;
            buffer[start + 1] = 1;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 0;
            buffer[start + 5] = 1;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 1;
            buffer[start + 9] = 1;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 0;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
        BlockShape::ShapeI => {
            buffer[start + 0] = 1;
            buffer[start + 1] = 0;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 1;
            buffer[start + 5] = 0;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 1;
            buffer[start + 9] = 0;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 1;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
        BlockShape::Shape4 => {
            buffer[start + 0] = 1;
            buffer[start + 1] = 0;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 1;
            buffer[start + 5] = 1;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 0;
            buffer[start + 9] = 1;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 0;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
        BlockShape::ShapeRev4 => {
            buffer[start + 0] = 0;
            buffer[start + 1] = 1;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 1;
            buffer[start + 5] = 1;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 1;
            buffer[start + 9] = 0;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 0;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
        BlockShape::ShapeRect => {
            buffer[start + 0] = 1;
            buffer[start + 1] = 1;
            buffer[start + 2] = 0;
            buffer[start + 3] = 0;

            buffer[start + 4] = 1;
            buffer[start + 5] = 1;
            buffer[start + 6] = 0;
            buffer[start + 7] = 0;

            buffer[start + 8] = 0;
            buffer[start + 9] = 0;
            buffer[start + 10] = 0;
            buffer[start + 11] = 0;

            buffer[start + 12] = 0;
            buffer[start + 13] = 0;
            buffer[start + 14] = 0;
            buffer[start + 15] = 0;
        }
    }
}

pub fn random_block_type(rng: &mut rand::ThreadRng) -> BlockShape {
    match rng.gen_range(0, 5) {
        0 => BlockShape::ShapeL,
        1 => BlockShape::ShapeRevL,
        2 => BlockShape::ShapeI,
        3 => BlockShape::Shape4,
        4 => BlockShape::ShapeRev4,
        5 => BlockShape::ShapeRect,
        _ => BlockShape::ShapeL,
    }
}


use rand::prelude::*;
use super::rand;

use std::fmt;

use sdl2;
use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use std::time::Duration;

#[derive(Debug)]
enum BlockShape {
    ShapeL,
    ShapeRevL,
    ShapeI,
    Shape4,
    ShapeRev4,
    ShapeRect,
}

pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub buffer: [u8; 252],
    pub rng: rand::ThreadRng,
    // pub empty_cell: sdl2::render::Texture,
    // pub taken_cell: sdl2::render::Texture,
}

impl fmt::Debug for SnakeGame {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.buffer[..].fmt(formatter)
    }
}

impl SnakeGame {
    pub fn fill_next(&mut self) {
        let w = self.width;
        let h = self.height;
        let grid = w * h;
        let block_type = self.random_block_type();
        for i in (grid + 9)..(grid + 18) {
            self.buffer[i - 9] = self.buffer[i];
        }
        self.fill_block(grid + 9, block_type);
    }

    fn fill_block(&mut self, start: usize, shape: BlockShape) {
        match shape {
            BlockShape::ShapeL => {
                self.buffer[start + 0] = 1;
                self.buffer[start + 1] = 0;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 1;
                self.buffer[start + 5] = 0;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 1;
                self.buffer[start + 9] = 1;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 0;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
            BlockShape::ShapeRevL => {
                self.buffer[start + 0] = 0;
                self.buffer[start + 1] = 1;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 0;
                self.buffer[start + 5] = 1;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 1;
                self.buffer[start + 9] = 1;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 0;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
            BlockShape::ShapeI => {
                self.buffer[start + 0] = 1;
                self.buffer[start + 1] = 0;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 1;
                self.buffer[start + 5] = 0;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 1;
                self.buffer[start + 9] = 0;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 1;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
            BlockShape::Shape4 => {
                self.buffer[start + 0] = 1;
                self.buffer[start + 1] = 0;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 1;
                self.buffer[start + 5] = 1;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 0;
                self.buffer[start + 9] = 1;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 0;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
            BlockShape::ShapeRev4 => {
                self.buffer[start + 0] = 0;
                self.buffer[start + 1] = 1;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 1;
                self.buffer[start + 5] = 1;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 1;
                self.buffer[start + 9] = 0;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 0;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
            BlockShape::ShapeRect => {
                self.buffer[start + 0] = 1;
                self.buffer[start + 1] = 1;
                self.buffer[start + 2] = 0;
                self.buffer[start + 3] = 0;

                self.buffer[start + 4] = 1;
                self.buffer[start + 5] = 1;
                self.buffer[start + 6] = 0;
                self.buffer[start + 7] = 0;

                self.buffer[start + 8] = 0;
                self.buffer[start + 9] = 0;
                self.buffer[start + 10] = 0;
                self.buffer[start + 11] = 0;

                self.buffer[start + 12] = 0;
                self.buffer[start + 13] = 0;
                self.buffer[start + 14] = 0;
                self.buffer[start + 15] = 0;
            }
        }
    }

    fn random_block_type(&mut self) -> BlockShape {
        match self.rng.gen_range(0, 5) {
            0 => BlockShape::ShapeL,
            1 => BlockShape::ShapeRevL,
            2 => BlockShape::ShapeI,
            3 => BlockShape::Shape4,
            4 => BlockShape::ShapeRev4,
            5 => BlockShape::ShapeRect,
            _ => BlockShape::ShapeL,
        }
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let buffer = self.buffer;

        // 11 * 20 = 220 / 2 = 110
        // 800 / 2 = 400
        // 400 - 110 = 290 / 2 145
        for y in 0..h {
            for x in 0..w {
                let color = match buffer[(y * w) + x] {
                    1 => Color::RGB(0, 255, 0),
                    _ => Color::RGB(255, 0, 0),
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                    ((x * 20) + 145 + (x * 3)) as i32,
                    ((y * 20) + 40 + (y * 3)) as i32,
                    20, 20,
                )).unwrap();
            }
        }
    }

    pub fn update(&mut self, _key: u16) {
        //
    }
}


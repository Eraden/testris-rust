use super::rand;

use std::fmt;

use sdl2;
use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use std::time::Duration;

use shape;

pub const WAIT: f64 = 10.0;

pub enum Action {
    NoOP,
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}

struct Pos {
    x: u8,
    y: u8,
}

pub struct TetrisGame {
    pub wait: f64,
    pub score: u64,
    pub width: usize,
    pub height: usize,
    pub buffer: [u8; 252],
    pub rng: rand::ThreadRng,
    pub pos: Pos,
}

impl fmt::Debug for TetrisGame {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.buffer[..].fmt(formatter)
    }
}

impl TetrisGame {
    pub fn new() -> TetrisGame {
        TetrisGame {
            rng: rand::thread_rng(),
            wait: WAIT,
            score: 0,
            width: 11,
            height: 20,
            buffer: [
                // grid
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 5
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 10
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 15
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20
                // current block
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                // next block
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                ],
                pos: Pos { x: 5, y: 0 },
        }
    }
    pub fn fill_next(&mut self) {
        let w = self.width;
        let h = self.height;
        let grid = w * h;
        // println!("{:?}", grid);
        let block_type = shape::random_block_type(&mut self.rng);
        // println!("Current shape: {:?}", block_type);
        for i in 0..16 {
            self.buffer[grid + i] = self.buffer[grid + i + 16];
        }
        shape::fill_block(&mut self.buffer, grid + 16, block_type);
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.render_grid(&mut *canvas);
        self.render_current(&mut *canvas);
        self.render_current_preview(&mut *canvas);
        self.render_next_preview(&mut *canvas);
    }

    fn render_grid(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
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

    fn render_current_preview(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let grid_size = w * h;
        let buffer = self.buffer;
        let grid_end_x = (11 * 20) + 145 + (11 * 3);

        for y in 0..4 {
            for x in 0..4 {
                let color = match buffer[grid_size + (y * 4) + x] {
                    1 => Color::RGB(0, 255, 0),
                    _ => Color::RGB(255, 0, 0),
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                        (grid_end_x + (x * 20) + 40 + (x * 3)) as i32,
                        ((y * 20) + 40 + (y * 3)) as i32,
                        20, 20,
                        )).unwrap();
            }
        }
    }

    fn render_next_preview(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let grid_size = w * h;
        let buffer = self.buffer;
        let grid_end_x = (11 * 20) + 145 + (11 * 3);
        for y in 0..4 {
            for x in 0..4 {
                let color = match buffer[grid_size + 16 + (y * 4) + x] {
                    1 => Color::RGB(0, 255, 0),
                    _ => Color::RGB(255, 0, 0),
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                        (grid_end_x + (x * 20) + 40 + (x * 3)) as i32,
                        ((y * 20) + 200 + (y * 3)) as i32,
                        20, 20,
                        )).unwrap();
            }
        }
    }

    fn render_current(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let px = self.pos.x as usize;
        let py = self.pos.y as usize;
        let grid_size = w * h;
        let buffer = self.buffer;

        for y in 0..4 {
            for x in 0..4 {
                let color = match buffer[grid_size + (y * 4) + x] {
                    1 => Color::RGB(0, 255, 0),
                    _ => Color::RGB(255, 0, 0),
                };
                let cx = x + px;
                let cy = y + py;
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                        ((cx * 20) + 145 + (cx * 3)) as i32,
                        ((cy * 20) + 40 + (cy * 3)) as i32,
                        20, 20,
                        )).unwrap();
            }
        }
    }

    pub fn update(&mut self, action: Action) {
    }
}


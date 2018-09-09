use rand;

use std::fmt;

use sdl2;
use sdl2::pixels::Color;

use shape;
use app::swap_tools;
use app::swap_tools::TrimResult;

pub const WAIT: f64 = 10.0_f64;
pub const DECREASE_STEP_DELAY: f64 = 0.01_64;
pub const CELL_DRAW_WIDTH: u32 = 20;
pub const CELL_DRAW_HEIGHT: u32 = 20;

#[derive(Debug, PartialEq)]
pub enum Action {
    NoOP,
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
    Update,
}

#[derive(Debug, PartialEq)]
pub enum AvailabilityResult {
    Free,
    Taken,
}

#[derive(Debug, PartialEq)]
pub enum CollisionResult {
    NoCollision,
    Collide,
}

#[derive(Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct TetrisGame {
    pub step: f64,
    pub step_delay: f64,
    pub score: u64,
    pub width: usize,
    pub height: usize,
    pub buffer: [u8; 252],
    pub rng: rand::ThreadRng,
    pub pos: Pos,
    pub taken_cell: sdl2::pixels::Color,
    pub free_cell: sdl2::pixels::Color,
}

impl fmt::Debug for TetrisGame {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.buffer[..].fmt(formatter)
    }
}

impl TetrisGame {
    pub fn new() -> TetrisGame {
        TetrisGame {
            taken_cell: Color::RGB(200, 0, 0),
            free_cell: Color::RGB(200, 200, 200),
            rng: rand::thread_rng(),
            step: WAIT,
            step_delay: WAIT,
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
        let block_type = shape::random_block_type(&mut self.rng);
        for i in 0..16 {
            self.buffer[grid + i] = self.buffer[grid + i + 16];
        }
        shape::fill_block(&mut self.buffer, grid + 16, block_type);
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.render_grid(&mut *canvas);
        self.render_current_preview(&mut *canvas);
        self.render_next_preview(&mut *canvas);
    }

    fn render_grid(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;

        // 11 * 20 = 220 / 2 = 110
        // 800 / 2 = 400
        // 400 - 110 = 290 / 2 145
        for y in 0..h {
            for x in 0..w {
                let color = match self.is_taken(x, y) {
                    AvailabilityResult::Taken => self.taken_cell,
                    AvailabilityResult::Free => self.free_cell,
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                    ((x * CELL_DRAW_WIDTH as usize) + 145 + (x * 3)) as i32,
                    ((y * CELL_DRAW_HEIGHT as usize) + 40 + (y * 3)) as i32,
                    CELL_DRAW_WIDTH, CELL_DRAW_HEIGHT,
                )).unwrap();
            }
        }
    }

    fn render_current_preview(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let grid_size = w * h;
        let buffer = self.buffer;
        let grid_end_x = (11 * CELL_DRAW_WIDTH as usize) + 145 + (11 * 3);

        for y in 0..4 {
            for x in 0..4 {
                let color = match buffer[grid_size + (y * 4) + x] {
                    1 => self.taken_cell,
                    _ => self.free_cell,
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                    (grid_end_x + (x * CELL_DRAW_WIDTH as usize) + 40 + (x * 3)) as i32,
                    ((y * CELL_DRAW_HEIGHT as usize) + 40 + (y * 3)) as i32,
                    CELL_DRAW_WIDTH, CELL_DRAW_HEIGHT,
                )).unwrap();
            }
        }
    }

    fn render_next_preview(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let w = self.width;
        let h = self.height;
        let grid_size = w * h;
        let buffer = self.buffer;
        let grid_end_x = (11 * CELL_DRAW_HEIGHT as usize) + 145 + (11 * 3);
        for y in 0..4 {
            for x in 0..4 {
                let color = match buffer[grid_size + 16 + (y * 4) + x] {
                    1 => self.taken_cell,
                    _ => self.free_cell,
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                    (grid_end_x + (x * CELL_DRAW_WIDTH as usize) + 40 + (x * 3)) as i32,
                    ((y * CELL_DRAW_HEIGHT as usize) + 200 + (y * 3)) as i32,
                    CELL_DRAW_WIDTH, CELL_DRAW_HEIGHT,
                )).unwrap();
            }
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::MoveDown => {
                let next_pos = Pos { x: self.pos.x, y: self.pos.y + 1 };
                match self.check_collision(next_pos) {
                    CollisionResult::Collide => {}
                    CollisionResult::NoCollision => {
                        self.pos.y += 1;
                    }
                }
            }
            Action::MoveLeft => {
                if self.pos.x == 0 {
                    return;
                }
                let next_pos = Pos { x: self.pos.x - 1, y: self.pos.y };
                match self.check_collision(next_pos) {
                    CollisionResult::Collide => {}
                    CollisionResult::NoCollision => {
                        self.pos.x -= 1;
                    }
                }
            }
            Action::MoveRight => {
                if self.pos.x == self.width - 1 {
                    return;
                }
                let next_pos = Pos { x: self.pos.x + 1, y: self.pos.y };
                match self.check_collision(next_pos) {
                    CollisionResult::Collide => {}
                    CollisionResult::NoCollision => {
                        self.pos.x += 1;
                    }
                }
            }
            Action::Rotate => {
                self.rotate_current();
            }
            Action::Update => {
                self.do_step();
            }
            Action::NoOP => {}
        }
    }

    fn do_step(&mut self) {
        self.step -= 0.1_f64;

        if self.step > 0.0f64 {
            return;
        }
        let next_pos = Pos { x: self.pos.x, y: self.pos.y + 1 };
        match self.check_collision(next_pos) {
            CollisionResult::Collide => {
                self.copy_block();
                self.fill_next();
                self.step_delay = self.step_delay - 0.01f64;
                self.pos.x = 5_usize;
                self.pos.y = 0_usize;
            }
            CollisionResult::NoCollision => {
                self.pos.y += 1_usize;
            }
        }
        self.step = self.step_delay.clone();
    }

    fn copy_block(&mut self) {
        let w: usize = self.width;
        let pos = &self.pos;
        for y in 0..4 {
            for x in 0..4 {
                let cx: usize = pos.x + x;
                let cy: usize = pos.y + y;
                let block: u8 = self.at_block(x, y);
                if block == 1_u8 {
                    self.buffer[(cy * w) + cx] = block;
                }
            }
        }
    }

    pub fn check_collision(&mut self, next_pos: Pos) -> CollisionResult {
        if next_pos.x > self.width - 1_usize {
            return CollisionResult::Collide;
        }
        if next_pos.y > self.height - 1_usize {
            return CollisionResult::Collide;
        }
        for y in 0..4 {
            for x in 0..4 {
                let cx = next_pos.x + x;
                let cy = next_pos.y + y;
                let block = self.at_block(x, y) == 1;
                if cx > self.width - 1 && block {
                    return CollisionResult::Collide;
                }
                if cy > self.height - 1 && block {
                    return CollisionResult::Collide;
                }
                let cell = self.at_cell(next_pos.x + x, next_pos.y + y) == 1;
                if block && cell {
                    return CollisionResult::Collide;
                }
            }
        }
        CollisionResult::NoCollision
    }

    fn at_block(&self, x: usize, y: usize) -> u8 {
        self.buffer[(self.width * self.height) + (y * 4_usize) + x]
    }

    fn at_cell(&self, x: usize, y: usize) -> u8 {
        self.buffer[(y * self.width) + x]
    }

    pub fn is_taken(&self, x: usize, y: usize) -> AvailabilityResult {
        let pos = &self.pos;
        for cy in 0..4 {
            for cx in 0..4 {
                let py = pos.y + cy;
                let px = pos.x + cx;
                let cell = self.at_cell(x, y);
                let block = self.at_block(cx, cy);
                if (cell == 1) || (px == x && py == y && block == 1) {
                    return AvailabilityResult::Taken;
                }
            }
        }
        AvailabilityResult::Free
    }

    pub fn rotate_current(&mut self) {
        let b = &mut self.buffer;
        let w = self.width;
        let h = self.height;
        let s = w * h;
        let mut swap = [
            b[s + 3], b[s + 7], b[s + 11], b[s + 15],
            b[s + 2], b[s + 6], b[s + 10], b[s + 14],
            b[s + 1], b[s + 5], b[s + 9], b[s + 13],
            b[s + 0], b[s + 4], b[s + 8], b[s + 12],
        ];

        loop {
            match swap_tools::need_trim(&swap) {
                TrimResult::Nothing => break,
                TrimResult::Left => swap_tools::trim_left(&mut swap),
                TrimResult::Top => swap_tools::trim_top(&mut swap),
                TrimResult::Both => {
                    swap_tools::trim_left(&mut swap);
                    swap_tools::trim_top(&mut swap);
                }
            }
        }

        for i in 0..16 {
            b[s + i] = swap[i];
        }
    }
}


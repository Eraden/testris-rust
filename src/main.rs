extern crate rand;
extern crate sdl2;

pub mod app;
pub mod shape;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use app::tetris_game::{ TetrisGame, Action };

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut app = TetrisGame::new();
    app.fill_next();
    app.fill_next();
    app.clear(&mut canvas);

    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let sleep_time = Duration::new(0, 1_000_000_000u32 / 60);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        app.update(Action::Rotate);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        app.update(Action::MoveDown);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        app.update(Action::MoveRight);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        app.update(Action::MoveLeft);
                    },
                    _ => {
                        app.update(Action::NoOP);
                    }
            }
        }
        app.update(Action::Update);
        app.render(&mut canvas);
        canvas.present();
        ::std::thread::sleep(sleep_time);
    }
}

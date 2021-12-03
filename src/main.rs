extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const BACKGROUND: Color = Color::RGB(0, 255, 255);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut pos_x: i32 = 100;
    let mut pos_y: i32 = 100;

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BACKGROUND);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        canvas.set_draw_color(BACKGROUND);
        canvas.clear();

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.fill_rect(Rect::new(pos_x, pos_y, 20, 20)).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => pos_y -= 5,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => pos_y += 5,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => pos_x -= 5,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => pos_x += 5,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

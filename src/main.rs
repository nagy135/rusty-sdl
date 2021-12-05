extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MOVE_MULTIPLIER: i32 = 5;
const BACKGROUND: Color = Color::RGB(0, 255, 255);

struct Snake {
    body: Vec<(i32, i32)>,
    heading: (i32, i32),
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("snejk", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(BACKGROUND);
    canvas.clear();
    canvas.present();

    let mut snake = Snake {
        body: vec![((WIDTH as i32) / 2, (HEIGHT as i32) / 2)],
        heading: (1, 0),
    };

    let mut food = Food { x: 100, y: 100 };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut color_i = 0;
    'running: loop {
        // clear bg
        canvas.set_draw_color(BACKGROUND);
        canvas.clear();

        // prepare color
        color_i = (color_i + 1) % 255;
        canvas.set_draw_color(Color::RGB(color_i, 64, 255 - color_i));

        // draw body
        for (x, y) in &snake.body {
            canvas.fill_rect(Rect::new(*x, *y, 20, 20)).unwrap();
        }

        // perform move
        for piece in snake.body.iter_mut() {
            *piece = (
                piece.0 + snake.heading.0 * MOVE_MULTIPLIER,
                piece.1 + snake.heading.1 * MOVE_MULTIPLIER,
            );
        }

        // key handling
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
                } => snake.heading = (0, -1),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => snake.heading = (0, 1),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => snake.heading = (-1, 0),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => snake.heading = (1, 0),
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

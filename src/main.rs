extern crate sdl2;

#[allow(unused_imports)]
use chrono::{offset::Utc, DateTime, Timelike};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const BLOCK_SIZE: u32 = 20;

const TICK_RATE: i64 = 100; // tick every N milis

const BACKGROUND: Color = Color::RGB(0, 255, 255);
const RED: Color = Color::RGB(255, 0, 0);

struct Snake {
    body: Vec<(i32, i32)>,
    heading: (i32, i32),
}

#[derive(Debug)]
struct Food {
    x: i32,
    y: i32,
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

    let mut rng = rand::thread_rng();
    let mut time: Option<i64> = None;

    let mut snake = Snake {
        body: vec![((WIDTH as i32) / 2, (HEIGHT as i32) / 2)],
        heading: (1, 0),
    };

    let mut food = Food { x: 100, y: 100 };

    // lengtens next frame after food eaten
    let mut lengten = false;

    let mut tail_piece;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut color_i = 0;
    let mut repaint;

    'running: loop {
        repaint = false;
        let now = Utc::now().timestamp_millis();
        match time {
            Some(time_val) => {
                if now - time_val >= TICK_RATE {
                    repaint = true;
                    time = Some(now);
                }
            }
            None => {
                repaint = true;
                time = Some(now);
            }
        }
        if repaint {
            time = Some(now);
            // clear bg
            canvas.set_draw_color(BACKGROUND);
            canvas.clear();

            // prepare color
            color_i = (color_i + 1) % 255;
            canvas.set_draw_color(Color::RGB(color_i, 64, 255 - color_i));

            // draw body
            for (x, y) in &snake.body {
                canvas
                    .fill_rect(Rect::new(*x, *y, BLOCK_SIZE, BLOCK_SIZE))
                    .unwrap();
            }

            // draw food
            canvas.set_draw_color(RED);
            canvas
                .fill_rect(Rect::new(food.x, food.y, BLOCK_SIZE, BLOCK_SIZE))
                .unwrap();

            // capture last piece
            tail_piece = snake.body[snake.body.len() - 1];

            // perform move
            for piece in snake.body.iter_mut() {
                *piece = (
                    piece.0 + snake.heading.0 * BLOCK_SIZE as i32,
                    piece.1 + snake.heading.1 * BLOCK_SIZE as i32,
                );
            }
            // println!("body {:?}", snake.body);
            if lengten {
                snake.body.push(tail_piece);
                lengten = false;
            }

            // handle food
            if snake.body[0].0 >= food.x
                && snake.body[0].0 <= food.x + BLOCK_SIZE as i32
                && snake.body[0].1 >= food.y
                && snake.body[0].1 <= food.y + BLOCK_SIZE as i32
            {
                lengten = true;
                food.x = (rng.gen::<u32>() % (WIDTH + BLOCK_SIZE)) as i32;
                food.y = (rng.gen::<u32>() % (HEIGHT + BLOCK_SIZE)) as i32;
            }
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

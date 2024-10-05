mod agent;
mod environment;
mod utils;
use rand::{thread_rng, Rng};
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 450;
const SNAKE_SIZE: f32 = 20.0;

pub fn app() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Moving snake head")
        .build();

    let mut snake_head = Rectangle::new(400.0, 225.0, 20.0, 20.0);

    let speed = 0.09;

    let mut should_quit = false;
    let mut food = generate_food();

    let rect = Rectangle {
        x: 300.0,
        y: 300.0,
        width: 200.0,
        height: 50.0,
    };
    let mut food_timer = 0.0;

    while !rl.window_should_close() && !should_quit {
        let delta_time = rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            snake_head.x = (snake_head.x + speed).min(WINDOW_WIDTH as f32 - SNAKE_SIZE);
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            snake_head.x = (snake_head.x - speed).max(0.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            snake_head.y = (snake_head.y - speed).max(0.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            snake_head.y = (snake_head.y + speed).min(WINDOW_HEIGHT as f32 - SNAKE_SIZE);
        }

        let mut d = rl.begin_drawing(&thread);
        food_timer += delta_time;
        if food_timer >= 5.0 {
            food = generate_food();
            food_timer = 0.0;
        }

        d.clear_background(Color::RAYWHITE);
        d.draw_rectangle_rec(snake_head, Color::GREEN);
        d.draw_rectangle_rec(food, Color::BLACK);
        d.draw_text("Use arrow keys to move", 20, 20, 20, Color::GRAY);

        if d.gui_button(rect, Some(rstr!("Quit"))) {
            should_quit = true;
        }
    }
}

fn generate_food() -> Rectangle {
    Rectangle::new(
        thread_rng().gen_range(0.0..800.0),
        thread_rng().gen_range(0.0..450.0),
        10.0,
        10.0,
    )
}

mod bot;
mod field;

use std::thread::sleep;
use std::time::Duration;
use crate::bot::Bot;
use crate::bot::random::RandomBot;
use crate::field::Field;

fn play() -> i32 {
    let mut field = Field::new();
    field.fill();
    let mut bot = RandomBot::new();

    // field.draw();

    let mut turn_counter = 1;

    loop {
        let (x, y) = bot.turn(&field);
        if !field.shoot(x, y) {
            turn_counter += 1;
        }

        // sleep(Duration::from_millis(250));
        // field.draw();
        if field.game_over() {
            break;
        }
    }

    turn_counter
}

fn main() {
    let mut vec = Vec::with_capacity(1000);

    for _ in 0..1000 {
        vec.push(play());
    }

    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);

    println!("Bot won in {avg} moves on average");
}
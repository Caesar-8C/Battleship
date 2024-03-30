mod bot;
mod field;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use crate::bot::Bot;
use crate::bot::random::RandomBot;
use crate::field::{Field, ShotResult};

fn play(draw: bool) -> i32 {
    let mut field = Field::new();
    field.fill();
    let mut bot = RandomBot::new();

    if draw {
        field.draw();
    }

    let mut turn_counter = 1;

    loop {
        let (x, y) = bot.turn();
        let shot_result = field.shoot(x, y);
        if shot_result == ShotResult::Miss {
            turn_counter += 1;
        }
        bot.shot_result(x, y, shot_result);

        if draw {
            sleep(Duration::from_millis(250));
            field.draw();
        }

        if field.game_over() {
            break;
        }
    }

    turn_counter
}

fn main() {
    let mut vec = Vec::with_capacity(1000);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "draw" {
        vec.push(play(true));
    } else {
        for _ in 0..1000 {
            vec.push(play(false));
        }
    }

    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);

    println!("Bot won in {avg} moves on average");
}

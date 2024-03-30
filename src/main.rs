mod bot;
mod field;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use crate::bot::alina::AlinaBot;
use crate::bot::Bot;
use crate::bot::random::RandomBot;
use crate::field::{Field, ShotResult};

fn play(draw: bool, bot: &mut dyn Bot) -> i32 {
    let mut field = Field::new();
    field.fill();

    if draw {
        field.draw();
    }

    let mut turn_counter = 1;

    loop {
        let c = bot.turn();
        let shot_result = field.shoot(c);
        if shot_result == ShotResult::Miss {
            turn_counter += 1;
        }
        bot.shot_result(c, shot_result);

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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1] {
            _ if args[1] == "random" => {
                let moves = play(true, &mut RandomBot::new());
                println!("RandomBot won in {moves} moves");
            }
            _ if args[1] == "alina" => {
                let moves = play(true, &mut AlinaBot::new());
                println!("AlinaBot won in {moves} moves");
            }
            _ => return,
        }
        return;
    }

    let mut vec = Vec::with_capacity(1000);
    for _ in 0..1000 {
        vec.push(play(false, &mut RandomBot::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("RandomBot won in {avg} moves on average");

    vec.clear();
    for _ in 0..1000 {
        vec.push(play(false, &mut AlinaBot::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("AlinaBot won in {avg} moves on average");
}

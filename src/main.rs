mod bot;
mod field;

use crate::bot::alina::AlinaBot;
use crate::bot::bot1::Bot1;
use crate::bot::random::RandomBot;
use crate::bot::Bot;
use crate::field::{Field, ShotResult};
use std::env;
use std::thread::sleep;
use std::time::Duration;
use crate::bot::bot2::Bot2;
use crate::bot::bot3::Bot3;
use crate::bot::marching::MarchingBot;
use crate::bot::three_step::ThreeStepBot;

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

    let mut testnum = 1000;

    if args.len() > 1 {
        match args[1] {
            _ if args[1] == "random" => {
                let moves = play(true, &mut RandomBot::new());
                println!("RandomBot won in {moves} moves");
                return;
            }
            _ if args[1] == "alina" => {
                let moves = play(true, &mut AlinaBot::new());
                println!("AlinaBot won in {moves} moves");
                return;
            }
            _ if args[1] == "bot1" => {
                let moves = play(true, &mut Bot1::new());
                println!("Bot1 won in {moves} moves");
                return;
            }
            _ if args[1] == "bot2" => {
                let moves = play(true, &mut Bot2::new());
                println!("Bot2 won in {moves} moves");
                return;
            }
            _ if args[1] == "bot3" => {
                let moves = play(true, &mut Bot3::new());
                println!("Bot3 won in {moves} moves");
                return;
            }
            _ if args[1] == "marching" => {
                let moves = play(true, &mut MarchingBot::new());
                println!("MarchingBot won in {moves} moves");
                return;
            }
            _ if args[1] == "three_step" => {
                if args.len() > 2 {
                    let steps = args[2].clone().parse().unwrap();
                    let moves = play(true, &mut ThreeStepBot::new(steps));
                    println!("ThreeStepBot with {steps} steps won in {moves} moves");
                }
                return;
            }
            _ => testnum = args[1].parse().unwrap_or(testnum),
        }
    }

    let mut vec = Vec::with_capacity(testnum);
    for _ in 0..testnum {
        vec.push(play(false, &mut RandomBot::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("RandomBot won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut AlinaBot::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("AlinaBot won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut Bot1::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("Bot1 won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut Bot2::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("Bot2 won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut Bot3::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("Bot3 won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut MarchingBot::new()));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("MarchingBot won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut ThreeStepBot::new(1)));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("ThreeStepBot with 1 steps won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut ThreeStepBot::new(2)));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("ThreeStepBot with 2 steps won in {avg} moves on average");

    vec.clear();
    for _ in 0..testnum {
        vec.push(play(false, &mut ThreeStepBot::new(3)));
    }
    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);
    println!("ThreeStepBot with 3 steps won in {avg} moves on average");
}

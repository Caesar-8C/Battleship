mod bot;
mod field;

use crate::bot::alina::AlinaBot;
use crate::bot::bot1::Bot1;
use crate::bot::bot2::Bot2;
use crate::bot::bot3::Bot3;
use crate::bot::marching::MarchingBot;
use crate::bot::random::RandomBot;
use crate::bot::three_step::ThreeStepBot;
use crate::bot::Bot;
use crate::field::{Field, ShotResult};
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn get_bot(bot_name: &str) -> Box<dyn Bot> {
    match bot_name {
        name if name == "RandomBot" => Box::new(RandomBot::new()),
        name if name == "AlinaBot" => Box::new(AlinaBot::new()),
        name if name == "Bot1" => Box::new(Bot1::new()),
        name if name == "Bot2" => Box::new(Bot2::new()),
        name if name == "Bot3" => Box::new(Bot3::new()),
        name if name == "MarchingBot" => Box::new(MarchingBot::new()),
        name if name == "ThreeStepBot1" => Box::new(ThreeStepBot::new(1)),
        name if name == "ThreeStepBot2" => Box::new(ThreeStepBot::new(2)),
        name if name == "ThreeStepBot3" => Box::new(ThreeStepBot::new(3)),
        _ => Box::new(RandomBot::new()),
    }
}

fn play(draw: bool, bot: &mut Box<dyn Bot>) -> i32 {
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
        let moves = play(true, &mut get_bot(&args[1]));
        println!("{} won in {moves} moves", args[1]);
    }

    let mut bots: Vec<&str> = vec![
        "RandomBot",
        "AlinaBot",
        "Bot1",
        "Bot2",
        "Bot3",
        "MarchingBot",
        "ThreeStepBot1",
        "ThreeStepBot2",
        "ThreeStepBot3",
    ];

    let mut vec = Vec::with_capacity(testnum);

    bots.iter().for_each(|bot_name| {
        for _ in 0..testnum {
            let mut bot = get_bot(bot_name.clone());
            vec.push(play(false, &mut bot));
        }
        let sum: i32 = vec.iter().sum();
        let avg = sum as f64 / (vec.len() as f64);
        vec.clear();
        println!("{} won in {avg} moves on average", bot_name);
    });
}

use crate::bot::{Bot, BotCell};
use rand::Rng;
use crate::field::ShotResult;

pub struct RandomBot {
    field: Vec<Vec<BotCell>>,
}

impl RandomBot {
    pub fn new() -> Self {
        Self {
            field: vec![vec![BotCell::Value(0); 10]; 10],
        }
    }
}

impl Bot for RandomBot {
    fn turn(&mut self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        loop {
            let (x, y): (usize, usize) = (rng.gen::<usize>()%10, rng.gen::<usize>()%10);
            if let BotCell::Value(_) = self.field[x][y] {
                return (x as i32, y as i32);
            }
        }
    }

    fn shot_result(&mut self, x: i32, y: i32, result: ShotResult) {
        match result {
            ShotResult::Hit => self.field[x as usize][y as usize] = BotCell::Hit,
            ShotResult::Miss => self.field[x as usize][y as usize] = BotCell::Miss,
            _ => (),
        }
    }
}

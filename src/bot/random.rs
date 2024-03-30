use crate::bot::{Bot, BotCell};
use rand::Rng;
use crate::field::{Coord, ShotResult};

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
    fn turn(&mut self) -> Coord {
        let mut rng = rand::thread_rng();
        loop {
            let (x, y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if let BotCell::Value(_) = self.field[x as usize][y as usize] {
                return Coord{x, y};
            }
        }
    }

    fn shot_result(&mut self, c: Coord, result: ShotResult) {
        match result {
            ShotResult::Hit => self.field[c.x as usize][c.y as usize] = BotCell::Hit,
            ShotResult::Miss => self.field[c.x as usize][c.y as usize] = BotCell::Miss,
            ShotResult::Kill => {
                self.field[c.x as usize][c.y as usize] = BotCell::Hit;
            }
        }
    }
}

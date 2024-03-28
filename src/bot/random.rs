use crate::bot::{Bot, BotCell};
use crate::{Field, FieldCell};
use rand::Rng;

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
    fn turn(&mut self, field: &Field) -> (usize, usize) {
        self.field = vec![vec![BotCell::Value(0); 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                match field.get(i, j) {
                    FieldCell::Ship(true) => self.field[i][j] = BotCell::Hit,
                    FieldCell::NoShip(true) => self.field[i][j] = BotCell::Miss,
                    _ => (),
                }
            }
        }

        let mut rng = rand::thread_rng();
        loop {
            let (x, y): (usize, usize) = (rng.gen::<usize>()%10, rng.gen::<usize>()%10);
            if let BotCell::Value(_) = self.field[x][y] {
                return (x, y);
            }
        }
    }
}
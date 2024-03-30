use crate::bot::{Bot, BotCell};
use rand::Rng;
use crate::bot::BotCell::{Miss, Value, Hit};
use crate::field::{Coord, Direction, ShotResult};
use crate::field::Direction::{Down, Left, Right, Up};

pub struct RandomBot {
    field: Vec<Vec<BotCell>>,
}

impl RandomBot {
    pub fn new() -> Self {
        Self {
            field: vec![vec![Value(0); 10]; 10],
        }
    }

    fn mark_dead(&mut self, c: Coord, direction: Direction) {
        if (0..=9).contains(&c.x) && (0..=9).contains(&c.y) {
            if self.field[c.x_u()][c.y_u()] == Hit {
                self.mark_miss(c.next(Up));
                self.mark_miss(c.next(Down));
                self.mark_miss(c.next(Left));
                self.mark_miss(c.next(Right));
                self.mark_miss(c.next(Up).next(Right));
                self.mark_miss(c.next(Up).next(Left));
                self.mark_miss(c.next(Down).next(Right));
                self.mark_miss(c.next(Down).next(Left));

                self.mark_dead(c.next(direction), direction);
            }
        }
    }

    fn mark_miss(&mut self, c: Coord) {
        if (0..=9).contains(&c.x) && (0..=9).contains(&c.y) {
            if let Value(_) = self.field[c.x_u()][c.y_u()] {
                self.field[c.x_u()][c.y_u()] = Miss;
            }
        }
    }
}

impl Bot for RandomBot {
    fn turn(&mut self) -> Coord {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..10);
            let y = rng.gen_range(0..10);
            let c = Coord { x, y };
            if let Value(_) = self.field[c.x_u()][c.y_u()] {
                return c;
            }
        }
    }

    fn shot_result(&mut self, c: Coord, result: ShotResult) {
        match result {
            ShotResult::Hit => self.field[c.x_u()][c.y_u()] = Hit,
            ShotResult::Miss => self.field[c.x_u()][c.y_u()] = Miss,
            ShotResult::Kill => {
                self.field[c.x_u()][c.y_u()] = Hit;
                Direction::ALL.iter().for_each(|direction| self.mark_dead(c, *direction));
            }
        }
    }
}

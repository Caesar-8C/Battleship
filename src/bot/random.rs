use crate::bot::BotCell::{Hit, Miss, Value};
use crate::bot::{Bot, BotCell};
use crate::field::Direction::{Down, Left, Right, Up};
use crate::field::{Coord, Direction, ShotResult};
use rand::Rng;

pub struct RandomBot {
    field: Vec<Vec<BotCell>>,
}

impl RandomBot {
    pub fn new() -> Self {
        Self {
            field: vec![vec![Value(0); 10]; 10],
        }
    }

    fn field_contains(&self, c: Coord) -> bool {
        (0..=9).contains(&c.x) && (0..=9).contains(&c.y)
    }

    fn mark_dead(&mut self, c: Coord, direction: Direction) {
        if self.field_contains(c) {
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
        if self.field_contains(c) {
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
                Direction::ALL
                    .iter()
                    .for_each(|direction| self.mark_dead(c, *direction));
            }
        }
    }
}

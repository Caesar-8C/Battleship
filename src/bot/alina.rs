use crate::bot::{Bot, BotCell};
use rand::Rng;
use crate::bot::BotCell::{Miss, Value, Hit};
use crate::field::{Coord, Direction, ShotResult};
use crate::field::Direction::{Down, Left, Right, Up};

pub struct AlinaBot {
    field: Vec<Vec<BotCell>>,
}

impl AlinaBot {
    pub fn new() -> Self {
        Self {
            field: vec![vec![Value(0); 10]; 10],
        }
    }

    fn mark_dead(&mut self, c: Coord, direction: Direction) {
        if (0..=9).contains(&c.x) && (0..=9).contains(&c.y) {
            if self.field[c.x as usize][c.y as usize] == Hit {
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
            if let Value(_) = self.field[c.x as usize][c.y as usize] {
                self.field[c.x as usize][c.y as usize] = Miss;
            }
        }
    }
}

impl Bot for AlinaBot {
    fn turn(&mut self) -> Coord {
        let mut rng = rand::thread_rng();
        loop {
            let (x, y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if let Value(_) = self.field[x as usize][y as usize] {
                return Coord { x, y };
            }
        }
    }

    fn shot_result(&mut self, c: Coord, result: ShotResult) {
        match result {
            ShotResult::Hit => self.field[c.x as usize][c.y as usize] = Hit,
            ShotResult::Miss => self.field[c.x as usize][c.y as usize] = Miss,
            ShotResult::Kill => {
                self.field[c.x as usize][c.y as usize] = Hit;
                Direction::ALL.iter().for_each(|direction| self.mark_dead(c, *direction));
            }
        }
    }
}

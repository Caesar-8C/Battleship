use crate::bot::BotCell::{Hit, Miss, Value};
use crate::bot::{Bot, BotCell};
use crate::field::Direction::{Down, Left, Right, Up};
use crate::field::{Coord, Direction, ShotResult};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Bot1 {
    field: Vec<Vec<BotCell>>,
    first_hit: Option<Coord>,
}

impl Bot1 {
    pub fn new() -> Self {
        Self {
            field: vec![vec![Value(10); 10]; 10],
            first_hit: None,
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

    fn devalue(&mut self, c: Coord) {
        if (0..=9).contains(&c.x) && (0..=9).contains(&c.y) {
            if let Value(v) = self.field[c.x_u()][c.y_u()] {
                self.field[c.x_u()][c.y_u()] = Value(v - 1);
            }
        }
    }

    fn pick_target(&self) -> Coord {
        let mut vec = Vec::new();
        let mut maxval = 0;
        for x in 0..10 {
            for y in 0..10 {
                let c = Coord { x, y };
                if let Value(v) = self.field[c.x_u()][c.y_u()] {
                    if v == maxval {
                        vec.push(c);
                    } else if v > maxval {
                        vec.clear();
                        vec.push(c);
                        maxval = v;
                    }
                }
            }
        }

        *vec.choose(&mut rand::thread_rng()).unwrap()
    }

    fn march(&self, c: Coord) -> Coord {
        let mut vec = vec![Left, Right, Up, Down];
        vec.shuffle(&mut thread_rng());

        if let Some(res) = self.march_direction(c, vec[0]) {
            return res;
        }
        if let Some(res) = self.march_direction(c, vec[1]) {
            return res;
        }
        if let Some(res) = self.march_direction(c, vec[2]) {
            return res;
        }
        if let Some(res) = self.march_direction(c, vec[3]) {
            return res;
        }
        panic!("Bot logic failure");
    }

    fn march_direction(&self, c: Coord, direction: Direction) -> Option<Coord> {
        if !(0..=9).contains(&c.x) || !(0..=9).contains(&c.y) {
            return None;
        }
        match self.field[c.x_u()][c.y_u()] {
            Hit => self.march_direction(c.next(direction), direction),
            Miss => None,
            Value(_) => Some(c),
        }
    }
}

impl Bot for Bot1 {
    fn turn(&mut self) -> Coord {
        match self.first_hit {
            Some(c) => self.march(c),
            None => self.pick_target(),
        }
    }

    fn shot_result(&mut self, c: Coord, result: ShotResult) {
        match result {
            ShotResult::Hit => {
                self.field[c.x_u()][c.y_u()] = Hit;
                self.mark_miss(c.next(Up).next(Left));
                self.mark_miss(c.next(Up).next(Right));
                self.mark_miss(c.next(Down).next(Left));
                self.mark_miss(c.next(Down).next(Right));
                if self.first_hit.is_none() {
                    self.first_hit = Some(c);
                }
            }
            ShotResult::Miss => {
                self.field[c.x_u()][c.y_u()] = Miss;
                self.devalue(c.next(Up));
                self.devalue(c.next(Down));
                self.devalue(c.next(Left));
                self.devalue(c.next(Right));
                self.devalue(c.next(Up).next(Left));
                self.devalue(c.next(Up).next(Right));
                self.devalue(c.next(Down).next(Left));
                self.devalue(c.next(Down).next(Right));
            }
            ShotResult::Kill => {
                self.field[c.x_u()][c.y_u()] = Hit;
                self.first_hit = None;
                Direction::ALL
                    .iter()
                    .for_each(|direction| self.mark_dead(c, *direction));
            }
        }
    }
}

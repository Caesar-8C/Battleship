use crate::bot::BotCell::{Hit, Miss, Value};
use crate::bot::{Bot, BotCell};
use crate::field::Direction::{Down, Left, Right, Up};
use crate::field::{Coord, Direction, ShotResult};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Bot2 {
    field: Vec<Vec<BotCell>>,
    first_hit: Option<Coord>,
}

impl Bot2 {
    pub fn new() -> Self {
        Self {
            field: vec![vec![Value(0); 10]; 10],
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

    fn evaluate(&mut self) {
        for x in 0..10 {
            for y in 0..10 {
                let c = Coord { x, y };
                if let Value(_) = self.field[c.x_u()][c.y_u()] {
                    self.evaluate_cell(c);
                }
            }
        }
    }

    fn evaluate_cell(&mut self, c: Coord) {
        let mut counter = 0;
        self.evaluate_neighbor(c.next(Up), &mut counter);
        self.evaluate_neighbor(c.next(Down), &mut counter);
        self.evaluate_neighbor(c.next(Left), &mut counter);
        self.evaluate_neighbor(c.next(Right), &mut counter);
        self.evaluate_neighbor(c.next(Up).next(Left), &mut counter);
        self.evaluate_neighbor(c.next(Up).next(Right), &mut counter);
        self.evaluate_neighbor(c.next(Down).next(Left), &mut counter);
        self.evaluate_neighbor(c.next(Down).next(Right), &mut counter);

        self.field[c.x_u()][c.y_u()] = Value(counter);
    }

    fn evaluate_neighbor(&self, c: Coord, counter: &mut i32) {
        if (0..=9).contains(&c.x) && (0..=9).contains(&c.y) {
            match self.field[c.x_u()][c.y_u()] {
                Miss => {
                    if *counter < 10 {
                        *counter += 1;
                    }
                }
                Hit => {
                    *counter = 10;
                }
                Value(_) => (),
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

        *vec.choose(&mut thread_rng()).unwrap()
    }

    fn march(&self, c: Coord) -> Coord {
        let mut vec = [Left, Right, Up, Down];
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

impl Bot for Bot2 {
    fn turn(&mut self) -> Coord {
        match self.first_hit {
            Some(c) => self.march(c),
            None => {
                self.evaluate();
                self.pick_target()
            }
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
            ShotResult::Miss => self.field[c.x_u()][c.y_u()] = Miss,
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

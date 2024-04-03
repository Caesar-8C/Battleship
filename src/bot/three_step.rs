use crate::bot::BotCell::{Hit, Miss, Value};
use crate::bot::{Bot, BotCell};
use crate::field::Direction::{Down, Left, Right, Up};
use crate::field::{Coord, Direction, ShotResult};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct ThreeStepBot {
    field: Vec<Vec<BotCell>>,
    first_hit: Option<Coord>,
    steps: i32,
}

impl ThreeStepBot {
    pub fn new(steps: i32) -> Self {
        Self {
            field: vec![vec![Value(0); 10]; 10],
            first_hit: None,
            steps,
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

    fn pick_target(&mut self) -> Coord {
        let mut vec = Vec::new();

        self.first_step(&mut vec);
        if self.steps > 1 {
            self.second_step(&mut vec);
        }
        if self.steps > 2 {
            self.third_step(&mut vec);
        }

        *vec.choose(&mut thread_rng()).unwrap()
    }

    fn first_step(&mut self, vec: &mut Vec<Coord>) {
        for x in 0..10 {
            for y in 0..10 {
                let c = Coord { x, y };
                if let Value(_) = self.field[c.x_u()][c.y_u()] {
                    self.evaluate_cell(c, 1);
                }
            }
        }

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
    }

    fn second_step(&mut self, vec: &mut Vec<Coord>) {
        vec.iter_mut().for_each(|c| {
            self.evaluate_cell(*c, 2);
        });

        let mut new_vec = Vec::new();
        let mut maxval = 0;

        vec.iter().for_each(|c| {
            if let Value(v) = self.field[c.x_u()][c.y_u()] {
                if v == maxval {
                    new_vec.push(*c);
                } else if v > maxval {
                    new_vec.clear();
                    new_vec.push(*c);
                    maxval = v;
                }
            }
        });

        *vec = new_vec;
    }

    fn evaluate_cell(&mut self, c: Coord, step: i32) {
        let mut counter_h = 0;
        let mut counter_v = 0;
        self.march_direction(c.next(Up), Up, &mut counter_v);
        self.march_direction(c.next(Down), Down, &mut counter_v);
        self.march_direction(c.next(Left), Left, &mut counter_h);
        self.march_direction(c.next(Right), Right, &mut counter_h);

        if step == 1 {
            self.field[c.x_u()][c.y_u()] = Value(counter_h.max(counter_v));
        }
        if step == 2 {
            self.field[c.x_u()][c.y_u()] = Value(counter_h.min(counter_v));
        }
    }

    fn third_step(&mut self, vec: &mut Vec<Coord>) {
        vec.iter_mut().for_each(|c| {
            self.evaluate_cell_third(*c);
        });

        let mut new_vec = Vec::new();
        let mut maxval = 0;

        vec.iter().for_each(|c| {
            if let Value(v) = self.field[c.x_u()][c.y_u()] {
                if v == maxval {
                    new_vec.push(*c);
                } else if v > maxval {
                    new_vec.clear();
                    new_vec.push(*c);
                    maxval = v;
                }
            }
        });

        *vec = new_vec;
    }

    fn evaluate_cell_third(&mut self, c: Coord) {
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
        if self.field_contains(c) {
            match self.field[c.x_u()][c.y_u()] {
                Value(_) => {
                    *counter += 1;
                }
                _ => (),
            }
        }
    }

    fn march_direction(
        &self,
        c: Coord,
        direction: Direction,
        counter: &mut i32,
    ) {
        if self.field_contains(c) {
            if let Value(_) = self.field[c.x_u()][c.y_u()] {
                *counter += 1;
                self.march_direction(c.next(direction), direction, counter);
            }
        }
    }

    fn march_after_hit(&self, c: Coord) -> Coord {
        let mut vec = [Left, Right, Up, Down];
        vec.shuffle(&mut thread_rng());

        if let Some(res) = self.march_direction_after_hit(c, vec[0]) {
            return res;
        }
        if let Some(res) = self.march_direction_after_hit(c, vec[1]) {
            return res;
        }
        if let Some(res) = self.march_direction_after_hit(c, vec[2]) {
            return res;
        }
        if let Some(res) = self.march_direction_after_hit(c, vec[3]) {
            return res;
        }
        panic!("Bot logic failure");
    }

    fn march_direction_after_hit(
        &self,
        c: Coord,
        direction: Direction,
    ) -> Option<Coord> {
        if !(0..=9).contains(&c.x) || !(0..=9).contains(&c.y) {
            return None;
        }
        match self.field[c.x_u()][c.y_u()] {
            Hit => self.march_direction_after_hit(c.next(direction), direction),
            Miss => None,
            Value(_) => Some(c),
        }
    }
}

impl Bot for ThreeStepBot {
    fn turn(&mut self) -> Coord {
        match self.first_hit {
            Some(c) => self.march_after_hit(c),
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

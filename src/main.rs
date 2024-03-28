mod bot;
mod field;

use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use crate::bot::Bot;
use crate::bot::random::RandomBot;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum FieldCell {
    Ship(bool),
    NoShip(bool),
}

impl fmt::Display for FieldCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldCell::Ship(false) => write!(f, "O"),
            FieldCell::Ship(true) => write!(f, "X"),
            FieldCell::NoShip(false) => write!(f, " "),
            FieldCell::NoShip(true) => write!(f, "·"),
        }
    }
}

struct Field(
    Vec<Vec<FieldCell>>,
);

impl Field {
    pub fn new() -> Self {
        Self(vec![vec![FieldCell::NoShip(false); 10]; 10])
    }

    pub fn fill(&mut self) {
        self.put_ship(4);
        self.put_ship(3);
        self.put_ship(3);
        self.put_ship(2);
        self.put_ship(2);
        self.put_ship(2);
        self.put_ship(1);
        self.put_ship(1);
        self.put_ship(1);
        self.put_ship(1);
    }

    pub fn get(&self, x: usize, y: usize) -> FieldCell {
        self.0[x][y]
    }

    pub fn put_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen::<u8>() % 2;
        let (mut x, mut y): (usize, usize);
        'outer: loop {
            (x, y) = (rng.gen::<usize>() % 10, rng.gen::<usize>() % 10);
            for i in 0..size {
                match direction {
                    0 => {
                        if x + size > 10 {
                            continue 'outer;
                        }
                        if self.0[x+i][y] != FieldCell::NoShip(false) {
                            continue 'outer;
                        }
                    }
                    _ => {
                        if y + size > 10 {
                            continue 'outer;
                        }
                        if self.0[x][y+i] != FieldCell::NoShip(false) {
                            continue 'outer;
                        }
                    }
                }
            }

            for i in (-1)..((size as i32)+1) {
                match direction {
                    0 => {
                        if (x as i32) + i < 0 || (x as i32) + i > 9 {
                            continue;
                        }
                        if self.0[((x as i32)+i) as usize][y] != FieldCell::NoShip(false) {
                            continue 'outer;
                        }
                        if y as i32 -1 >= 0 {
                            if self.0[((x as i32) + i) as usize][y - 1] != FieldCell::NoShip(false) {
                                continue 'outer;
                            }
                        }
                        if y+1 < 10 {
                            if self.0[((x as i32) + i) as usize][y + 1] != FieldCell::NoShip(false) {
                                continue 'outer;
                            }
                        }
                    }
                    _ => {
                        if (y as i32) + i < 0 || (y as i32) + i > 9 {
                            continue;
                        }
                        if self.0[x][((y as i32)+i) as usize] != FieldCell::NoShip(false) {
                            continue 'outer;
                        }
                        if x as i32 -1 >= 0 {
                            if self.0[x - 1][((y as i32) + i) as usize] != FieldCell::NoShip(false) {
                                continue 'outer;
                            }
                        }
                        if x+1 < 10 {
                            if self.0[x + 1][((y as i32) + i) as usize] != FieldCell::NoShip(false) {
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            break;
        }

        for i in 0..size {
            match direction {
                0 => self.0[x+i][y] = FieldCell::Ship(false),
                _ => self.0[x][y+i] = FieldCell::Ship(false),
            }
        }
    }

    pub fn game_over(&self) -> bool {
        !self.0.iter().any(
            |row| row.iter().any(
                |cell| *cell == FieldCell::Ship(false)
            )
        )
    }

    pub fn shoot(&mut self, x: usize, y: usize) -> bool {
        match self.0[x][y] {
            FieldCell::Ship(false) => {
                self.0[x][y] = FieldCell::Ship(true);
                true
            }
            FieldCell::NoShip(false) => {
                self.0[x][y] = FieldCell::NoShip(true);
                false
            }
            _ => false,
        }
    }

    pub fn draw(&self) {
        let mut s = "".to_string();
        self.0.iter().for_each(
            |row| {
                row.iter().for_each(
                    |cell| s.push_str(&cell.to_string())
                );
                s.push_str("\n")
            }
        );
        s.push_str("==========");
        println!("{s}");
    }
}

fn play() -> i32 {
    let mut field = Field::new();
    field.fill();
    let mut bot = RandomBot::new();

    // field.draw();

    let mut turn_counter = 1;

    loop {
        let (x, y) = bot.turn(&field);
        if !field.shoot(x, y) {
            turn_counter += 1;
        }

        // sleep(Duration::from_millis(250));
        // field.draw();
        if field.game_over() {
            break;
        }
    }

    turn_counter
}

fn main() {
    let mut vec = Vec::with_capacity(1000);

    for _ in 0..1000 {
        vec.push(play());
    }

    let sum: i32 = vec.iter().sum();
    let avg = sum as f64 / (vec.len() as f64);

    println!("Bot won in {avg} moves on average");
}
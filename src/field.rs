use std::fmt;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum FieldCell {
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

pub struct Field(
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

    pub fn get(&self, x: i32, y: i32) -> Option<FieldCell> {
        if x < 0 || x > 9 || y < 0 || y > 9 {
            None
        } else {
            Some(self.0[x as usize][y as usize])
        }
    }

    fn set(&mut self, x: i32, y: i32, value: FieldCell) {
        self.0[x as usize][y as usize] = value;
    }

    fn put_ship(&mut self, size: i32) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen::<u8>() % 2;
        let (mut x, mut y);

        loop {
            (x, y) = (rng.gen_range(0..10), rng.gen_range(0..10));
            if self.check_ship_validity(x, y, size, direction) {
                break;
            }
        }

        for i in 0..size {
            match direction {
                0 => self.set(x+i, y, FieldCell::Ship(false)),
                _ => self.set(x, y+i, FieldCell::Ship(false)),
            }
        }
    }

    fn check_ship_validity(&self, x: i32, y: i32, size: i32, direction: u8) -> bool {
        for i in 0..size {
                match direction {
                    0 => {
                        if x + size > 10 {
                            return false;
                        }
                        if self.get(x+i, y).unwrap() != FieldCell::NoShip(false) {
                            return false;
                        }
                    }
                    _ => {
                        if y + size > 10 {
                            return false;
                        }
                        if self.get(x, y+i).unwrap() != FieldCell::NoShip(false) {
                            return false;
                        }
                    }
                }
            }

            for i in (-1)..(size+1) {
                match direction {
                    0 => {
                        if x + i < 0 || x + i > 9 {
                            continue;
                        }
                        if self.get(x+i, y).unwrap() != FieldCell::NoShip(false) {
                            return false;
                        }
                        if y -1 >= 0 {
                            if self.get(x + i, y - 1).unwrap() != FieldCell::NoShip(false) {
                                return false;
                            }
                        }
                        if y+1 < 10 {
                            if self.get(x + i, y + 1).unwrap() != FieldCell::NoShip(false) {
                                return false;
                            }
                        }
                    }
                    _ => {
                        if y + i < 0 || y + i > 9 {
                            continue;
                        }
                        if self.get(x, y+i).unwrap() != FieldCell::NoShip(false) {
                            return false;
                        }
                        if x -1 >= 0 {
                            if self.get(x - 1, y + i).unwrap() != FieldCell::NoShip(false) {
                                return false;
                            }
                        }
                        if x+1 < 10 {
                            if self.get(x + 1, y + i).unwrap() != FieldCell::NoShip(false) {
                                return false;
                            }
                        }
                    }
                }
            }
        true
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
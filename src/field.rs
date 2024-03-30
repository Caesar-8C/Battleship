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
                0 => self.set(x + i, y, FieldCell::Ship(false)),
                _ => self.set(x, y + i, FieldCell::Ship(false)),
            }
        }
    }

    fn check_ship_validity(&self, x: i32, y: i32, size: i32, direction: u8) -> bool {
        for i in 0..size {
            let (x_i, y_i);
            match direction {
                0 => {
                    x_i = x + i;
                    y_i = y;
                }
                _ => {
                    x_i = x;
                    y_i = y + i;
                }
            }
            match self.get(x_i, y_i) {
                Some(FieldCell::NoShip(_)) => (),
                _ => return false,
            }
        }

        for i in (-1)..(size + 1) {
            match direction {
                0 => {
                    if let Some(FieldCell::Ship(_)) = self.get(x + i, y) {
                        return false;
                    }
                    if let Some(FieldCell::Ship(_)) = self.get(x + i, y - 1) {
                        return false;
                    }
                    if let Some(FieldCell::Ship(_)) = self.get(x + i, y + 1) {
                        return false;
                    }
                }
                _ => {
                    if let Some(FieldCell::Ship(_)) = self.get(x, y + i) {
                        return false;
                    }
                    if let Some(FieldCell::Ship(_)) = self.get(x - 1, y + i) {
                        return false;
                    }
                    if let Some(FieldCell::Ship(_)) = self.get(x + 1, y + i) {
                        return false;
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
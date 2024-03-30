use std::fmt;
use rand::Rng;
use crate::field::Direction::{Up, Down, Left, Right};

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

#[derive(PartialEq)]
pub enum ShotResult {
    Miss,
    Hit,
    Kill,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] =
        [Up, Down, Left, Right];

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0..4);
        match rand {
            0 => Up,
            1 => Down,
            2 => Right,
            _ => Left,
        }
    }

    pub fn side(self) -> (Self, Self) {
        match self {
            Up => (Right, Left),
            Right => (Up, Down),
            Down => (Right, Left),
            Left => (Up, Down),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn next(self, dir: Direction) -> Self {
        match dir {
            Up => Self { x: self.x - 1, y: self.y },
            Down => Self { x: self.x + 1, y: self.y },
            Left => Self { x: self.x, y: self.y - 1 },
            Right => Self { x: self.x, y: self.y + 1 },
        }
    }

    pub fn next_i(self, i: i32, dir: Direction) -> Self {
        match dir {
            Up => Self { x: self.x - i, y: self.y },
            Down => Self { x: self.x + i, y: self.y },
            Left => Self { x: self.x, y: self.y - i },
            Right => Self { x: self.x, y: self.y + i },
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

    pub fn get(&self, c: Coord) -> Option<FieldCell> {
        if !(0..=9).contains(&c.x) || !(0..=9).contains(&c.y) {
            None
        } else {
            Some(self.0[c.x as usize][c.y as usize])
        }
    }

    fn set(&mut self, c: Coord, value: FieldCell) {
        self.0[c.x as usize][c.y as usize] = value;
    }

    fn put_ship(&mut self, size: i32) {
        let mut rng = rand::thread_rng();
        let direction = Direction::random();
        let mut c: Coord;

        loop {
            c = Coord { x: rng.gen_range(0..10), y: rng.gen_range(0..10) };
            if self.check_ship_validity(c, size, direction) {
                break;
            }
        }

        for i in 0..size {
            self.set(c.next_i(i, direction), FieldCell::Ship(false));
        }
    }

    fn check_ship_validity(&self, c: Coord, size: i32, direction: Direction) -> bool {
        for i in 0..size {
            let c_i = c.next_i(i, direction);
            match self.get(c_i) {
                Some(FieldCell::NoShip(_)) => (),
                _ => return false,
            }
        }

        for i in (-1)..(size + 1) {
            if let Some(FieldCell::Ship(_)) = self.get(c.next_i(i, direction)) {
                return false;
            }
            if let Some(FieldCell::Ship(_)) = self.get(c.next(direction.side().0).next_i(i, direction)) {
                return false;
            }
            if let Some(FieldCell::Ship(_)) = self.get(c.next(direction.side().1).next_i(i, direction)) {
                return false;
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

    pub fn shoot(&mut self, c: Coord) -> ShotResult {
        match self.get(c) {
            Some(FieldCell::Ship(false)) => {
                self.set(c, FieldCell::Ship(true));
                match self.is_ship_dead(c) {
                    true => ShotResult::Kill,
                    false => ShotResult::Hit,
                }
            }
            Some(FieldCell::NoShip(false)) => {
                self.set(c, FieldCell::NoShip(true));
                ShotResult::Miss
            }
            _ => ShotResult::Miss,
        }
    }

    fn is_ship_dead(&self, c: Coord) -> bool {
        // let mut vec = vec![c];
        // loop {
        //     let mut new_vec = Vec::new();
        //     vec.iter().for_each(|c| self.add_neighbors(*c, &mut new_vec));
        //     vec = new_vec;
        // }
        false
    }

    fn add_neighbors(&self, c: Coord, vec: &mut Vec<Coord>) {}

    pub fn draw(&self) {
        let mut s = "".to_string();
        self.0.iter().for_each(
            |row| {
                row.iter().for_each(
                    |cell| s.push_str(&cell.to_string())
                );
                s.push('\n')
            }
        );
        s.push_str("==========");
        println!("{s}");
    }
}

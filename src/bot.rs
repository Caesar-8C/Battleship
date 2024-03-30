pub mod random;

use crate::field::{Coord, ShotResult};

#[derive(Clone, Copy, PartialEq)]
enum BotCell {
    Miss,
    Hit,
    Value(i32),
}

pub trait Bot {
    fn turn(&mut self) -> Coord;

    fn shot_result(&mut self, c: Coord, result: ShotResult);
}

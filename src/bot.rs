pub mod alina;
pub mod bot1;
pub mod bot2;
pub mod bot3;
pub mod marching;
pub mod random;
pub mod three_step;

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

pub mod random;

use crate::field::ShotResult;

#[derive(Clone, Copy)]
enum BotCell {
    Miss,
    Hit,
    Value(i32),
}

pub trait Bot {
    fn turn(&mut self) -> (i32, i32);

    fn shot_result(&mut self, x: i32, y: i32, result: ShotResult);
}

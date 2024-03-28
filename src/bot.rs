pub mod random;

use crate::Field;

#[derive(Clone, Copy)]
enum BotCell {
    Miss,
    Hit,
    Value(i32),
}

pub trait Bot {
    fn turn(&mut self, filed: &Field) -> (usize, usize);
}
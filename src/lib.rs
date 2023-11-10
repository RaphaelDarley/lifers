use std::fmt::Display;

pub mod basic_swap;
pub mod game1;

// Any live cell with two or three live neighbours survives.
// Any dead cell with three live neighbours becomes a live cell.
// All other live cells die in the next generation. Similarly, all other dead cells stay dead.

pub trait Game: Display + From<Vec<Vec<bool>>> + Clone {
    fn tick(&mut self) -> &mut Self;
}

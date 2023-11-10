use core::fmt;
use std::iter::{repeat, repeat_with};

use crate::Game;

#[derive(Debug, Clone)]
pub struct State {
    pub current: Vec<Vec<bool>>,
    pub reserve: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl From<Vec<Vec<bool>>> for State {
    fn from(value: Vec<Vec<bool>>) -> Self {
        let width = value.get(0).unwrap().len();
        let height = value.len();
        let blank = repeat_with(|| repeat(false).take(width).collect::<Vec<bool>>())
            .take(height)
            .collect();
        State {
            width,
            height,
            current: value,
            reserve: blank,
        }
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        let vec: Vec<Vec<bool>> = value
            .split("\n")
            .map(|row| row.chars().map(|c| c == '&').collect())
            .collect();
        vec.into()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let outstr = self
            .current
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c { '&' } else { '`' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", outstr)
    }
}

impl Game for State {
    fn tick(&mut self) -> &mut Self {
        let mut next = std::mem::take(&mut self.reserve);
        for (i, row) in next.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let current_cell = self.current[i][j];
                let adj_count = self.get_adjs(i, j);
                *cell = match (current_cell, adj_count) {
                    (true, 2..=3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                }
            }
        }
        self.reserve = next;
        std::mem::swap(&mut self.reserve, &mut self.current);
        self
    }
}

impl State {
    pub fn get_adjs(&self, row: usize, col: usize) -> u8 {
        let mut acc = 0;
        let max_row = self.height - 1;
        let max_col = self.width - 1;
        if row > 0 {
            acc += self.current[row - 1][col] as u8 * 1;
        }
        if col > 0 {
            acc += self.current[row][col - 1] as u8 * 1;
        }
        if row > 0 && col < max_col {
            acc += self.current[row - 1][col + 1] as u8 * 1;
        }
        if col > 0 && row < max_row {
            acc += self.current[row + 1][col - 1] as u8 * 1;
        }
        if row > 0 && col > 0 {
            acc += self.current[row - 1][col - 1] as u8 * 1;
        }
        if row < max_row {
            acc += self.current[row + 1][col] as u8 * 11
        }
        if col < max_col {
            acc += self.current[row][col + 1] as u8 * 11
        }
        if row < max_row && col < max_col {
            acc += self.current[row + 1][col + 1] as u8 * 11
        }
        acc
    }
}

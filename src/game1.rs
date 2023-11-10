use core::fmt;

use crate::Game;

#[derive(Debug, Clone)]
pub struct State {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<bool>>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let outstr = self
            .board
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

impl From<String> for State {
    fn from(value: String) -> Self {
        let rows = value.split('\n').collect::<Vec<&str>>();
        let board: Vec<Vec<bool>> = rows
            .iter()
            .map(|row| row.chars().map(|c| c == '&').collect::<Vec<bool>>())
            .collect();
        State {
            width: board[0].len(),
            height: rows.len(),
            board: board,
        }
    }
}

impl From<Vec<Vec<bool>>> for State {
    fn from(value: Vec<Vec<bool>>) -> Self {
        State {
            width: value.get(0).unwrap().len(),
            height: value.len(),
            board: value,
        }
    }
}

impl Game for State {
    fn tick(&mut self) -> &mut Self {
        let mut adjs: Vec<Vec<u8>> = vec![vec![0; self.width]; self.height];

        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell {
                    self.add_adjs(&mut adjs, i, j)
                }
            }
        }

        for (c_row, a_row) in self.board.iter_mut().zip(adjs.iter()) {
            for (cell, adj) in c_row.iter_mut().zip(a_row.iter()) {
                match (*cell, *adj) {
                    (true, 2..=3) => {}
                    (true, _) => *cell = false,
                    (false, 3) => *cell = true,
                    (false, _) => {}
                }
            }
        }

        // println!("{:#?}", adjs);

        self
    }
}
impl State {
    fn add_adjs(&self, acc: &mut Vec<Vec<u8>>, row: usize, col: usize) {
        let max_row = self.height - 1;
        let max_col = self.width - 1;
        if row > 0 {
            acc[row - 1][col] += 1;
        }
        if col > 0 {
            acc[row][col - 1] += 1;
        }
        if row > 0 && col < max_col {
            acc[row - 1][col + 1] += 1;
        }
        if col > 0 && row < max_row {
            acc[row + 1][col - 1] += 1;
        }
        if row > 0 && col > 0 {
            acc[row - 1][col - 1] += 1;
        }
        if row < max_row {
            acc[row + 1][col] += 1
        }
        if col < max_col {
            acc[row][col + 1] += 1
        }
        if row < max_row && col < max_col {
            acc[row + 1][col + 1] += 1
        }
    }
}

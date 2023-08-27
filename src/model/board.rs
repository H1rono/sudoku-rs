use std::{fmt::Display, rc::Rc};

use itertools::Itertools;

use super::{Cell, Group};

#[derive(Debug, Clone)]
pub struct Board {
    pub base_number: u32,
    board: Vec<Rc<Cell>>,
}

impl Board {
    pub fn new(base_number: u32) -> Self {
        let sq_base = base_number * base_number;
        let board = (0..sq_base * sq_base)
            .map(|_| {
                Rc::new(Cell::Empty {
                    candidates: (1..=sq_base).collect(),
                })
            })
            .collect();
        Self { base_number, board }
    }

    pub fn sq_base(&self) -> u32 {
        self.base_number * self.base_number
    }

    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.sq_base() + col) as usize
    }

    pub fn get(&self, row: u32, col: u32) -> Option<&Rc<Cell>> {
        let i = self.index(row, col);
        self.board.get(i)
    }

    pub fn get_mut(&mut self, row: u32, col: u32) -> Option<&mut Rc<Cell>> {
        let i = self.index(row, col);
        self.board.get_mut(i)
    }

    pub fn is_valid(&self) -> bool {
        let sq_base = self.sq_base();
        let mut groups = Vec::new();
        for i in 0..sq_base {
            groups.push(self.row(i));
            groups.push(self.col(i));
            groups.push(self.square(
                (i / self.base_number) * self.base_number,
                (i % self.base_number) * self.base_number,
            ));
        }
        groups.iter().all(|g| g.is_valid())
    }

    pub fn fullfilled(&self) -> bool {
        self.board.iter().all(|c| c.is_number())
    }

    pub fn row(&self, row: u32) -> Group {
        let mut cells = Vec::new();
        let sq_base = self.sq_base();
        for col in 0..sq_base {
            if let Some(cell) = self.get(row, col) {
                cells.push(cell.clone());
            }
        }
        Group::new(cells)
    }

    pub fn row_except_cell(&self, row: u32, col: u32) -> Group {
        let mut cells = Vec::new();
        let sq_base = self.sq_base();
        for c in 0..sq_base {
            if let Some(cell) = self.get(row, c) {
                if c != col {
                    cells.push(cell.clone());
                }
            }
        }
        Group::new(cells)
    }

    pub fn col(&self, col: u32) -> Group {
        let mut cells = Vec::new();
        let sq_base = self.sq_base();
        for row in 0..sq_base {
            if let Some(cell) = self.get(row, col) {
                cells.push(cell.clone());
            }
        }
        Group::new(cells)
    }

    pub fn col_except_cell(&self, row: u32, col: u32) -> Group {
        let mut cells = Vec::new();
        let sq_base = self.sq_base();
        for r in 0..sq_base {
            if let Some(cell) = self.get(r, col) {
                if r != row {
                    cells.push(cell.clone());
                }
            }
        }
        Group::new(cells)
    }

    pub fn square(&self, row: u32, col: u32) -> Group {
        let mut cells = Vec::new();
        let row = (row / self.base_number) * self.base_number;
        let col = (col / self.base_number) * self.base_number;
        for r in row..(row + self.base_number) {
            for c in col..(col + self.base_number) {
                if let Some(cell) = self.get(r, c) {
                    cells.push(cell.clone());
                }
            }
        }
        Group::new(cells)
    }

    pub fn square_except_cell(&self, row: u32, col: u32) -> Group {
        let mut cells = Vec::new();
        let row = (row / self.base_number) * self.base_number;
        let col = (col / self.base_number) * self.base_number;
        for r in row..(row + self.base_number) {
            for c in col..(col + self.base_number) {
                if let Some(cell) = self.get(r, c) {
                    if r != row || c != col {
                        cells.push(cell.clone());
                    }
                }
            }
        }
        Group::new(cells)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sq_base = self.base_number * self.base_number;
        let s = (0..sq_base)
            .map(|row| {
                (0..sq_base)
                    .map(|col| self.get(row, col).unwrap())
                    .map(|cell| format!("{}", cell))
                    .join(" | ")
            })
            .join("\n");
        write!(f, "{}", s)
    }
}

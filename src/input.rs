use std::{io::Read, rc::Rc};

use crate::model::Board;

pub trait Input: Sized {
    type Error: std::error::Error;

    fn read<R: Read>(r: &mut R) -> Result<Self, Self::Error>;
}

impl Input for Board {
    type Error = std::io::Error;

    fn read<R: Read>(r: &mut R) -> Result<Self, Self::Error> {
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let mut lines = buf.lines();
        let base_number = {
            let line = lines.next().ok_or(std::io::ErrorKind::InvalidData)?;
            line.parse::<u32>()
                .map_err(|_| std::io::ErrorKind::InvalidData)?
        };
        let mut board = Board::new(base_number);
        for (row, line) in lines.enumerate() {
            for (col, s) in line.split_whitespace().enumerate() {
                let cell = board.get_mut(row as u32, col as u32).unwrap();
                let cell = Rc::get_mut(cell).unwrap();
                if let Ok(n) = s.parse::<u32>() {
                    cell.set_number(n);
                }
            }
        }
        Ok(board)
    }
}

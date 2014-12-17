use std::fmt;

use cell::*;
use unit::Unit;

pub struct Board {
    cells: [[Cell, ..9], ..9]
}

impl Board {

    pub fn empty() -> Board {
        Board {
            cells: [[Cell::empty(), ..9], ..9]
        }
    }


    pub fn new(input: &str) -> Option<Board> {
        let mut board = Board::empty();
        let mut x = 0;
        let mut y = 0;
        for c in input.chars() {
            if x >= 9 {
                x = 0;
                y += 1;
            }

            match c {
                '1'...'9' => board.set(x, y, Cell::from_char(c).unwrap()),
                '.' => board.set(x, y, Cell::empty()),
                _ => return None
            }

            x += 1;
        }
        Some(board)
    }


    pub fn force_set(&mut self, x: uint, y: uint, to: Cell) {
        self.cells[y][x] = to;
    }


    pub fn set(&mut self, x: uint, y: uint, to: Cell) {
        self.force_set(x, y, to)
    }


    pub fn get(&self, x: uint, y: uint) -> Option<Cell> {
        if x > 8 || y > 8 {
            None
        } else {
            Some(self.cells[y][x])
        }
    }


    pub fn get_row(&self, y: uint) -> Unit {
        Unit::Row(self.cells[y])
    }


    pub fn get_column(&self, x: uint) -> Unit {
        let mut col = [Cell::empty(), ..9];

        for (y, row) in self.cells.iter().enumerate() {
            col[y] = row[x];
        };

        Unit::Column(col)
    }


    pub fn get_grid(&self, grid_number: uint) -> Unit {
        let mut grid = [Cell::empty(), ..9];
        let grid_x = grid_number % 3;
        let grid_y = grid_number / 3;

        let mut i = 0u;
        for y in range(grid_y * 3, grid_y * 3 + 3) {
            for x in range(grid_x * 3, grid_x * 3 + 3) {
                grid[i] = self.cells[y][x];
                i+=1;
            }
        }

        Unit::Grid(grid)
    }
}

impl fmt::Show for Board {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for y in range(0, 9) {
            for x in range(0, 9) {
                let cell = self.get(x, y).unwrap().to_string();
                try!(write!(out, "{:^9}", cell));

                if x == 2 || x == 5 {
                    try!(write!(out, "|"));
                } else {
                    try!(write!(out, " "));
                }
            }
            try!(write!(out, "\n"));
            if y == 2 || y == 5 {
                try!(write!(out, "{0:-^59}{0:-<30}\n", "+"));
            }
        }
        Ok(())
    }
}

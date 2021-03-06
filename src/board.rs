use std::fmt;

use cell::*;
use unit::Unit;

#[derive(Clone, PartialEq)]
pub struct Board {
    cells: [[Cell; 9]; 9]
}

impl Board {

    pub fn empty() -> Board {
        Board {
            cells: [[Cell::empty(); 9]; 9]
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

            let able = match c {
                '1'...'9' => board.set(x, y, Cell::from_char(c).unwrap()),
                '.' => board.set(x, y, Cell::empty()),
                _ => return None
            };

            if !able {
                return None
            }

            x += 1;
        }
        Some(board)
    }


    pub fn force_set(&mut self, x: u8, y: u8, to: Cell) {
        self.cells[y as usize][x as usize] = to;
    }


    pub fn set(&mut self, x: u8, y: u8, to: Cell) -> bool {
        if x > 8 || y > 8 {
            return false
        };

        let g = 3 * (y / 3) + (x / 3);

        match to {
            Cell::Known(cell) => {
                if to != self.get(x, y).unwrap() &&
                   (self.get_column(x).contains(to) ||
                    self.get_row(y).contains(to) ||
                    self.get_grid(g).contains(to)) { false }
                else {
                    self.force_set(x, y, to);
                    true
                }
            },

            Cell::Unknown(_) => {
                self.force_set(x, y, to);
                true
            }
        }
    }


    pub fn get(&self, x: u8, y: u8) -> Option<Cell> {
        match (x, y) {
            (0...9, 0...9) => Some(self.cells[y as usize][x as usize]),
            _ => None
        }
    }


    pub fn get_row(&self, y: u8) -> Unit {
        Unit(self.cells[y as usize])
    }


    pub fn get_column(&self, x: u8) -> Unit {
        let mut col = [Cell::empty(); 9];

        for (y, row) in self.cells.iter().enumerate() {
            col[y] = row[x as usize];
        };

        Unit(col)
    }


    pub fn get_grid(&self, grid_number: u8) -> Unit {
        let mut grid = [Cell::empty(); 9];
        let grid_x = grid_number % 3;
        let grid_y = grid_number / 3;

        let mut i = 0u;
        for y in range(grid_y * 3, grid_y * 3 + 3) {
            for x in range(grid_x * 3, grid_x * 3 + 3) {
                grid[i] = self.cells[y as usize][x as usize];
                i+=1;
            }
        }

        Unit(grid)
    }


    pub fn set_row(&mut self, y: u8, row: Unit) -> bool {
        for (x, cell) in row.get_cells().iter().enumerate() {
            if !self.set(x as u8, y as u8, *cell) {
                return false
            };
        };

        true
    }


    pub fn set_column(&mut self, x: u8, column: Unit) -> bool {
        for (y, cell) in column.get_cells().iter().enumerate() {
            if !self.set(x as u8, y as u8, *cell) {
                return false
            };
        };

        true
    }


    pub fn set_grid(&mut self, grid_number: u8, grid: Unit) -> bool {
        let grid_x = grid_number % 3;
        let grid_y = grid_number / 3;

        let mut i = 0u8;
        for y in range(grid_y * 3, grid_y * 3 + 3) {
            for x in range(grid_x * 3, grid_x * 3 + 3) {
                if !self.set(x as u8, y as u8, grid.get(i)) {
                    return false
                };

                i+=1;
            };
        };

        true
    }
}

impl fmt::Show for Board {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for y in range(0, 9) {
            for x in range(0, 9) {
                let cell = self.get(x, y).unwrap();
                try!(write!(out, "{:^9?}", cell));

                match x {
                    2 | 5 => try!(write!(out, "|")),
                    _ => try!(write!(out, " "))
                };
            }

            match y {
                2 | 5 => try!(write!(out, "\n{0:-^59}{0:-<30}\n", "+")),
                _ => try!(write!(out, "\n"))
            };
        }
        Ok(())
    }
}

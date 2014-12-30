use std::fmt;

use cell::Cell;

#[deriving(Clone)]
pub enum Unit {
    Column([Cell, ..9]),
    Row([Cell, ..9]),
    Grid([Cell, ..9])
}

impl Unit {
    pub fn get_cells(&self) -> [Cell, ..9] {
        match *self {
            Unit::Column(cells) => cells,
            Unit::Row(cells) => cells,
            Unit::Grid(cells) => cells
        }
    }

    pub fn contains(&self, cell: Cell) -> bool {
        let cells = self.get_cells();

        for item in cells.iter() {
            if *item == cell {
                return true;
            }
        };

        false

    }
}


impl fmt::Show for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.get_cells())
    }
}

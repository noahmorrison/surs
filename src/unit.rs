use std::fmt;

use cell::Cell;

#[deriving(Clone)]
pub struct Unit(pub [Cell, ..9]);

impl Unit {
    pub fn get(&self, x: uint) -> Cell {
        if x > 9 {
            panic!("index out of range")
        };

        self.get_cells()[x]
    }

    pub fn get_cells(&self) -> [Cell, ..9] {
        match *self {
            Unit(cells) => cells
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

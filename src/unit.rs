use std::fmt;

use cell::Cell;
use cell::Number;

#[deriving(Clone)]
pub struct Unit(pub [Cell, ..9]);

impl Unit {
    pub fn new(input: &str) -> Option<Unit> {
        let mut cells = [Cell::empty(), ..9];

        let mut x = 0;
        for c in input.chars() {
            match c {
                '1'...'9' => cells[x] = Cell::from_char(c).unwrap(),
                '.' => {}
                _ => return None
            }

            x += 1;
        };

        Some(Unit(cells))
    }

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

    pub fn without(&self, num: Number) -> Unit {
        let mut cells = self.get_cells();

        for (i, other) in self.get_cells().iter().enumerate() {
            if other.contains(num) {
                cells[i] = other.without(num);
            }
        }

        Unit(cells)
    }
}


impl fmt::Show for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.get_cells())
    }
}

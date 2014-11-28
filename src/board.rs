use cell::*;

struct Grid {
    top: [Cell, ..3],
    middle: [Cell, ..3],
    bottom: [Cell, ..3]
}


impl Grid {

    fn empty() -> Grid {
        Grid {
            top: [Cell::empty(), Cell::empty(), Cell::empty()],
            middle: [Cell::empty(), Cell::empty(), Cell::empty()],
            bottom: [Cell::empty(), Cell::empty(), Cell::empty()],
        }
    }


    fn set(&mut self, x: uint, y: uint, to: Cell) {
        match y {
            0 => self.top[x] = to,
            1 => self.middle[x] = to,
            2 => self.bottom[x] = to,
            _ => panic!("Index out of range")
        };
    }


    fn get(&self, x: uint, y: uint) -> Option<&Cell> {
        match y {
            0 => Some(&self.top[x]),
            1 => Some(&self.middle[x]),
            2 => Some(&self.bottom[x]),
            _ => None
        }
    }
}


pub struct Board {
    top: [Grid, ..3],
    middle: [Grid, ..3],
    bottom: [Grid, ..3]
}


impl Board {

    pub fn empty() -> Board {
        Board {
            top: [Grid::empty(), Grid::empty(), Grid::empty()],
            middle: [Grid::empty(), Grid::empty(), Grid::empty()],
            bottom: [Grid::empty(), Grid::empty(), Grid::empty()],
        }
    }


    pub fn new(input: &str) -> Board {
        let mut board = Board::empty();
        let mut x = 0;
        let mut y = 0;
        for c in input.chars() {
            if x >= 9 {
                x = 0;
                y += 1;
            }

            match c {
                '1'...'9' => board.set(x, y, Cell::from_char(c)),
                '.' => board.set(x, y, Cell::empty()),
                _ => panic!("Unable to parse input")
            }

            x += 1;
        }
        board
    }


    pub fn force_set(&mut self, x: uint, y: uint, to: Cell) {
        match y {
            0...2 => self.top[x / 3].set(x % 3, y % 3, to),
            3...5 => self.middle[x / 3].set(x % 3, y % 3, to),
            6...8 => self.bottom[x / 3].set(x % 3, y % 3, to),
            _ => panic!("Index out of range")
        };

    }


    pub fn set(&mut self, x: uint, y: uint, to: Cell) {
        self.force_set(x, y, to)
    }


    pub fn get(&self, x: uint, y: uint) -> Option<&Cell> {
        match y {
            0...2 => self.top[x / 3].get(x % 3, y % 3),
            3...5 => self.middle[x / 3].get(x % 3, y % 3),
            6...8 => self.bottom[x / 3].get(x % 3, y % 3),
            _ => None
        }
    }


    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for y in range(0, 9) {
            for x in range(0, 9) {
                out = out + self.get(x, y).unwrap().to_string();
                if x == 2 || x == 5 {
                    out = out + "|";
                } else {
                    out = out + " ";
                }
            }
            out = out + "\n";
            if y == 2 || y == 5 {
                out = out + "-----+-----+-----\n";
            }
        }
        out
    }
}

use cell::*;

pub struct Board {
    cells: [[Cell, ..9], ..9]
}

impl Board {

    pub fn empty() -> Board {
        Board {
            cells: [[Cell::empty(), ..9], ..9]
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
        self.cells[y][x] = to;
    }


    pub fn set(&mut self, x: uint, y: uint, to: Cell) {
        self.force_set(x, y, to)
    }


    pub fn get(&self, x: uint, y: uint) -> Option<Cell> {
        Some(self.cells[y][x])
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

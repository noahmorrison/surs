use std::fmt;

use cell::Cell::*;
use cell::Number::*;

#[deriving(Copy, PartialEq)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl fmt::Show for Number {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            One => write!(out, "1"),
            Two => write!(out, "2"),
            Three => write!(out, "3"),
            Four => write!(out, "4"),
            Five => write!(out, "5"),
            Six => write!(out, "6"),
            Seven => write!(out, "7"),
            Eight => write!(out, "8"),
            Nine => write!(out, "9"),
        }
    }
}

#[deriving(Copy, PartialEq)]
pub enum Cell {
    Known(Number),
    Unknown([bool, ..9])
}

impl Cell {

    pub fn empty() -> Cell {
        Unknown([true, ..9])
    }


    pub fn from_char(c: char) -> Option<Cell> {
        match c {
            '1' => Some(Known(One)),
            '2' => Some(Known(Two)),
            '3' => Some(Known(Three)),
            '4' => Some(Known(Four)),
            '5' => Some(Known(Five)),
            '6' => Some(Known(Six)),
            '7' => Some(Known(Seven)),
            '8' => Some(Known(Eight)),
            '9' => Some(Known(Nine)),
            '.' => Some(Cell::empty()),
            _ => None
        }
    }
}


impl fmt::Show for Cell {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Known(x) => write!(out, "{}", x),
            &Unknown(nums) => {
                for (i, num) in nums.iter().enumerate() {
                    if *num {
                        try!(write!(out, "{}", i + 1));
                    }
                }
                Ok(())
            }
        }
    }
}

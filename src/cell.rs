use std::fmt;

use cell::Cell::*;
use cell::Number::*;

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

pub enum Cell {
    Known(Number),
    Unknown(Vec<Number>)
}

impl Cell {

    pub fn empty() -> Cell {
        Unknown(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine])
    }


    pub fn from_char(c: char) -> Cell {
        match c {
            '1' => Known(One),
            '2' => Known(Two),
            '3' => Known(Three),
            '4' => Known(Four),
            '5' => Known(Five),
            '6' => Known(Six),
            '7' => Known(Seven),
            '8' => Known(Eight),
            '9' => Known(Nine),
            '.' => Cell::empty(),
            _ => panic!("Invalid character for cell")
        }
    }
}


impl fmt::Show for Cell {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Known(x) => write!(out, "{}", x),
            _ => write!(out, ".")
        }
    }
}

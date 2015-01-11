use std::fmt;

use cell::Cell::*;
use cell::Number::*;

#[derive(Copy, PartialEq)]
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
        write!(out, "{}", (*self as uint) + 1)
    }
}

#[derive(Copy, PartialEq)]
pub enum Cell {
    Known(Number),
    Unknown([bool; 9])
}

impl Cell {

    pub fn empty() -> Cell {
        Unknown([true; 9])
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

    pub fn contains(&self, num: Number) -> bool {
        match *self {
            Known(n) => n == num,

            Unknown(nums) => nums[num as uint]
        }
    }

    pub fn without(&self, num: Number) -> Cell {
        match *self {
            Known(n) => {
                if n == num {
                    panic!("Cannot remove {:?} because I am {:?}.", num, n);
                };

                Known(n)
            },

            Unknown(nums) => {
                let mut new_nums = nums.clone();
                new_nums[num as uint] = false;
                Unknown(new_nums)
            }
        }
    }
}


impl fmt::Show for Cell {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Known(x) => write!(out, "{:?}", x),
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

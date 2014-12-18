#![feature(globs)]

use board::Board;

mod board;
mod cell;
mod unit;


fn main() {
    let board = Board::new("4.....8.5.3......9...7......2.....6.....8.4......1.......6.3.7.5..2.....1.4.....").unwrap();
    println!("{}", board);

    let row = board.get_row(2);
    println!("third row has a 7: {}", row.contains(cell::Cell::Known(cell::Number::Seven)));
}

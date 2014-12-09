#![feature(globs)]

use board::Board;

mod board;
mod cell;


fn main() {
    let board = Board::new("4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4.....");
    let board = board.unwrap();

    println!("{}", board.to_string());
}

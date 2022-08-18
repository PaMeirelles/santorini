use crate::board::new_board;

mod board;

fn main() {
    let b1: board::Board = new_board();
    println!("Hello, world!");
}

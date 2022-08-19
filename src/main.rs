use crate::board::{new_board, print_blocks, print_board, print_workers};

mod board;

fn main() {
    let b1: board::Board = new_board([14, 15, 9, 19]);
    let m1: board::Move = board::new_move(10, 16, 21);
    board::print_move(m1);
    print_board(b1);
}



use crate::board::{make_move, new_board, print_blocks, print_board, print_workers};

mod board;

fn main() {
    let mut b1: board::Board = new_board([14, 15, 9, 19]);
    let m1: board::Move = board::new_move(15, 16, 21);
    board::print_move(&m1);
    print_board(&b1);
    make_move(&m1, &mut b1);
    print_board(&b1);
}



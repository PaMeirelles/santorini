use crate::board::{new_board, print_blocks};

mod board;

fn main() {
    let b1: board::Board = new_board();
    let m1: board::Move = board::new_move(10, 16, 21);
    board::print_move(m1);
    print_blocks(b1.blocks);
}



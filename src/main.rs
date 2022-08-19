use crate::board::{make_move, new_board, print_blocks, print_board, print_move, print_workers};
use crate::rules::{half_move, init_neighbours, Neighbours};

mod board;
mod rules;

fn main() {
    let mut n:Neighbours = init_neighbours();

    let mut b1: board::Board = new_board([14, 15, 9, 19]);
    let m1: board::Move = board::new_move(15, 16, 21);
    print_move(&m1);
    print_board(&b1);
    make_move(&m1, &mut b1);
    print_board(&b1);
    let v1: Vec<board::Move> = half_move(&b1, &0, n);
    for mv in v1{
        print_move(&mv);
    }
}


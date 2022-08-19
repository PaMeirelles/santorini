use crate::board::{make_move, new_board, print_board, print_move};
use crate::rules::{gen_move, init_neighbours, Neighbours};

mod board;
mod rules;

fn main() {
    let n:Neighbours = init_neighbours();
    let mut b1: board::Board = new_board([14, 15, 9, 19]);
    let v1: Vec<board::Move> = gen_move(&mut b1, &0, &n);
    for mv in v1{
        print_move(&mv);
    }
}


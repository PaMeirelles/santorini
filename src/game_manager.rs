use crate::board::{Board, Move, new_board, new_move};
use crate::rules::{Neighbours, init_neighbours};
use crate::engine::{get_best_move, Node};
use crate::{make_move, print_board, print_move};
use std::collections::HashMap;

pub fn play_game(eval1:fn(board:&Board, nei:&Neighbours) -> f64, eval2:fn(board:&Board, nei:&Neighbours) -> f64){
    let mut b1:Board = new_board([13, 6, 18, 9]);
    let n:&Neighbours = &init_neighbours();
    let mut tt:HashMap<Board, Node> = HashMap::new();
    let mut best:Move = new_move(&-1, &0, &-1);
    let mut color:i32 = 1;

    loop {
        if color == 1 {
            best = get_best_move(b1, 5, 1, n, eval1, &mut tt)
        } else {
            best = get_best_move(b1, 5, -1, n, eval2, &mut tt)
        }
        print_move(&best);
        make_move(&best, &mut b1);
        print_board(&b1);
        if best.build == -2 {
            break;
        }
        color *= -1;
    }
}
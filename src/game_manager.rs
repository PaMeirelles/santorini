use crate::board::{Board, Move, new_board, new_move};
use crate::rules::{Neighbours, init_neighbours};
use crate::engine::{get_best_move, Node};
use crate::{make_move, print_board, print_move};
use std::collections::HashMap;

pub fn play_game(eval1:fn(board:&Board, nei:&Neighbours) -> f64, eval2:fn(board:&Board, nei:&Neighbours) -> f64){
    let b1:&mut Board = &mut new_board([24, 18, 23, 19]);
    let n:&Neighbours = &init_neighbours();
    let mut tt:HashMap<u64, Node> = HashMap::new();
    let mut best:Move = new_move(&-1, &0, &-1);
    let mut color:i32 = 1;

    loop {
        if color == 1 {
            best = get_best_move(b1, 4, 1, n, eval1, &tt)
        } else {
            best = get_best_move(b1, 4, -1, n, eval2, &tt)
        }
        print_move(&best);
        make_move(&best, b1);
        print_board(&b1);
        if best.build == -2 {
            break;
        }
        color *= -1;
    }
}
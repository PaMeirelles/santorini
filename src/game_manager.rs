use crate::board::{Board, Move};
use crate::rules::{Neighbours};
use crate::engine::{get_best_move};

pub fn play_game(eval1:fn(board:&Board, nei:&Neighbours) -> f64, eval2:fn(board:&Board, nei:&Neighbours) -> f64){

}
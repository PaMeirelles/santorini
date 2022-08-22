use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;

mod game_manager;
use piston_window::*;
use crate::engine::{negamax, random_eval, neighbour_high, get_best_move};
use crate::game_manager::play_game;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        let mut b1:Board = new_board([8, 11, 12, 16]);
        let n:Neighbours = init_neighbours();
        play_game(neighbour_high, neighbour_high, &mut b1, &n);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
use std::collections::HashMap;
use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;

mod game_manager;
use crate::engine::{alpha_beta, random_eval, neighbour_high, get_best_move, Node};
use crate::game_manager::play_game;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        play_game(neighbour_high, neighbour_high);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
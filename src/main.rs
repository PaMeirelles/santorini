use std::collections::HashMap;
use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;

mod game_manager;
use crate::engine::{alpha_beta, neighbour_high, get_best_move, Node};
use crate::game_manager::{play_game, get_counter, update_counter, register_game, write_moves};

fn main() {
    //register_game(0, "Hero", "Lumberjack", 1200.5454, 1200.4444, 344, true, "3+0").expect("TODO: panic message");
    /*
    use std::time::Instant;
    let now = Instant::now();
    {
        play_game("Lumberjack", "Conqueror");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    */
    let mvs = vec![Move{
        from: 0,
        to: 0,
        build: 0
    }, Move{
        from: 0,
        to: 0,
        build: 0
    }];
    write_moves(mvs, get_counter());
}
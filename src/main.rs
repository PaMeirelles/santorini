use std::collections::HashMap;
use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;

mod game_manager;
use crate::engine::{alpha_beta, neighbour_high, get_best_move, Node};
use crate::game_manager::{play_game, get_counter, update_counter, register_game, write_moves, assembly_start_pos, break_start_pos};

fn main() {
    let sp:[i32;4] = [12, 13, 11, 17];
    println!("{}\n", assembly_start_pos(sp));

    let stp:i32 = 272837;
    println!("{} {} {} {}\n", break_start_pos(stp)[0], break_start_pos(stp)[1], break_start_pos(stp)[2], break_start_pos(stp)[3]);

    /*
    use std::time::Instant;
    let now = Instant::now();
    {
        play_game("Lumberjack", "Conqueror");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    */
}
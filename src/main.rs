use std::collections::HashMap;
use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;

mod game_manager;
use crate::engine::{alpha_beta, neighbour_high, get_best_move, Node};
use crate::game_manager::{play_game, get_counter, update_counter, register_game, write_moves, assembly_start_pos, break_start_pos, play_match, get_pairings, play_tournament};

fn main() {
    play_tournament(vec!["Hero".to_string(), "Lumberjack".to_string(), "Conqueror".to_string(), "Sniper".to_string()
    ], 3 * 60, 32, 1, "Sniper".to_string());
}
use crate::board::{Board, Move, make_move, new_board, print_board, print_move, new_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
mod board;
mod rules;
mod visual;
mod engine;
mod game_manager;

use piston_window::*;
use crate::engine::{negamax, random_eval, neighbour_high, get_best_move};

fn main() {
    /*
    let mut window: PistonWindow =
        WindowSettings::new("Santorini", [600, 600])
            .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 1.0, 0.0, 1.0], // red
                      [0.0, 100.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
    */
    let mut b1:Board = new_board([0, 4, 7, 11]);
    let n:Neighbours = init_neighbours();
    let mut best:Move = new_move(&-1, &0, &-1);

    b1.blocks[7] = 2;
    b1.blocks[8] = 3;
    println!("{}", neighbour_high(&b1, &n));

    best = get_best_move(&mut b1, 1, -1, &n, neighbour_high);
    print_move(&best);
    make_move(&best, &mut b1);
    println!("{}", neighbour_high(&b1, &n));
}
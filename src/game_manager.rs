use crate::board::{Board, Move, new_board, new_move};
use crate::rules::{Neighbours, init_neighbours};
use crate::engine::{get_best_move, Node};
use crate::{gen_all_moves, make_move, print_board, print_move};
use std::time::{Duration, Instant};

pub fn play_game(eval1:fn(board:&Board, nei:&Neighbours) -> f64, eval2:fn(board:&Board, nei:&Neighbours) -> f64){
    let mut b1:Board = new_board([12, 13, 7, 17]);
    let n:&Neighbours = &init_neighbours();
    let mut best:Move = new_move(&-1, &0, &-1);
    let mut color:i32 = 1;
    let mut time_a = Duration::new(5 * 60, 0);
    let mut time_b = Duration::new(5 * 60, 0);
    let mut now = Instant::now();
    let mut zero = Duration::new(0, 0);
    loop{
        now = Instant::now();
        if color == 1 {
            best = get_best_move(b1, 1, n, "negamax", "nh", "standart", time_a);
            time_a -= now.elapsed();
        } else {
            best = get_best_move(b1, -1, n, "alpha_beta", "nh", "standart",  time_b);
            time_b -= now.elapsed();
        }
        color *= -1;
        print_move(&best);
        make_move(&best, &mut b1);
        print_board(&b1);
        println!("Time a: {:.2?} Time b: {:.2?}", time_a, time_b);
        if time_a < zero || time_b < zero{
            break;
        }
        if gen_all_moves(b1, &color, n).len() == 0{
            break;
        }
        if best.build == -2 {
            break;
        }
    }
}
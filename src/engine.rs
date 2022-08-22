use std::cmp::max;
use crate::board::{Board, undo_move};
use crate::board::Move;
use crate::rules::Neighbours;
use f64;
use crate::rules::gen_all_moves;
use rand::prelude::*;
use crate::{make_move, print_board, print_move};

pub fn random_eval(b: &Board, n:&Neighbours) -> f64{
    return rand::random();
}

pub fn neighbour_high(b:&Board, n:&Neighbours) -> f64{
    let wh1 = i32::pow(10, b.blocks[b.workers[0] as usize] as u32) as f64;
    let wh2 = i32::pow(10, b.blocks[b.workers[1] as usize] as u32) as f64;
    let wh3 = i32::pow(10, b.blocks[b.workers[2] as usize] as u32) as f64;
    let wh4 = i32::pow(10, b.blocks[b.workers[3] as usize] as u32) as f64;

    let wn1 = n.neighbours[b.workers[0]as usize].len() as f64;
    let wn2 = n.neighbours[b.workers[1]as usize].len() as f64;
    let wn3 = n.neighbours[b.workers[2]as usize].len() as f64;
    let wn4 = n.neighbours[b.workers[3]as usize].len() as f64;

    let p1 = wh1 + wh2 + wn1 +wn2;
    let p2 = wh3 + wh4 + wn3 + wn4;
    return p1 - p2;
}

pub fn negamax(b:&mut Board, depth:i32, color:i32, n:&Neighbours, eval:fn(board:&Board, nei:&Neighbours) -> f64) -> f64{
    if depth == 0{
        return eval(b, n) * color as f64;
    }
    let mut value:f64 = -10000.0;
    let mut result:f64;
    let moves:Vec<Move> = gen_all_moves(b, &color, n);
    if moves.len() == 0{
        return (10000.0 + depth as f64) * -color as f64;
    }
    for mv in moves{
        //print_move(&mv);
        if mv.build == -2{
            return (10000.0 + depth as f64) * color as f64;
        }
        make_move(&mv, b);
        result = -negamax(b, depth-1, -color, &n, eval);
        if result > value {
            value = result;
        }
        undo_move(&mv, b);
    }
    return value;
}

pub fn get_best_move(b:&mut Board, depth:i32, color:i32, n:&Neighbours, eval:fn(board:&Board, nei:&Neighbours) -> f64) -> Move {
    let mvs:Vec<Move> = gen_all_moves(b, &color, n);
    let mut scores:Vec<f64> = vec!();

    for mv in &mvs{
        make_move(mv, b);
        scores.push(-negamax(b, depth -1, -color, n, eval));
        undo_move(mv, b);
    }

    let mut best_score:f64 = scores[0];
    let mut best_score_id:usize = 0;
    for (i, score) in scores.iter().enumerate(){
        //print!("{} ", score);
        //print_move(&mvs[i]);
        if *score > best_score{
            best_score= *score;
            best_score_id = i;
        }
    }
    println!("{}", best_score);
    return mvs[best_score_id];
}
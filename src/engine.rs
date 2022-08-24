use std::cmp::max;
use crate::board::{Board, undo_move};
use crate::board::Move;
use crate::rules::Neighbours;
use f64;
use crate::rules::gen_all_moves;
use rand::prelude::*;
use crate::{make_move, print_board, print_move};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Node {
    pub board: Board,
    pub flag: char,
    pub depth: i32,
    pub value: u64
}

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
pub fn game_is_over(b: &Board) -> f64{
    if b.blocks[b.workers[0] as usize] == 3|| b.blocks[b.workers[1] as usize] == 3{
        return 10000.0;
    }
    if b.blocks[b.workers[2] as usize] == 3|| b.blocks[b.workers[3] as usize] == 3{
        return -10000.0;
    }
    return 0.0;
}

pub fn negamax(mut b:Board, depth:i32, color:i32, n:&Neighbours, eval:fn(board:&Board, nei:&Neighbours) -> f64, mut alpha:f64, mut beta:f64, tt: &mut HashMap<Board, Node>) -> f64{
    let alpha_orig = alpha;
    let mut found = true;
    let mut nd:Node = Node{
        board: b,
        flag: ' ',
        depth:0,
        value:0
    };
    match tt.get(&b) {
        Some(node) => nd = *node,
        None => {found = false}
    }

    if found && nd.depth >= depth{
        if nd.flag == 'e'{
            return nd.value as f64;
        }
        else if nd.flag == 'l'{
            if nd.value as f64> alpha{
                alpha = nd.value as f64;
            }
        }
        else if nd.flag == 'u'{
            if (nd.value as f64) < beta {
                beta = nd.value as f64;
            }
        }

        if alpha >= beta{
            return nd.value as f64;
        }
    }

    let game_over:f64 = game_is_over(&b);
    if game_over != 0.0{
        let db;
        if game_over > 0.0{
            db = depth;
        }
        else{
            db = -depth;
        }
        return ((game_over + db as f64) as f64) * color as f64;
    }

    if depth == 0{
        return eval(&b, n) * color as f64;
    }
    let mut value:f64 = -10000.0;
    let mut result:f64;
    let moves:Vec<Move> = gen_all_moves(b, &color, n);
    if moves.len() == 0{
        return (10000.0 + depth as f64) * -color as f64;
    }
    for mv in moves{
        make_move(&mv, &mut b);
        result = -negamax(b, depth-1, -color, &n, eval, -beta, -alpha, tt);
        undo_move(&mv, &mut b);
        if result > value {
            value = result;
        }
        if value > alpha{
            alpha = value;
        }
        if alpha >= beta{
            break;
        }
    }

    nd.value = value as u64;
    if value <= alpha_orig{
        nd.flag = 'u';
    }
    else if value >= beta{
        nd.flag = 'l';
    }
    else{
        nd.flag = 'e';
    }
    nd.depth = depth;
    (*tt).insert(b, nd);

    return value;
}

pub fn get_best_move(mut b:Board, depth:i32, color:i32, n:&Neighbours, eval:fn(board:&Board, nei:&Neighbours) -> f64, tt:&mut HashMap<Board, Node>) -> Move {
    let mvs:Vec<Move> = gen_all_moves(b, &color, n);
    let mut scores:Vec<f64> = vec!();

    for mv in &mvs{
        make_move(mv, &mut b);
        scores.push(-negamax(b, depth -1, -color, n, eval, -10000.0 as f64, 10000.0 as f64, tt));
        undo_move(mv, &mut b);
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
    println!("{}", best_score * color as f64);
    return mvs[best_score_id];
}
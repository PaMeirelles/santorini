use std::cmp::max;
use crate::board::{Board, undo_move};
use crate::board::Move;
use crate::rules::Neighbours;
use f64;
use crate::rules::gen_all_moves;
use crate::{make_move, print_board, print_move};

use std::time::Instant;
use std::time::Duration;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Node {
    pub board: Board,
    pub flag: char,
    pub depth: i32,
    pub value: u64
}

pub fn neighbour_high(b:&Board, n:&Neighbours, p:[i32;3]) -> i32{
    let wh1 = p[0] * i32::pow(p[1], b.blocks[b.workers[0] as usize] as u32);
    let wh2 = p[0] * i32::pow(p[1], b.blocks[b.workers[1] as usize] as u32);
    let wh3 = p[0] * i32::pow(p[1], b.blocks[b.workers[2] as usize] as u32);
    let wh4 = p[0] * i32::pow(p[1], b.blocks[b.workers[3] as usize] as u32);

    let wn1 = p[2] * n.neighbours[b.workers[0]as usize].len() as i32;
    let wn2 = p[2] * n.neighbours[b.workers[1]as usize].len() as i32;
    let wn3 = p[2] * n.neighbours[b.workers[2]as usize].len() as i32;
    let wn4 = p[2] * n.neighbours[b.workers[3]as usize].len() as i32;

    let p1 = wh1 + wh2 + wn1 +wn2;
    let p2 = wh3 + wh4 + wn3 + wn4;
    return p1 - p2;
}
pub fn game_is_over(b: &Board) -> i32{
    if b.blocks[b.workers[0] as usize] == 3|| b.blocks[b.workers[1] as usize] == 3{
        return 10000;
    }
    if b.blocks[b.workers[2] as usize] == 3|| b.blocks[b.workers[3] as usize] == 3{
        return -10000;
    }
    return 0;
}
pub fn negamax(mut b:Board, depth:i32, color:i32, n:&Neighbours, eval: fn(&Board, &Neighbours) -> i32) -> i32{
    let game_over:i32 = game_is_over(&b);
    if game_over != 0{
        let db;
        if game_over > 0{
            db = depth;
        }
        else{
            db = -depth;
        }
        return (game_over + db) * color ;
    }

    if depth == 0{
        return eval(&b, n) * color;
    }

    let mut value:i32 = -10000;
    let mut result:i32;
    let moves:Vec<Move> = gen_all_moves(b, &color, n);
    if moves.len() == 0{
        return (10000 + depth) * -color;
    }
    for mv in moves{
        make_move(&mv, &mut b);
        result = -negamax(b, depth-1, -color, &n, eval);
        undo_move(&mv, &mut b);
        if result > value {
            value = result;
        }
    }
    return value;
}
pub fn alpha_beta(mut b:Board, depth:i32, color:i32, n:&Neighbours, eval: fn(&Board, &Neighbours) -> i32, mut alpha:i32, mut beta:i32) -> i32{
    let game_over:i32 = game_is_over(&b);
    if game_over != 0{
        let db;
        if game_over > 0{
            db = depth;
        }
        else{
            db = -depth;
        }
        return (game_over + db) * color;
    }

    if depth == 0{
        return eval(&b, n) * color
    }
    let mut value:i32 = -10000;
    let mut result:i32;
    let moves:Vec<Move> = gen_all_moves(b, &color, n);
    if moves.len() == 0{
        return (10000 + depth) * -color
    }
    for mv in moves{
        make_move(&mv, &mut b);
        result = -alpha_beta(b, depth-1, -color, &n, eval, -beta, -alpha);
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
    return value;
}

pub fn nhs(b:&Board, n:&Neighbours) -> i32{
    return neighbour_high(b, n, [6, 2, 1]);
}
pub fn nhc(b:&Board, n:&Neighbours) -> i32{
    return neighbour_high(b, n, [4, 3, 3]);
}
pub fn get_best_move(mut b:Board, color:i32, n:&Neighbours, search_s:&str, eval_s:&str, time_s:&str, remaining_time:Duration) -> Move {
    let now = Instant::now();
    let mut eval:fn(&Board, &Neighbours) -> i32 = nhs;
    let mut time:Duration = Duration::new(1,0);
    let mut n_moves;
    let mut c_moves;
    let mut max_score:i32 = 0;
    let mut best:Move = Move {
        from: 0,
        to: 0,
        build: 0
    };

    let mut depth:i32 = 1;
    match eval_s {
        "mnhs-0" => {eval = nhs; max_score = 46},
        "mnhc-0" => {eval = nhc; max_score = 92}
        _ => {}
    }

    match time_s {
        "ets-0" => {time = remaining_time / 15},
        _ => {}
    }

    loop{
        if now.elapsed() > time{
            break;
        }
        let mvs:Vec<Move> = gen_all_moves(b, &color, n);
        n_moves = mvs.len();
        let mut scores:Vec<i32> = vec!();
        c_moves = 0;
      
        for mv in &mvs{
            c_moves += 1;
            if now.elapsed() > time{
                break;
            }
            make_move(mv, &mut b);
            match search_s {
                "mvb-0" => scores.push(-alpha_beta(b, depth -1, -color, n, eval, -100000, 100000)),
                "mvb-1" => scores.push(-alpha_beta(b, depth -1, -color, n, eval, -100000, 100000)),
                _ => {}
            }
            undo_move(mv, &mut b);
        }

        if now.elapsed() > time{
            break;
        }
        let mut best_score:i32 = scores[0];
        let mut best_score_id:usize = 0;
        for (i, score) in scores.iter().enumerate(){
            if *score > best_score{
                best_score= *score;
                best_score_id = i;
            }
        }
        best = mvs[best_score_id];
        println!("Depth {} ({}). Total time: {:.2?}. Score: {:.2} Best move:", depth, n_moves, now.elapsed(), (best_score * color) as f64 / max_score as f64);
        print_move(&best);
        depth += 1;
    }

    return best;
}
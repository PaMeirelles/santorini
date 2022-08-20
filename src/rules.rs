use crate::{board, make_move, print_board, print_move};
use crate::board::{inverse_move, Move};
use crate::board::Board;
use crate::board::new_move;

pub struct Neighbours {
    pub neighbours:[Vec<i32>; 25]
}
pub fn init_neighbours() -> Neighbours{
    Neighbours{
        neighbours:  [
        vec![1, 5, 6],
        vec![0, 2, 5, 6, 7],
        vec![1, 3, 6, 7, 8],
        vec![2, 4, 7, 8, 9],
        vec![3, 8, 9],

        vec![0, 1, 6, 10, 11],
        vec![0, 1, 2, 5, 7, 10, 11, 12],
        vec![1, 2, 3, 6, 8, 11, 12, 13],
        vec![2, 3, 4, 7, 9, 12, 13, 14],
        vec![3, 4, 8, 13, 14],

        vec![5, 6, 11, 15, 16],
        vec![5, 6, 7, 10, 12, 15, 16, 17],
        vec![6, 7, 8, 11, 13, 16, 17, 18],
        vec![7, 8, 9, 12, 14, 17, 18, 19],
        vec![8, 9, 13, 18, 19],

        vec![10, 11, 16, 20, 21],
        vec![10, 11, 12, 15, 17, 20, 21, 22],
        vec![11, 12, 13, 16, 18, 21, 22, 23],
        vec![12, 13, 14, 17, 19, 22, 23, 24],
        vec![13, 14, 18, 23, 24],

        vec![15, 16, 21],
        vec![15, 16, 17, 20, 22],
        vec![16, 17, 18, 21, 23],
        vec![17, 18, 19, 22, 24],
        vec![18, 19, 23]
            ]
    }
}
pub fn square_is_free(square:&i32, board:&Board) -> bool{
    for worker in board.workers{
        if worker == *square{
            return false;
        }
    }
    if board.blocks[*square as usize] == 4 {
        return false
    }
    return true;
}

pub fn half_move(board:&Board, worker:&usize, neighbours:&Neighbours) -> Vec<Move>{
    let mut moves:Vec<Move> = vec![];

    for neighbour in &neighbours.neighbours[board.workers[*worker]as usize]{
        if square_is_free(&neighbour, board) && board.blocks[*neighbour as usize] - board.blocks[board.workers[*worker] as usize] <= 1{
            if board.blocks[*neighbour as usize] == 3{
                moves.push(new_move(&board.workers[*worker], neighbour, &-2));
            }
            else{moves.push(new_move(&board.workers[*worker], neighbour, &-1));
            }
        }
    }
    return moves;
}

pub fn build_move(board:&Board, square:&i32, neighbours:&Neighbours) -> Vec<Move>{
    let mut moves:Vec<Move> = vec![];

    for neighbour in &neighbours.neighbours[*square as usize]{
        if square_is_free(neighbour, board){
            moves.push(new_move(&-1, square, neighbour))
        }
    }
    return moves;
}

pub fn gen_move(board:&mut Board, worker:&usize, neighbours:&Neighbours) -> Vec<Move>{
    let mut all_moves: Vec<Move> = vec![];
    let half_moves: Vec<Move> = half_move(board, worker, neighbours);

    for hm in half_moves{
        if hm.build == -2{
            all_moves.push(hm);
        }
        else{
            make_move(&hm, board);
            for bm in &build_move(board, &hm.to, neighbours){
                all_moves.push(new_move(&hm.from, &hm.to, &bm.build))
            }
            make_move(&inverse_move(&hm), board);
        }
    }
    return all_moves;
}

pub fn gen_all_moves(board:&mut Board, turn:&i32, neighbours:&Neighbours) -> Vec<Move> {
    let mut worker_one: Vec<Move>;
    let mut worker_two: Vec<Move>;
    if *turn == -1 {
        worker_one = gen_move(board, &2, neighbours);
        worker_two = gen_move(board, &3, neighbours);
    }
    else{
        worker_one = gen_move(board, &0, neighbours);
        worker_two = gen_move(board, &1, neighbours);
    }
    worker_one.append(&mut worker_two);
    return worker_one
}
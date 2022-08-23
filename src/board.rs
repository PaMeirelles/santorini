use std::process::exit;

pub struct Board {
    pub blocks: [i32; 25],
    pub workers: [i32; 4]
}

pub fn new_board(workers:[i32; 4]) -> Board{
    Board {
        blocks: [0; 25],
        workers,
    }
}

pub fn print_blocks(blocks:&[i32;25]){
   let mut counter:i32 = 0;
    for block in blocks{
        print!("{} ", block);
        counter += 1;
        if counter % 5 == 0 {
            println!()
        }
    }
}

pub fn print_workers(workers:&[i32;4]){
    println!("[{}, {}], [{}, {}]", workers[0],workers[1], workers[2], workers[3]);
}

pub fn print_board(board:&Board){
    println!("Blocks: ");
    print_blocks(&board.blocks);
    println!("Workers: ");
    print_workers(&board.workers);
}

#[derive(Clone, Copy)]
pub struct Move {
    pub from: i32,
    pub to: i32,
    // build = -1 -> not set
    // build = -2 -> winning move
    pub build: i32
}

pub fn new_move(from:&i32, to:&i32, build:&i32) -> Move{
    Move{
        from:*from,
        to:*to,
        build:*build,
    }
}
pub fn get_square(id:&i32) -> String {
    let letters: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
    let numbers: [char; 5] = ['1', '2', '3', '4', '5'];
    let x:i32 = *id % 5;
    let y:i32 = *id / 5;
    let mut str = String::new();
    str.push(letters[y as usize]);
    str.push(numbers[x as usize]);
    return str;
}
pub fn print_move(mv:&Move){
    if mv.build >= 0{
        println!("{}({}) -> {}({}) [{}({})]", &mv.from, get_square(&mv.from), &mv.to,
                 get_square(&mv.to), &mv.build, get_square(&mv.build));
    }
    else{
        if mv.build == -1{
            println!("{}({}) -> {}({}) NOT SET", &mv.from, get_square(&mv.from), &mv.to,
                     get_square(&mv.to));
        }
        else if mv.build == -2{
            println!("{}({}) -> {}({}) WON", &mv.from, get_square(&mv.from), &mv.to,
                     get_square(&mv.to));
        }
    }
}

pub fn make_move(mv:&Move, board: &mut Board){
    let mut worker_found:bool = false;
    for worker in board.workers.iter_mut(){
        if *worker == mv.from {
            *worker = mv.to;
            worker_found = true;
            break;
        }
    }
    if !worker_found {
        println!("No worker in 'from' square");
        exit(1);
    }
    if mv.build >= 0 {
        board.blocks[mv.build as usize] += 1;
    }
}

pub fn undo_move(mv:&Move, board: &mut Board){
    let inv:Move = inverse_move(mv);
    make_move(&inv, board);
    if mv.build >=0{
        board.blocks[mv.build as usize] -= 2;

    }
}
pub fn inverse_move(mv: &Move) -> Move{
    return new_move(&mv.to, &mv.from, &mv.build);
}

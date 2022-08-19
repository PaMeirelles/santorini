use std::char::from_digit;

pub struct Board {
    blocks: [i32; 25],
    workers: [i32; 4]
}

pub fn new_board() -> Board{
    Board {
        blocks: [0; 25],
        workers: [0; 4],
    }
}


pub struct Move {
    pub from: i32,
    pub to: i32,
    pub build: i32
}

pub fn new_move(from:i32, to:i32, build:i32) -> Move{
    Move{
        from,
        to,
        build,
    }
}
pub fn get_square(id:i32) -> String {
    let letters: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
    let numbers: [char; 5] = ['1', '2', '3', '4', '5'];
    let x:i32 = id % 5;
    let y:i32 = id / 5;
    let mut str = String::new();
    str.push(letters[x as usize]);
    str.push(numbers[y as usize]);
    return str;
}
pub fn print_move(mv:Move){
    println!("{}({}) -> {}({}) [{}({})]", mv.from, get_square(mv.from), mv.to,
                     get_square(mv.to), mv.build, get_square(mv.build));
}
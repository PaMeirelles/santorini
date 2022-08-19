mod board;

fn main() {
    let m1: board::Move = board::new_move(12, 13, 14);
    board::print_move(m1);
    println!("Hello, world!");
}


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
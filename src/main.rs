use crate::board::{make_move, new_board, print_board, print_move};
use crate::rules::{gen_all_moves, gen_move, init_neighbours, Neighbours};
use eframe::run_native;
mod board;
mod rules;
mod visual;
use eframe::egui;


#[derive(Default)]
struct MyEguiApp {}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

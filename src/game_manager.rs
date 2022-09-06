use crate::board::{Board, Move, move_to_string, new_board, new_move};
use crate::rules::{Neighbours, init_neighbours};
use crate::engine::{get_best_move, Node};
use crate::{gen_all_moves, make_move, print_board, print_move};
use std::time::{Duration, Instant};
use std::fs;
use rand::Rng;
use std::fs::File;
use std::io::Write;
pub fn register_game(id:i32, name_a:&str, name_b:&str, starting_pos:i32, result:bool, time_control: String){
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open("data/matches.csv")
        .unwrap();
    write!(file, "{},{},{},{},{},{}\n", id.to_string(), name_a, name_b, starting_pos.to_string(), result.to_string(), time_control).expect("TODO: panic message");
}

pub fn get_counter() -> i32{
    let data = fs::read_to_string("data/counter.dat").expect("Unable to read file");
    return data.parse::<i32>().unwrap();
}

pub fn update_counter(){
    let mut counter:i32 = get_counter();
    counter += 1;
    fs::write("data/counter.dat", counter.to_string()).expect("Unable to write file");
}

pub fn write_moves(mvs: Vec<Move>, id:i32){
    let mut s:&String = &format!("data/matches/{}.dat", id);
    let file = &File::create(s);
    for mv in &mvs{
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true) // This is needed to append to file
            .open(s)
            .unwrap();
        write!(file, "{}\n", move_to_string(mv)).expect("Unable to write file");
    }
   }

pub fn break_start_pos(start_pos:i32) -> [i32;4]{
    return [start_pos % 25, (start_pos / 25) % 25, (start_pos / 625) % 25, (start_pos / 15625) % 25];
}

pub fn assembly_start_pos(start_pos:[i32;4]) -> i32{
    return start_pos[0] + start_pos[1] * 25 + start_pos[2] * 625 + start_pos[3] * 15625;
}

pub fn invert_start_pos(start_pos:i32) -> i32{
    let sp = break_start_pos(start_pos);
    let new_sp = [sp[2], sp[3], sp[0], sp[1]];
    return assembly_start_pos(new_sp);
}

pub fn play_game(name_a:&str, name_b:&str, starting_pos:i32, t:u64) -> bool{
    let mut b1:Board = new_board(break_start_pos(starting_pos));
    let n:&Neighbours = &init_neighbours();
    let mut best:Move = new_move(&-1, &0, &-1);
    let mut color:i32 = 1;
    let mut time_a = Duration::new(t, 0);
    let mut time_b = Duration::new(t, 0);
    let mut now = Instant::now();
    let mut zero = Duration::new(0, 0);
    let mut result:bool;
    let mut search:[&str;2] = ["", ""];
    let mut eval:[&str;2] = ["", ""];
    let mut time:[&str;2] = ["", ""];
    let mut moves = vec![];
    match name_a {
        "Hero" => {search[0] = "mvb-0"; eval[0] = "mnhs-0"; time[0] = "ets-0"},
        "Lumberjack" => {search[0] = "mvb-1"; eval[0] = "mnhs-0"; time[0] = "ets-0"},
        "Conqueror" => {search[0] = "mvb-1"; eval[0] = "mnhc-0"; time[0] = "ets-0"},
        "Sniper" => {search[0] = "mvb-0"; eval[0] = "mnhc-0"; time[0] = "ets-0"},
        _ => {}
    }
    match name_b {
        "Hero" => {search[1] = "mvb-0"; eval[1] = "mnhs-0"; time[1] = "ets-0"},
        "Lumberjack" => {search[1] = "mvb-1"; eval[1] = "mnhs-0"; time[1] = "ets-0"},
        "Conqueror" => {search[1] = "mvb-1"; eval[1] = "mnhc-0"; time[1] = "ets-0"},
        "Sniper" => {search[1] = "mvb-0"; eval[1] = "mnhc-0"; time[1] = "ets-0"}
        _ => {}
    }

    loop{
        now = Instant::now();
        if color == 1 {
            best = get_best_move(b1, 1, n, search[0], eval[0], time[0], time_a);
            if now.elapsed() < time_a{
                time_a -= now.elapsed();
            }
        } else {
            best = get_best_move(b1, -1, n, search[1], eval[1], time[1],  time_b);
            if now.elapsed() < time_b{
                time_b -= now.elapsed();
            }
        }
        moves.push(best);
        // print_move(&best);
        make_move(&best, &mut b1);
        // print_board(&b1);
        // println!("Time a: {:.2?} Time b: {:.2?}", time_a, time_b);
        if time_a < now.elapsed() || time_b < now.elapsed(){
            if color == 1{
                result = false;
            }
            else{
                result = true;
            }
            break;
        }
        if gen_all_moves(b1, &color, n).len() == 0{
            if color == 1{
                result = false;
            }
            else{
                result = true;
            }
            break;

        }
        if best.build == -2 {
            if color == 1{
                result = true;
            }
            else{
                result = false;
            }
            break;
        }
        color *= -1;
    }
    let time_string = format!("{}+0", (t/60).to_string());
    register_game(get_counter(), name_a, name_b, starting_pos, result, time_string);
    write_moves(moves, get_counter());
    update_counter();
    if result{
        println!("{} 1 x 0 {}", name_a, name_b);
    }
    else{
        println!("{} 0 x 1 {}", name_a, name_b);
    }
    return result;
}

pub fn complete_game(name_a:&str, name_b: &str, starting_pos:i32, t:u64) -> [i32;2]{
    let mut results:[i32;4] = [0, 0, 0, 0];
    results[0] = i32::from(play_game(name_a, name_b, starting_pos, t));
    results[1] = i32::from(play_game(name_a, name_b, invert_start_pos(starting_pos), t));
    results[2] = i32::from(play_game(name_b, name_a, starting_pos, t));
    results[3] = i32::from(play_game(name_b, name_a, invert_start_pos(starting_pos), t));
    let mut final_result:[i32;2] = [0, 0];
    final_result[0] = 2 + results[0] + results[1] - results[2] - results[3];
    final_result[1] = 2 - results[0] - results[1] + results[2] + results[3];

    return final_result;
}

pub fn play_match(name_a:&str, name_b:&str, t:u64, n:i32){
    let mut starting_pos;
    let mut ng = n;
    let mut results:[i32;2] = [0, 0];
    let mut temp_result:[i32;2];
    while ng > 0{
        starting_pos = rand::thread_rng().gen_range(0..390624);
        temp_result = complete_game(name_a, name_b, starting_pos, t);
        ng -= 1;
        results[0] += temp_result[0];
        results[1] += temp_result[1];
        println!("{} x {}", results[0], results[1]);
    }
}

pub fn get_pairings(players:&Vec<String>) -> Vec<(String, String)> {
    let mut pairings = vec![];
    for i in 0..players.len(){
        for j in (i+1)..players.len(){
            pairings.push((players[i].to_string(), players[j].to_string()));
        }
    }
    return pairings;
}

pub fn get_1vall(players:&Vec<String>, star:&String) -> Vec<(String, String)> {
    let mut pairings = vec![];
    for i in 0..players.len() {
        if players[i].to_string() != *star{
            pairings.push((star.to_string(), players[i].to_string()));
        }
    }
    return pairings;
}

pub fn play_tournament(players:Vec<String>, t:u64, n:i32, mode:i32, star:String){
    let pairings:Vec<(String,String)>;
    if mode == 0{
        pairings = get_pairings(&players);
    }
    else {
        pairings = get_1vall(&players, &star);
    }
    let mut scores = vec![];
    let mut starting_pos;
    let mut temp;

    for i in 0..(players.len()){
        scores.push(0);
    }
    for round in 0..n{
        starting_pos = rand::thread_rng().gen_range(0..390624);
        for pairing in &pairings{
            temp = complete_game(&*pairing.0, &*pairing.1, starting_pos, t);
            for i in 0..players.len(){
                if players[i] == pairing.0{
                    scores[i] += temp[0];
                }
                if players[i] == pairing.1{
                    scores[i] += temp[1];
                }
            }
        }
        println!("Round {}:", round);
        for i in 0..players.len(){
            println!("{}: {}", players[i], scores[i]);
        }
    }
}
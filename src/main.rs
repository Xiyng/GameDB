extern crate rustc_serialize;

mod file_io;
mod game;

use file_io::{read_games, write_games};
use game::Game;

fn main() {
    println!("Hello, world!");
}

fn load(path: &str) -> Vec<Game> {
    read_games(path)
}

fn save(path: &str, games: &Vec<Game>) -> () {
    write_games(path, games);
}

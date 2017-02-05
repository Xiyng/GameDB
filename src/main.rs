extern crate rustc_serialize;

mod file_io;
mod game;

use file_io::*;
use game::{Game, GameEdition, GameContent, GameContentEdition};

fn main() {
    println!("Hello, world!");
}

fn load(path: &str) -> () {
    let games = read_games(path);
    let game_editions = read_game_editions(path);
    let game_contents = read_game_contents(path);
    let game_content_editions = read_game_content_editions(path);
    unimplemented!(); // TODO: Add a proper return type and value.
}

fn save(path: &str, games: &Vec<Game>, game_editions: &Vec<GameEdition>, game_contents: &Vec<GameContent>, game_content_editions: &Vec<GameContentEdition>) -> () {
    write_games(path, games);
    write_game_editions(path, game_editions);
    write_game_contents(path, game_contents);
    write_game_content_editions(path, game_content_editions)
}

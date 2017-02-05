extern crate rustc_serialize;

use std::error::Error;
use std::io::{Read, Write};
use std::fs::File;
use rustc_serialize::json;
use game::{Game, GameEdition, GameContent, GameContentEdition};

pub fn write_games(path: &str, games: &Vec<Game>) -> () {
    write(path, games, "Couldn't save the games file", "")
}

pub fn write_game_editions(path: &str, game_editions: &Vec<GameEdition>) -> () {
    write(path, game_editions, "Couldn't save the game editions file", "")
}

pub fn write_game_contents(path: &str, game_contents: &Vec<GameContent>) -> () {
    write(path, game_contents, "Couldn't save the game contents file", "")
}

pub fn write_game_content_editions(path: &str, game_content_editions: &Vec<GameContentEdition>) -> () {
    write(path, game_content_editions, "Couldn't save the game content editions file", "")
}

pub fn read_games(path: &str) -> Vec<Game> {
    read(path, "Couldn't load the games file", "Couldn't read the games file to a string", "Couldn't decode the games data")
}

pub fn read_game_editions(path: &str) -> Vec<GameEdition> {
    read(path, "Couldn't load the game editions file", "Couldn't read the game editions file to a string", "Couldn't decode the game editions data")
}

pub fn read_game_contents(path: &str) -> Vec<GameContent> {
    read(path, "Couldn't load the game contentss file", "Couldn't read the game contents file to a string", "Couldn't decode the game contents data")
}

pub fn read_game_content_editions(path: &str) -> Vec<GameContentEdition> {
    read(path, "Couldn't load the games file", "Couldn't read the game content editions file to a string", "Couldn't decode the game content editions list data")
}

fn write<T>(path: &str, x: &Vec<T>, file_create_error_message: &str, file_write_error_message: &str) -> () where T: rustc_serialize::Encodable {
    let json = json::as_pretty_json(&x);
    let mut file = match File::create(path) {
        Err(error) => panic!("{}: {}", file_create_error_message, error.description()),
        Ok(file)   => (file)
    };
    match file.write_all(json.to_string().as_bytes()) {
        Err(error) => panic!("{}: {}", file_write_error_message, error.description()),
        Ok(_)      => ()
    }
}

pub fn read<T>(path: &str, file_open_error_message: &str, file_read_error_message: &str, decode_error_message: &str) -> T where T: rustc_serialize::Decodable {
    let mut file = match File::open(path) {
        Err(error) => panic!("{}: {}", file_open_error_message, error.description()),
        Ok(file)   => file
    };
    let mut file_content: String = String::new();
    match file.read_to_string(&mut file_content) {
        Err(error) => panic!("{}: {}", file_read_error_message, error.description()),
        Ok(n)      => ()
    };
    let decode_result: json::DecodeResult<T> = json::decode(&file_content);
    match decode_result {
        Err(error) => panic!("{}: {}", decode_error_message, error.description()),
        Ok(data)  => data
    }
}

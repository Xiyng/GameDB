extern crate rustc_serialize;

use std::error::Error;
use std::io::{Read, Write};
use std::fs::File;
use rustc_serialize::json;
use game::{Game, GameEdition, GameContent, GameContentEdition};

pub fn write_games(path: &str, games: &Vec<Game>) -> () {
    let json = json::as_pretty_json(&games);
    let mut file = match File::create(path) {
        Err(error) => panic!("Couldn't save the games file: {}", error.description()),
        Ok(file)   => (file)
    };
    match file.write_all(json.to_string().as_bytes()) {
        Err(error) => panic!(""),
        Ok(_)      => ()
    }
}

pub fn read_games(path: &str) -> Vec<Game> {
    let mut file = match File::open(path) {
        Err(error) => panic!("Couldn't load the games file: {}", error.description()),
        Ok(file)   => file
    };
    let mut file_content: String = String::new();
    match file.read_to_string(&mut file_content) {
        Err(error) => panic!("Couldn't read the file to a string: {}", error.description()),
        Ok(n)      => ()
    };
    let decode_result: json::DecodeResult<Vec<Game>> = json::decode(&file_content);
    match decode_result {
        Err(error) => panic!("Couldn't decode the games list: {}", error.description()),
        Ok(games)  => games
    }
}

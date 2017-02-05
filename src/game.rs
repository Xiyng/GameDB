#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
    pub name: String,
    pub original_platforms: Vec<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GameEdition {
    pub game: Game,
    pub name: String,
    pub platform: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GameContent {
    pub game: Game,
    pub name: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GameContentEdition {
    pub content: GameContent,
    pub edition: GameEdition
}

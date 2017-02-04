#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
    pub name: String,
    pub original_platforms: Vec<String>
}

pub struct GameEdition {
    pub game: Game,
    pub name: String,
    pub platform: String
}

pub struct GameContent {
    pub game: Game,
    pub name: String,
}

pub struct GameContentEdition {
    pub content: GameContent,
    pub edition: GameEdition
}

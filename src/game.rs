pub struct Game {
    name: String,
    original_platforms: Vec<String>
}

pub struct GameEdition {
    game: Game,
    name: String,
    platform: String
}

pub struct GameContent {
    game: Game,
    name: String,
}

pub struct ContentEdition {
    content: GameContent,
    edition: GameEdition
}

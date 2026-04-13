use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub feeds: Vec<Feed>,
    pub down: Option<char>,
    pub up: Option<char>,
    pub select: Option<char>,
    pub back: Option<char>,
}

#[derive(Deserialize)]
pub struct Feed {
    pub name: String,
    pub link: String,
}

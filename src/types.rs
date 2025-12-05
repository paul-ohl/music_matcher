use std::{collections::HashMap, fmt::Display};

pub struct SongSearchResult {
    pub missing_songs: Vec<Song>,
    pub partial_songs: HashMap<Song, Vec<Song>>,
    pub present_songs: Vec<Song>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Song {
    pub artist: String,
    pub title: String,
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.artist, self.title)
    }
}

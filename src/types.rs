use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

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

#[derive(Clone)]
pub struct ArtistWithSongs {
    pub artist: String,
    pub titles: Vec<String>,
}

impl Display for ArtistWithSongs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} songs)", self.artist, self.titles.len())
    }
}

impl Debug for ArtistWithSongs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.artist, self.titles)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DownloadSource {
    Bandcamp,
    YggTorrent,
    SoundCloud,
    YouTube,
    Skip,
    Quit,
}

pub enum DownloadableSource {
    SoundCloud(ArtistWithSongs),
    YouTube(ArtistWithSongs),
}

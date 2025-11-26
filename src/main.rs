#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::Display,
    fs::{DirEntry, read_dir},
    path::{Path, PathBuf},
};

use unidecode::unidecode;

use crate::tag_reader::read_tags;

mod tag_reader;

const PLAYLIST_FILE: &str = "playlist.txt";
const SONGS_DIR: &str = "/home/astro/Music/zik/";
const MISSING_SONGS_FILE: &str = "missing_songs.txt";
const PARTIAL_SONGS_FILE: &str = "partial_matches.txt";
const PRESENT_SONGS_FILE: &str = "present_songs.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Song {
    artist: String,
    title: String,
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.artist, self.title)
    }
}

fn main() {
    let start_path = PathBuf::from(SONGS_DIR);
    let existing_songs = get_file_songs(start_path, 5);
    let playlist = get_playlist_files();
    let mut missing_songs: Vec<&Song> = Vec::new();
    let mut partial_songs: HashMap<&Song, Vec<&Song>> = HashMap::new();
    let mut present_songs: Vec<&Song> = Vec::new();

    println!(
        "Got {} songs from {} artists in the existing folder.",
        existing_songs.len(),
        existing_songs
            .iter()
            .map(|s| s.artist.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    println!(
        "Got {} songs from {} artists in the playlist.",
        playlist.len(),
        playlist
            .iter()
            .map(|s| s.artist.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    playlist.iter().for_each(|song| {
        let found = existing_songs
            .iter()
            .any(|s| s.artist == song.artist && s.title == song.title);
        if found {
            present_songs.push(song);
        } else {
            let partial_found: Vec<&Song> = existing_songs
                .iter()
                .filter(|s| s.title == song.title)
                .collect();
            if partial_found.len() > 0 {
                partial_songs.insert(song, partial_found);
            } else {
                missing_songs.push(song);
            }
        }
    });

    std::fs::write(
        MISSING_SONGS_FILE,
        missing_songs
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .expect("Failed to write missing songs file");
    std::fs::write(
        PARTIAL_SONGS_FILE,
        partial_songs
            .iter()
            .map(|(song, matches)| {
                format!(
                    "{}\n  Matches:\n{}",
                    song,
                    matches
                        .iter()
                        .map(|s| format!("    {}", s))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n"),
    )
    .expect("Failed to write partial songs file");
    std::fs::write(
        PRESENT_SONGS_FILE,
        present_songs
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .expect("Failed to write present songs file");
}

fn get_playlist_files() -> Vec<Song> {
    let contents = std::fs::read_to_string(PLAYLIST_FILE).expect("Failed to read file");
    let file_list_songs: Vec<Song> = contents
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, " - ").collect();
            if parts.len() == 2 {
                Some(Song {
                    artist: unidecode(parts[0]).to_lowercase(),
                    title: unidecode(parts[1]).to_lowercase(),
                })
            } else {
                println!("Invalid line format: {}", line);
                None
            }
        })
        .collect();
    file_list_songs
}

fn get_file_songs(start_path: PathBuf, max_depth: u8) -> Vec<Song> {
    if max_depth == 0 {
        return vec![];
    }
    if let Ok(elements) = read_dir(start_path) {
        elements
            .map(|el| {
                if let Ok(entry) = el {
                    let path = entry.path();
                    if path.is_dir() {
                        get_file_songs(path, max_depth - 1)
                    } else {
                        match read_tags(&path) {
                            Ok(song) => vec![song],
                            Err(e) => match e {
                                tag_reader::TagReadingError::SkippedFileFormat => vec![],
                                _ => {
                                    eprintln!("Error reading tags from {:?}: {}", path, e);
                                    vec![]
                                }
                            },
                        }
                    }
                } else {
                    eprintln!("Error reading directory entry");
                    vec![]
                }
            })
            .flatten()
            .collect()
    } else {
        eprintln!("There was an error reading a directory");
        return vec![];
    }
}

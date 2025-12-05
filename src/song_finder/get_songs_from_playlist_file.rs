use std::{fs::read_to_string, path::PathBuf};

use unidecode::unidecode;

use crate::types::Song;

pub fn get_songs_from_playlist_file(playlist_file: PathBuf) -> Vec<Song> {
    let contents = read_to_string(playlist_file).expect("Failed to read file");
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

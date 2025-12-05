use std::{collections::HashMap, path::PathBuf};

use crate::{
    song_finder::{get_songs_from_playlist_file, get_songs_from_songs_dir},
    types::{Song, SongSearchResult},
};

pub fn find_songs(playlist_file: PathBuf, songs_directory: PathBuf) -> SongSearchResult {
    let existing_songs = get_songs_from_songs_dir(songs_directory, 5);
    let playlist = get_songs_from_playlist_file(playlist_file);
    let mut missing_songs: Vec<Song> = Vec::new();
    let mut partial_songs: HashMap<Song, Vec<Song>> = HashMap::new();
    let mut present_songs: Vec<Song> = Vec::new();

    playlist.into_iter().for_each(|song: Song| {
        let found = existing_songs
            .iter()
            .any(|s| s.artist == song.artist && s.title == song.title);
        if found {
            present_songs.push(song);
        } else {
            let partial_found: Vec<Song> = existing_songs
                .iter()
                .filter(|s| {
                    s.title == song.title
                        || (s.artist == song.artist
                            && (s.title.contains(&song.title) || song.title.contains(&s.title)))
                })
                .cloned()
                .collect();
            if !partial_found.is_empty() {
                partial_songs.insert(song, partial_found);
            } else {
                missing_songs.push(song);
            }
        }
    });

    SongSearchResult {
        missing_songs,
        partial_songs,
        present_songs,
    }
}

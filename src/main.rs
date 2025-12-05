use std::{fs::read_to_string, path::PathBuf};

use music_matcher::{song_finder::find_songs::find_songs, types::SongSearchResult};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <playlist_file> <songs_directory>", args[0]);
        std::process::exit(1);
    }
    let playlist_file: PathBuf = (&args[1]).into();
    let songs_directory: PathBuf = (&args[2]).into();

    let playlist_len = read_to_string(playlist_file.clone())
        .expect("Failed to read file")
        .lines()
        .count();

    let SongSearchResult {
        missing_songs,
        partial_songs,
        present_songs,
    } = find_songs(playlist_file, songs_directory);

    println!(
        "Songs present: {}; partial matches: {}; missing: {}; {}% complete.",
        present_songs.len(),
        partial_songs.len(),
        missing_songs.len(),
        (present_songs.len() as f64 / playlist_len as f64) * 100.0
    );
}

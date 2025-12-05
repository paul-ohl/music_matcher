use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use music_matcher::{
    download_songs::{
        formatters::{bandcamp_search_formatter, ygg_torrent_search_formatter},
        select_source::select_source,
    },
    song_finder::find_songs::find_songs,
    types::{ArtistWithSongs, DownloadSource, DownloadableSource, Song, SongSearchResult},
};

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
        ((present_songs.len() as f64 / playlist_len as f64) * 100.0).round()
    );

    let sorted_songs = get_songs_sorted(&missing_songs);
    let mut dl_list: Vec<DownloadableSource> = Vec::new();
    for artist_with_songs in &sorted_songs {
        loop {
            let choice = select_source(artist_with_songs);
            match choice {
                DownloadSource::Skip => {
                    println!("Skipping {}", artist_with_songs.artist);
                    break;
                }
                DownloadSource::Quit => {
                    println!("Quitting.");
                    return;
                }
                DownloadSource::YouTube => {
                    println!("{} added to youtube dl list", artist_with_songs.artist);
                    dl_list.push(DownloadableSource::YouTube(artist_with_songs.clone()));
                    break;
                }
                DownloadSource::SoundCloud => {
                    println!("{} added to soundcloud dl list", artist_with_songs.artist);
                    dl_list.push(DownloadableSource::SoundCloud(artist_with_songs.clone()));
                    break;
                }
                DownloadSource::YggTorrent => {
                    webbrowser::open(&ygg_torrent_search_formatter(&artist_with_songs.artist))
                        .unwrap();
                }
                DownloadSource::Bandcamp => {
                    webbrowser::open(&bandcamp_search_formatter(&artist_with_songs.artist))
                        .unwrap();
                }
            }
        }
    }
    println!("Download list prepared with {} sources.", dl_list.len());
}

fn get_songs_sorted(missing_songs: &[Song]) -> Vec<ArtistWithSongs> {
    let mut songs: Vec<ArtistWithSongs> = missing_songs
        .iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<String>>, song| {
                acc.entry(song.artist.clone())
                    .or_default()
                    .push(song.title.clone());
                acc
            },
        )
        .iter()
        .map(|(artist, titles)| ArtistWithSongs {
            artist: artist.clone(),
            titles: titles.clone(),
        })
        .collect();
    songs.sort_by(|a, b| b.titles.len().cmp(&a.titles.len()));
    songs
}

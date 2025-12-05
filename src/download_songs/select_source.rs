use crate::types::{ArtistWithSongs, DownloadSource};
use std::io::{Write, stdin, stdout};

pub fn select_source(artist_with_songs: &ArtistWithSongs) -> DownloadSource {
    println!(
        "For artist: {:?}\nWhat do you want to do?",
        artist_with_songs.artist
    );
    loop {
        print!(
            "Select from options: [b]andcamp, ygg[t]orrent, [s]oundcloud, [y]outube, skip[x], [q]uit: "
        );
        stdout().flush().unwrap();
        let mut input = String::new();
        let input = match stdin().read_line(&mut input) {
            Ok(_) => Some(input.trim().to_ascii_lowercase()),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                None
            }
        };

        let source = match input.as_deref() {
            Some("b") => Some(DownloadSource::Bandcamp),
            Some("t") => Some(DownloadSource::YggTorrent),
            Some("s") => Some(DownloadSource::SoundCloud),
            Some("y") => Some(DownloadSource::YouTube),
            Some("x") => Some(DownloadSource::Skip),
            Some("q") => Some(DownloadSource::Quit),
            _ => {
                println!("Invalid input. Please try again.");
                None
            }
        };
        if let Some(source) = source {
            break source;
        }
    }
}

use console::Term;

use crate::types::{ArtistWithSongs, DownloadSource};
use std::io::{Write, stdout};

pub fn select_source(artist_with_songs: &ArtistWithSongs) -> DownloadSource {
    println!(
        "For artist: {:?}\nWhat do you want to do?",
        artist_with_songs
    );
    let term = Term::stdout();

    loop {
        print!(
            "Select from options: [b]andcamp, ygg[t]orrent, [s]oundcloud, [y]outube, skip[x], [q]uit: "
        );
        stdout().flush().unwrap();

        let input = match term.read_char() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        let source = match input {
            'b' => Some(DownloadSource::Bandcamp),
            't' => Some(DownloadSource::YggTorrent),
            's' => Some(DownloadSource::SoundCloud),
            'y' => Some(DownloadSource::YouTube),
            'x' => Some(DownloadSource::Skip),
            'q' => Some(DownloadSource::Quit),
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

use std::{fs::read_dir, path::PathBuf};

use crate::{
    song_finder::tag_reader::{TagReadingError, read_tags},
    types::Song,
};

pub fn get_songs_from_songs_dir(start_path: PathBuf, max_depth: u8) -> Vec<Song> {
    if max_depth == 0 {
        return vec![];
    }
    if let Ok(elements) = read_dir(start_path) {
        elements
            .flat_map(|el| {
                if let Ok(entry) = el {
                    let path = entry.path();
                    if path.is_dir() {
                        get_songs_from_songs_dir(path, max_depth - 1)
                    } else {
                        match read_tags(&path) {
                            Ok(song) => vec![song],
                            Err(e) => match e {
                                TagReadingError::SkippedFileFormat => vec![],
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
            .collect()
    } else {
        eprintln!("There was an error reading a directory");
        vec![]
    }
}

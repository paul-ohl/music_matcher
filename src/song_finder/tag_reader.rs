use unidecode::unidecode;

use crate::types::Song;

#[derive(Debug, thiserror::Error)]
pub enum TagReadingError {
    #[error("Skipped file format")]
    SkippedFileFormat,
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("ID3 error: {0}")]
    Id3Error(#[from] multitag::Error),
    #[error("Missing artist tag in {0}")]
    MissingArtistTag(std::path::PathBuf),
    #[error("Missing title tag in {0}")]
    MissingTitleTag(std::path::PathBuf),
}

pub fn read_tags(path: &std::path::Path) -> Result<Song, TagReadingError> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("jpg") | Some("pdf") | Some("png") | Some("txt") => {
            return Err(TagReadingError::SkippedFileFormat);
        }
        _ => {}
    }
    let tag = multitag::Tag::read_from_path(path)?;
    if let Some(artist) = tag.artist() {
        if let Some(title) = tag.title() {
            Ok(Song {
                artist: unidecode(&artist).to_lowercase(),
                title: unidecode(title).to_lowercase(),
            })
        } else {
            Err(TagReadingError::MissingTitleTag(path.to_path_buf()))
        }
    } else {
        Err(TagReadingError::MissingArtistTag(path.to_path_buf()))
    }
}

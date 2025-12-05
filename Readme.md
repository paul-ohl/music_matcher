# Music matcher

I use this project to see how up-to-date my local music library is compared to
my Spotify liked songs.

## Usage

As of right now, the project is pretty simplistic, so most things are hard-coded
in the `main.rs` file.

First, export your spotify favorite playlist and/or any other playlists you want
to check to txt format using [TuneMyMusic](https://www.tunemymusic.com/). By the
way this works with many other platforms.

Then copy that file to the project root and rename it to `playlist.txt`. I have
left mine in place for example.

Then edit the `SONGS_DIR` variable in the `main.rs` to point to your music
directory. (As I said, simplistic project, I just wanted it to work)

Finally run `cargo run`. This project requires the rust tooling, but no special
other dependencies.

It will produce 3 files:
- `present_songs.txt`: These songs are in your local library, meaning the title
and artist match (I haven't included checking for album as it doesn't make sense
in my use case)
- `missing_songs.txt`: These songs are not in your local library.
- `partial_matches.txt`: The title matched but not the artist. I added this
because it often happens that the song was made by multiple artists and that the
way the artists are saved in the metadata vary from one service to another so I
can't be certain that it's going to match.

## Next steps

Improved finding algorithm: If the title of the needle matches a substring of an
entry in the haystack, this should be considered a partial match instead of a
missing song. I should be able to compare artists one by one for one song,
instead of appending the names with commas.

Other functionality related to file metadata.

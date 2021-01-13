use crate::track::Track;
use crate::Rocket;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to open file for reading track data")]
    OpenTrackFile(#[source] std::io::Error),
    #[error("Failed to deserialize track data")]
    DeserializeTracks(#[source] bincode::Error),
}

pub struct Player {
    tracks: HashMap<String, Track>,
}

impl Rocket for Player {
    /// Get Track by name.
    fn get_track(&self, name: &str) -> Option<&Track> {
        self.tracks.get(name)
    }
}

impl Player {
    /// Load track data from file for playback.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        // Load from file
        let file = File::open(path).map_err(Error::OpenTrackFile)?;
        let tracks_vec: Vec<Track> =
            bincode::deserialize_from(file).map_err(Error::DeserializeTracks)?;

        // Convert to a HashMap for perf (not benchmarked)
        let mut tracks_map = HashMap::with_capacity(tracks_vec.len());
        for track in tracks_vec {
            tracks_map.insert(track.get_name().to_owned(), track);
        }

        Ok(Self { tracks: tracks_map })
    }
}
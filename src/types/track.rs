use musickeyfinder::types::Key;

/// A struct representing a track that is stored on the computer
pub struct Track {
    /// id of the track used to identify it in the array
    id: Option<i32>,
    /// name of the trac
    name: String,
    /// path to the file of the track on the pc
    path: std::path::PathBuf,
    /// (melodic) key of the track
    key: Option<Key>,
}

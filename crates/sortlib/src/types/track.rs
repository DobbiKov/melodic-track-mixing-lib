use crate::types::key::Key;

/// A struct representing a track that is stored on the computer
#[derive(Debug, Clone)]
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

impl Track {
    pub fn new(
        id: Option<i32>,
        name: impl Into<String>,
        path: impl Into<std::path::PathBuf>,
        key: Option<Key>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            path: path.into(),
            key,
        }
    }
    pub fn from_pair(name: &str, key: &str) -> Self {
        let path = std::path::PathBuf::new();
        let t_key = Key::from_camelot(key).unwrap();
        Track::new(None, name.to_string(), path, Some(t_key))
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

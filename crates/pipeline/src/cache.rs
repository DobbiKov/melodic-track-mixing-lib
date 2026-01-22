use std::error::Error;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};

use sortlib::types::key::Key;

#[derive(Debug, Clone)]
pub struct KeyCacheEntry {
    pub key: Key,
    pub confidence: f32,
}

pub struct KeyCache {
    conn: Connection,
}

impl KeyCache {
    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS track_keys (
                path TEXT PRIMARY KEY,
                mtime INTEGER NOT NULL,
                size INTEGER NOT NULL,
                key TEXT NOT NULL,
                key_confidence REAL NOT NULL,
                analyzed_at INTEGER NOT NULL
            );",
        )?;
        Ok(Self { conn })
    }

    pub fn get_cached_key(&self, path: &Path) -> Result<Option<KeyCacheEntry>, Box<dyn Error>> {
        let (mtime, size) = file_signature(path)?;
        let path_key = path.to_string_lossy();

        let row = self
            .conn
            .query_row(
                "SELECT key, key_confidence, mtime, size FROM track_keys WHERE path = ?1",
                params![path_key.as_ref()],
                |row| {
                    let key: String = row.get(0)?;
                    let confidence: f64 = row.get(1)?;
                    let cached_mtime: i64 = row.get(2)?;
                    let cached_size: i64 = row.get(3)?;
                    Ok((key, confidence, cached_mtime, cached_size))
                },
            )
            .optional()?;

        let Some((key_str, confidence, cached_mtime, cached_size)) = row else {
            return Ok(None);
        };

        if cached_mtime != mtime || cached_size != size {
            return Ok(None);
        }

        let key = Key::from_camelot(&key_str)?;
        Ok(Some(KeyCacheEntry {
            key,
            confidence: confidence as f32,
        }))
    }

    pub fn store_key(&self, path: &Path, entry: &KeyCacheEntry) -> Result<(), Box<dyn Error>> {
        let (mtime, size) = file_signature(path)?;
        let analyzed_at = unix_now();
        let path_key = path.to_string_lossy();
        let key_str = entry.key.to_string();
        let confidence = entry.confidence as f64;

        self.conn.execute(
            "INSERT INTO track_keys (path, mtime, size, key, key_confidence, analyzed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(path) DO UPDATE SET
                mtime = excluded.mtime,
                size = excluded.size,
                key = excluded.key,
                key_confidence = excluded.key_confidence,
                analyzed_at = excluded.analyzed_at",
            params![path_key.as_ref(), mtime, size, key_str, confidence, analyzed_at],
        )?;
        Ok(())
    }
}

fn file_signature(path: &Path) -> Result<(i64, i64), Box<dyn Error>> {
    let metadata = std::fs::metadata(path)?;
    let mtime = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;
    let size = metadata.len() as i64;
    Ok((mtime, size))
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

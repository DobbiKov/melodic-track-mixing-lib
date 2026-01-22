use std::path::Path;

use loggit::{info, warn};
use stratum_dsp::{analyze_audio, AnalysisConfig};

use crate::audio::decode_audio_mono_f32;
use crate::cache::{KeyCache, KeyCacheEntry};
use sortlib::algorithm::melodic_sort;
use sortlib::types::key::{Key, KeyLetter};
use sortlib::types::track::Track;

pub fn analyze_tracks<P: AsRef<Path>>(paths: &[P]) -> Vec<Track> {
    analyze_tracks_with_cache(paths, None)
}

pub fn analyze_tracks_with_cache<P: AsRef<Path>>(
    paths: &[P],
    cache: Option<&KeyCache>,
) -> Vec<Track> {
    let mut tracks = Vec::with_capacity(paths.len());

    for (idx, path) in paths.iter().enumerate() {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy();
        info!("analyze_tracks: analyzing {}", path_str);
        let cached_key = cache.and_then(|cache| match cache.get_cached_key(path_ref) {
            Ok(value) => value.map(|entry| entry.key),
            Err(err) => {
                warn!("analyze_tracks: cache lookup failed for {} ({})", path_str, err);
                None
            }
        });

        let key = if cached_key.is_some() {
            cached_key
        } else {
            match decode_audio_mono_f32(path_ref) {
                Ok((samples, sample_rate)) => match analyze_audio(&samples, sample_rate, AnalysisConfig::default()) {
                    Ok(result) => match stratum_key_to_camelot(result.key) {
                        Ok(key) => {
                            if let Some(cache) = cache {
                                let entry = KeyCacheEntry {
                                    key: key.clone(),
                                    confidence: result.key_confidence,
                                };
                                if let Err(err) = cache.store_key(path_ref, &entry) {
                                    warn!("analyze_tracks: cache store failed for {} ({})", path_str, err);
                                }
                            }
                            Some(key)
                        }
                        Err(err) => {
                            warn!("analyze_tracks: invalid key for {} ({})", path_str, err);
                            None
                        }
                    },
                    Err(err) => {
                        warn!("analyze_tracks: analysis failed for {} ({})", path_str, err);
                        None
                    }
                },
                Err(err) => {
                    warn!("analyze_tracks: decode failed for {} ({})", path_str, err);
                    None
                }
            }
        };
        if let Some(u_key) = &key {
            info!("Analyzed, the key is: {}", u_key);
        } else {
            info!("No key!")
        }

        let name = path_ref
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("unknown")
            .to_string();

        tracks.push(Track::new(
            Some(idx as i32),
            name,
            path_ref.to_path_buf(),
            key,
        ));
    }

    tracks
}

pub fn analyze_and_sort_tracks<P: AsRef<Path>>(
    paths: &[P],
    limit: usize,
) -> std::collections::LinkedList<Track> {
    let tracks = analyze_tracks(paths);
    melodic_sort(&tracks, limit)
}

pub fn analyze_and_sort_tracks_with_cache<P: AsRef<Path>>(
    paths: &[P],
    limit: usize,
    cache_path: &Path,
) -> std::collections::LinkedList<Track> {
    let cache = match KeyCache::open(cache_path) {
        Ok(cache) => Some(cache),
        Err(err) => {
            warn!("analyze_and_sort_tracks: cache open failed ({})", err);
            None
        }
    };

    let tracks = analyze_tracks_with_cache(paths, cache.as_ref());
    melodic_sort(&tracks, limit)
}

fn stratum_key_to_camelot(value: stratum_dsp::Key) -> Result<Key, String> {
    let (number, letter) = match value {
        stratum_dsp::Key::Major(pitch_class) => {
            const MAJOR_NUMBERS: [u8; 12] = [8, 3, 10, 5, 12, 7, 2, 9, 4, 11, 6, 1];
            let idx = (pitch_class % 12) as usize;
            (MAJOR_NUMBERS[idx], KeyLetter::B)
        }
        stratum_dsp::Key::Minor(pitch_class) => {
            const MINOR_NUMBERS: [u8; 12] = [5, 12, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10];
            let idx = (pitch_class % 12) as usize;
            (MINOR_NUMBERS[idx], KeyLetter::A)
        }
    };

    Key::new(number, letter).map_err(|err| err.to_string())
}

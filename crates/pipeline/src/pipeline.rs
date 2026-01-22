use std::path::Path;

use loggit::{info, warn};
use rayon::prelude::*;
use stratum_dsp::{analyze_audio, AnalysisConfig};

use crate::audio::decode_audio_mono_f32;
use crate::cache::{KeyCache, KeyCacheEntry};
use sortlib::algorithm::melodic_sort;
use sortlib::types::key::{Key, KeyLetter};
use sortlib::types::track::Track;

pub fn analyze_tracks<P: AsRef<Path> + Sync>(paths: &[P]) -> Vec<Track> {
    analyze_tracks_with_cache(paths, None)
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessingMode {
    Parallel,
    Serial,
}

pub fn analyze_tracks_with_cache<P: AsRef<Path> + Sync>(
    paths: &[P],
    cache_path: Option<&Path>,
) -> Vec<Track> {
    analyze_tracks_with_cache_mode(paths, cache_path, ProcessingMode::Parallel)
}

pub fn analyze_tracks_with_cache_mode<P: AsRef<Path> + Sync>(
    paths: &[P],
    cache_path: Option<&Path>,
    mode: ProcessingMode,
) -> Vec<Track> {
    let mut tracks: Vec<(usize, Track)> = match mode {
        ProcessingMode::Parallel => paths
            .par_iter()
            .enumerate()
            .map(|(idx, path)| analyze_one_track(idx, path.as_ref(), cache_path))
            .collect(),
        ProcessingMode::Serial => paths
            .iter()
            .enumerate()
            .map(|(idx, path)| analyze_one_track(idx, path.as_ref(), cache_path))
            .collect(),
    };

    tracks.sort_by_key(|(idx, _)| *idx);
    tracks.into_iter().map(|(_, track)| track).collect()
}

pub fn analyze_and_sort_tracks<P: AsRef<Path> + Sync>(
    paths: &[P],
    limit: usize,
) -> std::collections::LinkedList<Track> {
    let tracks = analyze_tracks(paths);
    melodic_sort(&tracks, limit)
}

pub fn analyze_and_sort_tracks_with_cache<P: AsRef<Path> + Sync>(
    paths: &[P],
    limit: usize,
    cache_path: &Path,
) -> std::collections::LinkedList<Track> {
    let tracks = analyze_tracks_with_cache(paths, Some(cache_path));
    melodic_sort(&tracks, limit)
}

pub fn analyze_and_sort_tracks_with_cache_mode<P: AsRef<Path> + Sync>(
    paths: &[P],
    limit: usize,
    cache_path: &Path,
    mode: ProcessingMode,
) -> std::collections::LinkedList<Track> {
    let tracks = analyze_tracks_with_cache_mode(paths, Some(cache_path), mode);
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

fn analyze_one_track(idx: usize, path: &Path, cache_path: Option<&Path>) -> (usize, Track) {
    let path_str = path.to_string_lossy();
    info!("analyze_tracks: analyzing {}", path_str);

    let cache = cache_path.and_then(|cache_path| match KeyCache::open(cache_path) {
        Ok(cache) => Some(cache),
        Err(err) => {
            warn!("analyze_tracks: cache open failed ({})", err);
            None
        }
    });

    let cached_key = cache.as_ref().and_then(|cache| match cache.get_cached_key(path) {
        Ok(value) => value.map(|entry| entry.key),
        Err(err) => {
            warn!("analyze_tracks: cache lookup failed for {} ({})", path_str, err);
            None
        }
    });

    let key = if cached_key.is_some() {
        cached_key
    } else {
        match decode_audio_mono_f32(path) {
            Ok((samples, sample_rate)) => match analyze_audio(&samples, sample_rate, AnalysisConfig::default()) {
                Ok(result) => match stratum_key_to_camelot(result.key) {
                    Ok(key) => {
                        if let Some(cache) = cache.as_ref() {
                            let entry = KeyCacheEntry {
                                key: key.clone(),
                                confidence: result.key_confidence,
                            };
                            if let Err(err) = cache.store_key(path, &entry) {
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
        info!("No key!");
    }

    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string();

    (
        idx,
        Track::new(Some(idx as i32), name, path.to_path_buf(), key),
    )
}

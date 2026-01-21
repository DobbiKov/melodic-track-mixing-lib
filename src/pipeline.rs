use std::path::Path;

use loggit::{info, warn};
use stratum_dsp::{analyze_audio, AnalysisConfig};

use crate::algorithm::melodic_sort;
use crate::audio::decode_audio_mono_f32;
use crate::types::key::Key;
use crate::types::track::Track;

pub fn analyze_tracks<P: AsRef<Path>>(paths: &[P]) -> Vec<Track> {
    let mut tracks = Vec::with_capacity(paths.len());

    for (idx, path) in paths.iter().enumerate() {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy();
        info!("analyze_tracks: analyzing {}", path_str);
        let key = match decode_audio_mono_f32(path_ref) {
            Ok((samples, sample_rate)) => {
                info!("Start analyze key");
                let an_k = analyze_audio(&samples, sample_rate, AnalysisConfig::default());
                info!("Stop analyze key");
                match an_k {
                    Ok(result) => match Key::try_from(result.key) {
                        Ok(key) => Some(key),
                        Err(err) => {
                            warn!("analyze_tracks: invalid key for {} ({})", path_str, err);
                            None
                        }
                    },
                    Err(err) => {
                        warn!("analyze_tracks: analysis failed for {} ({})", path_str, err);
                        None
                    }
                }
            }
            Err(err) => {
                warn!("analyze_tracks: decode failed for {} ({})", path_str, err);
                None
            }
        };
        if let Some(u_key) = key {
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

use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;

use crate::audio_analyze::{analyze_audio_file, AudioData};
use crate::types::key::Key;

const MAJOR_PROFILE: [f64; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];
const MINOR_PROFILE: [f64; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

fn samples_to_mono(samples: &[f64], channels: usize) -> Vec<f64> {
    samples
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f64>() / channels as f64)
        .collect()
}

fn get_pitches(audio_data: AudioData) -> Vec<(f64, f64)> {
    let (samples, channels, frame_rate) = audio_data.data();
    let mono = samples_to_mono(&samples, channels);

    const WINDOW: usize = 8192;
    const PADDING: usize = WINDOW / 2;
    const POWER_THRESHOLD: f64 = 0.3; // tune for your scaling
    const CLARITY_THRESHOLD: f64 = 0.4; // less strict

    let mut detector = McLeodDetector::new(WINDOW, PADDING);

    mono.chunks(WINDOW)
        .filter(|chunk| chunk.len() == WINDOW) // skip partial tail
        .filter_map(|chunk| {
            detector.get_pitch(
                chunk,
                frame_rate as usize,
                POWER_THRESHOLD,
                CLARITY_THRESHOLD,
            )
        })
        .map(|p| (p.frequency, p.clarity))
        .collect()
}

fn get_histogram_from_pitches(pitches: Vec<(f64, f64)>) -> [f64; 12] {
    let mut histogram = [0.064; 12];
    for &(freq, clarity) in &pitches {
        if freq > 0.0 && clarity >= 0.7 {
            let midi = 69.0 + 12.0 * (freq / 440.0).log2();
            let pitch_class = (midi.round() as i32).rem_euclid(12) as usize;

            // Weight by clarity
            histogram[pitch_class] += clarity;
        }
    }
    histogram
}

fn rotate(profile: &[f64; 12], shift: usize) -> [f64; 12] {
    let mut out = [0.0; 12];
    for i in 0..12 {
        out[i] = profile[(i + shift) % 12];
    }
    out
}

fn correlation(hist: &[f64; 12], profile: &[f64; 12]) -> f64 {
    let mean_h = hist.iter().copied().sum::<f64>() / 12.0;
    let mean_p = profile.iter().sum::<f64>() / 12.0;
    let mut num = 0.0;
    let mut denom_h = 0.0;
    let mut denom_p = 0.0;

    for i in 0..12 {
        let h = hist[i] as f64 - mean_h;
        let p = profile[i] - mean_p;
        num += h * p;
        denom_h += h * h;
        denom_p += p * p;
    }

    if denom_h == 0.0 || denom_p == 0.0 {
        0.0
    } else {
        num / (denom_h.sqrt() * denom_p.sqrt())
    }
}

fn detect_key(histogram: &[f64; 12]) -> Option<Key> {
    let mut best_score = -1.0;
    let mut best_key: Option<Key> = None;

    for i in 0..12 {
        let major = rotate(&MAJOR_PROFILE, i);
        let minor = rotate(&MINOR_PROFILE, i);

        let major_score = correlation(histogram, &major);
        let minor_score = correlation(histogram, &minor);

        if major_score > best_score {
            best_score = major_score;
            best_key = Some(Key::from(match i {
                0 => "8B",  // C major
                1 => "3B",  // C# major
                2 => "10B", // D major
                3 => "5B",  // D# major
                4 => "12B", // E major
                5 => "7B",  // F major
                6 => "2B",  // F# major
                7 => "9B",  // G major
                8 => "4B",  // G# major
                9 => "11B", // A major
                10 => "6B", // A# major
                11 => "1B", // B major
                _ => unreachable!(),
            }));
        }

        if minor_score > best_score {
            best_score = minor_score;
            best_key = Some(Key::from(match i {
                0 => "5A",   // C minor
                1 => "12A",  // C# minor
                2 => "7A",   // D minor
                3 => "2A",   // D# minor
                4 => "9A",   // E minor
                5 => "4A",   // F minor
                6 => "11A",  // F# minor
                7 => "6A",   // G minor
                8 => "1A",   // G# minor
                9 => "8A",   // A minor
                10 => "3A",  // A# minor
                11 => "10A", // B minor
                _ => unreachable!(),
            }));
        }
    }
    best_key
}

pub fn analyze_audio_key_from_file(file: impl Into<std::path::PathBuf>) -> Option<Key> {
    let audio_data = analyze_audio_file(file).expect("Couldn't analyze audio file!");

    let pitches = get_pitches(audio_data);
    let histogram = get_histogram_from_pitches(pitches);
    detect_key(&histogram)
}

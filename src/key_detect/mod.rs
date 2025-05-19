use std::{ffi::c_uint, mem::MaybeUninit, path::PathBuf};

use symphonia::core::{
    audio::SampleBuffer, codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream,
    meta::MetadataOptions, probe::Hint,
};

use crate::audio_analyze::{analyze_audio_file, AudioData};

/// All the raw C++ symbols live in the crate produced by `bindgen`.
/// Here we assume you declared it as `bindings` in `build.rs`.
use crate::bindings as kf;

/// A lazily-constructed, process-wide `KeyFinder::KeyFinder` –
/// equivalent to the C++ `static` in the original snippet.
//fn keyfinder_instance() -> &'static mut kf::KeyFinder_KeyFinder {
//    // `once_cell` lets us build the object exactly once, on first use.
//
//    unsafe {
//        // C++ default-ctor: KeyFinder::KeyFinder()
//        let mut tmp = MaybeUninit::<kf::KeyFinder_KeyFinder>::uninit();
//        kf::KeyFinder_KeyFinder_KeyFinder(tmp.as_mut_ptr());
//        tmp.assume_init()
//    }
//}

/// Convenience wrapper that fills a `KeyFinder::AudioData` from an
/// interleaved slice of `f64` samples (`channels` must match the slice).
fn build_audio_data(
    samples: &[f64],
    frame_rate: c_uint,
    channels: c_uint,
) -> kf::KeyFinder_AudioData {
    let mut audio = unsafe { kf::KeyFinder_AudioData::new() };

    unsafe {
        audio.setFrameRate(frame_rate);
        audio.setChannels(channels);
        audio.addToSampleCount(samples.len() as c_uint);

        for (i, &s) in samples.iter().enumerate() {
            audio.setSample(i as c_uint, s);
        }
    }

    audio
}

pub fn analyze_key_from_file(path: impl Into<std::path::PathBuf>) -> kf::KeyFinder_key_t {
    let path: std::path::PathBuf = path.into();
    let (samples1, channels1, frame_rate1) = analyze_audio_file(path)
        .expect("Coudln't process the file")
        .data();
    // ------------------------------------------------------------
    let frame_rate: c_uint = frame_rate1 as c_uint;
    let channels: c_uint = channels1 as c_uint;
    let samples: Vec<f64> = samples1;
    // ------------------------------------------------------------

    let mut audio = build_audio_data(&samples, frame_rate, channels);

    let finder = unsafe { kf::KeyFinder_KeyFinder_new() };

    // Call KeyFinder::keyOfAudio()
    let key = unsafe { kf::KeyFinder_KeyFinder_keyOfAudio(finder, &audio) };

    // Now do something useful with the detected key
    key
}

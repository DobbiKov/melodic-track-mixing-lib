use std::{ffi::c_uint, mem::MaybeUninit};

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

fn main() {
    // ------------------------------------------------------------
    // ---  Replace this with real data from your audio pipeline  ---
    let frame_rate: c_uint = 44_100;
    let channels: c_uint = 1;
    let samples:   Vec<f64> = /* grab audio samples here */ vec![0.0; 16_384];
    // ------------------------------------------------------------
    // TODO: make loading data from audio file

    let mut audio = build_audio_data(&samples, frame_rate, channels);

    let finder = unsafe { kf::KeyFinder_KeyFinder_new() };
    // Call KeyFinder::keyOfAudio()
    let key = unsafe { kf::KeyFinder_KeyFinder_keyOfAudio(finder, &audio) };

    // Now do something useful with the detected key
    println!("Detected key id: {}", key);
}

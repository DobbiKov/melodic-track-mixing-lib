use std::{ops::Deref, path::PathBuf};

use crate::errors::AnalyzeAudioError;
use symphonia::core::{
    audio::SampleBuffer, codecs::DecoderOptions, conv::IntoSample, formats::FormatOptions,
    io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};

/// An audio data for key analyze
pub struct AudioData {
    samples: Vec<f64>,
    channels: usize,
    frame_rate: u64,
}

impl AudioData {
    /// Create an instance of [AudioData] using (samples, channels, frame_rate)
    pub fn new(samples: Vec<f64>, channels: usize, frame_rate: u64) -> Self {
        Self {
            samples,
            channels,
            frame_rate,
        }
    }

    /// Returns a tuple of (samples, channels, frame_rate)
    pub fn data(self) -> (Vec<f64>, usize, u64) {
        (self.samples, self.channels, self.frame_rate)
    }
}

pub fn analyze_audio_file(path: impl Into<PathBuf>) -> Result<AudioData, AnalyzeAudioError> {
    let path: PathBuf = path.into();

    // Create a media source. Note that the MediaSource trait is automatically implemented for File,
    // among other types.
    let file = Box::new(std::fs::File::open(path).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate. In this
    // example we'll leave it empty.
    let hint = Hint::new();

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    let channels = track.codec_params.channels.unwrap().count();
    let frame_rate = track.codec_params.sample_rate.unwrap();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    let mut sample_count = 0;
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        // Get the next packet from the format reader.
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(_) => break,
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    sample_count += buf.samples().len();
                    print!("\rDecoded {} samples", sample_count);
                }
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    if sample_buf.is_none() {
        return Err(AnalyzeAudioError::EmptySampleBuffer);
    }
    let sample_buf = sample_buf.unwrap();
    let samples = Vec::from(sample_buf.samples())
        .into_iter()
        .map(|e| e.into())
        .collect();

    Ok(AudioData::new(samples, channels, frame_rate as u64))
}

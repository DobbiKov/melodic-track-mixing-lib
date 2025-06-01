#[derive(thiserror::Error, Debug)]
pub enum AnalyzeAudioError {
    #[error("empty sample buffer")]
    EmptySampleBuffer,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateKeyError {
    #[error("invalid letter given")]
    InvalidLetterError,
    #[error("invalid number given")]
    InvalidNumberError,
}

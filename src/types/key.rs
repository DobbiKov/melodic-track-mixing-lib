use thiserror::Error;

pub struct Key {
    num: i8,
    letter: char,
}

#[derive(Error)]
pub enum CreateKeyError {
    #[error("invalid letter given")]
    InvalidLetterError,
    #[error("invalid number given")]
    InvalidNumberError,
}

impl Key {
    pub fn new(num: i8, letter: char) -> Result<Self, CreateKeyError> {
        if num > 12 || num <= 0 {
            return Err(CreateKeyError::InvalidNumberError);
        }

        if !['A', 'B'].contains(letter) {
            return Err(CreateKeyError::InvalidLetterError);
        }

        Ok(Self { num, letter })
    }
}

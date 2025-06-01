use crate::errors::CreateKeyError;

/// A struct representing melodic key of the track
#[derive(Copy, Clone, Debug)]
pub struct Key {
    /// num (1..=12) of the key
    num: i8,
    /// letter (A or B) of the key
    letter: char,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_str: String = (*self).into();
        write!(f, "{}", key_str)
    }
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        format!("{}{}", value.num, value.letter)
    }
}

impl Key {
    pub fn new(num: i8, letter: char) -> Result<Self, CreateKeyError> {
        if num > 12 || num <= 0 {
            return Err(CreateKeyError::InvalidNumberError);
        }

        if !['A', 'B'].contains(&letter) {
            return Err(CreateKeyError::InvalidLetterError);
        }

        Ok(Self { num, letter })
    }
    pub fn new_force(num: i8, letter: char) -> Self {
        Key::new(num, letter).unwrap()
    }
}

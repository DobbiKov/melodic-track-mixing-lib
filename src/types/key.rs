use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyLetter {
    A,
    B,
}

impl KeyLetter {
    pub fn from_char(value: char) -> Option<Self> {
        match value {
            'A' | 'a' => Some(Self::A),
            'B' | 'b' => Some(Self::B),
            _ => None,
        }
    }
}

impl fmt::Display for KeyLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyLetter::A => write!(f, "A"),
            KeyLetter::B => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    number: u8,
    letter: KeyLetter,
}

impl Key {
    pub fn new(number: u8, letter: KeyLetter) -> Result<Self, KeyError> {
        if (1..=12).contains(&number) {
            Ok(Self { number, letter })
        } else {
            Err(KeyError::InvalidNumber(number))
        }
    }

    pub fn from_camelot(value: &str) -> Result<Self, KeyError> {
        let trimmed = value.trim();
        if trimmed.len() < 2 || trimmed.len() > 3 {
            return Err(KeyError::InvalidFormat(value.to_string()));
        }

        let (num_part, letter_part) = trimmed.split_at(trimmed.len() - 1);
        let number: u8 = num_part
            .parse()
            .map_err(|_| KeyError::InvalidFormat(value.to_string()))?;
        let letter = letter_part
            .chars()
            .next()
            .and_then(KeyLetter::from_char)
            .ok_or_else(|| KeyError::InvalidLetter(letter_part.to_string()))?;

        Key::new(number, letter)
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn letter(&self) -> KeyLetter {
        self.letter
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number, self.letter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    InvalidNumber(u8),
    InvalidLetter(String),
    InvalidFormat(String),
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyError::InvalidNumber(value) => write!(f, "invalid key number {value}"),
            KeyError::InvalidLetter(value) => write!(f, "invalid key letter {value}"),
            KeyError::InvalidFormat(value) => write!(f, "invalid key format {value}"),
        }
    }
}

impl std::error::Error for KeyError {}

impl TryFrom<stratum_dsp::Key> for Key {
    type Error = KeyError;

    fn try_from(value: stratum_dsp::Key) -> Result<Self, Self::Error> {
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
        Key::new(number, letter)
    }
}

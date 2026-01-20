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

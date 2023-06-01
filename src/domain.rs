use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub mod subscriber {
    use super::name::SubscriberName;

    #[derive(Debug, Clone)]
    pub struct NewSubscriber {
        pub email: String,
        pub name: SubscriberName,
    }
}

pub mod name {
    use unicode_segmentation::UnicodeSegmentation;

    use super::ParseError;

    pub const MAX_NAME_LENGTH: usize = 256;
    pub const FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    #[derive(Debug, Clone)]
    pub struct SubscriberName(String);

    impl SubscriberName {
        pub fn parse(name: String) -> Result<SubscriberName, ParseError<String>> {
            if name.trim().is_empty() {
                return Err(ParseError::Empty(name));
            }

            if name.graphemes(true).count() > MAX_NAME_LENGTH {
                return Err(ParseError::TooLong(name));
            }

            if name.chars().any(|n| FORBIDDEN_CHARACTERS.contains(&n)) {
                return Err(ParseError::InvalidSequence(name));
            }

            return Ok(Self(name));
        }

        pub fn inner(self) -> String {
            return self.0;
        }

        pub fn inner_mut(&mut self) -> &mut String {
            return &mut self.0;
        }

        pub fn inner_ref(&self) -> &String {
            return &self.0;
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<T: Display + Debug + Clone> {
    InvalidSequence(T),
    TooLong(T),
    Empty(T),
}

impl<T: Display + Debug + Clone> Display for ParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSequence(value) => write!(f, "value {} contains invalid sequence", value),
            Self::TooLong(value) => write!(f, "value {} is too long", value),
            Self::Empty(value) => write!(f, "value {} is empty", value),
        }
    }
}

impl<T: Display + Debug + Clone> Error for ParseError<T> {}

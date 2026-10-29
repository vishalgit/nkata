use thiserror::Error;

pub const MAX_TITLE_CHARS: usize = 120;

/// A todo title. The field is private: `parse` is the only way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoTitle(String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TitleError {
    #[error("title must not be empty")]
    Empty,
    #[error("title must be at most {max} characters (got {actual})")]
    TooLong { max: usize, actual: usize },
}

impl TodoTitle {
    /// Smart constructor.
    pub fn parse(raw: &str) -> Result<Self, TitleError> {
        let trimmed = raw.trim();
        match trimmed.chars().count() {
            0 => Err(TitleError::Empty),
            n if n > MAX_TITLE_CHARS => Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: n,
            }),
            _ => Ok(Self(trimmed.to_owned())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_normal_title() {
        let title = TodoTitle::parse("Buy milk").unwrap();
        assert_eq!(title.as_str(), "Buy milk");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        assert_eq!(
            TodoTitle::parse("  Buy milk \n").unwrap().as_str(),
            "Buy milk"
        );
    }

    #[test]
    fn rejects_an_empty_title() {
        assert_eq!(TodoTitle::parse(""), Err(TitleError::Empty));
    }

    #[test]
    fn rejects_a_whitespace_only_title() {
        assert_eq!(TodoTitle::parse("   \t"), Err(TitleError::Empty));
    }

    #[test]
    fn rejects_one_character_over_the_maximum() {
        let raw = "a".repeat(MAX_TITLE_CHARS + 1);
        assert_eq!(
            TodoTitle::parse(&raw),
            Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: MAX_TITLE_CHARS + 1
            })
        );
    }

    #[test]
    fn accepts_exactly_the_maximum_length() {
        assert!(TodoTitle::parse(&"a".repeat(MAX_TITLE_CHARS)).is_ok());
    }

    #[test]
    fn counts_characters_not_bytes() {
        // 'é' is 2 bytes in UTF-8: 120 of them are 240 bytes but 120 characters
        assert!(TodoTitle::parse(&"é".repeat(MAX_TITLE_CHARS)).is_ok());
    }

    #[test]
    fn errors_explain_themselves() {
        assert_eq!(TitleError::Empty.to_string(), "title must not be empty");
        assert_eq!(
            TitleError::TooLong {
                max: 120,
                actual: 121
            }
            .to_string(),
            "title must be at most 120 characters (got 121)"
        );
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parse_never_panics(raw in ".*") {
            let _ = TodoTitle::parse(&raw);
        }

        #[test]
        fn every_parsed_title_obeys_the_invariants(raw in ".*") {
            if let Ok(title) = TodoTitle::parse(&raw) {
                let n = title.as_str().chars().count();
                prop_assert!((1..=MAX_TITLE_CHARS).contains(&n));
                prop_assert_eq!(title.as_str().trim(), title.as_str());
            }
        }
    }
}

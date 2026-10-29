pub const MAX_TITLE_CHARS: usize = 120;

/// A todo title. The field is private: `parse` is the only way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoTitle(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl TodoTitle {
    /// Smart constructor.
    pub fn parse(raw: &str) -> Result<Self, TitleError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(TitleError::Empty);
        }
        if trimmed.len() > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: trimmed.len(),
            });
        }
        Ok(Self(trimmed.to_owned()))
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
}

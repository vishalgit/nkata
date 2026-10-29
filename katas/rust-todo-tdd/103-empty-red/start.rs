/// A todo title. The field is private: `parse` is the only way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoTitle(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleError {
    Empty,
}

impl TodoTitle {
    /// Smart constructor.
    pub fn parse(raw: &str) -> Result<Self, TitleError> {
        Ok(Self(raw.trim().to_owned()))
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
}

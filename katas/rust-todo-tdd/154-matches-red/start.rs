use super::Todo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    /// Unknown or missing values fall back to All: a bad URL never breaks the page.
    pub fn parse(raw: Option<&str>) -> Self {
        match raw {
            Some("active") => Filter::Active,
            Some("completed") => Filter::Completed,
            _ => Filter::All,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Filter::All => "all",
            Filter::Active => "active",
            Filter::Completed => "completed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_values_and_falls_back_to_all() {
        assert_eq!(Filter::parse(Some("active")), Filter::Active);
        assert_eq!(Filter::parse(Some("completed")), Filter::Completed);
        assert_eq!(Filter::parse(Some("bogus")), Filter::All);
        assert_eq!(Filter::parse(None), Filter::All);
    }
}

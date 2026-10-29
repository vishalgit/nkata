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

    pub fn matches(self, todo: &Todo) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !todo.is_completed(),
            Filter::Completed => todo.is_completed(),
        }
    }

    pub fn completed(self) -> Option<bool> {
        match self {
            Filter::All => None,
            Filter::Active => Some(false),
            Filter::Completed => Some(true),
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

    use crate::todos::workflows::fixtures::{active, completed};

    #[test]
    fn matches_by_state() {
        let todos = [active("a"), completed("c")];
        let count = |f: Filter| todos.iter().filter(|t| f.matches(t)).count();
        assert_eq!(
            (
                count(Filter::All),
                count(Filter::Active),
                count(Filter::Completed)
            ),
            (2, 1, 1)
        );
    }

    #[test]
    fn a_filter_says_which_completion_state_it_wants() {
        assert_eq!(Filter::All.completed(), None);
        assert_eq!(Filter::Active.completed(), Some(false));
        assert_eq!(Filter::Completed.completed(), Some(true));
    }
}

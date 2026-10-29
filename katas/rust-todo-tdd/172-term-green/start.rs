use thiserror::Error;

pub const PER_PAGE: u32 = 10;
pub const MAX_SEARCH_CHARS: usize = 100;

/// A 1-based page number. The field is private: page 0 cannot exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    number: u32,
}

impl PageRequest {
    pub fn parse(raw: Option<u32>) -> Self {
        Self {
            number: raw.unwrap_or(1).max(1),
        }
    }

    pub fn number(self) -> u32 {
        self.number
    }

    pub fn limit(self) -> u64 {
        u64::from(PER_PAGE)
    }

    pub fn offset(self) -> u64 {
        u64::from(self.number - 1) * self.limit()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub page: PageRequest,
    pub total: u64,
}

impl<T> Paged<T> {
    pub fn total_pages(&self) -> u32 {
        self.total.div_ceil(u64::from(PER_PAGE)).max(1) as u32
    }

    pub fn has_previous(&self) -> bool {
        self.page.number() > 1
    }

    pub fn has_next(&self) -> bool {
        self.page.number() < self.total_pages()
    }
}

/// What the user typed in the search box: trimmed, 1..=100 characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchTerm(String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SearchError {}

impl SearchTerm {
    pub fn parse(raw: &str) -> Result<Option<Self>, SearchError> {
        let _ = raw;
        Ok(None)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_or_zero_page_is_the_first_page() {
        assert_eq!(PageRequest::parse(None).number(), 1);
        assert_eq!(PageRequest::parse(Some(0)).number(), 1);
        assert_eq!(PageRequest::parse(Some(3)).number(), 3);
    }

    #[test]
    fn page_three_skips_twenty_and_takes_ten() {
        let page = PageRequest::parse(Some(3));
        assert_eq!((page.offset(), page.limit()), (20, 10));
    }

    #[test]
    fn a_page_knows_its_neighbours() {
        let last = Paged {
            items: vec![1, 2, 3, 4, 5],
            page: PageRequest::parse(Some(3)),
            total: 25,
        };
        assert_eq!(last.total_pages(), 3);
        assert!(last.has_previous());
        assert!(!last.has_next());

        let empty: Paged<u8> = Paged {
            items: vec![],
            page: PageRequest::parse(None),
            total: 0,
        };
        assert_eq!(
            empty.total_pages(),
            1,
            "an empty list is still one (empty) page"
        );
        assert!(!empty.has_next());
    }

    #[test]
    fn blank_input_means_no_search() {
        assert_eq!(SearchTerm::parse("   "), Ok(None));
    }

    #[test]
    fn a_term_is_trimmed() {
        assert_eq!(
            SearchTerm::parse("  milk ").unwrap().unwrap().as_str(),
            "milk"
        );
    }
}

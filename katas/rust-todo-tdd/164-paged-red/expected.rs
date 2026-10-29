pub const PER_PAGE: u32 = 10;

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
}

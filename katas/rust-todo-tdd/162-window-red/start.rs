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
}

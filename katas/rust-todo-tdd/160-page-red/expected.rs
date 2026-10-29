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

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

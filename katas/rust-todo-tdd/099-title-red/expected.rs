#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_normal_title() {
        let title = TodoTitle::parse("Buy milk").unwrap();
        assert_eq!(title.as_str(), "Buy milk");
    }
}

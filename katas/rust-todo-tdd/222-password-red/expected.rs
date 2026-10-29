#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_twelve_or_more_characters() {
        assert_eq!(
            Password::parse("correct horse").unwrap().expose_secret(),
            "correct horse"
        );
    }
}

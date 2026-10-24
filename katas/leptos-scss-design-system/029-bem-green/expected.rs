//! The contract between Rust and SCSS: BEM class names.
//!
//! SCSS writes `.btn` and `&--primary`; Rust must produce `btn btn--primary`.
//! Every component builds its class list through this one function.

/// `bem("btn", &["primary", "lg"])` gives `"btn btn--primary btn--lg"`.
/// Empty modifiers are skipped, so callers can pass `if x { "block" } else { "" }`.
pub fn bem(block: &str, modifiers: &[&str]) -> String {
    modifiers
        .iter()
        .filter(|m| !m.is_empty())
        .fold(block.to_string(), |mut class, m| {
            class.push(' ');
            class.push_str(block);
            class.push_str("--");
            class.push_str(m);
            class
        })
}

#[cfg(test)]
mod tests {
    use super::bem;

    #[test]
    fn block_alone() {
        assert_eq!(bem("card", &[]), "card");
    }

    #[test]
    fn each_modifier_is_prefixed_with_the_block() {
        assert_eq!(bem("btn", &["primary", "lg"]), "btn btn--primary btn--lg");
    }

    #[test]
    fn empty_modifiers_are_skipped() {
        assert_eq!(bem("btn", &["", "sm", ""]), "btn btn--sm");
    }
}

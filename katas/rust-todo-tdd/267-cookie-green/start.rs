#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_session_cookie_is_locked_down() {
        let c = session_cookie("abc", true);
        assert!(c.starts_with("todo_session=abc;"));
        for attr in [
            "HttpOnly",
            "SameSite=Lax",
            "Path=/",
            "Max-Age=604800",
            "Secure",
        ] {
            assert!(c.contains(attr), "missing {attr} in {c}");
        }
        assert!(!session_cookie("abc", false).contains("Secure"));
    }
}

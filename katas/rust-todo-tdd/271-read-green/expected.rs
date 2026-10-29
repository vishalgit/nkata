pub const SESSION_COOKIE: &str = "todo_session";
pub const SESSION_TTL_DAYS: i64 = 7;

pub fn session_cookie(token: &str, secure: bool) -> String {
    let max_age = SESSION_TTL_DAYS * 24 * 60 * 60;
    let secure = if secure { "; Secure" } else { "" };
    format!("{SESSION_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}")
}

pub fn clear_session_cookie() -> String {
    format!("{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
}

/// Finds our token in a `Cookie:` header such as `a=1; todo_session=abc; b=2`.
pub fn read_session_token(cookie_header: &str) -> Option<&str> {
    cookie_header
        .split(';')
        .map(str::trim)
        .find_map(|pair| pair.strip_prefix(SESSION_COOKIE)?.strip_prefix('='))
}

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

    #[test]
    fn clearing_expires_the_cookie_immediately() {
        assert!(clear_session_cookie().contains("Max-Age=0"));
    }

    #[test]
    fn the_token_is_found_among_other_cookies() {
        assert_eq!(
            read_session_token("a=1; todo_session=abc; b=2"),
            Some("abc")
        );
    }
}

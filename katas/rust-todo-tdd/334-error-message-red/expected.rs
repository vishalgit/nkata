#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::ui::testing::render;

    #[test]
    fn shows_the_error_text_in_an_alert() {
        let html = render(
            || view! { <ErrorMessage error=Signal::stored(Some(AppError::Unauthenticated))/> },
        );
        assert!(html.contains(r#"class="alert alert--error""#), "{html}");
        assert!(html.contains("please log in"));
    }
}

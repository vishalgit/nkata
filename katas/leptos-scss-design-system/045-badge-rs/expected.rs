use leptos::prelude::*;

use super::bem;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Brand,
    Success,
    Warning,
    Danger,
}

impl BadgeTone {
    pub const ALL: [Self; 5] = [
        Self::Neutral,
        Self::Brand,
        Self::Success,
        Self::Warning,
        Self::Danger,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Brand => "brand",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional)] tone: BadgeTone,
    #[prop(optional)] dot: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=bem("badge", &[tone.as_str()])>
            {dot.then(|| view! { <span class="badge__dot" aria-hidden="true"></span> })}
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_becomes_a_modifier() {
        assert_eq!(
            bem("badge", &[BadgeTone::Success.as_str()]),
            "badge badge--success"
        );
    }

    #[test]
    fn every_tone_has_a_scss_row() {
        let scss = include_str!("../../style/components/_badge.scss");
        for tone in BadgeTone::ALL.map(BadgeTone::as_str) {
            assert!(
                scss.contains(&format!("'{tone}':")),
                "_badge.scss has no '{tone}': row"
            );
        }
    }
}

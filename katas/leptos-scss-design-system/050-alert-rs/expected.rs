use leptos::prelude::*;

use super::bem;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertTone {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertTone {
    pub const ALL: [Self; 4] = [Self::Info, Self::Success, Self::Warning, Self::Danger];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    /// Screen readers read `alert` out at once and `status` when idle.
    pub fn role(self) -> &'static str {
        match self {
            Self::Warning | Self::Danger => "alert",
            Self::Info | Self::Success => "status",
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] tone: AlertTone,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] dismissible: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(true);

    view! {
        <div class=bem("alert", &[tone.as_str()]) role=tone.role() hidden=move || !open.get()>
            {title.map(|title| view! { <p class="alert__title">{title}</p> })}
            <p class="alert__body">{children()}</p>
            {dismissible
                .then(|| {
                    view! {
                        <button
                            type="button"
                            class="alert__close"
                            aria-label="Dismiss"
                            on:click=move |_| open.set(false)
                        >
                            "×"
                        </button>
                    }
                })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urgent_tones_interrupt_the_screen_reader() {
        assert_eq!(AlertTone::Danger.role(), "alert");
        assert_eq!(AlertTone::Info.role(), "status");
    }

    #[test]
    fn every_tone_has_a_scss_row() {
        let scss = include_str!("../../style/components/_alert.scss");
        for tone in AlertTone::ALL.map(AlertTone::as_str) {
            assert!(
                scss.contains(&format!("'{tone}':")),
                "_alert.scss has no '{tone}': row"
            );
        }
    }
}

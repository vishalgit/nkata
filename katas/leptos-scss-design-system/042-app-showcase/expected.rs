use leptos::prelude::*;

use crate::components::{Button, ButtonSize, ButtonVariant};
use crate::pages::Showcase;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    /// Value for the `data-theme` attribute that themes/_dark.scss matches.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::default());

    view! {
        <div class="app" data-theme=move || theme.get().as_str()>
            <header class="site-header">
                <div class="container site-header__inner">
                    <span class="site-header__brand">"Design Kit"</span>
                    <Button
                        variant=ButtonVariant::Secondary
                        size=ButtonSize::Sm
                        on_click=move |_| theme.update(|t| *t = t.toggled())
                    >
                        {move || match theme.get() {
                            Theme::Light => "Dark theme",
                            Theme::Dark => "Light theme",
                        }}
                    </Button>
                </div>
            </header>
            <main class="container">
                <Showcase />
            </main>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn toggling_twice_returns_to_the_start() {
        assert_eq!(Theme::Light.toggled().toggled(), Theme::Light);
    }

    #[test]
    fn dark_matches_the_scss_selector() {
        assert_eq!(Theme::Dark.as_str(), "dark");
    }
}

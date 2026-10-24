use leptos::prelude::*;

use super::bem;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Outlined,
    Elevated,
}

impl CardVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Elevated => "elevated",
        }
    }
}

/// Optional footer, passed as a slot: `<CardFooter slot>…</CardFooter>`.
#[slot]
pub struct CardFooter {
    children: Children,
}

pub fn card_class(variant: CardVariant, interactive: bool) -> String {
    let interactive = if interactive { "interactive" } else { "" };
    bem("card", &[variant.as_str(), interactive])
}

#[component]
pub fn Card(
    #[prop(optional)] variant: CardVariant,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] interactive: bool,
    #[prop(optional)] card_footer: Option<CardFooter>,
    children: Children,
) -> impl IntoView {
    view! {
        <article class=card_class(variant, interactive) tabindex=interactive.then_some("0")>
            {title
                .map(|title| {
                    view! {
                        <header class="card__header">
                            <h3 class="card__title">{title}</h3>
                        </header>
                    }
                })}
            <div class="card__body">{children()}</div>
            {card_footer
                .map(|footer| view! { <footer class="card__footer">{(footer.children)()}</footer> })}
        </article>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interactive_adds_a_modifier() {
        assert_eq!(
            card_class(CardVariant::Elevated, true),
            "card card--elevated card--interactive"
        );
    }

    #[test]
    fn every_variant_has_a_scss_block() {
        let scss = include_str!("../../style/components/_card.scss");
        for variant in [CardVariant::Outlined, CardVariant::Elevated] {
            let selector = format!("&--{}", variant.as_str());
            assert!(scss.contains(&selector), "_card.scss has no {selector}");
        }
    }
}

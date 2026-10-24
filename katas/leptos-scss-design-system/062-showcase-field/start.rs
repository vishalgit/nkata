use leptos::prelude::*;

use crate::components::{
    Alert, AlertTone, Badge, BadgeTone, Button, ButtonSize, ButtonVariant, Card, CardFooter,
    CardVariant,
};

/// Color tokens shown as swatches. A test keeps this list honest
/// against `$colors` in style/abstracts/_tokens.scss.
const COLOR_TOKENS: [&str; 18] = [
    "brand",
    "brand-strong",
    "brand-soft",
    "success",
    "success-strong",
    "success-soft",
    "warning",
    "warning-strong",
    "warning-soft",
    "danger",
    "danger-strong",
    "danger-soft",
    "surface",
    "surface-raised",
    "border",
    "text",
    "text-muted",
    "on-brand",
];

#[component]
pub fn Showcase() -> impl IntoView {
    view! {
        <div class="showcase">
            <h1>"Components"</h1>
            <ColorSection />
            <ButtonSection />
            <BadgeSection />
            <AlertSection />
            <CardSection />
        </div>
    }
}

#[component]
fn ShowcaseSection(
    #[prop(into)] title: String,
    #[prop(into)] note: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="showcase__section">
            <header>
                <h2>{title}</h2>
                <p class="showcase__note">{note}</p>
            </header>
            {children()}
        </section>
    }
}

#[component]
fn ColorSection() -> impl IntoView {
    view! {
        <ShowcaseSection
            title="Color tokens"
            note="Each chip reads var(--color-*). Toggle the theme and watch them change."
        >
            <div class="showcase__swatches">
                {COLOR_TOKENS
                    .into_iter()
                    .map(|name| {
                        view! {
                            <figure class="showcase__swatch">
                                <span
                                    class="showcase__chip"
                                    style=format!("background: var(--color-{name})")
                                ></span>
                                <figcaption>
                                    <code>{name}</code>
                                </figcaption>
                            </figure>
                        }
                    })
                    .collect_view()}
            </div>
        </ShowcaseSection>
    }
}

#[component]
fn ButtonSection() -> impl IntoView {
    let loading = RwSignal::new(false);

    view! {
        <ShowcaseSection
            title="Button"
            note="Variant x size. Every cell is one component with different props."
        >
            <div class="showcase__scroll">
                <table class="showcase__matrix">
                    <thead>
                        <tr>
                            <th></th>
                            {ButtonSize::ALL
                                .into_iter()
                                .map(|size| view! { <th>{size.as_str()}</th> })
                                .collect_view()}
                        </tr>
                    </thead>
                    <tbody>
                        {ButtonVariant::ALL
                            .into_iter()
                            .map(|variant| {
                                view! {
                                    <tr>
                                        <th scope="row">{variant.as_str()}</th>
                                        {ButtonSize::ALL
                                            .into_iter()
                                            .map(|size| {
                                                view! {
                                                    <td>
                                                        <Button variant=variant size=size>
                                                            "Button"
                                                        </Button>
                                                    </td>
                                                }
                                            })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
            <div class="cluster cluster--3">
                <Button disabled=true>"Disabled"</Button>
                <Button
                    variant=ButtonVariant::Secondary
                    loading=loading
                    on_click=move |_| loading.set(true)
                >
                    "Click to load"
                </Button>
                <Button variant=ButtonVariant::Ghost on_click=move |_| loading.set(false)>
                    "Reset"
                </Button>
            </div>
            <Button block=true>"Block button"</Button>
        </ShowcaseSection>
    }
}

#[component]
fn BadgeSection() -> impl IntoView {
    view! {
        <ShowcaseSection title="Badge" note="Each tone pairs a -soft background with -strong text.">
            <div class="cluster">
                {BadgeTone::ALL
                    .into_iter()
                    .map(|tone| view! { <Badge tone=tone>{tone.as_str()}</Badge> })
                    .collect_view()}
            </div>
            <div class="cluster">
                {BadgeTone::ALL
                    .into_iter()
                    .map(|tone| view! { <Badge tone=tone dot=true>{tone.as_str()}</Badge> })
                    .collect_view()}
            </div>
        </ShowcaseSection>
    }
}

#[component]
fn AlertSection() -> impl IntoView {
    view! {
        <ShowcaseSection title="Alert" note="Warning and danger use role=alert; the rest use role=status.">
            <div class="stack stack--3">
                {AlertTone::ALL
                    .into_iter()
                    .map(|tone| {
                        view! {
                            <Alert tone=tone title=format!("{} alert", tone.as_str())>
                                "The body says what happened and what to do next."
                            </Alert>
                        }
                    })
                    .collect_view()}
                <Alert dismissible=true>"No title, and dismissible. Click x to hide me."</Alert>
            </div>
        </ShowcaseSection>
    }
}

#[component]
fn CardSection() -> impl IntoView {
    view! {
        <ShowcaseSection title="Card" note="Outlined uses a border token; elevated uses shadow tokens.">
            <div class="showcase__cards">
                <Card title="Outlined">"The default card: a border, no shadow."</Card>
                <Card variant=CardVariant::Elevated title="Elevated">
                    "A shadow instead of a border."
                </Card>
                <Card variant=CardVariant::Elevated title="Interactive" interactive=true>
                    "Hover me, or reach me with Tab."
                    <CardFooter slot>
                        <Button size=ButtonSize::Sm>"Action"</Button>
                    </CardFooter>
                </Card>
            </div>
        </ShowcaseSection>
    }
}

#[cfg(test)]
mod tests {
    use super::COLOR_TOKENS;

    #[test]
    fn every_swatch_is_a_real_token() {
        let scss = include_str!("../../style/abstracts/_tokens.scss");
        for name in COLOR_TOKENS {
            assert!(
                scss.contains(&format!("'{name}':")),
                "no '{name}' in $colors"
            );
        }
    }
}

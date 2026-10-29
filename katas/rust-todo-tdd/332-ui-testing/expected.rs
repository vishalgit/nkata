use leptos::prelude::*;
use leptos::reactive::owner::Owner;

pub fn render<V: IntoView>(view: impl FnOnce() -> V) -> String {
    Owner::new().with(|| view().to_html())
}

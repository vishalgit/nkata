use leptos::prelude::*;

/// The ids the input points at with `aria-describedby`.
pub fn described_by(id: &str, has_hint: bool) -> String {
    if has_hint {
        format!("{id}-hint {id}-error")
    } else {
        format!("{id}-error")
    }
}

#[component]
pub fn TextField(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional, into)] error: Signal<Option<String>>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let invalid = move || error.with(Option::is_some);
    let describedby = described_by(&id, hint.is_some());
    let hint_id = format!("{id}-hint");
    let error_id = format!("{id}-error");

    view! {
        <div class="field">
            <label class="field__label" for=id.clone()>
                {label}
            </label>
            <input
                id=id
                type="text"
                class="input"
                class:input--invalid=invalid
                placeholder=placeholder
                disabled=move || disabled.get()
                aria-invalid=move || invalid().then_some("true")
                aria-describedby=describedby
                prop:value=move || value.get()
                on:input:target=move |ev| value.set(ev.target().value())
            />
            {hint.map(|hint| view! { <p class="field__hint" id=hint_id>{hint}</p> })}
            <p class="field__error" id=error_id>
                {move || error.get()}
            </p>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::described_by;

    #[test]
    fn points_at_hint_and_error() {
        assert_eq!(described_by("email", true), "email-hint email-error");
    }

    #[test]
    fn without_a_hint_points_at_error_only() {
        assert_eq!(described_by("email", false), "email-error");
    }
}

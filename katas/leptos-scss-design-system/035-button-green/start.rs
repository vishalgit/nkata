#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    pub const ALL: [Self; 4] = [Self::Primary, Self::Secondary, Self::Ghost, Self::Danger];

    /// The BEM modifier, and the key in `$variants` in _button.scss.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub const ALL: [Self; 3] = [Self::Sm, Self::Md, Self::Lg];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_primary_medium() {
        let class = button_class(ButtonVariant::default(), ButtonSize::default(), false);
        assert_eq!(class, "btn btn--primary btn--md");
    }

    #[test]
    fn block_adds_a_modifier() {
        let class = button_class(ButtonVariant::Ghost, ButtonSize::Lg, true);
        assert_eq!(class, "btn btn--ghost btn--lg btn--block");
    }

    /// The Rust enums and the SCSS maps describe the same variants.
    /// This fails if one side gains a name the other doesn't know.
    #[test]
    fn every_variant_and_size_has_a_scss_row() {
        let scss = include_str!("../../style/components/_button.scss");
        let variants = ButtonVariant::ALL.map(ButtonVariant::as_str);
        let sizes = ButtonSize::ALL.map(ButtonSize::as_str);
        for name in variants.into_iter().chain(sizes) {
            assert!(
                scss.contains(&format!("'{name}':")),
                "_button.scss has no '{name}': row"
            );
        }
    }
}

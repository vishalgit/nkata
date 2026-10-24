//! Design-system components. One Rust file and one SCSS partial each:
//! src/components/button.rs  <->  style/components/_button.scss

mod bem;
mod button;

use bem::bem;

pub use button::{Button, ButtonSize, ButtonVariant};

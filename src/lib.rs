pub mod event;
pub mod input_mods;
pub mod key_code;
mod utils;

pub mod prelude {
    use super::*;
    pub use evdev::{EventType, InputEvent, KeyCode, KeyEvent};
    pub use input_mods::InterceptInput;
    pub use key_code::KeyCodeModifier;
}

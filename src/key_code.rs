use evdev::InputEvent;
use evdev::{EventType, KeyCode};

fn ie_helper(kc: u16, val: i32) -> InputEvent {
    InputEvent::new(EventType::KEY.0, kc, val)
}

pub trait KeyCodeModifier {
    fn get_key_code(&self) -> &KeyCode;

    fn up(&self) -> InputEvent {
        ie_helper(self.get_key_code().code(), 0)
    }

    fn down(&self) -> InputEvent {
        ie_helper(self.get_key_code().code(), 1)
    }

    fn repeat(&self) -> InputEvent {
        ie_helper(self.get_key_code().code(), 2)
    }
}

impl KeyCodeModifier for KeyCode {
    fn get_key_code(&self) -> &KeyCode {
        self
    }
}

use evdev::{EventType, InputEvent, KeyCode};

pub trait InterceptInput {
    fn get_ev(&self) -> &InputEvent;

    fn is_key(&self) -> bool {
        self.get_ev().event_type() == EventType::KEY
    }

    fn key_code(&self) -> KeyCode {
        KeyCode::new(self.get_ev().code())
    }

    fn debug(&self) {
        if self.is_key() {
            let state = {
                if self.get_ev().value() == 1 {
                    "Down"
                } else if self.get_ev().value() == 2 {
                    "Repeat"
                } else {
                    "Up"
                }
            };
            println!("{:?} {:?}", self.key_code(), state);
        } else {
            println!("{:?}", &self.get_ev().event_type());
        }
    }

    fn get_down_ev(&self) -> InputEvent {
        InputEvent::new(self.get_ev().event_type().0, self.get_ev().code(), 1)
    }
    fn get_up_ev(&self) -> InputEvent {
        InputEvent::new(self.get_ev().event_type().0, self.get_ev().code(), 0)
    }

    fn is_repeat(&self) -> bool {
        self.get_ev().value() == 2
    }

    fn is_down(&self) -> bool {
        self.get_ev().value() == 1
    }

    fn is_up(&self) -> bool {
        self.get_ev().value() == 0
    }
}

impl InterceptInput for InputEvent {
    fn get_ev(&self) -> &InputEvent {
        &self
    }
}

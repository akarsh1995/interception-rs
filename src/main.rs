use evdev::KeyCode;
use interception_rs;
use interception_rs::event::EventManager;
use interception_rs::input_mods::InterceptInput;
use interception_rs::key_code::KeyCodeModifier;

fn main() {
    let mut evm = EventManager::default();
    // evm.debug("/dev/input/by-path/platform-i8042-serio-0-event-kbd");

    loop {
        for event in evm.fetch_events_batch() {
            if event.is_key() && KeyCode::KEY_CAPSLOCK == event.key_code() {
                if event.is_down() {
                    evm.write_event(KeyCode::KEY_ESC.down());
                } else if event.is_repeat() {
                    evm.write_event(KeyCode::KEY_ESC.repeat());
                } else {
                    evm.write_event(KeyCode::KEY_ESC.up());
                }
            } else {
                evm.write_event(event);
            }
        }
    }
}

use interception_rs::event::EventManager;
use interception_rs::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut kbd_path: Option<&String> = None;
    if args.len() > 1 {
        kbd_path = Some(&args[1])
    }

    let mut evm = EventManager::default();

    if let Some(kbd_path) = kbd_path {
        evm.debug(kbd_path);
    }
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

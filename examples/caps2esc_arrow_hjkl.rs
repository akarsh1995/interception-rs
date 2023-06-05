use interception_rs::event::EventManager;
use interception_rs::prelude::*;

pub trait HelperMods: InterceptInput {
    fn is_caps(&self) -> bool {
        self.key_code() == KeyCode::KEY_CAPSLOCK
    }

    fn is_hjkl(&self) -> bool {
        match self.key_code() {
            KeyCode::KEY_H | KeyCode::KEY_J | KeyCode::KEY_K | KeyCode::KEY_L => true,
            _ => false,
        }
    }

    fn get_hjkl_equivalent(&self) -> InputEvent {
        let eq_code = match self.key_code() {
            KeyCode::KEY_H => KeyCode::KEY_LEFT,
            KeyCode::KEY_J => KeyCode::KEY_DOWN,
            KeyCode::KEY_K => KeyCode::KEY_UP,
            KeyCode::KEY_L => KeyCode::KEY_RIGHT,
            a => a,
        };
        if self.is_down() {
            eq_code.down()
        } else if self.is_repeat() {
            eq_code.repeat()
        } else {
            eq_code.up()
        }
    }
}

impl HelperMods for InputEvent {}

trait CapsHjklEventMods {
    fn get_key_code(&self) -> &KeyCode;

    fn up(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 0)
    }

    fn down(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 1)
    }

    fn repeat(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 2)
    }
}

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
        for ev in evm.fetch_events_batch() {
            if !ev.is_key() {
                evm.write_event(ev);
                continue;
            }
            if ev.is_caps() && (ev.is_down() || ev.is_repeat()) {
                if evm.get_last_event().is_repeat() {
                    evm.write_sync_event(20_000);
                    evm.write_event(evm.get_last_event().get_up_ev());
                }
                'caps_down: loop {
                    for cde in evm.fetch_events_batch() {
                        if !cde.is_key() {
                            evm.write_event(cde);
                            continue;
                        };
                        if cde.is_caps() && cde.is_up() {
                            evm.write_event(KeyCode::KEY_ESC.down());
                            evm.write_sync_event(20_000);
                            evm.write_event(KeyCode::KEY_ESC.up());
                            break 'caps_down;
                        } else {
                            evm.write_event(cde.get_hjkl_equivalent());
                            loop {
                                for chjkl in evm.fetch_events_batch() {
                                    if !chjkl.is_key() {
                                        evm.write_event(chjkl);
                                        continue;
                                    };
                                    if chjkl.is_caps() {
                                        if chjkl.is_up() {
                                            if evm.get_last_event().is_repeat() {
                                                evm.write_sync_event(20_000);
                                                evm.write_event(evm.get_last_event().get_up_ev())
                                            }
                                            break 'caps_down;
                                        }
                                    }
                                    evm.write_event(chjkl.get_hjkl_equivalent());
                                }
                            }
                        }
                    }
                }
            } else {
                evm.write_event(ev);
            }
        }
    }
}

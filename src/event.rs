use crate::{
    input_mods::InterceptInput,
    utils::{fetch_events, write_events},
};
use evdev::{EventType, InputEvent};
use libc;
use std::{fs::File, os::fd::AsFd};
use std::{thread::sleep, time::Duration};

pub struct EventFetcher {
    in_fd: File,
    ev_buff: Vec<libc::input_event>,
    last_event: InputEvent,
}

impl EventFetcher {
    pub fn new(in_fd: File) -> Self {
        Self {
            in_fd,
            ev_buff: vec![],
            last_event: InputEvent::new(EventType::KEY.0, 0, 0),
        }
    }
}

impl Default for EventFetcher {
    fn default() -> Self {
        let in_fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/stdin") // 19
            .expect("Cannot open stdin.");
        Self::new(in_fd)
    }
}

impl Iterator for EventFetcher {
    type Item = Vec<InputEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let fetched_events = fetch_events(&mut self.ev_buff, &self.in_fd)
            .expect("Cannot Fetch events from the event buff");
        let evs: Vec<InputEvent> = fetched_events.collect();
        if evs.len() >= 1 {
            self.last_event = evs[evs.len() - 1];
        };
        Some(evs)
    }
}

pub struct EventWriter {
    out_fd: File,
}

impl EventWriter {
    fn new(out_fd: File) -> Self {
        Self { out_fd }
    }
}

impl Default for EventWriter {
    fn default() -> Self {
        Self::new(
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/stdout")
                .expect("Cannot open stdout."),
        )
    }
}

pub struct EventManager {
    fetcher: EventFetcher,
    writer: EventWriter,
    debug: bool,
}

impl EventManager {
    // debug
    pub fn debug(&mut self, kbd_path: &str) {
        self.fetcher.in_fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(kbd_path)
            .expect("Cannot open stdout.");
        self.debug = true;
    }

    pub fn write_event(&self, event: InputEvent) {
        if self.debug {
            event.debug()
        } else {
            write_events(self.writer.out_fd.as_fd(), &[event])
                .expect("Failed writing event to the file descriptor");
        }
    }

    pub fn fetch_events_batch(&mut self) -> Vec<InputEvent> {
        self.fetcher.next().expect("No events found")
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            fetcher: EventFetcher::default(),
            writer: EventWriter::default(),
            debug: false,
        }
    }
}

pub fn get_sync_event(delay: u64) -> InputEvent {
    sleep(Duration::from_micros(delay));
    InputEvent::new(EventType::SYNCHRONIZATION.0, 0, 0)
}

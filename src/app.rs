use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

use rdev::{simulate, Button, EventType, Key};

use device_query::Keycode;

#[derive(PartialEq, Copy, Clone)]
pub enum AutoclickerMode {
    Click,
    KeyPress,
}

pub struct AutoclickerApp {
    pub running: Arc<AtomicBool>,

    pub mode: AutoclickerMode,

    pub key_press_type: Key,
    pub click_type: Button,

    pub s: u32,
    pub s_str: String,
    pub ms: u32,
    pub ms_str: String,

    pub hold: bool,

    pub hold_s: u32,
    pub hold_s_str: String,
    pub hold_ms: u32,
    pub hold_ms_str: String,

    pub hotkey: Keycode,
    pub hotkey_pressed: bool,
}

impl AutoclickerApp {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            mode: AutoclickerMode::Click,

            key_press_type: Key::Space,
            click_type: Button::Left,

            s: 1,
            s_str: String::from("1"),
            ms: 0,
            ms_str: String::from("0"),

            hold: false,

            hold_s: 0,
            hold_s_str: String::from("0"),
            hold_ms: 100,
            hold_ms_str: String::from("100"),

            hotkey: Keycode::F6,
            hotkey_pressed: false,
        }
    }

    pub fn start(&mut self) {
        let running = self.running.clone();

        if running.load(Ordering::Relaxed) {
            return;
        }
        running.store(true, Ordering::Relaxed);

        let ms: u64 = (self.s * 1000 + self.ms) as u64;
        let hold_ms: u64 = (self.hold_s * 1000 + self.hold_ms) as u64;

        let mode = self.mode;

        let click_type = self.click_type;
        let key_press_type = self.key_press_type;

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                if mode == AutoclickerMode::Click {
                    simulate(&EventType::ButtonPress(click_type)).unwrap();
                    thread::sleep(time::Duration::from_millis(hold_ms));
                    simulate(&EventType::ButtonRelease(click_type)).unwrap();
                } else if mode == AutoclickerMode::KeyPress {
                    simulate(&EventType::KeyPress(key_press_type)).unwrap();
                    thread::sleep(time::Duration::from_millis(hold_ms));
                    simulate(&EventType::KeyRelease(key_press_type)).unwrap();
                }

                thread::sleep(time::Duration::from_millis(ms));
            }
        });
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

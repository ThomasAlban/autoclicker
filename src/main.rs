use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let device_state = DeviceState::new();

    println!("Enter the number of milliseconds between each click");
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let millis = input_text.trim().parse::<u64>().unwrap();

    println!("Press escape to end program");

    let running_clone = running.clone();
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        while running_clone.load(Ordering::Relaxed) {
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(millis));
        }
    });

    loop {
        if device_state.get_keys().contains(&Keycode::Escape) {
            running.store(false, Ordering::Relaxed);
            break;
        }
    }
}

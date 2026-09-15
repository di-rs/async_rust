use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Condvar, Mutex, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

#[allow(clippy::unwrap_used)]
fn main() {
    let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
    let shared_data_clone = Arc::clone(&shared_data);
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);

    let background_thread = thread::spawn(move || {
        let (lock, cvar) = &*shared_data_clone;
        while !stop.load(Relaxed) {
            let value = {
                let received_value_lock = lock.lock().unwrap();
                let received_value = cvar.wait(received_value_lock).unwrap();
                *received_value
            };
            println!("Received value: {value}");
        }
    });

    let updater_thread = thread::spawn(move || {
        let (lock, cvar) = &*shared_data;
        let values = [false, true, false, true];

        for update_value in values {
            println!("Updating value to {update_value}...");
            *lock.lock().unwrap() = update_value;
            cvar.notify_one();
            thread::sleep(Duration::from_secs(4));
        }
        stop_clone.store(true, Relaxed);
        println!("STOP has been updated");
        cvar.notify_one();
    });

    background_thread.join().unwrap();
    updater_thread.join().unwrap();
}

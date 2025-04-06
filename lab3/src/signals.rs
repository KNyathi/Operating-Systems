use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use signal_hook::consts::SIGINT;
use signal_hook::flag;

fn main() {
    let term = Arc::new(AtomicBool::new(false));
    flag::register(SIGINT, Arc::clone(&term)).unwrap();

    let handle = thread::spawn(move || {
        while !term.load(Ordering::Relaxed) {
            println!("Working...");
            thread::sleep(Duration::from_secs(1));
        }
        println!("Received SIGINT, shutting down gracefully...");
    });

    handle.join().unwrap();
}

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::thread;
use std::time::Duration;
use rand::random;

fn allocate_memory() {
    let mut vec: Vec<u8> = Vec::new();
    // Allocate memory dynamically
    for _ in 0..100000 {
        vec.push(0);
    }
    // Sleep for random 100-1000 ns
    let rand_delay = random::<u64>() % 900 + 100;
    thread::sleep(Duration::from_nanos(rand_delay));
}

fn free_memory() {
    // Free memory here when interrupted
    println!("Memory freed");
}

fn handle_signals() {
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
    for signal in signals.forever() {
        match signal {
            SIGINT | SIGTERM => {
                // Handle signal to free memory
                free_memory();
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mode = std::env::args().nth(1).expect("Please provide mode as argument");

    if mode == "malloc" {
        loop {
            thread::spawn(|| allocate_memory());
        }
    } else if mode == "free" {
        // Handle signals asynchronously to free memory
        handle_signals();
    }
}

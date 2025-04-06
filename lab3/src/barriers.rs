use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} is waiting at the barrier", i);
            barrier.wait(); // Wait for all threads to reach the barrier
            println!("Thread {} passed the barrier", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

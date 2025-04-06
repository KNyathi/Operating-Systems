use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;

fn main() {
    let data_queue = Arc::new((Mutex::new(VecDeque::new()), Condvar::new())); // Shared queue and condition variable
    let mut handles = vec![];

    // Producer thread
    let data_queue_producer = Arc::clone(&data_queue);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let (lock, cvar) = &*data_queue_producer;
            let mut queue = lock.lock().unwrap();
            queue.push_back(i); // Produce data
            println!("Produced: {}", i);
            cvar.notify_one(); // Notify the consumer
            thread::sleep(std::time::Duration::from_secs(1)); // Simulate production time
        }
    });
    handles.push(producer);

    // Consumer thread
    let data_queue_consumer = Arc::clone(&data_queue);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*data_queue_consumer;
        for _ in 0..10 {
            let mut queue = lock.lock().unwrap();
            while queue.is_empty() {
                queue = cvar.wait(queue).unwrap(); // Wait for data
            }
            let data = queue.pop_front().unwrap(); // Consume data
            println!("Consumed: {}", data);
        }
    });
    handles.push(consumer);

    for handle in handles {
        handle.join().unwrap();
    }
}

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let parking_spaces = 5; // Total parking spaces
    let semaphore = Arc::new(Semaphore::new(parking_spaces)); // Semaphore for parking spaces
    let mut handles = vec![];

    for i in 0..10 {
        let semaphore = Arc::clone(&semaphore);
        let handle = task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap(); // Acquire a parking space
            println!("Car {} is parking", i);
            sleep(Duration::from_secs(1)).await; // Simulate parking time
            println!("Car {} is leaving", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

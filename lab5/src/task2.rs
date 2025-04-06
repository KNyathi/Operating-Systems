// src/task2.rs
use std::env;
use std::fs;
use std::process::Command;
use std::time::SystemTime;
use std::io::{self, Write};
use chrono::Local;

fn create_memory_map(pid: u32, output_dir: &str) -> io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timestamp_str = format!("{}", timestamp);
    
    
    // Read the process memory map from /proc/{pid}/maps
    let maps_path = format!("/proc/{}/maps", pid);
    let content = fs::read_to_string(maps_path)?;

    // Get the current timestamp for unique filenames    
    let timestamp = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let map_filepath = format!("{}/map_{}_{}.txt", output_dir, pid, timestamp);


    // Save the map file
    let mut file = fs::File::create(map_filepath)?;
    file.write_all(content.as_bytes())?;
    println!("Memory map saved");

    Ok(())
}

fn main() {
    let pid: u32 = env::args().nth(1).expect("PID argument missing").parse().unwrap();
    let output_dir = env::args().nth(2).expect("Output directory missing");
    
    match create_memory_map(pid, &output_dir) {
        Ok(_) => println!("Successfully created memory map"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

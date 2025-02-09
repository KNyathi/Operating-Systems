use std::env;
use std::fs;
use std::process::Command;
use sysinfo::{System, SystemExt, CpuExt};
use chrono::Local;


fn main() {
    // 1. Computer name and username
    let computer_name = gethostname::gethostname().into_string().unwrap_or_else(|_| "Unknown".to_string());
    let username = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    println!("Computer Name: {}", computer_name);
    println!("Username: {}", username);

    // 2. Operating system version
    let os_version = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute command");
    println!("OS Version: {}", String::from_utf8_lossy(&os_version.stdout));

    // 3. System metrics
    let mut system = System::new_all();
    system.refresh_all();

    // CPU usage
    let cpu_usage = system.global_cpu_info().cpu_usage();
    println!("CPU Usage: {:.2}%", cpu_usage);

    // Memory usage
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    println!("Memory Usage: {}/{} MB", used_memory / 1024, total_memory / 1024);

    // Disk usage
    let disk_usage = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to execute command");
    println!("Disk Usage:\n{}", String::from_utf8_lossy(&disk_usage.stdout));

    // 4. Functions for working with time
    // Current time
    let now = Local::now();
    println!("Current Time: {}", now.format("%Y-%m-%d %H:%M:%S"));

    // Elapsed time (example: sleep for 2 seconds)
    let start = Local::now();
    std::thread::sleep(std::time::Duration::from_secs(2));
    let end = Local::now();
    let elapsed = end - start;
    println!("Elapsed Time: {} seconds", elapsed.num_seconds());

    // 5. Additional API functions
    // List files in a directory
    println!("\nFiles in current directory:");
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }

    // Check network interfaces
    let network_interfaces = Command::new("ifconfig")
        .output()
        .expect("Failed to execute command");
    println!("\nNetwork Interfaces:\n{}", String::from_utf8_lossy(&network_interfaces.stdout));

    // Get environment variables
    println!("\nEnvironment Variables:");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    // Get process ID
    let pid = std::process::id();
    println!("\nProcess ID: {}", pid);
}

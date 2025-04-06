use std::process::Command;
use std::thread;
use std::time::Duration;

fn get_task1_pid() -> Option<u32> {
    let output = Command::new("pgrep")
        .arg("task1")  // Find the PID of task1
        .output()
        .expect("Failed to execute pgrep");

    if output.status.success() {
        let pid_str = String::from_utf8_lossy(&output.stdout);
        if let Some(pid) = pid_str.lines().next() {
            return pid.parse::<u32>().ok();
        }
    }

    None
}

fn run_service(output_dir: String, max_runs: u32) {
    if let Some(pid) = get_task1_pid() {
        println!("Found Task 1 running with PID: {}", pid);

        for i in 1..=max_runs {
            println!("Run {}/{}: Capturing memory map for Task 1 (PID {})...", i, max_runs, pid);
            
            // Execute Task 2 (Runs `cargo run --release --bin task2 -- <pid> <output_dir>`)
            let output = Command::new("cargo")
                .args(&["run", "--release", "--bin", "task2", "--", &pid.to_string(), &output_dir])
                .output()
                .expect("Failed to execute Task 2");

            if !output.status.success() {
                eprintln!(
                    "Task 2 failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            println!("Task 2 completed. Sleeping for 30 seconds...");

            // Wait for 30 seconds before the next run
            if i < max_runs {
                thread::sleep(Duration::from_secs(30));
            }
        }

        println!("Task 3 has completed {} runs and is stopping.", max_runs);
    } else {
        eprintln!("Error: Task 1 is not running. Start Task 1 first!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: task3 <output_directory> <number_of_runs>");
        std::process::exit(1);
    }

    let output_dir = args[1].clone();
    let max_runs: u32 = args[2].parse().expect("Invalid number of runs");

    println!("Starting Task 3 service to monitor Task 1...");
    run_service(output_dir, max_runs);
}

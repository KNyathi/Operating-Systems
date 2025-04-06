use nix::unistd::{fork, ForkResult};
use nix::sys::wait::waitpid;
use std::process::Command;
use std::thread;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use libc::execvp;
use std::ffi::CString;
use procfs::process::Process;

// Function to get the current time in "hours:minutes:seconds" format
fn get_current_time() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%H:%M:%S").to_string()
}

// Function to display PID and PPID
fn display_info(process_name: &str) {
    println!(
        "{}: PID = {}, PPID = {}, Time = {}",
        process_name,
        std::process::id(),
        unsafe { libc::getppid() },
        get_current_time()
    );
}

// Function to execute the `ps -x` command
fn execute_ps_command() {
    let output = Command::new("ps").arg("-x").output().expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// Function to display I/O statistics
fn display_io_statistics() {
    let pid = std::process::id();
    if let Ok(process) = Process::new(pid as i32) {
        if let Ok(io) = process.io() {
            println!("I/O Statistics for PID {}:", pid);
            println!("  Read bytes: {}", io.read_bytes);
            println!("  Write bytes: {}", io.write_bytes);
            println!("  Read operations: {}", io.rchar);
            println!("  Write operations: {}", io.wchar);
        } else {
            eprintln!("Failed to retrieve I/O statistics for PID {}", pid);
        }
    } else {
        eprintln!("Failed to retrieve process information for PID {}", pid);
    }
}

// Function to replace the second child process with a threaded program
fn replace_with_threaded_program() {
    let program = CString::new("./target/debug/process_thread_example").expect("CString::new failed");
    let arg0 = CString::new("threaded").expect("CString::new failed");
    let args = [arg0.as_ptr(), std::ptr::null()]; // Null-terminated array of arguments
    unsafe {
        execvp(program.as_ptr(), args.as_ptr());
    }
}

// Threaded program logic
fn threaded_program() {
    display_info("Threaded Program: Main Thread");

    let handle1 = thread::spawn(|| {
        display_info("Threaded Program: Child Thread 1");
    });

    let handle2 = thread::spawn(|| {
        display_info("Threaded Program: Child Thread 2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    // Check if the program is running as the threaded program
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "threaded" {
        threaded_program();
        return;
    }

    // Parent process
    display_info("Parent Process");

    // First fork to create the first child process
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            // Parent process continues
            println!("Parent Process: Created first child with PID {}", child);

            // Second fork to create the second child process
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child }) => {
                    println!("Parent Process: Created second child with PID {}", child);

                    // Execute `ps -x` in the parent process
                    execute_ps_command();

                    // Display I/O statistics
                    display_io_statistics();

                    // Wait for both child processes to finish
                    waitpid(child, None).unwrap();
                    waitpid(child, None).unwrap();
                }
                Ok(ForkResult::Child) => {
                    // Second child process
                    display_info("Second Child Process");

                    // Replace the second child process with the threaded program
                    replace_with_threaded_program();
                }
                Err(_) => eprintln!("Fork failed for the second child"),
            }
        }
        Ok(ForkResult::Child) => {
            // First child process
            display_info("First Child Process");

            // Execute `ps -x` using the `system()` function
            let _ = Command::new("ps").arg("-x").status().expect("Failed to execute command");

            // Display I/O statistics
            display_io_statistics();
        }
        Err(_) => eprintln!("Fork failed for the first child"),
    }
}

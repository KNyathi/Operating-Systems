use std::sync::{Arc, Mutex};
use std::thread;
use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs::OpenOptions;
use std::sync::mpsc;
use std::mem;
use memmap2::MmapMut;
use std::fs::File;
use std::os::unix::io::AsRawFd;

const MATRIX_SIZE: usize = 3;

fn multiply_row_by_col(row: &[i32], col: &[i32]) -> i32 {
    row.iter().zip(col.iter()).map(|(r, c)| r * c).sum()
}

fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    let matrix_b = vec![
        vec![9, 8, 7],
        vec![6, 5, 4],
        vec![3, 2, 1]
    ];
    
    let mut matrix_c = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];
    let (tx, rx) = mpsc::channel();
    
    let matrix_b_transposed: Vec<Vec<i32>> = (0..MATRIX_SIZE)
        .map(|i| matrix_b.iter().map(|row| row[i]).collect())
        .collect();
    
    let mut children = vec![];
    
    // Shared memory setup
    let file = File::options().read(true).write(true).create(true).open("/tmp/matrix_shm").unwrap();
    file.set_len((MATRIX_SIZE * MATRIX_SIZE * mem::size_of::<i32>()) as u64).unwrap();
    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    let shared_matrix = Arc::new(Mutex::new(mmap));
    
    // Socket setup
    let socket_path = "/tmp/matrix_socket";
    let _ = std::fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path).unwrap();
    
    for i in 0..MATRIX_SIZE {
        let row = matrix_a[i].clone();
        let col_matrix = matrix_b_transposed.clone();
        let tx = tx.clone();
        let shared_matrix = Arc::clone(&shared_matrix);
        
        let handle = thread::spawn(move || {
            let mut result_row = vec![0; MATRIX_SIZE];
            for j in 0..MATRIX_SIZE {
                result_row[j] = multiply_row_by_col(&row, &col_matrix[j]);
            }
            
            // Write results to shared memory
            let mut mmap = shared_matrix.lock().unwrap();
            let offset = i * MATRIX_SIZE;
            for (j, &val) in result_row.iter().enumerate() {
                mmap[(offset + j) * mem::size_of::<i32>()..(offset + j + 1) * mem::size_of::<i32>()]
                    .copy_from_slice(&val.to_ne_bytes());
            }
            
            tx.send((i, result_row)).expect("Failed to send data");
        });
        
        children.push(handle);
    }
    
    // Accept connections from socket
    thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = socket.read(&mut buffer).unwrap();
        println!("Received from socket: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    });
    
    // Send data to socket
    let mut stream = UnixStream::connect(socket_path).unwrap();
    stream.write_all(b"Matrix multiplication completed").unwrap();
    
    for _ in 0..MATRIX_SIZE {
        let (row_index, row_data) = rx.recv().unwrap();
        matrix_c[row_index] = row_data;
    }
    
    for child in children {
        child.join().expect("Child thread panicked");
    }
    
    println!("Resulting Matrix:");
    for row in &matrix_c {
        println!("{:?}", row);
    }
}

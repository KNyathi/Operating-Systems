use std::net::TcpStream;
use std::io::{Read, Write};
use shared_memory::Shmem;
use std::sync::{Arc, Mutex};

const MATRIX_SIZE: usize = 2; // 2x2 matrices

fn main() {
    // Connect to master process
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // Get row index
    let row_idx: u8 = 0; // Example: Worker is processing row 0
    stream.write_all(&[row_idx]).unwrap();

    // Access shared memory for matrices
    let shmem_a = Arc::new(Mutex::new(Shmem::open("shmem_a").unwrap()));
    let shmem_b = Arc::new(Mutex::new(Shmem::open("shmem_b").unwrap()));
    let shmem_result = Arc::new(Mutex::new(Shmem::open("shmem_result").unwrap()));

    let matrix_a: Vec<i32> = shmem_a.lock().unwrap().as_slice()
        .chunks_exact(4)
        .map(|chunk| i32::from_ne_bytes(chunk.try_into().unwrap()))
        .collect();

    let matrix_b: Vec<i32> = shmem_b.lock().unwrap().as_slice()
        .chunks_exact(4)
        .map(|chunk| i32::from_ne_bytes(chunk.try_into().unwrap()))
        .collect();

    // Perform row-column multiplication
    let mut result_row = vec![0; MATRIX_SIZE];
    for j in 0..MATRIX_SIZE {
        let mut sum = 0;
        for k in 0..MATRIX_SIZE {
            sum += matrix_a[row_idx as usize * MATRIX_SIZE + k] * matrix_b[k * MATRIX_SIZE + j];
        }
        result_row[j] = sum;
    }

    // Send result back to master via TCP
    let mut result_bytes = vec![];
    for &value in &result_row {
        result_bytes.extend(&value.to_ne_bytes());
    }
    stream.write_all(&result_bytes).unwrap();
}

use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

impl From<std::num::ParseIntError> for TribonacciError {
    fn from(_: std::num::ParseIntError) -> Self {
        TribonacciError(String::from("Please enter a valid size"))
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    _error_msg: String,
) -> Result<(), TribonacciError> {
    let val = size?;

    let mut tri_vec: Vec<i128> = vec![1, 1, 1];
    for i in 2..val-1 {
        tri_vec.push(tri_vec[i] + tri_vec[i-1] + tri_vec[i-2]);
    }
    println!("Values: {:?}", tri_vec);
    println!();
    let sum: i128 = tri_vec.iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}
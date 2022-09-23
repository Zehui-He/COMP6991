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

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    // TODO: complete this function!
    Ok(())
}

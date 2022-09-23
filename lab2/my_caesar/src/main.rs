use std::io::{stdin, BufRead};
use std::env;
use std::char;

const DEFAULT_SHIFT: i32 = 5;

fn main() {
    let shift_by: i32 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_SHIFT);
    
    for line in stdin().lock().lines() {
        let shifted = shift(shift_by, line.expect("no input line"));

        println!("Shifted ascii by {shift_by} is: {shifted}");
    }
}


fn shift(/*TODO*/) -> /*TODO*/ {
    
}

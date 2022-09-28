use std::io::{stdin, BufRead};
use std::env;

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

fn convert_nagetive_u32(num: i128) -> u32 {
    let mut res:i128 = num;
    while res < 0 {
        res += 26;
    }
    return  res as u32;
}


fn shift(shift_by:i32, line:String) -> String {
    let mut res = String::new();
    line.chars().for_each(|c| {
        if c.is_ascii_lowercase() {
            let new_ascii_val = convert_nagetive_u32(c as i128 - 'a' as i128 + shift_by as i128) % 26 + 'a' as u32;
            res.push(std::char::from_u32(new_ascii_val).unwrap());
        }
        else if c.is_ascii_uppercase() {
            let new_ascii_val = convert_nagetive_u32(c as i128 - 'A' as i128 + shift_by as i128) % 26 + 'A' as u32;
            res.push(std::char::from_u32(new_ascii_val).unwrap());
        }
        else {
            res.push(c);
        }
    });
    return res;
}

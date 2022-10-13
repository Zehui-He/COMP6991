//! # doctor_who crate
//! This module provide a function caesar_shift.


/// # DEFAULT_SHIFT: If no input argument is given, the default shift is 5.
pub const DEFAULT_SHIFT: i32 = 5;
/// 
/// UPPERCASE_A: The ascii number of uppercase 'A'.
pub const UPPERCASE_A: i32 = 65;
/// 
/// LOWERCASE_A: The ascii number of lowercase 'a'.
pub const LOWERCASE_A: i32 = 97;
/// 
/// ALPHABET_SIZE: The number of characters from 'a' to 'z' or from 'A' to 'Z'.
pub const ALPHABET_SIZE: i32 = 26;

/// A function which takes the number of shifts and a vector of string and print out the reusltant
/// string such that every character in the string is shifted by the number of shifts. For example:
/// string "abcd" would print "bcde".
/// 
/// # Example
/// ```
/// let mut lines = vec!["abcd".to_string()];
/// doctor_who::caesar_shift(Some(1), lines);
/// // Expect "bcde" would be printed.
/// ```
/// 
pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);
    lines.into_iter().for_each(|line| {
        println!(
            "Shifted ascii by {shift_number} is: {}",
            shift(shift_number, line)
        );
    });
}

/// This function take the number of shifts and the original string as input and return
/// a new string that ever character is shifted by the number of shifts.
/// 
/// # Example
/// 
/// ```
/// use doctor_who::shift;
/// let mut string = String::from("abcd");
/// string = shift(1, string);
/// assert_eq!(string, String::from("bcde"));
///```
pub fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

/// This functon would take two intergers a and b and return the absolute value of a%b.
/// # Example
/// ```
/// let a = -3;
/// let b = 2;
/// assert_eq!(doctor_who::abs_modulo(a, b), 1);
/// ```
pub fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

/// This function receives a i32 integer and return its ascii character.
/// # Example
/// ```
/// let a = doctor_who::to_ascii(97);
/// assert_eq!(a, 'a');
/// ```
pub fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}

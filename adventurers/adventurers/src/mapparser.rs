//! This module defines a single function which reads the data file
//! and convert it into a Hashmap<(i32, i32), Blocks> object. 
use std::{collections::HashMap, fs::File};
use ron::{de::from_reader, error::SpannedError};
use crate::blocks::Blocks;

/// This function accepet a string which represents the file path and return a map
/// object. If the file doesn't exist or the input file cannot be resolved, it would
/// panic.
/// 
/// # Example
/// 
/// ```
/// use adventurers::mapparser::read_map;
/// let map = read_map(String::from("/../maps/testing_game.ron"));
/// ```
pub fn read_map(file_path: String) -> Result<HashMap<(i32, i32), Blocks>, SpannedError> {
    let input_path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), file_path);
    let f = File::open(&input_path).expect("Failed opening file");
    from_reader(f)
}

#[test]
fn tesread_map() {
    let map = read_map(String::from("/../maps/testing_game.ron"));
    println!("Config: {:?}", &map);
}
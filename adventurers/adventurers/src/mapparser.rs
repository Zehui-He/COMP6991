//! This module defines a single function which reads the data file
//! and convert it into a Hashmap<(i32, i32), Blocks> object. 
use std::{collections::HashMap, fs::File};
use ron::{de::from_reader, error::SpannedError};
use crate::blocks::Blocks;

/// This function consumes a String object which represents the file path and return
/// a Result object. If the file is resolved correctly, it would return OK(HashMap<i32, i32>, Blocks) object.
/// Otherwise, Err(SpannedError) would be returned.
/// 
/// # Example
/// 
/// ```
/// use adventurers::mapparser::read_map;
/// use adventurers::blocks::Blocks;
/// use std::collections::HashMap;
/// 
/// let mut correct_map = HashMap::new();
/// correct_map.insert((0,0), Blocks::Barrier);
/// correct_map.insert((0,1), Blocks::Flowerbush);
/// correct_map.insert((0,2), Blocks::Water);
/// correct_map.insert((0,3), Blocks::Grass);
/// let test_file = String::from("/../maps/test.ron");
/// let map = read_map(test_file).ok().unwrap();
/// assert_eq!(map, correct_map);
/// ```
pub fn read_map(file_path: String) -> Result<HashMap<(i32, i32), Blocks>, SpannedError> {
    let input_path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), file_path);
    let f = match File::open(&input_path) {
        Ok(f) => f,
        Err(_) => return Err(SpannedError { code: ron::Error::ExpectedOption, position: ron::error::Position{ line: 0, col: 0 } }),
    };
    from_reader(f)
}

#[test]
fn test_exist_map() {
    let mut correct_map = HashMap::new();
    correct_map.insert((0,0), Blocks::Barrier);
    correct_map.insert((0,1), Blocks::Flowerbush);
    correct_map.insert((0,2), Blocks::Water);
    correct_map.insert((0,3), Blocks::Grass);

    let test_file = String::from("/../maps/test.ron");
    let map = read_map(test_file).ok().unwrap();
    assert_eq!(map, correct_map);
}

#[test]
fn test_unsupport_map() {
    let unsupport_file = String::from("/../maps/bad_data.ron");
    let _err = read_map(unsupport_file).unwrap_err();
}

#[test]
fn test_non_exist_map() {
    let non_exist_file = String::from("/../maps/hello.ron");
    let _err = read_map(non_exist_file).unwrap_err();
}
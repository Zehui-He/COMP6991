#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum DiffResult<'a, 'b> {
    LeftOnly(&'a str),
    RightOnly(&'b str),
    Both(&'a str)
}

pub fn compare_rolls<'a, 'b>(left_roll: &'a str, right_roll: &'b str) -> Vec<DiffResult<'a, 'b>> {
    let mut res = Vec::<DiffResult<'a, 'b>>::new();
    
    let left_names:Vec<&str>= left_roll.split('\n').collect();
    let right_names:Vec<&str> = right_roll.split('\n').collect();
    
    for name in left_names.iter() {
        if right_names.contains(&name) {
            res.push(
                DiffResult::Both(name)
            )
        }
        else {
            res.push(DiffResult::LeftOnly(name))
        }
    }

    for name in right_names.iter() {
        if !left_names.contains(&name) {
            res.push(
                DiffResult::RightOnly(name)
            )
        }
    }

    res.sort();
    res
}

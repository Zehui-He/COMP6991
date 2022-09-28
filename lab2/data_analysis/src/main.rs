const ENROLMENTS_PATH: &str = "enrolments.psv";
use std::{collections::{HashMap, HashSet}, io::{stdin, stdout, Write}};
use csv::{ReaderBuilder};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Row<'a> {
    course_code: &'a str,
    student_id: u32,
    name: &'a str,
    program: &'a str,
    plan: &'a str,
    wam: f32,
    session: &'a str,
    bday: &'a str,
    sex: &'a str,
}

fn main() {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)
        .unwrap();

    let mut record_vec = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        record_vec.push(record);
    }

    let mut seen_id = HashSet::new();
    let mut course_student: HashMap<String, HashSet<u32>> = HashMap::new();
    let mut cse_grade: HashMap<u32, f32> = HashMap::new();
    record_vec.iter().for_each(|record| {
        let row = record.deserialize::<Row>(None).unwrap();

        // Count the number of student
        seen_id.insert(row.student_id);

        // Record which student enrolled in which course
        course_student.entry(row.course_code.to_string())
            .and_modify(|student_list| { student_list.insert(row.student_id); })
            .or_insert(HashSet::from([row.student_id]));

        // Record score for each cse stduent
        cse_grade.insert(row.student_id, row.wam);
    });

    // Find max course and min course
    let (max_course_name, max_studnet) = course_student.iter()
        .max_by_key(|(_course, student)| student
        .len()).unwrap();
    let (min_course_name, min_studnet) = course_student.iter()
        .min_by_key(|(_course, student)| student
        .len()).unwrap();

    // Print the number of studnets
    println!("Number of students: {}", seen_id.len());
    println!("Most common course: {} with {} students", max_course_name, max_studnet.len());
    println!("Least common course: {} with {} students", min_course_name, min_studnet.len());
    println!("Average WAM: {:.2}", cse_grade.values().sum::<f32>() / cse_grade.len() as f32);

    let mut input_string = String::new();
    print!("Enter a student id: ");
    // Remove the newline
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string).expect("input error");
    // Print not found and exit if no student id is given
    let target_id = match input_string.trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("== NOT FOUND ==");
            std::process::exit(0);
        }
    };
    record_vec.iter().for_each(|record| {
        let row = record.deserialize::<Row>(None).unwrap();
        if target_id == row.student_id {
            println!("== STUDENT FOUND ==");
            println!("Name: {}", row.name);
            println!("Program: {}", row.program);
            println!("Plan: {}", row.plan);
            println!("WAM: {:.2}", row.wam);
            println!("Session: {}", row.session);
            println!("Birthdate: {}", row.bday);
            println!("Sex: {}", row.sex);
            std::process::exit(0);
        }
    });
    println!("== NOT FOUND ==");
}

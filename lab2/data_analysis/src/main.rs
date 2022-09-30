const ENROLMENTS_PATH: &str = "enrolments.psv";
use std::{collections::{HashMap, HashSet}, io::{stdin, stdout, Write}};
use csv::{ReaderBuilder};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Row {
    course_code: String,
    student_id: u32,
    name: String,
    program: String,
    plan: String,
    wam: f32,
    session: String,
    bday: String,
    sex: String,
}

fn main() {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)
        .unwrap();

    let enrolments = rdr.deserialize::<Row>()
        .filter_map(|result| result.ok())
        .collect::<Vec<Row>>();

    // Find all stduent
    let mut students = HashSet::new();
    for enrolment in enrolments.iter() {
        students.insert(enrolment.student_id);
    }
    println!("Number of students: {}", students.len());

    // Find the most and least comment course
    let mut course_student: HashMap<String, HashSet<u32>> = HashMap::new();
    enrolments.iter().for_each(|enrolment| {
        course_student.entry(enrolment.course_code.clone())
            .and_modify(|set| { set.insert(enrolment.student_id); })
            .or_insert(HashSet::from([enrolment.student_id]));
    });
    let (max_course, max_studnet) = course_student.iter().max_by_key(|(_, x)| {x.len()}).unwrap();
    let (min_course, min_studnet) = course_student.iter().min_by_key(|(_, x)| {x.len()}).unwrap();
    println!("Most common course: {} with {} students", max_course, max_studnet.len());
    println!("Least common course: {} with {} students", min_course, min_studnet.len());

    // Calculate the wam of all students
    let grade:HashMap<u32, f32> = enrolments.iter().map(|enrolment| {
        (enrolment.student_id, enrolment.wam)
    }).collect();
    println!("Average WAM: {:.2}", grade.values().sum::<f32>() / grade.len() as f32);

    // Search for the stduent
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
            return;
        }
    };
    let target =  match find_stduent(target_id, &enrolments) {
        Some(enrolment) => enrolment,
        None => {
            println!("== NOT FOUND ==");
            return;
        }
    };

    println!("== STUDENT FOUND ==");
    println!("Name: {}", target.name);
    println!("Program: {}", target.program);
    println!("Plan: {}", target.plan);
    println!("WAM: {:.2}", target.wam);
    println!("Session: {}", target.session);
    println!("Birthdate: {}", target.bday);
    println!("Sex: {}", target.sex);
}

fn find_stduent(target_id: u32, enrolments: &Vec<Row>) -> Option<&Row> {
    for enrolment in enrolments {
        if target_id == enrolment.student_id {
            return Some(enrolment);
        }
    }
    return None;
}

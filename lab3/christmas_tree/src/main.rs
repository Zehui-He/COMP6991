use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

impl Light {
    fn create_left(&mut self) {
        let new_light = Box::new(Light { left: None, right: None, brightness: 0});
        self.left = Some(new_light);
    }

    fn create_right(&mut self) {
        let new_light = Box::new(Light { left: None, right: None, brightness: 0});
        self.right = Some(new_light);
    }

    fn set_brightness(&mut self, brightness: i32) {
        self.brightness = brightness;
    }
}

#[derive(Debug)]
struct Tree {
    head: Option<Box<Light>>,
    total_brightness: i32,
    total_lights: i32
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn main() {
    let instructions = get_instructions_from_stdin();
    // Create a tree struct with a startring light
    let mut tree = Tree{head: Some(Box::new(Light {left: None, right: None, brightness: 0})), total_brightness: 0, total_lights: 1 };
    let mut current_light = tree.head.as_mut().unwrap();
    
    // Loop over the instructions 
    for instruct in instructions {
        match instruct {
            Instruction::Set(brightness) => {
                if current_light.brightness > 0 {
                    tree.total_brightness -= current_light.brightness;
                    tree.total_brightness += brightness;
                    current_light.set_brightness(brightness);
                }
                current_light.set_brightness(brightness);
                tree.total_brightness += brightness;
            },
            Instruction::Left => {
                current_light.create_left();
                current_light = current_light.left.as_mut().unwrap();
                tree.total_lights += 1;
            },
            Instruction::Right => {
                current_light.create_right();
                current_light = current_light.right.as_mut().unwrap();
                tree.total_lights += 1;
            },
            Instruction::Reset => {
                current_light = tree.head.as_mut().unwrap();
            },
        }
    }
    
    println!("{:?}", tree.total_brightness / tree.total_lights);
    // println!("{:?}", tree);
}

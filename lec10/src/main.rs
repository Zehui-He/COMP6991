#![allow(unused)]
struct Sheep {
    name: String,
    age: u32,
    at_party: bool
}

struct Cow {
    name: String,
    age: u32,
    angry: bool
}

struct Crab {
    age: u32
}

trait Animal {
    fn name(&self) -> String;
    fn age(&self) -> u32;
    fn say_hello_to (&self, animal:&dyn Animal);
    fn speak(&self);
}

impl Animal for Sheep {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn say_hello_to (&self, animal:&dyn Animal) {
        println!("Hello, {}", animal.name());
    }

    fn speak(&self) {
        println!("meeee");
    }
}


impl Animal for Cow {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn say_hello_to (&self, animal:&dyn Animal) {
        println!("Hello, {}", animal.name());
    }

    fn speak(&self) {
        println!("moooo");
    }
}

fn all_animals_greet(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        animal.speak();
    }
}


fn main() {
    let dean = Sheep {
        name: "dean".to_string(),
        age: 21,
        at_party: true
    };

    let alan = Cow {
        name: "Alan".to_string(),
        age: 10,
        angry: true
    };

    let mut animals: Vec<Box<dyn Animal>> = vec![Box::from(dean), Box::from(alan)];

    all_animals_greet(animals);
}

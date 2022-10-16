struct English;
struct Spanish;
struct French;

trait Greeting {
    fn greet(&self);
}

impl Greeting for English {
    fn greet(&self) {
        println!("Hello!");
    }
}

impl Greeting for Spanish {
    fn greet(&self) {
        println!("Hola!");
    }
}

impl Greeting for French {
    fn greet(&self) {
        println!("Bonjour!");
    }
}

struct Person {
    name: String,
    greetings: Vec<Box<dyn Greeting>>,
}

fn main() {
    // john can speak English and Spanish
    let person = Person {
        name: "John".to_string(),
        greetings: vec!["English".into(), "Spanish".into()],
    };

    speak_all_greetings(&person);

    // jane can speak French
    let person = Person {
        name: "Jane".to_string(),
        greetings: vec!["French".into()],
    };

    speak_all_greetings(&person);
}

fn speak_all_greetings(person: &Person) {
    println!("{} says:", person.name);
    //TODO: iterate over the greetings and call greet() on each one
    for greeting in person.greetings.iter() {
        greeting.greet();
    }
}

impl std::convert::From<&str> for Box<dyn Greeting> {
    fn from(language: &str) -> Self {
        if language.contains("French") {
            Box::new(French)
        }
        else if language.contains("English") {
            Box::new(English)
        }
        else {
            Box::new(Spanish)
        }
    }
}

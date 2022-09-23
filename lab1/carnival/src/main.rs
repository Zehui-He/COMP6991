// You should NOT be constructing 
// a new ticket, so you should never use this directly.
#[derive(Debug)]
struct Ticket;

struct Person {
    name: String,
    ticket: Option<Ticket>,
}

fn main() {
    // TODO - make this code compile!
    let entry_pass = Ticket; 
    let shrey_name = String::from("shrey");
    let tom_name = String::from("tom");

    // Only change code past this line!
    let shrey_upper_name = to_uppercase(shrey_name);
    let tom_upper_name = to_uppercase(tom_name);

    // HINT: using `shrey_name` here won't compile!
    let mut shrey = Person {
        name: shrey_upper_name,
        ticket: Some(entry_pass),
    };

    // HINT: using `tom_name` here won't compile!
    let mut tom = Person {
        name: tom_upper_name,
        ticket: None,
    };

    // HINT you might want to assign the return value
    (shrey, tom) = move_ticket(shrey, tom);

    println!("{} ticket: {:?}", shrey.name, shrey.ticket);
    println!("{} ticket: {:?}", tom.name, tom.ticket);
}


fn to_uppercase(s: String) -> String {
    s.to_uppercase()
}

fn move_ticket(mut from: Person, mut to: Person) -> (Person, Person) {
    // TODO:
    // the ticket should be moved to the "from" person
    // to the 'to' person
    to.ticket = from.ticket;
    from.ticket = None;
    (from, to)
}

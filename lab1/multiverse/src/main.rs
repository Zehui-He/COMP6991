// hmm this doesn't look right!!
struct UniverseDetails{
    universe_name: String,
    universe_winner: String,
    universe_population : u32,
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {
    // does this even compile??
    let details = if universe_id % 3 == 0 && universe_id % 5 == 0 {
        UniverseDetails{
            universe_name: "Stardew Valley".to_string(),
            universe_winner: "Jojo Corp".to_string(),
            universe_population: 1,
        }
    } else if universe_id % 5 == 0 {
        UniverseDetails{
            universe_name: "Miraculous".to_string(),
            universe_winner: "Hawk Moth".to_string(),
            universe_population: 22,
        }
    } else if universe_id % 3 == 0 {
        UniverseDetails{
            universe_name: "Star Wars".to_string(),
            universe_winner: "Humans".to_string(),
            universe_population: 4294967295,
        }
    } else {
        return None;
    };
    Some(details)
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        let res = get_universe_details(id);
        if let Some(details) = res {
            println!("Universe with id {} is called {}, won by {} and has a population of {}", 
                id, 
                details.universe_name, 
                details.universe_winner, 
                details.universe_population
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}

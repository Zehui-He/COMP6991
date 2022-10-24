use adventurers::game::MyGame;
use termgame::{SimpleEvent, GameSettings, run_game, KeyCode};
use std::error::Error;
use std::time::Duration;


fn main() -> Result<(), Box<dyn Error>> {
    // TODO: the map path is HARD CODED
    let mut controller = match MyGame::new(String::from("/../maps/full_game.ron")) {
        Ok(mygame_obj) => mygame_obj,
        Err(_) => {
            panic!("The map file cannot be read!");
        },
    };

    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into()))
    )?;

    println!("Game Ended!");

    Ok(())
}
use adventurers::game::MyGame;
use termgame::{SimpleEvent, GameSettings, run_game, KeyCode};
use std::error::Error;
use std::time::Duration;
use adventurers::game::GameInitializationError;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("No enough argument is given");
        std::process::exit(0);
    }
    let file_path = args[1].clone();
    let quest_num = args[2].clone();

    let mut controller = match MyGame::new(file_path, &quest_num) {
        Ok(mygame_obj) => mygame_obj,
        Err(GameInitializationError::ReadMapError) => {
            println!("The map file cannot be read!");
            std::process::exit(0);
        },
        Err(GameInitializationError::QuestError) => {
            println!("The quest {} cannot be initialized!", quest_num);
            std::process::exit(0);
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
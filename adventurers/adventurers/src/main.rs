use termgame::{SimpleEvent, Controller, Game, GameEvent, GameSettings, StyledCharacter, run_game, KeyCode, ViewportLocation};
use std::error::Error;
use std::time::Duration;
// Self-defined modules
use adventurers::player::Player;
use adventurers::movement::{Movement, MovementTrait};

struct MyGame {
    player: Player
}

impl MyGame {
    fn new() -> MyGame {
        MyGame { player: Player::new() }
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        // Render the initial position of a player
        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::from(self.player.repr)));
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                if ch == 'w' {
                    game.set_screen_char(self.player.x, self.player.y, None);
                    self.player.move_by(Movement::up());
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::from(self.player.repr)));
                }
                else if ch == 's' {
                    game.set_screen_char(self.player.x, self.player.y, None);
                    self.player.move_by(Movement::down());
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::from(self.player.repr)));
                }
                else if ch == 'a' {
                    game.set_screen_char(self.player.x, self.player.y, None);
                    self.player.move_by(Movement::left());
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::from(self.player.repr)));
                }
                else if ch == 'd' {
                    game.set_screen_char(self.player.x, self.player.y, None);
                    self.player.move_by(Movement::right());
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::from(self.player.repr)));
                }
            },
            _ => {}
        }

    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame::new();

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
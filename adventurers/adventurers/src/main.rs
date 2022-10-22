use termgame::{SimpleEvent, Controller, Game, GameEvent, GameSettings, StyledCharacter, run_game, KeyCode, ViewportLocation, GameColor, GameStyle};
use std::error::Error;
use std::time::Duration;
// Self-defined modules
use adventurers::player::Player;
use adventurers::movement::{Coordinate, MovementTrait};

struct MyGame {
    player: Player
    //TODO: Track the view port position with a coordinate
}

impl MyGame {
    fn new() -> MyGame {
        MyGame { player: Player::default() }
    }

    fn move_player(&mut self, game: &mut Game, movement: Coordinate) {
        // TODO: Check if the next block if movable
        let original_style = game.get_screen_char(self.player.get_x_pos(), self.player.get_y_pos());
        match original_style {
            Some(style) => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(style.character(' ')));
            }
            None => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), None);
            },
        }
        self.player.move_by(movement);
        let next_style = game.get_screen_char(self.player.get_x_pos(), self.player.get_y_pos());
        match next_style {
            Some(style) => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(style.character(self.player.repr)));
            }
            None => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(StyledCharacter::from(self.player.repr)));
            },
        }
        // TODO: If the player move out of the map, the viewport should follow
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        // Render the initial position of a player
        game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(StyledCharacter::from(self.player.repr)));
        game.set_screen_char(2, 2, Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Red)))));
        game.set_screen_char(1, 1, Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Blue)))));
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                if ch == 'w' {
                    self.move_player(game, Coordinate::up());
                }
                else if ch == 's' {
                    self.move_player(game, Coordinate::down());
                }
                else if ch == 'a' {
                    self.move_player(game, Coordinate::left());
                }
                else if ch == 'd' {
                    self.move_player(game, Coordinate::right());
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
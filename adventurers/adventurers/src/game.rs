use termgame::{SimpleEvent, Controller, Game, GameEvent, StyledCharacter, KeyCode, ViewportLocation, GameColor, GameStyle, Message};
use std::collections::HashMap;
// Self-defined modules
use crate::player::Player;
use crate::movement::{Coordinate, MovementTrait};
use crate::mapparser::read_map;
use crate::blocks::Blocks;
use adventurers_quest::quest::PlayerWalkEvent;

pub struct ReadMapError {}

pub struct MyGame {
    player: Player,
    map: HashMap<(i32, i32), Blocks>
}

impl MyGame {
    pub fn new(file_path: String) -> Result<MyGame, ReadMapError> {
        let mygame_obj = match read_map(file_path) {
            Ok(res) => Ok(MyGame { 
                player: Player::default(),
                map: res
            }),
            Err(_) => Err(ReadMapError{}),
        };

        mygame_obj
    }

    pub fn player_in_viewport(&self, game: &Game) -> bool {
        let view_pos = game.get_viewport();
        let player_pos = self.player.get_position();
        let player_rel_x_pos = player_pos.x - view_pos.x;
        let player_rel_y_pos = player_pos.y - view_pos.y;
        if player_rel_x_pos < 0 || player_rel_x_pos > 77 || player_rel_y_pos < 0 || player_rel_y_pos > 21 {
            return false;
        }
        return true;
    }

    pub fn next_block_is_barrier(&self, movement: &Coordinate) -> bool {
        let curr_player_block = self.player.get_position();
        match self.map.get(&(curr_player_block.x + movement.x, curr_player_block.y + movement.y)) {
            Some(Blocks::Barrier) => true,
            _ => false,
        }
    }

    pub fn get_curr_block(&self) -> Option<&Blocks> {
        let curr_player_block = self.player.get_position();
        self.map.get(&(curr_player_block.x, curr_player_block.y))
    }
    
    pub fn curr_block_is_sign(&self) -> bool {
        let curr_player_block = self.player.get_position();
        match self.map.get(&(curr_player_block.x, curr_player_block.y)) {
            Some(Blocks::Sign(_)) => true,
            _ => false,
        }
    }

    pub fn render_player_move(&mut self, game: &mut Game, movement: &Coordinate) {
        // When the player move out from the block, the block should remain the same style and the character should gone
        let original_style = game.get_screen_char(self.player.get_x_pos(), self.player.get_y_pos());
        match original_style {
            Some(style) => {
                // If current block is sign, need to rerender the chat balloon
                if self.curr_block_is_sign() {
                    game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(style.character('💬')));
                } else {
                    game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(style.character(' ')));
                }
            }
            None => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), None);
            },
        }

        // Update the position of the player
        self.player.move_by(&movement);

        // When the player steps on the next block, the character of the block should become player
        let next_style = game.get_screen_char(self.player.get_x_pos(), self.player.get_y_pos());
        match next_style {
            Some(style) => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(style.character(self.player.repr)));
            }
            None => {
                game.set_screen_char(self.player.get_x_pos(), self.player.get_y_pos(), Some(StyledCharacter::from(self.player.repr)));
            },
        }
    }

    pub fn move_player(&mut self, game: &mut Game, ch: Option<char>) {

        // Check momvement type
        let movement = match ch {
            Some('w') => Coordinate::up(),
            Some('s') => Coordinate::down(),
            Some('a') => Coordinate::left(),
            Some('d') => Coordinate::right(),
            None => Coordinate::no_move(),
            _ => return
        };

        // If the next block is barrier, the player should not move
        if self.next_block_is_barrier(&movement) {
            return;
        }

        // Move the player into the target block
        self.render_player_move(game, &movement);

        //If the player move out of the map, the viewport should follow
        if !self.player_in_viewport(game) {
            let mut viewport_pos: ViewportLocation = game.get_viewport();
            viewport_pos.x += movement.x;
            viewport_pos.y += movement.y;
            game.set_viewport(viewport_pos);
        }

        // Find what current block is and change the state of player
        let curr_block = self.get_curr_block();
        match curr_block {
            Some(block) => {
                match block {
                    // The player walk in water block
                    Blocks::Water => {
                        self.player.walk_in_water(game);
                    },
                    // The player walk in Sign block
                    Blocks::Sign(msg) => {
                        game.set_message(Some(Message{ title: Some("Message".to_string()), text:msg.to_string() }));
                        self.player.walk_out_water();
                    },
                    // The player walk in other block
                    _ => {
                        self.player.walk_out_water();
                    }
                }
            },
            // The dark undefined block is considered as non-water block
            None => {
                self.player.walk_out_water();
            },
        }
    }

    pub fn render_map(&self, game: &mut Game) {
        for ((x,y), block) in &self.map {
            match block {
                Blocks::Grass => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));},
                Blocks::Sand => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));},
                Blocks::Rock => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));},
                Blocks::Cinderblock => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));},
                Blocks::Flowerbush => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));},
                Blocks::Barrier => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));},
                Blocks::Water => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(' ').style(GameStyle::new().background_color(Some(GameColor::Blue)))));},
                Blocks::Object(character) => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from(character.clone()).style(GameStyle::new().background_color(Some(GameColor::Black)))));},
                Blocks::Sign(_) => {game.set_screen_char(x.clone(), y.clone(), Some(StyledCharacter::from('💬').style(GameStyle::new().background_color(Some(GameColor::Black)))));},
            }
        }
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        // Render the map according to the map file
        self.render_map(game);
        // Render the initial position of a player
        self.move_player(game, None);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                // If there is a message on screen, press any botton to dismiss.
                // The game ends if it say the player is dead.
                match game.get_message() {
                    Some(msg) => {
                        if msg.text == "You are drowned.".to_string() {
                            game.end_game();
                        }
                        else {
                            game.set_message(None);
                        }
                    },
                    None => {},
                }
                
                // Call function to move player and return an event
                self.move_player(game, Some(ch));
            },
            _ => {}
        }

    }

    fn on_tick(&mut self, _game: &mut Game) {}
}
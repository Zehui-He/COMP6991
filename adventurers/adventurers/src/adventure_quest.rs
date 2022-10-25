use adventurers_quest::quest::SimpleQuest;
use crate::game::GameInitializationError;


pub fn initialize_quest(quest_num: &String) -> Result<SimpleQuest, GameInitializationError> {
    if *quest_num == "q1" {
        Ok(SimpleQuest::WalkQuest { target_block: "sand".to_string(), num: 1, repeat: 5, completed: false })
    } else {
        return Err(GameInitializationError::QuestError);
    }
}
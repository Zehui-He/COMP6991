use adventurers_quest::quest::{SimpleQuest, Quests, QuestStatus, SimpleQuestTrait, GameEvent};
use crate::{game::GameInitializationError, blocks::Blocks};


pub trait MyGameEventTrait {}

impl <Target> MyGameEventTrait for GameEvent<Target> 
where Target: std::cmp::PartialEq
{}

enum GiveQuest {
    Q1,
    Q2,
    Q3,
}

impl GiveQuest {
    fn give_value(&self) -> Vec<Box<dyn SimpleQuestTrait>> {
        match self {
            GiveQuest::Q1 => vec![Box::from(SimpleQuest::<Blocks> { target: Blocks::Sand, count: 5, repeat: 1, completed: false })],
            GiveQuest::Q2 => vec![
                Box::from(SimpleQuest::<char> { target: 'x', count: 5, repeat: 1, completed: false }),
                Box::from(SimpleQuest::<char> { target: 'y', count: 3, repeat: 1, completed: false })
            ],
            GiveQuest::Q3 => vec![
                Box::from(SimpleQuest::<Blocks> { target: Blocks::Sand, count: 5, repeat: 1, completed: false }),
                Box::from(SimpleQuest::<char> { target: 'y', count: 1, repeat: 1, completed: false }),
                Box::from(SimpleQuest::<Blocks> { target: Blocks::Water, count: 9, repeat: 3, completed: false })
            ],
        }
    }
}

pub fn initialize_quest(quest_num: &String) -> Result<Quests, GameInitializationError> {
    if *quest_num == "q1" {
        let quests = GiveQuest::Q1.give_value();
        Ok(Quests { quests , status: QuestStatus::Ongoing })
    }
    else if *quest_num == "q2" {
        let quests = GiveQuest::Q2.give_value();
        Ok(Quests { quests , status: QuestStatus::Ongoing })
    }
    else if *quest_num == "q3" {
        let quests = GiveQuest::Q3.give_value();
        Ok(Quests { quests , status: QuestStatus::Ongoing })
    }
    else {
        return Err(GameInitializationError::QuestError);
    }
}
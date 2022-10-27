//! This crate defines the behaviour of quest system.
use adventurers_quest::{quest::QuestStatus, Quest};

use crate::{blocks::Blocks, game::GameInitializationError};

/// The Goal trait is a supertrait of three traits. The 'Debug' and 'Display' trait
/// is used to debug or show the object in output. The 'PatialEq' trait is used to
/// compare the if the targets are equal in SingleQuest object and GameEvent object.
pub trait Goal: std::fmt::Debug + std::fmt::Display + std::cmp::PartialEq{}

impl Goal for char {}
impl Goal for Blocks {}

/// This is most simple quest that is avaliable for the game. This kind of
/// quests only consist of one objective. The Target type must implement 
/// 'goal' trait. 
#[derive(Debug)]
pub struct SingleQuest<T:Goal> {
    target: T,
    count: u8,
    repeat: u8,
    completed: bool
}

impl std::fmt::Display for SingleQuest<Blocks> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        if self.completed {
            res += "\t[✅] ";
        } else {
            res += "\t[  ] ";
        }
        res += &format!("Walk on a {}", self.target);
        if self.count > 1 {
            res += &format!(" for {} times", self.count);
        }
        if self.repeat > 1 {
            res += &format!(" (repeat {} times)", self.repeat);
        }
        write!(f, "{}\n", res)
    }
}

impl std::fmt::Display for SingleQuest<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        if self.completed {
            res += "\t[✅] ";
        } else {
            res += "\t[  ] ";
        }
        res += &format!("Collect {} '{}' object", self.count, self.target);
        if self.repeat > 1 {
            res += &format!(" (repeat {} times)", self.repeat);
        }
        write!(f, "{}\n", res)
    }
}

impl Quest<GameEvent> for SingleQuest<Blocks>
{
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        match event {
            GameEvent::BlockEvent { target, count } => {
                if self.completed {
                    return QuestStatus::Complete;
                }
        
                if self.target != *target {
                    return QuestStatus::Ongoing;
                }
        
                if count % self.count == 0 {
                    self.repeat -= 1;
                }
        
                if self.repeat > 0 {
                    return QuestStatus::Ongoing;
                }
        
                self.completed = true;
                
                return QuestStatus::Complete;
            },
            _ => {
                if self.completed {
                    return QuestStatus::Complete;
                }
                else {
                    return  QuestStatus::Ongoing;
                }
            }
        }
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl Quest<GameEvent> for SingleQuest<char>
{
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        match event {
            GameEvent::CollectEvent { target } => {
                if self.completed {
                    return QuestStatus::Complete;
                }
        
                if self.target != *target {
                    return QuestStatus::Ongoing;
                }
        
                self.repeat -= 1;
        
                if self.repeat > 0 {
                    return QuestStatus::Ongoing;
                }
        
                self.completed = true;
        
                return QuestStatus::Complete;
            },
            _ => {
                if self.completed {
                    return QuestStatus::Complete;
                }
                else {
                    return  QuestStatus::Ongoing;
                }
            }
        }
    }

    fn reset(&mut self) {
        todo!()
    }
}

/// This struct is used to pass information of the game. If a player
/// walked 3 steps on sand block continuously, the game would generate
/// a GameEvent object with Game::BlockEvent{ target:Blocks::Sand, count:3 }.
pub enum GameEvent {
    BlockEvent {
        target: Blocks,
        count: u8
    },
    CollectEvent {
        target: char
    }
}

/// The SequenceQuest object consists a Vec of Quests. These Quests can be any
/// other Quest Object, even SequenceQuest! The SequenceQuest object require player
/// to complete the task one after one.
#[derive(Debug)]
pub struct SequenceQuest {
    quests: Vec<Box<dyn Quest<GameEvent>>>,
    completed: bool
}

impl std::fmt::Display for SequenceQuest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        if self.completed {
            res += "[✅] ";
        } else {
            res += "[  ] ";
        }
        res += "You should complete in order: \n";
        for quest in &self.quests {
            res += &format!("\t{}", quest);
        }
        write!(f, "{}\n", res)
    }
}

impl Quest<GameEvent> for SequenceQuest
{
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        if self.completed {
            return  QuestStatus::Complete;
        }

        for quest in &mut self.quests {
            let status = quest.register_event(event);
            if status == QuestStatus::Ongoing {
                return QuestStatus::Ongoing;
            }
        }

        self.completed = true;
        QuestStatus::Complete
    }

    fn reset(&mut self) {
        todo!()
    }
}

/// The SequenceQuest object consists a Vec of Quests and a count. The
/// count indicate how many quest need to be completed to win the game.
/// The ParrllelQuest object would allow players to complete several 
/// task simutaneously. 
#[derive(Debug)]
pub struct ParallelQuest {
    quests: Vec<Box<dyn Quest<GameEvent>>>,
    count: u8,
    completed: bool
}

impl std::fmt::Display for ParallelQuest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        if self.completed {
            res += "[✅] ";
        } else {
            res += "[  ] ";
        }
        res += &format!("You should complete at least {} of these quests: \n", self.count);
        for quest in &self.quests {
            res += &format!("{}", quest);
        }
        write!(f, "{}", res)
    }
}

impl Quest<GameEvent> for ParallelQuest
{
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        if self.completed {
            return  QuestStatus::Complete;
        }

        let mut complete_count = 0;
        for quest in &mut self.quests {
            let status = quest.register_event(event);
            if status == QuestStatus::Complete {
                complete_count += 1;
            }
        }

        if complete_count >= self.count {
            self.completed = true;
            return QuestStatus::Complete;
        }
        
        QuestStatus::Ongoing
    }

    fn reset(&mut self) {
        todo!()
    }
}

enum GiveQuest {
    Q1,
    Q2,
    Q3,
}

impl GiveQuest
{
    fn give_value(&self) -> Box<dyn Quest<GameEvent>>
    {
        match self {
            GiveQuest::Q1 => Box::from(SingleQuest { target: Blocks::Sand, count: 1, repeat: 5, completed: false }),

            GiveQuest::Q2 => Box::from(SequenceQuest{ quests: vec![
                Box::from(SingleQuest::<char> { target: 'x', count: 1, repeat: 5, completed: false }),
                Box::from(SingleQuest::<char> { target: 'y', count: 1, repeat: 5, completed: false })
                ], 
            completed: false }),

            GiveQuest::Q3 => Box::from(
                ParallelQuest { quests: vec![
                    Box::from(SequenceQuest { quests: vec![
                        Box::from(SingleQuest { target: Blocks::Sand, count: 1, repeat: 5, completed: false }),
                        Box::from(SingleQuest::<char> { target: 'x', count: 1, repeat: 1, completed: false })
                    ], completed: false }),
                    Box::from(SequenceQuest { quests: vec![
                        Box::from(SingleQuest::<char> { target: 'x', count: 1, repeat: 1, completed: false }),
                        Box::from(SingleQuest { target: Blocks::Grass, count: 1, repeat: 1, completed: false })
                    ], completed: false }),
                    Box::from(SingleQuest { target: Blocks::Water, count: 9, repeat: 3, completed: false })
                ], count: 2, completed: false }
            )
        }
    }
}

/// This function generate the quest according to which quest number is given.
pub fn initialize_quest(quest_num: &String) -> Result<Box<dyn Quest<GameEvent>>, GameInitializationError> {
    if *quest_num == "q1" {
        let quests = GiveQuest::Q1.give_value();
        Ok(quests)
    }
    else if *quest_num == "q2" {
        let quests = GiveQuest::Q2.give_value();
        Ok(quests)
    }
    else if *quest_num == "q3" {
        let quests = GiveQuest::Q3.give_value();
        Ok(quests)
    }
    else {
        return Err(GameInitializationError::QuestError);
    }
}
use crate::Quest;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing
}

#[derive(Debug)]
pub struct Quests {
    pub description: String,
    pub status: QuestStatus
}

// Struct that is used to tell what the game is happening
struct Event {}

impl std::fmt::Display for Quests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let quest_status: String;
        match self.status {
            QuestStatus::Complete => quest_status = "✅".to_string(),
            QuestStatus::Ongoing => quest_status = String::default(),
        }
        
        write!(f, "({}, {})", &self.description, quest_status)
    }
}


impl Quest<Event> for Quests {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
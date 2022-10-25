use crate::Quest;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing
}

#[derive(Debug)]
pub struct AllQuests {
    pub quests: Vec<SimpleQuest>,
    pub status: QuestStatus
}

#[derive(Debug)]
pub enum SimpleQuest {
    WalkQuest {
        target_block: String,
        steps: u32,
        repeat: u32,
        completed: bool
    },
    CollectQuest {
        target: char,
        num: u32,
        repeat: u32,
        completed: bool
    },
}

// Struct that is used to tell what the game is happening
pub struct PlayerWalkEvent {
    block_type: String,
    steps_on_same_type: u32
}

// impl std::fmt::Display for AllQuests {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut res = String::new();
//         for quest in &self.quests {
//             match quest {
//                 SimpleQuest::WalkQuest { target_block, steps, completed, repeat } => {
//                     if *completed {
//                         res += "[✅]";
//                     } else {
//                         res += "[ ]";
//                     }
//                 },
//                 SimpleQuest::CollectQuest { target, num, completed, repeat } => todo!(),
//             }
//         }
//         write!(f,"{}", res)
//     }
// }


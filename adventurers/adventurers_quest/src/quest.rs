#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing
}

pub struct Quests {
    pub quests: Vec<SimpleQuest>,
    pub status: QuestStatus
}

#[derive(Debug)]
pub enum SimpleQuest {
    WalkQuest {
        target_block: String,
        num: u32,
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
pub enum GameEvent {
    PlayerWalkEvent {
        block_type: String,
        steps_on_same_type: u32
    },
    PlayerCollectEvent {
        target: char,
    }
}

// impl std::fmt::Display for SimpleQuest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut res = String::default();
//         match self {
//             SimpleQuest::WalkQuest { target_block, num, repeat, completed } => {
//                 if *completed {
//                     res += "\t[✅] ";
//                 } else {
//                     res += "\t[  ] ";
//                 }
//                 res += &format!("Walk on a {} block", target_block);
//                 if *num > 1 {
//                     res += &format!(" for {} times", num);
//                 }
//                 if *repeat > 1 {
//                     res += &format!(" (repeat {} times)", repeat);
//                 }
//                 write!(f, "{}", res)
//             },
//             SimpleQuest::CollectQuest { target, num, repeat, completed } => {
//                 if *completed {
//                     res += "\t[✅] ";
//                 } else {
//                     res += "\t[  ] ";
//                 }
//                 res += &format!("Collect {} '{}' object", num, target);
//                 if *repeat > 1 {
//                     res += &format!(" (repeat {} times)", repeat);
//                 }
//                 write!(f, "{}", res)
//             },
//         }
//     }
// }

// #[test]
// fn test_display_simple_quest() {
//     let quest1 = SimpleQuest::WalkQuest { target_block: "water".to_string(), num: 9, repeat: 3, completed: false };
//     let quest2 = SimpleQuest::WalkQuest { target_block: "sand".to_string(), num: 1, repeat: 5, completed: true };
//     let quest3 = SimpleQuest::CollectQuest { target: 'x', num: 1, repeat: 3, completed: false };
//     let quest4 = SimpleQuest::CollectQuest { target: 'x', num: 2, repeat: 3, completed: true };
//     println!("{}", quest1);
//     println!("{}", quest2);
//     println!("{}", quest3);
//     println!("{}", quest4);
// }


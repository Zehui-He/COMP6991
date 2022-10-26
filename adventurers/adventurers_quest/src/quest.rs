use crate::Quest;

#[warn(missing_docs)]

/// This enum type is used to check the status of the quest.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    /// The quest is completed
    Complete,
    /// The quest is not completed
    Ongoing
}

/// This is most simple quest that is avaliable for the game. This kind of
/// quests only consist of one objective. The Target type must implement 
/// std::cmp::PartialEq trait because the target of SimpleQuest need to 
/// compare with the target of GameEvent.
#[derive(Debug)]
pub struct SimpleQuest<Target: std::cmp::PartialEq> {
    pub target: Target,
    pub count: u32,
    pub repeat: u32,
    pub completed: bool
}

// Impletement the Quest trait for the SimpleQuest structs.
impl <Target> Quest<MyGameEvent<Target>> for SimpleQuest<Target> 
where Target: std::cmp::PartialEq + std::fmt::Debug  + std::fmt::Display
{
    fn register_event(&mut self, event: &MyGameEvent<Target>) -> QuestStatus {
        if self.completed {
            return QuestStatus::Complete;
        }
        if self.target == event.target && self.count == event.count {
            self.repeat -= 1;
        }
        if self.repeat == 0 {
            self.completed = true;
        }
        return QuestStatus::Ongoing;
    }

    fn reset(&mut self) {
        todo!()
    }
}

impl <Target> std::fmt::Display for SimpleQuest<Target>
where Target: std::cmp::PartialEq + std::fmt::Debug  + std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::default();
        if self.completed {
            res += "\t[✅] ";
        } else {
            res += "\t[  ] ";
        }
        res += &format!("Collect or walk {} for {} times ", self.target, self.count);
        if self.repeat > 1 {
            res += &format!(" (repeat {} times)", self.repeat);
        }
        write!(f, "{}\n", res)
    }
}

// Struct that is used to tell what the game is happening
pub struct MyGameEvent<Target: std::cmp::PartialEq> {
    pub target: Target,
    pub count: u32
}

pub trait MyGameEventTrait {}
impl <Target> MyGameEventTrait for MyGameEvent<Target> 
where Target: std::cmp::PartialEq
{}

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


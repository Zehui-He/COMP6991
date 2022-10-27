/// This enum type is used to check the status of the quest.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    /// The quest is completed
    Complete,
    /// The quest is not completed
    Ongoing,
}

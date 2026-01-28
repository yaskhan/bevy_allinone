use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The status of a quest or an objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

/// A sub-objective within a quest.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Objective {
    pub name: String,
    pub description: String,
    pub status: QuestStatus,
}

/// A quest that can be assigned to a player.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub status: QuestStatus,
    pub rewards_description: String,
}

/// Component that handles the player's quest log.
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct QuestLog {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
}

/// Events for the quest system.
#[derive(Event)]
pub enum QuestEvent {
    Started(u32),
    ObjectiveCompleted(u32, usize),
    Completed(u32),
    Failed(u32),
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Register systems and events
    }
}

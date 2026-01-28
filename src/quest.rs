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

/// Component for entities that can give quests (NPCs, boards, etc.).
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct QuestStation {
    pub quest: Quest,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestEvent>()
            .register_type::<QuestLog>()
            .register_type::<QuestStation>()
            .add_systems(Update, (
                handle_quest_events,
                update_quest_status,
                handle_quest_interactions,
            ));
    }
}

/// System to handle interactions with QuestStations.
fn handle_quest_interactions(
    mut commands: Commands,
    mut events: ResMut<crate::interaction::InteractionEventQueue>,
    quest_stations: Query<&QuestStation>,
    mut quest_logs: Query<(Entity, &mut QuestLog)>,
) {
    let mut interactions_processed = Vec::new();
    
    for (idx, event) in events.0.iter().enumerate() {
        if let Ok(station) = quest_stations.get(event.target) {
            // Find the quest log for the source (interactor)
            if let Ok((_log_entity, mut log)) = quest_logs.get_mut(event.source) {
                // Check if quest is already in log
                let already_has = log.active_quests.iter().any(|q| q.id == station.quest.id) ||
                                  log.completed_quests.iter().any(|q| q.id == station.quest.id);
                
                if !already_has {
                    let mut quest = station.quest.clone();
                    quest.status = QuestStatus::InProgress;
                    log.active_quests.push(quest);
                    info!("Quest '{}' accepted!", station.quest.name);
                } else {
                    info!("Player already has quest '{}' (active or complete)", station.quest.name);
                }
                
                interactions_processed.push(idx);
            } else {
                // If source doesn't have QuestLog, give them one
                commands.entity(event.source).insert(QuestLog::default());
            }
        }
    }
    
    // Note: We are not removing from events.0 here because other systems might need to read it.
    // In a mature system, we'd use a more robust way to mark events as handled.
}

/// System to handle quest-related events and update the QuestLog.
fn handle_quest_events(
    mut events: EventReader<QuestEvent>,
    mut quest_logs: Query<&mut QuestLog>,
) {
    for event in events.read() {
        match event {
            QuestEvent::Started(id) => {
                info!("Quest started: {}", id);
            }
            QuestEvent::ObjectiveCompleted(quest_id, obj_idx) => {
                info!("Objective {} completed for quest {}", obj_idx, quest_id);
            }
            QuestEvent::Completed(id) => {
                info!("Quest completed: {}", id);
            }
            QuestEvent::Failed(id) => {
                warn!("Quest failed: {}", id);
            }
        }
    }
}

/// System to automatically update quest status based on objective progress.
fn update_quest_status(
    mut quest_logs: Query<&mut QuestLog>,
) {
    for mut log in quest_logs.iter_mut() {
        for quest in log.active_quests.iter_mut() {
            if quest.status == QuestStatus::InProgress {
                let all_completed = quest.objectives.iter().all(|obj| obj.status == QuestStatus::Completed);
                if all_completed {
                    quest.status = QuestStatus::Completed;
                }
            }
        }
        
        // Move completed quests to the completed list
        let mut completed_indices = Vec::new();
        for (idx, quest) in log.active_quests.iter().enumerate() {
            if quest.status == QuestStatus::Completed {
                completed_indices.push(idx);
            }
        }
        
        for idx in completed_indices.into_iter().rev() {
            let quest = log.active_quests.remove(idx);
            log.completed_quests.push(quest);
        }
    }
}

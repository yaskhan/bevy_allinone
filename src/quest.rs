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
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum QuestEvent {
    Started(u32),
    ObjectiveCompleted(u32, usize),
    Completed(u32),
    Failed(u32),
}

/// Custom queue for quest events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct QuestEventQueue(pub Vec<QuestEvent>);

/// Component for entities that can give quests (NPCs, boards, etc.).
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct QuestStation {
    pub quest: Quest,
    pub show_on_map: bool,
    pub map_description: String,
    pub map_icon_type: crate::map::types::MapIconType,
}

/// Component for entities that complete objectives when triggered.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ObjectiveTrigger {
    pub quest_id: u32,
    pub objective_index: usize,
    pub activate_on_interaction: bool,
    pub activate_on_enter: bool,
    pub enter_radius: f32,
    pub single_use: bool,
    pub is_active: bool,
    pub show_debug_log: bool,
    pub show_on_map: bool,
    pub map_description: String,
    pub map_icon_type: crate::map::types::MapIconType,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestEventQueue>()
            .register_type::<QuestLog>()
            .register_type::<QuestStation>()
            .register_type::<ObjectiveTrigger>()
            .add_systems(Update, (
                handle_quest_events,
                update_quest_status,
                handle_quest_interactions,
                handle_objective_trigger_interactions,
                handle_objective_trigger_enter,
            ));
    }
}

/// System to handle interactions with QuestStations.
fn handle_quest_interactions(
    mut commands: Commands,
    mut interaction_events: ResMut<crate::interaction::InteractionEventQueue>,
    quest_stations: Query<&QuestStation>,
    mut quest_logs: Query<(Entity, &mut QuestLog)>,
    mut quest_events: ResMut<QuestEventQueue>,
) {
    for event in interaction_events.0.iter() {
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

                    // Trigger quest started event
                    quest_events.0.push(QuestEvent::Started(station.quest.id));
                } else {
                    info!("Player already has quest '{}' (active or complete)", station.quest.name);
                }
            } else {
                // If source doesn't have QuestLog, give them one
                commands.entity(event.source).insert(QuestLog::default());
            }
        }
    }
}

/// System to handle interactions with ObjectiveTriggers.
fn handle_objective_trigger_interactions(
    mut interaction_events: ResMut<crate::interaction::InteractionEventQueue>,
    trigger_query: Query<&ObjectiveTrigger>,
    mut quest_logs: Query<&mut QuestLog>,
    mut quest_events: ResMut<QuestEventQueue>,
) {
    for event in interaction_events.0.iter() {
        let Ok(trigger) = trigger_query.get(event.target) else { continue };
        if !trigger.is_active || !trigger.activate_on_interaction {
            continue;
        }

        let Ok(mut log) = quest_logs.get_mut(event.source) else { continue };
        if mark_objective_completed(&mut log, trigger.quest_id, trigger.objective_index) {
            quest_events.0.push(QuestEvent::ObjectiveCompleted(trigger.quest_id, trigger.objective_index));
        }
    }
}

/// System to handle proximity-based ObjectiveTriggers.
fn handle_objective_trigger_enter(
    player_query: Query<(Entity, &GlobalTransform), With<crate::character::Player>>,
    mut trigger_query: Query<(Entity, &GlobalTransform, &mut ObjectiveTrigger)>,
    mut quest_logs: Query<&mut QuestLog>,
    mut quest_events: ResMut<QuestEventQueue>,
) {
    let Some((player_entity, player_transform)) = player_query.iter().next() else {
        return;
    };

    let player_pos = player_transform.translation();

    for (entity, transform, mut trigger) in trigger_query.iter_mut() {
        if !trigger.is_active || !trigger.activate_on_enter {
            continue;
        }

        let distance = player_pos.distance(transform.translation());
        if distance > trigger.enter_radius {
            continue;
        }

        let Ok(mut log) = quest_logs.get_mut(player_entity) else { continue };
        if mark_objective_completed(&mut log, trigger.quest_id, trigger.objective_index) {
            quest_events.0.push(QuestEvent::ObjectiveCompleted(trigger.quest_id, trigger.objective_index));
            if trigger.single_use {
                trigger.is_active = false;
            }
            if trigger.show_debug_log {
                info!(
                    "Objective trigger activated (quest {}, objective {})",
                    trigger.quest_id,
                    trigger.objective_index
                );
            }
        }
    }
}

/// System to handle quest-related events and update the QuestLog.
fn handle_quest_events(
    mut events: ResMut<QuestEventQueue>,
) {
    // Process events and then clear the queue
    for event in events.0.iter() {
        match event {
            QuestEvent::Started(id) => {
                info!("Quest started event: {}", id);
            }
            QuestEvent::ObjectiveCompleted(quest_id, obj_idx) => {
                info!("Objective {} completed for quest {}", obj_idx, quest_id);
            }
            QuestEvent::Completed(id) => {
                info!("Quest completed event: {}", id);
            }
            QuestEvent::Failed(id) => {
                warn!("Quest failed event: {}", id);
            }
        }
    }
    
    // Clear the queue after processing - similar to how Bevy events work
    events.0.clear();
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

fn mark_objective_completed(log: &mut QuestLog, quest_id: u32, objective_index: usize) -> bool {
    for quest in log.active_quests.iter_mut() {
        if quest.id == quest_id {
            if let Some(objective) = quest.objectives.get_mut(objective_index) {
                if objective.status != QuestStatus::Completed {
                    objective.status = QuestStatus::Completed;
                    return true;
                }
            }
        }
    }

    false
}

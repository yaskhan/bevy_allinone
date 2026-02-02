use bevy::prelude::*;
use super::types::*;

/// Queue for result events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct DamageResultQueue(pub Vec<DamageResultEvent>);

#[derive(Debug, Clone, Copy, Event)]
pub struct DamageResultEvent {
    pub target: Entity,
    pub source: Option<Entity>,
    pub original_amount: f32,
    pub final_amount: f32,
    pub damage_type: DamageType,
    pub shielded_amount: f32,
    pub is_crit: bool,
    pub is_block: bool,
}

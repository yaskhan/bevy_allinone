use bevy::prelude::*;
use super::types::InteractionType;

/// Event to add a device to the player's list
#[derive(Debug, Clone, Copy)]
pub struct AddDeviceEvent {
    pub player: Entity,
    pub device: Entity,
}

#[derive(Resource, Default)]
pub struct AddDeviceQueue(pub Vec<AddDeviceEvent>);

/// Event to remove a device from the player's list
#[derive(Debug, Clone, Copy)]
pub struct RemoveDeviceEvent {
    pub player: Entity,
    pub device: Entity,
}

#[derive(Resource, Default)]
pub struct RemoveDeviceQueue(pub Vec<RemoveDeviceEvent>);

/// Event triggered when a valid interaction occurs
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

/// Custom queue for interaction events
#[derive(Resource, Default)]
pub struct InteractionEventQueue(pub Vec<InteractionEvent>);

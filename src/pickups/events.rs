use bevy::prelude::*;

/// Event triggered when a pickup interaction occurs.
#[derive(Debug, Clone, Copy)]
pub struct PickupEvent {
    pub source: Entity,
    pub target: Entity,
}

/// Custom queue for pickup events (mirrors InteractionEventQueue pattern).
#[derive(Resource, Default)]
pub struct PickupEventQueue(pub Vec<PickupEvent>);

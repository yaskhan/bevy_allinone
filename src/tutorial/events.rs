use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Events for controlling the tutorial system.
#[derive(Event, Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum TutorialEvent {
    Open(u32),
    NextPanel,
    PreviousPanel,
    Close,
}

/// Custom queue for tutorial events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct TutorialEventQueue(pub Vec<TutorialEvent>);

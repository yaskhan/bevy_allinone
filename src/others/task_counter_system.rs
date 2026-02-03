use bevy::prelude::*;

/// Task counter tracking.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TaskCounterSystem {
    pub current: u32,
    pub goal: u32,
}

impl Default for TaskCounterSystem {
    fn default() -> Self {
        Self { current: 0, goal: 0 }
    }
}

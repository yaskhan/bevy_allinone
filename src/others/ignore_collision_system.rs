use bevy::prelude::*;

/// Defines collision ignore rules.
///
/// GKC reference: `ignoreCollisionSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct IgnoreCollisionSystem {
    pub target: Entity,
    pub ignore: Vec<Entity>,
    pub enabled: bool,
}

impl Default for IgnoreCollisionSystem {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            ignore: Vec::new(),
            enabled: true,
        }
    }
}

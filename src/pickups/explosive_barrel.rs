use bevy::prelude::*;

/// Explosive barrel pickup/container.
///
/// GKC reference: `explosiveBarrel.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExplosiveBarrel {
    pub exploded: bool,
}

impl Default for ExplosiveBarrel {
    fn default() -> Self {
        Self { exploded: false }
    }
}

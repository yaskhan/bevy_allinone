use bevy::prelude::*;

/// Breakable crate pickup container.
///
/// GKC reference: `crate.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CrateSystem {
    pub broken: bool,
}

impl Default for CrateSystem {
    fn default() -> Self {
        Self { broken: false }
    }
}

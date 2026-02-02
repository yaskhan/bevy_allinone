use bevy::prelude::*;

/// Console mode configuration.
///
/// GKC reference: `consoleMode.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ConsoleMode {
    pub active: bool,
}

impl Default for ConsoleMode {
    fn default() -> Self {
        Self { active: false }
    }
}

use bevy::prelude::*;

/// Chest pickup container.
///
/// GKC reference: `chestSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ChestSystem {
    pub opened: bool,
}

impl Default for ChestSystem {
    fn default() -> Self {
        Self { opened: false }
    }
}

pub fn update_chest_system(
    mut query: Query<&mut ChestSystem>,
) {
    for mut chest in query.iter_mut() {
        if chest.opened {
            continue;
        }
        // Placeholder for opening logic.
    }
}

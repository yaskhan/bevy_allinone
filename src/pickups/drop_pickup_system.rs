use bevy::prelude::*;

/// Drops a pickup prefab.
///
/// GKC reference: `dropPickUpSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DropPickUpSystem {
    pub prefab_name: String,
    pub dropped: bool,
}

impl Default for DropPickUpSystem {
    fn default() -> Self {
        Self {
            prefab_name: String::new(),
            dropped: false,
        }
    }
}

pub fn update_drop_pickup_system(
    mut commands: Commands,
    mut query: Query<&mut DropPickUpSystem>,
) {
    for mut drop in query.iter_mut() {
        if drop.dropped {
            continue;
        }
        commands.spawn((
            SpatialBundle::default(),
            Name::new(format!("Dropped {}", drop.prefab_name)),
        ));
        drop.dropped = true;
    }
}

use bevy::prelude::*;

/// Spawns an object placeholder.
///
/// GKC reference: `spawnObject.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SpawnObject {
    pub prefab_name: String,
    pub spawn_once: bool,
    pub spawned: bool,
}

impl Default for SpawnObject {
    fn default() -> Self {
        Self {
            prefab_name: String::new(),
            spawn_once: true,
            spawned: false,
        }
    }
}

pub fn update_spawn_object(
    mut commands: Commands,
    mut query: Query<&mut SpawnObject>,
) {
    for mut spawn in query.iter_mut() {
        if spawn.spawn_once && spawn.spawned {
            continue;
        }
        commands.spawn((
            SpatialBundle::default(),
            Name::new(format!("Spawned {}", spawn.prefab_name)),
        ));
        spawn.spawned = true;
    }
}

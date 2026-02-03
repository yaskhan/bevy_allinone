use bevy::prelude::*;

/// Destroys an entity after a delay.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DestroyGameObject {
    pub delay: f32,
    pub timer: f32,
    pub enabled: bool,
}

impl Default for DestroyGameObject {
    fn default() -> Self {
        Self {
            delay: 0.0,
            timer: 0.0,
            enabled: true,
        }
    }
}

pub fn update_destroy_game_object(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DestroyGameObject)>,
) {
    let delta = time.delta_seconds();
    for (entity, mut destroy) in query.iter_mut() {
        if !destroy.enabled {
            continue;
        }
        destroy.timer += delta;
        if destroy.timer >= destroy.delay {
            commands.entity(entity).despawn_recursive();
        }
    }
}

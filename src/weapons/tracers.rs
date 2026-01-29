//! Visual tracer system for projectiles

use bevy::prelude::*;
use super::types::BulletTracer;

/// Update visual tracers with interpolation
pub fn update_tracers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut BulletTracer)>,
) {
    for (entity, mut transform, mut tracer) in query.iter_mut() {
        // Interpolate position towards target
        let direction = tracer.target_pos - tracer.current_pos;
        let distance = direction.length();

        if distance < 0.1 {
            commands.entity(entity).despawn();
            continue;
        }

        let move_amount = tracer.speed * time.delta_secs();
        if move_amount >= distance {
            transform.translation = tracer.target_pos;
            commands.entity(entity).despawn();
        } else {
            let normalized_dir = direction / distance;
            tracer.current_pos += normalized_dir * move_amount;
            transform.translation = tracer.current_pos;
        }
    }
}

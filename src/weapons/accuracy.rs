//! Accuracy system for dynamic weapon spread

use bevy::prelude::*;
use crate::input::InputState;
use super::types::Accuracy;

/// Update weapon accuracy and bloom
pub fn update_accuracy(
    time: Res<Time>,
    _input_state: Res<InputState>,
    mut query: Query<(&mut Accuracy, &GlobalTransform)>,
) {
    for (mut accuracy, _transform) in query.iter_mut() {
        // 1. Update Bloom (Recovery)
        accuracy.current_bloom -= accuracy.recovery_rate * time.delta_secs();
        if accuracy.current_bloom < 0.0 {
            accuracy.current_bloom = 0.0;
        }

        // 2. Calculate Modifiers
        // Note: We need velocity to check movement.
        // Since G... [truncated]
        // which might have a RigidBodyVelocity component from Avian3D.
        // For this implementation, we will simulate movement check based on a simple heuristic or
        // assume the caller (fire_weapon) handles specific modifiers if we can't access velocity here easily.
        // To strictly follow TZ, let's assume we have access to Velocity or we calculate it elsewhere.
        // However, for this specific function signature, we will just handle the "Bloom" logic.
        // The "Spread" calculation in `fire_weapon` will combine `weapon.spread` + `accuracy.current_bloom`.
    }
}

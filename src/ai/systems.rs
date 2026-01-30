use bevy::prelude::*;
use super::types::*;

pub fn update_ai_state_visuals(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &AiController, &AiStateVisuals)>,
) {
    for (transform, ai, visuals) in query.iter() {
        if !visuals.show_state_icons { continue; }
        let pos = transform.translation() + visuals.icon_offset;
        match ai.state {
            AiBehaviorState::Idle => { gizmos.sphere(pos, 0.2, Color::srgb(0.0, 0.0, 1.0)); }
            AiBehaviorState::Chase => { gizmos.sphere(pos, 0.2, Color::srgb(1.0, 1.0, 0.0)); }
            AiBehaviorState::Attack => { gizmos.sphere(pos, 0.3, Color::srgb(1.0, 0.0, 0.0)); }
            AiBehaviorState::Patrol => { gizmos.sphere(pos, 0.1, Color::srgb(0.0, 1.0, 0.0)); }
            AiBehaviorState::Follow => { gizmos.sphere(pos, 0.15, Color::srgb(0.0, 1.0, 1.0)); }
            AiBehaviorState::Idle => {
                // If we want to show 'sleeping' or 'zzzz' as in sleepingStateIconSystem.cs
                // We'll use a specific color or simple gizmo for now
                gizmos.sphere(pos, 0.1, Color::srgb(0.3, 0.3, 1.0));
            }
            _ => {}
        }
    }
}

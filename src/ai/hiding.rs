use bevy::prelude::*;
use crate::ai::types::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct HidePositionsManager {
    pub hide_positions: Vec<Entity>,
}

pub fn update_ai_hiding(
    mut query: Query<(&mut AiController, &GlobalTransform)>,
    hide_positions: Query<&GlobalTransform, With<HidePosition>>,
) {
    for (mut ai, transform) in query.iter_mut() {
        if ai.state != AiBehaviorState::Hide {
            continue;
        }

        // Logic to find closest hide position and move toward it
        let mut closest = None;
        let mut min_dist = f32::MAX;
        let my_pos = transform.translation();

        for hide_xf in hide_positions.iter() {
            let dist = my_pos.distance(hide_xf.translation());
            if dist < min_dist {
                min_dist = dist;
                closest = Some(hide_xf.translation());
            }
        }

        if let Some(target) = closest {
            // Move toward target (would normally interface with pathfinding)
            if my_pos.distance(target) < 1.0 {
                // Arrived at hiding spot
            }
        }
    }
}

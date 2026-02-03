use bevy::prelude::*;
use super::grappling_hook_target::GrapplingHookTarget;

/// Tracks available grappling hook targets for a player.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrapplingHookTargetsSystem {
    pub show_targets_active: bool,
    pub detection_range: f32,
    pub closest_target: Option<Entity>,
    pub targets: Vec<Entity>,
}

impl Default for GrapplingHookTargetsSystem {
    fn default() -> Self {
        Self {
            show_targets_active: true,
            detection_range: 50.0,
            closest_target: None,
            targets: Vec::new(),
        }
    }
}

/// Update the list of grappling hook targets and select the closest.
pub fn update_grappling_hook_targets(
    mut system_query: Query<(&GlobalTransform, &mut GrapplingHookTargetsSystem)>,
    target_query: Query<(Entity, &GlobalTransform, &GrapplingHookTarget)>,
) {
    for (player_transform, mut system) in system_query.iter_mut() {
        if !system.show_targets_active {
            system.targets.clear();
            system.closest_target = None;
            continue;
        }

        let player_pos = player_transform.translation();
        system.targets.clear();
        system.closest_target = None;

        let mut min_dist = f32::MAX;

        for (entity, target_transform, target) in target_query.iter() {
            if !target.enabled {
                continue;
            }
            let dist = player_pos.distance(target_transform.translation());
            if dist <= system.detection_range.max(target.detection_radius) {
                system.targets.push(entity);
                if dist < min_dist {
                    min_dist = dist;
                    system.closest_target = Some(entity);
                }
            }
        }
    }
}

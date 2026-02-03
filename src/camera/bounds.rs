use bevy::prelude::*;
use crate::camera::types::{CameraController, CameraState, CameraZoneTracker};
use crate::character::Player;

pub struct CameraBoundsPlugin;

impl Plugin for CameraBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraBounds>()
           .add_systems(Update, apply_camera_bounds.after(crate::camera::update_camera_follow));
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum CameraBounds {
    Box {
        min: Vec3,
        max: Vec3,
    },
    Sphere {
        center: Vec3,
        radius: f32,
    },
}

impl Default for CameraBounds {
    fn default() -> Self {
        Self::Box {
            min: Vec3::splat(-100.0),
            max: Vec3::splat(100.0),
        }
    }
}

pub fn apply_camera_bounds(
    mut camera_query: Query<(&mut Transform, &CameraController, &CameraState)>,
    bounds_query: Query<(&CameraBounds, &GlobalTransform)>,
    zone_tracker_query: Query<&CameraZoneTracker, With<Player>>,
) {
    let (mut transform, _controller, _state) = match camera_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };

    let mut bounds_source: Option<(&CameraBounds, &GlobalTransform)> = None;

    if let Some(tracker) = zone_tracker_query.iter().next() {
        if let Some(zone_entity) = tracker.current_zone {
            if let Ok(found) = bounds_query.get(zone_entity) {
                bounds_source = Some(found);
            }
        }
    }

    if bounds_source.is_none() {
        bounds_source = bounds_query.iter().next();
    }

    if let Some((bounds, bounds_gt)) = bounds_source {
        let center = bounds_gt.translation();
        
        match bounds {
            CameraBounds::Box { min, max } => {
                let world_min = center + *min;
                let world_max = center + *max;
                
                transform.translation.x = transform.translation.x.clamp(world_min.x, world_max.x);
                transform.translation.y = transform.translation.y.clamp(world_min.y, world_max.y);
                transform.translation.z = transform.translation.z.clamp(world_min.z, world_max.z);
            }
            CameraBounds::Sphere { center: local_center, radius } => {
                let world_center = center + *local_center;
                let dist = transform.translation.distance(world_center);
                
                if dist > *radius {
                    let dir = (transform.translation - world_center).normalize();
                    transform.translation = world_center + dir * (*radius);
                }
            }
        }
    }
}

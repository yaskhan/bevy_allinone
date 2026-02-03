use bevy::prelude::*;
use avian3d::prelude::{LayerMask, SpatialQuery, SpatialQueryFilter};

#[derive(Debug, Clone)]
pub struct LaserVisionSliceEvent {
    pub entity: Entity,
    pub position: Vec3,
    pub direction: Vec3,
}

#[derive(Resource, Default)]
pub struct LaserVisionSliceEventQueue(pub Vec<LaserVisionSliceEvent>);

#[derive(Resource, Default)]
pub struct LaserVisionToggleEventQueue(pub Vec<(Entity, bool)>);

/// Laser vision ability system.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LaserVisionSystem {
    pub active: bool,
    pub slice_rate: f32,
    pub layer_mask: LayerMask,
    pub raycast_distance: f32,
    pub min_movement_to_slice: f32,

    pub laser_position: Entity,
    pub laser_direction: Entity,

    pub last_time_slice_active: f32,
    pub current_laser_position: Vec3,
    pub previous_laser_position: Vec3,
}

impl Default for LaserVisionSystem {
    fn default() -> Self {
        Self {
            active: false,
            slice_rate: 0.5,
            layer_mask: LayerMask::ALL,
            raycast_distance: 50.0,
            min_movement_to_slice: 0.2,
            laser_position: Entity::PLACEHOLDER,
            laser_direction: Entity::PLACEHOLDER,
            last_time_slice_active: 0.0,
            current_laser_position: Vec3::ZERO,
            previous_laser_position: Vec3::ZERO,
        }
    }
}

pub fn update_laser_vision_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut slice_events: ResMut<LaserVisionSliceEventQueue>,
    mut toggle_events: ResMut<LaserVisionToggleEventQueue>,
    transform_query: Query<&GlobalTransform>,
    mut query: Query<(Entity, &mut LaserVisionSystem, Ref<LaserVisionSystem>)>,
) {
    for (entity, mut system, system_ref) in query.iter_mut() {
        if system_ref.is_changed() {
            toggle_events.0.push((entity, system.active));
            system.last_time_slice_active = 0.0;
            system.previous_laser_position = Vec3::ZERO;
        }

        if !system.active {
            continue;
        }

        let Ok(pos_transform) = transform_query.get(system.laser_position) else { continue };
        let Ok(dir_transform) = transform_query.get(system.laser_direction) else { continue };

        let origin = pos_transform.translation();
        let direction = dir_transform.forward().as_vec3();
        let filter = SpatialQueryFilter::default().with_mask(system.layer_mask);

        if let Some(hit) = spatial_query.cast_ray(origin, direction, system.raycast_distance, true, &filter) {
            system.current_laser_position = hit.point;

            if system.previous_laser_position == Vec3::ZERO {
                system.previous_laser_position = system.current_laser_position;
            }

            if time.elapsed_secs() - system.last_time_slice_active >= system.slice_rate {
                let distance = system.current_laser_position.distance(system.previous_laser_position);
                if distance >= system.min_movement_to_slice {
                    let slice_dir = (system.current_laser_position - system.previous_laser_position).normalize_or_zero();
                    slice_events.0.push(LaserVisionSliceEvent {
                        entity,
                        position: system.current_laser_position,
                        direction: slice_dir,
                    });
                    system.last_time_slice_active = time.elapsed_secs();
                    system.previous_laser_position = system.current_laser_position;
                }
            }
        }
    }
}

use bevy::prelude::*;
use avian3d::prelude::{LayerMask, SpatialQuery, SpatialQueryFilter, LinearVelocity};
use crate::input::InputState;
use super::grappling_hook_targets_system::GrapplingHookTargetsSystem;
use super::ability_info::AbilityInfo;

/// Grappling hook system (core).
///
/// GKC reference: `grapplingHookSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrapplingHookSystem {
    pub ability_name: String,
    pub enabled: bool,
    pub active: bool,

    pub max_raycast_distance: f32,
    pub layer_mask: LayerMask,
    pub min_distance_to_attract: f32,

    pub rotate_player_toward_target: bool,
    pub rotate_player_speed: f32,
    pub min_angle_to_rotate: f32,

    pub regular_movement_speed: f32,
    pub increased_movement_speed: f32,
    pub input_movement_multiplier: f32,
    pub air_control_amount: f32,
    pub use_vertical_movement_on_hook: bool,
    pub ignore_backward_movement_on_hook: bool,
    pub add_vertical_falling_speed: bool,
    pub vertical_falling_speed: f32,

    pub attract_objects_enabled: bool,
    pub regular_attraction_force: f32,
    pub increased_attraction_force: f32,
    pub min_distance_to_stop_attract_object: f32,
    pub add_up_force_for_attraction: bool,
    pub up_force_for_attraction: f32,
    pub add_up_force_for_attraction_duration: f32,

    pub check_if_object_stuck: bool,
    pub time_to_stop_hook_if_stuck: f32,
    pub min_distance_to_check_stuck: f32,

    pub current_target: Option<Vec3>,
    pub current_target_entity: Option<Entity>,
    pub increase_speed_active: bool,
    pub attracting_object_active: bool,

    pub last_time_hook_active: f32,
    pub last_time_object_moving: f32,
    pub last_distance_to_object: f32,
}

impl Default for GrapplingHookSystem {
    fn default() -> Self {
        Self {
            ability_name: "GrapplingHook".to_string(),
            enabled: true,
            active: false,
            max_raycast_distance: 100.0,
            layer_mask: LayerMask::ALL,
            min_distance_to_attract: 0.5,
            rotate_player_toward_target: true,
            rotate_player_speed: 6.0,
            min_angle_to_rotate: 10.0,
            regular_movement_speed: 6.0,
            increased_movement_speed: 10.0,
            input_movement_multiplier: 3.0,
            air_control_amount: 20.0,
            use_vertical_movement_on_hook: false,
            ignore_backward_movement_on_hook: false,
            add_vertical_falling_speed: false,
            vertical_falling_speed: 2.0,
            attract_objects_enabled: true,
            regular_attraction_force: 8.0,
            increased_attraction_force: 12.0,
            min_distance_to_stop_attract_object: 1.0,
            add_up_force_for_attraction: false,
            up_force_for_attraction: 3.0,
            add_up_force_for_attraction_duration: 0.3,
            check_if_object_stuck: true,
            time_to_stop_hook_if_stuck: 2.0,
            min_distance_to_check_stuck: 1.0,
            current_target: None,
            current_target_entity: None,
            increase_speed_active: false,
            attracting_object_active: false,
            last_time_hook_active: 0.0,
            last_time_object_moving: 0.0,
            last_distance_to_object: 0.0,
        }
    }
}

/// Start or stop grappling hook based on ability input.
pub fn handle_grappling_hook_input(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    input: Res<InputState>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    target_query: Query<&GlobalTransform>,
    mut query: Query<(&mut GrapplingHookSystem, &mut AbilityInfo, Option<&GrapplingHookTargetsSystem>)>,
) {
    let Some(camera) = camera_query.iter().next() else { return };
    let cam_pos = camera.translation();
    let cam_forward = camera.forward().as_vec3();

    for (mut hook, mut ability, targets_system) in query.iter_mut() {
        if ability.name != hook.ability_name {
            continue;
        }

        if !hook.enabled {
            continue;
        }

        if input.ability_use_pressed && ability.is_current && !hook.active {
            if let Some(targets) = targets_system {
                if let Some(target_entity) = targets.closest_target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        hook.current_target = Some(target_transform.translation());
                        hook.current_target_entity = Some(target_entity);
                        hook.active = true;
                        hook.attracting_object_active = false;
                        hook.last_time_hook_active = time.elapsed_secs();
                        hook.last_time_object_moving = 0.0;
                        ability.active = true;
                        continue;
                    }
                }
            }

            let filter = SpatialQueryFilter::default().with_mask(hook.layer_mask);
            if let Some(hit) = spatial_query.cast_ray(
                cam_pos,
                cam_forward,
                hook.max_raycast_distance,
                true,
                &filter,
            ) {
                hook.current_target = Some(hit.point);
                hook.current_target_entity = Some(hit.entity);
                hook.active = true;
                hook.attracting_object_active = false;
                hook.last_time_hook_active = time.elapsed_secs();
                hook.last_time_object_moving = 0.0;
                ability.active = true;
            }
        }

        if input.ability_use_released && hook.active && ability.is_current {
            hook.active = false;
            hook.current_target = None;
            hook.current_target_entity = None;
            hook.increase_speed_active = false;
            hook.attracting_object_active = false;
            ability.deactivate();
        }

        hook.increase_speed_active = input.sprint_pressed && hook.active;
    }
}

/// Apply hook forces to player or target object.
pub fn update_grappling_hook_forces(
    time: Res<Time>,
    input: Res<InputState>,
    mut player_query: Query<(Entity, &mut GrapplingHookSystem, &GlobalTransform, &mut LinearVelocity)>,
) {
    for (entity, mut hook, player_transform, mut velocity) in player_query.iter_mut() {
        if !hook.active {
            continue;
        }

        let Some(target_pos) = hook.current_target else { continue };
        let player_pos = player_transform.translation();
        let mut to_target = target_pos - player_pos;
        let distance = to_target.length();

        if distance <= hook.min_distance_to_attract {
            hook.active = false;
            hook.current_target = None;
            hook.current_target_entity = None;
            continue;
        }

        let movement_speed = if hook.increase_speed_active {
            hook.increased_movement_speed
        } else {
            hook.regular_movement_speed
        };

        to_target = to_target.normalize_or_zero();
        let mut force = to_target * movement_speed;

        let mut movement_dir = Vec3::ZERO;
        if hook.use_vertical_movement_on_hook {
            movement_dir += Vec3::Y * input.movement.y;
        } else {
            let mut forward_input = input.movement.y;
            if hook.ignore_backward_movement_on_hook {
                forward_input = forward_input.clamp(0.0, 1.0);
            }
            movement_dir += to_target * forward_input;
        }

        let right = player_transform.right().as_vec3();
        movement_dir += right * input.movement.x;
        force += movement_dir * hook.input_movement_multiplier;

        if hook.add_vertical_falling_speed {
            force -= Vec3::Y * hook.vertical_falling_speed;
        }

        velocity.0 = force;

        if hook.rotate_player_toward_target {
            let flat_dir = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();
            if flat_dir.length_squared() > 0.001 {
                let angle = player_transform.forward().angle_between(flat_dir).to_degrees();
                if angle >= hook.min_angle_to_rotate {
                    let target_rot = Quat::from_rotation_arc(Vec3::Z, flat_dir);
                    let mut transform = player_transform.compute_transform();
                    transform.rotation = transform.rotation.slerp(target_rot, time.delta_secs() * hook.rotate_player_speed);
                }
            }
        }

        if hook.check_if_object_stuck {
            if hook.last_time_object_moving == 0.0 {
                hook.last_distance_to_object = distance;
                hook.last_time_object_moving = time.elapsed_secs();
            } else if time.elapsed_secs() - hook.last_time_object_moving > hook.time_to_stop_hook_if_stuck {
                hook.last_time_object_moving = time.elapsed_secs();
                if distance + hook.min_distance_to_check_stuck >= hook.last_distance_to_object {
                    hook.active = false;
                }
                hook.last_distance_to_object = distance;
            }
        }
    }
}

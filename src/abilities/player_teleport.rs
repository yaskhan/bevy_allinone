use bevy::prelude::*;
use avian3d::prelude::{LayerMask, SpatialQuery, SpatialQueryFilter};
use super::ability_info::AbilityInfo;
use super::player_abilities::PlayerAbilitiesSystem;
use crate::input::InputState;

/// Teleport ability controller.
///
/// GKC reference: `Custom Abilities/playerTeleportSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerTeleportAbility {
    pub ability_name: String,
    pub teleporting_enabled: bool,
    pub teleport_layer_mask: LayerMask,

    pub max_distance_to_teleport: f32,
    pub use_teleport_if_surface_not_found: bool,
    pub max_distance_to_teleport_air: f32,

    pub hold_button_time_to_activate: f32,
    pub stop_teleport_if_moving: bool,

    pub teleport_speed: f32,
    pub rotate_toward_teleport_position: bool,
    pub teleport_instantly: bool,

    pub searching_for_teleport: bool,
    pub teleport_can_be_executed: bool,
    pub teleport_surface_found: bool,
    pub teleport_in_process: bool,

    pub last_time_button_pressed: f32,
    pub current_teleport_position: Vec3,
    pub current_teleport_normal: Vec3,
}

impl Default for PlayerTeleportAbility {
    fn default() -> Self {
        Self {
            ability_name: "Teleport".to_string(),
            teleporting_enabled: true,
            teleport_layer_mask: LayerMask::ALL,
            max_distance_to_teleport: 100.0,
            use_teleport_if_surface_not_found: true,
            max_distance_to_teleport_air: 10.0,
            hold_button_time_to_activate: 0.4,
            stop_teleport_if_moving: false,
            teleport_speed: 10.0,
            rotate_toward_teleport_position: true,
            teleport_instantly: true,
            searching_for_teleport: false,
            teleport_can_be_executed: false,
            teleport_surface_found: false,
            teleport_in_process: false,
            last_time_button_pressed: 0.0,
            current_teleport_position: Vec3::ZERO,
            current_teleport_normal: Vec3::Y,
        }
    }
}

/// Update teleport target while searching.
pub fn update_teleport_target(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    mut query: Query<&mut PlayerTeleportAbility>,
) {
    let Some(camera) = camera_query.iter().next() else { return };
    let cam_pos = camera.translation();
    let cam_forward = camera.forward().as_vec3();

    for mut teleport in query.iter_mut() {
        if !teleport.searching_for_teleport {
            continue;
        }

        if time.elapsed_secs() - teleport.last_time_button_pressed >= teleport.hold_button_time_to_activate {
            teleport.teleport_can_be_executed = true;
        }

        if !teleport.teleport_can_be_executed {
            continue;
        }

        let filter = SpatialQueryFilter::default().with_mask(teleport.teleport_layer_mask);
        if let Some(hit) = spatial_query.cast_ray(
            cam_pos,
            cam_forward,
            teleport.max_distance_to_teleport,
            true,
            &filter,
        ) {
            teleport.current_teleport_position = hit.point + hit.normal * 0.4;
            teleport.current_teleport_normal = hit.normal;
            teleport.teleport_surface_found = true;
        } else {
            teleport.current_teleport_position = cam_pos + cam_forward * teleport.max_distance_to_teleport_air;
            teleport.current_teleport_normal = Vec3::Y;
            teleport.teleport_surface_found = false;
        }
    }
}

/// Handle teleport input and execution.
pub fn handle_teleport_input(
    time: Res<Time>,
    input: Res<InputState>,
    mut query: Query<(&mut Transform, &mut PlayerTeleportAbility, &mut AbilityInfo, &PlayerAbilitiesSystem)>,
) {
    for (mut transform, mut teleport, mut ability, system) in query.iter_mut() {
        if ability.name != teleport.ability_name {
            continue;
        }

        if !teleport.teleporting_enabled || !system.enabled || !system.abilities_mode_active {
            continue;
        }

        if teleport.stop_teleport_if_moving && input.movement.length_squared() > 0.01 {
            teleport.searching_for_teleport = false;
            teleport.teleport_can_be_executed = false;
            teleport.teleport_in_process = false;
            continue;
        }

        if input.ability_use_pressed && ability.is_current {
            teleport.searching_for_teleport = true;
            teleport.last_time_button_pressed = time.elapsed_secs();
        }

        if input.ability_use_released && ability.is_current {
            if teleport.teleport_can_be_executed {
                if teleport.use_teleport_if_surface_not_found || teleport.teleport_surface_found {
                    if teleport.teleport_instantly {
                        transform.translation = teleport.current_teleport_position;
                        teleport.teleport_in_process = false;
                    } else {
                        teleport.teleport_in_process = true;
                    }
                }
            }

            teleport.teleport_can_be_executed = false;
            teleport.searching_for_teleport = false;
            ability.deactivate();
        }

        if teleport.teleport_in_process && !teleport.teleport_instantly {
            let to_target = teleport.current_teleport_position - transform.translation;
            let distance = to_target.length();
            if distance <= 0.05 {
                transform.translation = teleport.current_teleport_position;
                teleport.teleport_in_process = false;
            } else {
                let dir = to_target / distance;
                transform.translation += dir * teleport.teleport_speed * time.delta_secs();

                if teleport.rotate_toward_teleport_position {
                    let flat_dir = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                    if flat_dir.length_squared() > 0.001 {
                        let target_rot = Quat::from_rotation_arc(Vec3::Z, flat_dir);
                        transform.rotation = transform.rotation.slerp(target_rot, time.delta_secs() * 6.0);
                    }
                }
            }
        }
    }
}

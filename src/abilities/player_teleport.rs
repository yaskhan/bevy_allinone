use bevy::prelude::*;
use avian3d::prelude::{LayerMask, SpatialQuery, SpatialQueryFilter};
use super::ability_info::AbilityInfo;
use super::player_abilities::PlayerAbilitiesSystem;
use crate::camera::CameraState;
use crate::character::CharacterController;
use crate::input::InputState;

#[derive(Debug, Clone)]
pub struct TeleportStartEvent {
    pub entity: Entity,
    pub position: Vec3,
}

#[derive(Debug, Clone)]
pub struct TeleportEndEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct TeleportStartEventQueue(pub Vec<TeleportStartEvent>);

#[derive(Resource, Default)]
pub struct TeleportEndEventQueue(pub Vec<TeleportEndEvent>);

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

    pub can_teleport_on_zero_gravity: bool,

    pub teleport_speed: f32,
    pub rotate_toward_teleport_position: bool,
    pub min_angle_to_rotate: f32,
    pub teleport_instantly: bool,

    pub use_teleport_mark: bool,
    pub teleport_mark: Option<Entity>,

    pub change_camera_fov_on_teleport: bool,
    pub camera_fov_on_teleport: f32,
    pub camera_fov_on_teleport_speed: f32,

    pub searching_for_teleport: bool,
    pub teleport_can_be_executed: bool,
    pub teleport_surface_found: bool,
    pub teleport_in_process: bool,

    pub last_time_button_pressed: f32,
    pub current_teleport_position: Vec3,
    pub current_teleport_normal: Vec3,

    pub cached_fov_override: Option<f32>,
    pub cached_fov_speed: Option<f32>,
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
            can_teleport_on_zero_gravity: false,
            teleport_speed: 10.0,
            rotate_toward_teleport_position: true,
            min_angle_to_rotate: 15.0,
            teleport_instantly: true,
            use_teleport_mark: false,
            teleport_mark: None,
            change_camera_fov_on_teleport: false,
            camera_fov_on_teleport: 40.0,
            camera_fov_on_teleport_speed: 8.0,
            searching_for_teleport: false,
            teleport_can_be_executed: false,
            teleport_surface_found: false,
            teleport_in_process: false,
            last_time_button_pressed: 0.0,
            current_teleport_position: Vec3::ZERO,
            current_teleport_normal: Vec3::Y,
            cached_fov_override: None,
            cached_fov_speed: None,
        }
    }
}

/// Update teleport target while searching.
pub fn update_teleport_target(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    mut query: Query<&mut PlayerTeleportAbility>,
    mut mark_query: Query<(&mut Transform, &mut Visibility)>,
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

        if teleport.use_teleport_mark {
            if let Some(mark_entity) = teleport.teleport_mark {
                if let Ok((mut mark_transform, mut visibility)) = mark_query.get_mut(mark_entity) {
                    if teleport.use_teleport_if_surface_not_found || teleport.teleport_surface_found {
                        mark_transform.translation = teleport.current_teleport_position;
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

/// Handle teleport input and execution.
pub fn handle_teleport_input(
    time: Res<Time>,
    input: Res<InputState>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut PlayerTeleportAbility,
        &mut AbilityInfo,
        &PlayerAbilitiesSystem,
        Option<&CharacterController>,
    )>,
    mut camera_query: Query<&mut CameraState>,
    mut mark_query: Query<(&mut Transform, &mut Visibility)>,
    mut start_events: ResMut<TeleportStartEventQueue>,
    mut end_events: ResMut<TeleportEndEventQueue>,
) {
    for (entity, mut transform, mut teleport, mut ability, system, controller) in query.iter_mut() {
        if ability.name != teleport.ability_name {
            continue;
        }

        if !teleport.teleporting_enabled || !system.enabled || !system.abilities_mode_active {
            continue;
        }

        if let Some(controller) = controller {
            if controller.zero_gravity_mode && !teleport.can_teleport_on_zero_gravity {
                continue;
            }
        }

        if teleport.stop_teleport_if_moving && input.movement.length_squared() > 0.01 {
            teleport.searching_for_teleport = false;
            teleport.teleport_can_be_executed = false;
            if teleport.teleport_in_process {
                teleport.teleport_in_process = false;
                if teleport.change_camera_fov_on_teleport {
                    if let Ok(mut cam_state) = camera_query.get_single_mut() {
                        cam_state.fov_override = teleport.cached_fov_override;
                        cam_state.fov_override_speed = teleport.cached_fov_speed;
                    }
                }
                end_events.0.push(TeleportEndEvent { entity });
            }
            continue;
        }

        if input.ability_use_pressed && ability.is_current {
            teleport.searching_for_teleport = true;
            teleport.last_time_button_pressed = time.elapsed_secs();
        }

        if input.ability_use_released && ability.is_current {
            if teleport.teleport_can_be_executed {
                if teleport.use_teleport_if_surface_not_found || teleport.teleport_surface_found {
                    if teleport.change_camera_fov_on_teleport {
                        if let Ok(mut cam_state) = camera_query.get_single_mut() {
                            teleport.cached_fov_override = cam_state.fov_override;
                            teleport.cached_fov_speed = cam_state.fov_override_speed;
                            cam_state.fov_override = Some(teleport.camera_fov_on_teleport);
                            cam_state.fov_override_speed = Some(teleport.camera_fov_on_teleport_speed);
                        }
                    }

                    start_events.0.push(TeleportStartEvent {
                        entity,
                        position: teleport.current_teleport_position,
                    });

                    if teleport.teleport_instantly {
                        transform.translation = teleport.current_teleport_position;
                        teleport.teleport_in_process = false;
                        if teleport.change_camera_fov_on_teleport {
                            if let Ok(mut cam_state) = camera_query.get_single_mut() {
                                cam_state.fov_override = teleport.cached_fov_override;
                                cam_state.fov_override_speed = teleport.cached_fov_speed;
                            }
                        }
                        end_events.0.push(TeleportEndEvent { entity });
                    } else {
                        teleport.teleport_in_process = true;
                    }
                }
            }

            teleport.teleport_can_be_executed = false;
            teleport.searching_for_teleport = false;
            ability.deactivate();

            if teleport.use_teleport_mark {
                if let Some(mark_entity) = teleport.teleport_mark {
                    if let Ok((_, mut visibility)) = mark_query.get_mut(mark_entity) {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }

        if teleport.teleport_in_process && !teleport.teleport_instantly {
            let to_target = teleport.current_teleport_position - transform.translation;
            let distance = to_target.length();
            if distance <= 0.05 {
                transform.translation = teleport.current_teleport_position;
                teleport.teleport_in_process = false;
                if teleport.change_camera_fov_on_teleport {
                    if let Ok(mut cam_state) = camera_query.get_single_mut() {
                        cam_state.fov_override = teleport.cached_fov_override;
                        cam_state.fov_override_speed = teleport.cached_fov_speed;
                    }
                }
                end_events.0.push(TeleportEndEvent { entity });
            } else {
                let dir = to_target / distance;
                transform.translation += dir * teleport.teleport_speed * time.delta_secs();

                if teleport.rotate_toward_teleport_position {
                    let flat_dir = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                    if flat_dir.length_squared() > 0.001 {
                        let target_rot = Quat::from_rotation_arc(Vec3::Z, flat_dir);
                        let angle = transform.forward().angle_between(flat_dir).to_degrees();
                        if angle >= teleport.min_angle_to_rotate {
                            transform.rotation = transform.rotation.slerp(target_rot, time.delta_secs() * 6.0);
                        }
                    }
                }
            }
        }
    }
}

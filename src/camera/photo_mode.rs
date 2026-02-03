use bevy::prelude::*;

use crate::input::InputState;
use super::captures::ScreenshotEventQueue;
use super::types::CameraController;

#[derive(Resource, Debug, Clone)]
pub struct PhotoModeSettings {
    pub enabled: bool,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub smooth_vertical_speed: f32,
    pub smooth_horizontal_speed: f32,
    pub clamp_tilt: Vec2,
    pub roll_speed: f32,
    pub clamp_camera_distance: bool,
    pub max_camera_radius: f32,
}

impl Default for PhotoModeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            movement_speed: 8.0,
            rotation_speed: 0.2,
            smooth_vertical_speed: 8.0,
            smooth_horizontal_speed: 8.0,
            clamp_tilt: Vec2::new(70.0, 70.0),
            roll_speed: 1.0,
            clamp_camera_distance: true,
            max_camera_radius: 15.0,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct PhotoModeState {
    pub active: bool,
    pub rotation_active: bool,
    pub look_angle: Vec2,
    pub roll_angle: f32,
    pub last_camera_pos: Vec3,
    pub stored_enabled: bool,
}

pub fn handle_photo_mode_toggle(
    input: Res<InputState>,
    settings: Res<PhotoModeSettings>,
    mut state: ResMut<PhotoModeState>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
) {
    if !settings.enabled {
        return;
    }

    if !input.reset_camera_pressed {
        return;
    }

    let Ok((mut transform, mut controller)) = camera_query.get_single_mut() else {
        return;
    };

    state.active = !state.active;

    if state.active {
        state.stored_enabled = controller.enabled;
        controller.enabled = false;
        state.last_camera_pos = transform.translation;
        state.look_angle = Vec2::ZERO;
        state.roll_angle = 0.0;
        state.rotation_active = true;
    } else {
        controller.enabled = state.stored_enabled;
    }
}

pub fn update_photo_mode(
    time: Res<Time>,
    input: Res<InputState>,
    settings: Res<PhotoModeSettings>,
    mut state: ResMut<PhotoModeState>,
    mut camera_query: Query<&mut Transform, With<CameraController>>,
    mut screenshot_queue: ResMut<ScreenshotEventQueue>,
) {
    if !state.active || !settings.enabled {
        return;
    }

    let Ok(mut transform) = camera_query.get_single_mut() else {
        return;
    };

    if input.interact_pressed {
        screenshot_queue.0.push(super::captures::TakeScreenshotEvent {
            path: None,
            metadata: None,
        });
    }

    let dt = time.delta_secs();

    if state.rotation_active {
        state.look_angle.x += input.look.x * settings.rotation_speed;
        state.look_angle.y -= input.look.y * settings.rotation_speed;
    }

    state.look_angle.y = state
        .look_angle
        .y
        .clamp(-settings.clamp_tilt.x, settings.clamp_tilt.y);

    if input.lean_right {
        state.roll_angle -= settings.roll_speed;
    }
    if input.lean_left {
        state.roll_angle += settings.roll_speed;
    }

    let target_pitch = Quat::from_rotation_x(state.look_angle.y.to_radians());
    let target_yaw = Quat::from_rotation_y(state.look_angle.x.to_radians());
    let target_roll = Quat::from_rotation_z(state.roll_angle.to_radians());

    let target_rot = target_yaw * target_pitch * target_roll;
    let rot_alpha = 1.0 - (-settings.smooth_horizontal_speed * dt).exp();
    transform.rotation = transform.rotation.slerp(target_rot, rot_alpha);

    let mut move_input = Vec3::ZERO;
    let move_axis = input.get_movement_axis();
    move_input += transform.forward() * move_axis.y;
    move_input += transform.right() * move_axis.x;

    if input.zoom_in_pressed {
        move_input += Vec3::Y;
    }
    if input.zoom_out_pressed {
        move_input -= Vec3::Y;
    }

    transform.translation += move_input * settings.movement_speed * dt;

    if settings.clamp_camera_distance {
        let distance = transform.translation.distance(state.last_camera_pos);
        if distance > settings.max_camera_radius {
            let dir = (transform.translation - state.last_camera_pos).normalize_or_zero();
            transform.translation = state.last_camera_pos + dir * settings.max_camera_radius;
        }
    }
}

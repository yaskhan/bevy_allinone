use bevy::prelude::*;
use crate::input::{InputState, InputAction};
use crate::camera::types::CameraController;

pub struct PhotoModePlugin;

impl Plugin for PhotoModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhotoModeState>()
           .register_type::<PhotoModeSettings>()
           .add_systems(Update, update_photo_mode);
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PhotoModeState {
    pub active: bool,
    pub original_time_scale: f32,
    pub camera_position: Vec3,
    pub camera_rotation: Quat,
}

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct PhotoModeSettings {
    pub enabled: bool,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub vertical_speed: f32,
    pub time_scale_on_active: f32,
    pub freeze_time: bool,
}

impl Default for PhotoModeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            movement_speed: 10.0,
            rotation_speed: 0.1,
            vertical_speed: 5.0,
            time_scale_on_active: 0.0,
            freeze_time: true,
        }
    }
}

pub fn update_photo_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    input_state: Res<InputState>,
    mut photo_state: ResMut<PhotoModeState>,
    mut time: ResMut<Time<Virtual>>,
    mut camera_query: Query<(&mut Transform, &CameraController)>,
    settings_query: Query<&PhotoModeSettings>,
) {
    // Toggle logic (e.g., P key or specific interaction)
    if keyboard.just_pressed(KeyCode::KeyP) {
        photo_state.active = !photo_state.active;
        
        if photo_state.active {
            photo_state.original_time_scale = time.relative_speed();
            // Set time scale (e.g., freeze)
            time.set_relative_speed(0.0);
        } else {
            time.set_relative_speed(photo_state.original_time_scale);
        }
    }

    if !photo_state.active {
        return;
    }

    let (mut transform, _) = match camera_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };

    let settings = settings_query.iter().next().cloned().unwrap_or_default();
    if !settings.enabled { return; }

    let dt = 0.016; // Use a fixed DT when time is frozen for smooth flight

    // 1. Rotation (Mouse)
    let mouse_delta = input_state.look;
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
    
    yaw -= mouse_delta.x * settings.rotation_speed * 0.1;
    pitch -= mouse_delta.y * settings.rotation_speed * 0.1;
    pitch = pitch.clamp(-1.5, 1.5);
    
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    // 2. Movement (WASD + Q/E)
    let mut move_vec = Vec3::ZERO;
    let forward = transform.forward();
    let right = transform.right();
    
    if keyboard.pressed(KeyCode::KeyW) { move_vec += *forward; }
    if keyboard.pressed(KeyCode::KeyS) { move_vec -= *forward; }
    if keyboard.pressed(KeyCode::KeyA) { move_vec -= *right; }
    if keyboard.pressed(KeyCode::KeyD) { move_vec += *right; }
    
    if keyboard.pressed(KeyCode::KeyE) { move_vec += Vec3::Y; }
    if keyboard.pressed(KeyCode::KeyQ) { move_vec -= Vec3::Y; }

    transform.translation += move_vec.normalize_or_zero() * settings.movement_speed * dt;
}

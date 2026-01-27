//! Input system module
//!
//! Flexible input handling for keyboard, mouse, gamepad, and touch.

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .add_systems(Update, (
                update_input_state,
                process_movement_input,
                process_action_input,
            ));
    }
}

/// Global input state resource
#[derive(Resource, Debug, Default)]
pub struct InputState {
    pub movement: Vec2,
    pub look: Vec2,
    pub jump_pressed: bool,
    pub crouch_pressed: bool,
    pub sprint_pressed: bool,
    pub interact_pressed: bool,
    pub aim_pressed: bool,
    
    // Leaning
    pub lean_left: bool,
    pub lean_right: bool,
    
    // Lock on
    pub lock_on_pressed: bool,
}

/// Input configuration
#[derive(Resource, Debug)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            gamepad_sensitivity: 1.0,
            invert_y_axis: false,
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Update input state from devices
fn update_input_state(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut input_state: ResMut<InputState>,
) {
    // Read keyboard movement
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { movement.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { movement.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { movement.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { movement.x += 1.0; }
    
    input_state.movement = movement.normalize_or_zero();
    input_state.jump_pressed = keyboard.just_pressed(KeyCode::Space);
    input_state.crouch_pressed = keyboard.pressed(KeyCode::ControlLeft);
    input_state.sprint_pressed = keyboard.pressed(KeyCode::ShiftLeft);
    input_state.interact_pressed = keyboard.just_pressed(KeyCode::KeyE);
    input_state.aim_pressed = mouse.pressed(MouseButton::Right);

    // Leaning
    input_state.lean_left = keyboard.pressed(KeyCode::KeyQ);
    input_state.lean_right = keyboard.pressed(KeyCode::KeyE);
    
    // Lock on
    input_state.lock_on_pressed = mouse.just_pressed(MouseButton::Middle) || keyboard.just_pressed(KeyCode::KeyR);

    // Read mouse motion for looking
    let mut look = Vec2::ZERO;
    for event in mouse_motion.read() {
        look += event.delta;
    }
    input_state.look = look;
}

/// Process movement input
fn process_movement_input(
    input_state: Res<InputState>,
) {
    let _movement = input_state.movement;
    let _look = input_state.look;
}

/// Process action input
fn process_action_input(
    _input_state: Res<InputState>,
) {
}

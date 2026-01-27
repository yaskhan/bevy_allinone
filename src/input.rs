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
///
/// TODO: Implement input mapping system
#[derive(Resource, Debug, Default)]
pub struct InputState {
    pub movement: Vec2,
    pub look: Vec2,
    pub jump_pressed: bool,
    pub crouch_pressed: bool,
    pub sprint_pressed: bool,
    pub interact_pressed: bool,
    pub aim_pressed: bool,
    
    // TODO: Add more input actions
    // TODO: Add input mapping configuration
    // TODO: Add gamepad support
    // TODO: Add touch support
}

/// Input action mapping
///
/// TODO: Implement input action mapping
#[derive(Debug, Clone)]
pub struct InputAction {
    pub name: String,
    pub key_binding: Vec<KeyCode>,
    pub gamepad_binding: Vec<GamepadButtonType>,
    pub mouse_binding: Option<MouseButton>,
}

/// Input configuration
///
/// TODO: Implement input customization
#[derive(Resource, Debug)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
    pub actions: Vec<InputAction>,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            gamepad_sensitivity: 1.0,
            invert_y_axis: false,
            actions: Vec::new(),
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Update input state from devices
///
/// TODO: Implement input reading
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

    // Read mouse motion for looking
    let mut look = Vec2::ZERO;
    for event in mouse_motion.read() {
        look += event.delta;
    }
    input_state.look = look;
}

/// Process movement input
///
/// TODO: Integrate with character controller
fn process_movement_input(
    input_state: Res<InputState>,
) {
    let _movement = input_state.movement;
    let _look = input_state.look;
    
    // TODO: Send input to character controller
    // TODO: Handle input smoothing
    // TODO: Handle input deadzone
}

/// Process action input
///
/// TODO: Implement action handling
fn process_action_input(
    input_state: Res<InputState>,
) {
    // TODO: Handle jump
    // TODO: Handle crouch
    // TODO: Handle sprint
    // TODO: Handle interact
    // TODO: Handle aim
    // TODO: Handle fire
}

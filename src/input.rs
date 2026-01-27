//! Input system module
//!
//! Flexible input handling with action remapping support.

use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .init_resource::<InputMap>()
            .init_resource::<RebindState>()
            .add_systems(Update, (
                update_input_state,
                handle_rebinding, // New system
                process_movement_input,
                process_action_input,
            ));
    }
}

/// Resource to track if we are currently waiting for a key to rebind an action
#[derive(Resource, Debug, Default)]
pub struct RebindState {
    pub action: Option<InputAction>,
}

/// Logical game actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum InputAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Sprint,
    Crouch,
    Interact,
    Aim,
    LeanLeft,
    LeanRight,
    LockOn,
}

/// Input binding types
#[derive(Debug, Clone, Reflect)]
pub enum InputBinding {
    Key(KeyCode),
    Mouse(MouseButton),
}

/// Mapping from actions to multiple potential bindings
#[derive(Resource, Debug, Reflect)]
pub struct InputMap {
    pub bindings: HashMap<InputAction, Vec<InputBinding>>,
}

impl Default for InputMap {
    fn default() -> Self {
        let mut bindings = HashMap::default();
        bindings.insert(InputAction::MoveForward, vec![InputBinding::Key(KeyCode::KeyW)]);
        bindings.insert(InputAction::MoveBackward, vec![InputBinding::Key(KeyCode::KeyS)]);
        bindings.insert(InputAction::MoveLeft, vec![InputBinding::Key(KeyCode::KeyA)]);
        bindings.insert(InputAction::MoveRight, vec![InputBinding::Key(KeyCode::KeyD)]);
        bindings.insert(InputAction::Jump, vec![InputBinding::Key(KeyCode::Space)]);
        bindings.insert(InputAction::Sprint, vec![InputBinding::Key(KeyCode::ShiftLeft)]);
        bindings.insert(InputAction::Crouch, vec![InputBinding::Key(KeyCode::ControlLeft)]);
        bindings.insert(InputAction::Interact, vec![InputBinding::Key(KeyCode::KeyE)]);
        bindings.insert(InputAction::Aim, vec![InputBinding::Mouse(MouseButton::Right)]);
        bindings.insert(InputAction::LeanLeft, vec![InputBinding::Key(KeyCode::KeyQ)]);
        bindings.insert(InputAction::LeanRight, vec![InputBinding::Key(KeyCode::KeyE)]);
        bindings.insert(InputAction::LockOn, vec![InputBinding::Mouse(MouseButton::Middle), InputBinding::Key(KeyCode::KeyR)]);
        Self { bindings }
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
    pub lean_left: bool,
    pub lean_right: bool,
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

/// Update input state from devices based on current InputMap
fn update_input_state(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    input_map: Res<InputMap>,
    mut input_state: ResMut<InputState>,
) {
    let check_action = |action: InputAction| -> bool {
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.pressed(*code),
                InputBinding::Mouse(button) => mouse_buttons.pressed(*button),
            })
        } else {
            false
        }
    };

    let check_action_just_pressed = |action: InputAction| -> bool {
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.just_pressed(*code),
                InputBinding::Mouse(button) => mouse_buttons.just_pressed(*button),
            })
        } else {
            false
        }
    };

    // Movement
    let mut movement = Vec2::ZERO;
    if check_action(InputAction::MoveForward) { movement.y += 1.0; }
    if check_action(InputAction::MoveBackward) { movement.y -= 1.0; }
    if check_action(InputAction::MoveLeft) { movement.x -= 1.0; }
    if check_action(InputAction::MoveRight) { movement.x += 1.0; }
    
    input_state.movement = movement.normalize_or_zero();
    
    // Actions
    input_state.jump_pressed = check_action_just_pressed(InputAction::Jump);
    input_state.crouch_pressed = check_action(InputAction::Crouch);
    input_state.sprint_pressed = check_action(InputAction::Sprint);
    input_state.interact_pressed = check_action_just_pressed(InputAction::Interact);
    input_state.aim_pressed = check_action(InputAction::Aim);
    input_state.lean_left = check_action(InputAction::LeanLeft);
    input_state.lean_right = check_action(InputAction::LeanRight);
    input_state.lock_on_pressed = check_action_just_pressed(InputAction::LockOn);

    // Look motion
    let mut look = Vec2::ZERO;
    for event in mouse_motion.read() {
        look += event.delta;
    }
    input_state.look = look;
}

/// System to handle runtime remapping of actions
fn handle_rebinding(
    mut rebind_state: ResMut<RebindState>,
    mut input_map: ResMut<InputMap>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(action) = rebind_state.action else { return };

    // Find the first key or mouse button just pressed
    let mut new_binding = None;
    if let Some(key) = keyboard.get_just_pressed().next() {
        new_binding = Some(InputBinding::Key(*key));
    } else if let Some(button) = mouse_buttons.get_just_pressed().next() {
        new_binding = Some(InputBinding::Mouse(*button));
    }

    if let Some(binding) = new_binding {
        // Update the map (overwrites existing bindings for this action)
        input_map.bindings.insert(action, vec![binding]);
        rebind_state.action = None;
    }
}

/// Process movement input
fn process_movement_input(
    _input_state: Res<InputState>,
) {
}

/// Process action input
fn process_action_input(
    _input_state: Res<InputState>,
) {
}

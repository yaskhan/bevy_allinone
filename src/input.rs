//! Input system module
//!
//! Flexible input handling with action remapping and buffering support.

use bevy::prelude::*;
use std::collections::HashMap;
// use bevy::ecs::event::EventReader;
use bevy::input::mouse::MouseMotion;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .init_resource::<InputMap>()
            .init_resource::<RebindState>()
            .init_resource::<InputBuffer>()
            .init_resource::<InputConfig>()
            .add_systems(Update, (
                update_input_state,
                handle_rebinding,
                cleanup_input_buffer,
                player_input_sync_system, // Moved here as per instruction to "fix syncing"
            ).chain())
            .add_systems(Update, (
                process_movement_input,
                process_action_input,
                player_input_sync_system,
            ));
    }
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
    Result, // 'R' key for LockOn or Reload? 
    // LockOn was defined below. Let's add Attack.
    LockOn,
    Attack,
    Block,
    SwitchCameraMode,
    Fire,
    Reload,
    NextWeapon,
    PrevWeapon,
    ToggleInventory,
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
        bindings.insert(InputAction::Attack, vec![InputBinding::Mouse(MouseButton::Left)]);
        bindings.insert(InputAction::Block, vec![InputBinding::Mouse(MouseButton::Right)]);
        bindings.insert(InputAction::SwitchCameraMode, vec![InputBinding::Key(KeyCode::KeyC)]);
        bindings.insert(InputAction::Fire, vec![InputBinding::Mouse(MouseButton::Left)]);
        bindings.insert(InputAction::Reload, vec![InputBinding::Key(KeyCode::KeyR)]);
        bindings.insert(InputAction::NextWeapon, vec![InputBinding::Key(KeyCode::Digit1)]); // Placeholder
        bindings.insert(InputAction::PrevWeapon, vec![InputBinding::Key(KeyCode::Digit2)]); // Placeholder
        bindings.insert(InputAction::ToggleInventory, vec![InputBinding::Key(KeyCode::KeyI), InputBinding::Key(KeyCode::Tab)]);
        Self { bindings }
    }
}

/// A buffered action that was recently pressed
#[derive(Debug, Clone)]
pub struct BufferedAction {
    pub action: InputAction,
    pub timestamp: f32,
}

/// Buffer for storing recently pressed actions
#[derive(Resource, Debug, Default)]
pub struct InputBuffer {
    pub actions: Vec<BufferedAction>,
}

impl InputBuffer {
    /// Check if an action is in the buffer and consume it if found
    pub fn consume(&mut self, action: InputAction) -> bool {
        if let Some(index) = self.actions.iter().position(|ba| ba.action == action) {
            self.actions.remove(index);
            return true;
        }
        false
    }
    
    /// Check if an action is in the buffer without consuming it
    pub fn is_buffered(&self, action: InputAction) -> bool {
        self.actions.iter().any(|ba| ba.action == action)
    }
}

/// Global input state resource and per-entity input component
#[derive(Component, Resource, Debug, Default, Reflect, Clone)]
#[reflect(Component, Resource)]
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
    pub attack_pressed: bool,
    pub block_pressed: bool,
    pub switch_camera_mode_pressed: bool,
    pub fire_pressed: bool,
    pub reload_pressed: bool,
    pub next_weapon_pressed: bool,
    pub prev_weapon_pressed: bool,
    pub toggle_inventory_pressed: bool,
}

/// Input configuration
#[derive(Resource, Debug)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
    pub buffer_ttl: f32, // Time to live for buffered inputs
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.15,
            gamepad_sensitivity: 1.0,
            invert_y_axis: false,
            buffer_ttl: 0.15, // 150ms buffer
        }
    }
}

/// Resource to track if we are currently waiting for a key to rebind an action
#[derive(Resource, Debug, Default)]
pub struct RebindState {
    pub action: Option<InputAction>,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Update input state from devices based on current InputMap
fn update_input_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // mut mouse_motion: EventReader<MouseMotion>,
    input_map: Res<InputMap>,
    mut input_state: ResMut<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let check_action = |action: InputAction| -> bool {
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.pressed(code.clone()),
                InputBinding::Mouse(button) => mouse_buttons.pressed(button.clone()),
            })
        } else {
            false
        }
    };

    let check_action_just_pressed = |action: InputAction| -> bool {
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.just_pressed(code.clone()),
                InputBinding::Mouse(button) => mouse_buttons.just_pressed(button.clone()),
            })
        } else {
            false
        }
    };

    // Buffer actions that were just pressed
    let actions_to_buffer = [
        InputAction::Jump,
        InputAction::Interact,
        InputAction::LockOn,
    ];

    for action in actions_to_buffer {
        if check_action_just_pressed(action) {
            input_buffer.actions.push(BufferedAction {
                action,
                timestamp: time.elapsed_secs(),
            });
        }
    }

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
    input_state.attack_pressed = check_action_just_pressed(InputAction::Attack);
    input_state.block_pressed = check_action(InputAction::Block);
    input_state.switch_camera_mode_pressed = check_action_just_pressed(InputAction::SwitchCameraMode);
    
    input_state.fire_pressed = check_action(InputAction::Fire); // Continuous for auto
    input_state.reload_pressed = check_action_just_pressed(InputAction::Reload);
    input_state.next_weapon_pressed = check_action_just_pressed(InputAction::NextWeapon);
    input_state.prev_weapon_pressed = check_action_just_pressed(InputAction::PrevWeapon);
    input_state.toggle_inventory_pressed = check_action_just_pressed(InputAction::ToggleInventory);

    // Look motion
    let mut look = Vec2::ZERO;
    // for event in mouse_motion.read() {
    //     look += event.delta;
    // }
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
        new_binding = Some(InputBinding::Key(key.clone()));
    } else if let Some(button) = mouse_buttons.get_just_pressed().next() {
        new_binding = Some(InputBinding::Mouse(button.clone()));
    }

    if let Some(binding) = new_binding {
        // Update the map (overwrites existing bindings for this action)
        input_map.bindings.insert(action, vec![binding]);
        rebind_state.action = None;
    }
}

/// Remove expired inputs from the buffer
fn cleanup_input_buffer(
    time: Res<Time>,
    config: Res<InputConfig>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let now = time.elapsed_secs();
    input_buffer.actions.retain(|ba| now - ba.timestamp <= config.buffer_ttl);
}

/// Process movement input (Stub)
fn process_movement_input(_input: Res<InputState>) {}

/// Process action input (Stub)
fn process_action_input(_input: Res<InputState>) {}

/// System to sync global input state to the player entity's component
fn player_input_sync_system(
    input_state: Res<InputState>,
    mut query: Query<&mut InputState, (With<crate::character::Player>, Without<crate::ai::AiController>)>,
) {
    for mut player_input in query.iter_mut() {
        *player_input = input_state.clone();
    }
}

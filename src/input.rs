//! Input system module
//!
//! Flexible input handling with action remapping and buffering support.

use bevy::prelude::*;
use std::collections::HashMap;

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
                player_input_sync_system,
            ).chain())
            .add_systems(Update, (
                process_movement_input,
                process_action_input,
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
    Attack,
    Block,
    SwitchCameraMode,
    Fire,
    Reload,
    NextWeapon,
    PrevWeapon,
    ToggleInventory,
    // Weapon selection keys
    SelectWeapon1,
    SelectWeapon2,
    SelectWeapon3,
    SelectWeapon4,
    SelectWeapon5,
    SelectWeapon6,
    SelectWeapon7,
    SelectWeapon8,
    SelectWeapon9,
    SelectWeapon0,
    // Stealth/Advanced actions
    Hide,
    Peek,
    CornerLean,
    ResetCamera,
    LockOn,
    ZoomIn,
    ZoomOut,
    SideSwitch,
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
        bindings.insert(InputAction::Attack, vec![InputBinding::Mouse(MouseButton::Left)]);
        bindings.insert(InputAction::Block, vec![InputBinding::Mouse(MouseButton::Right)]);
        bindings.insert(InputAction::SwitchCameraMode, vec![InputBinding::Key(KeyCode::KeyC)]);
        bindings.insert(InputAction::Fire, vec![InputBinding::Mouse(MouseButton::Left)]);
        bindings.insert(InputAction::Reload, vec![InputBinding::Key(KeyCode::KeyR)]);
        bindings.insert(InputAction::NextWeapon, vec![InputBinding::Key(KeyCode::ArrowRight)]); 
        bindings.insert(InputAction::PrevWeapon, vec![InputBinding::Key(KeyCode::ArrowLeft)]); 
        bindings.insert(InputAction::ToggleInventory, vec![InputBinding::Key(KeyCode::KeyI)]);

        // Weapon Selection
        bindings.insert(InputAction::SelectWeapon1, vec![InputBinding::Key(KeyCode::Digit1)]);
        bindings.insert(InputAction::SelectWeapon2, vec![InputBinding::Key(KeyCode::Digit2)]);
        bindings.insert(InputAction::SelectWeapon3, vec![InputBinding::Key(KeyCode::Digit3)]);
        bindings.insert(InputAction::SelectWeapon4, vec![InputBinding::Key(KeyCode::Digit4)]);
        bindings.insert(InputAction::SelectWeapon5, vec![InputBinding::Key(KeyCode::Digit5)]);
        bindings.insert(InputAction::SelectWeapon6, vec![InputBinding::Key(KeyCode::Digit6)]);
        bindings.insert(InputAction::SelectWeapon7, vec![InputBinding::Key(KeyCode::Digit7)]);
        bindings.insert(InputAction::SelectWeapon8, vec![InputBinding::Key(KeyCode::Digit8)]);
        bindings.insert(InputAction::SelectWeapon9, vec![InputBinding::Key(KeyCode::Digit9)]);
        bindings.insert(InputAction::SelectWeapon0, vec![InputBinding::Key(KeyCode::Digit0)]);

        // Stealth/Utility
        bindings.insert(InputAction::Hide, vec![InputBinding::Key(KeyCode::KeyH)]);
        bindings.insert(InputAction::Peek, vec![InputBinding::Key(KeyCode::KeyP)]);
        bindings.insert(InputAction::CornerLean, vec![InputBinding::Key(KeyCode::KeyC)]);
        bindings.insert(InputAction::ResetCamera, vec![InputBinding::Key(KeyCode::KeyX)]); // Changed from R to avoid conflict with Reload
        bindings.insert(InputAction::LockOn, vec![InputBinding::Key(KeyCode::Tab), InputBinding::Mouse(MouseButton::Middle)]);
        bindings.insert(InputAction::ZoomIn, vec![InputBinding::Key(KeyCode::NumpadAdd)]);
        bindings.insert(InputAction::ZoomOut, vec![InputBinding::Key(KeyCode::NumpadSubtract)]);
        bindings.insert(InputAction::SideSwitch, vec![InputBinding::Key(KeyCode::KeyV)]);
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
#[derive(Component, Resource, Debug, Reflect, Clone)]
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
    pub fire_just_pressed: bool,
    pub reload_pressed: bool,
    pub next_weapon_pressed: bool,
    pub prev_weapon_pressed: bool,
    pub toggle_inventory_pressed: bool,
    pub side_switch_pressed: bool,
    
    // Stealth/Utility
    pub hide_pressed: bool,
    pub peek_pressed: bool,
    pub corner_lean_pressed: bool,
    pub reset_camera_pressed: bool,
    pub zoom_in_pressed: bool,
    pub zoom_out_pressed: bool,
    
    pub select_weapon: Option<usize>,
    pub enabled: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            movement: Vec2::ZERO,
            look: Vec2::ZERO,
            jump_pressed: false,
            crouch_pressed: false,
            sprint_pressed: false,
            interact_pressed: false,
            aim_pressed: false,
            lean_left: false,
            lean_right: false,
            lock_on_pressed: false,
            attack_pressed: false,
            block_pressed: false,
            switch_camera_mode_pressed: false,
            fire_pressed: false,
            fire_just_pressed: false,
            reload_pressed: false,
            next_weapon_pressed: false,
            prev_weapon_pressed: false,
            toggle_inventory_pressed: false,
            side_switch_pressed: false,
            hide_pressed: false,
            peek_pressed: false,
            corner_lean_pressed: false,
            reset_camera_pressed: false,
            zoom_in_pressed: false,
            zoom_out_pressed: false,
            select_weapon: None,
            enabled: true,
        }
    }
}

impl InputState {
    pub fn set_input_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.movement = Vec2::ZERO;
            self.look = Vec2::ZERO;
            self.jump_pressed = false;
            self.crouch_pressed = false;
            self.sprint_pressed = false;
            self.interact_pressed = false;
            self.aim_pressed = false;
            self.lean_left = false;
            self.lean_right = false;
            self.lock_on_pressed = false;
            self.attack_pressed = false;
            self.block_pressed = false;
            self.switch_camera_mode_pressed = false;
            self.fire_pressed = false;
            self.fire_just_pressed = false;
            self.reload_pressed = false;
            self.next_weapon_pressed = false;
            self.prev_weapon_pressed = false;
            self.toggle_inventory_pressed = false;
            self.side_switch_pressed = false;
            self.hide_pressed = false;
            self.peek_pressed = false;
            self.corner_lean_pressed = false;
            self.reset_camera_pressed = false;
            self.zoom_in_pressed = false;
            self.zoom_out_pressed = false;
            self.select_weapon = None;
        }
    }

    /// Check if an action was just pressed (dynamic check)
    pub fn is_action_just_pressed(&self, action: InputAction) -> bool {
        match action {
            InputAction::Jump => self.jump_pressed,
            InputAction::Interact => self.interact_pressed,
            InputAction::LockOn => self.lock_on_pressed,
            InputAction::Reload => self.reload_pressed,
            InputAction::ResetCamera => self.reset_camera_pressed,
            InputAction::SwitchCameraMode => self.switch_camera_mode_pressed,
            InputAction::SideSwitch => self.side_switch_pressed,
            InputAction::Hide => self.hide_pressed,
            InputAction::Peek => self.peek_pressed,
            InputAction::CornerLean => self.corner_lean_pressed,
            InputAction::ZoomIn => self.zoom_in_pressed,
            InputAction::ZoomOut => self.zoom_out_pressed,
            _ => false,
        }
    }

    /// Get mouse axis for camera control
    pub fn get_mouse_axis(&self) -> Vec2 {
        self.look
    }

    /// Get movement axis for camera movement
    pub fn get_movement_axis(&self) -> Vec2 {
        self.movement
    }
}

/// Input configuration
#[derive(Resource, Debug)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
    pub buffer_ttl: f32, 
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.15,
            gamepad_sensitivity: 1.0,
            invert_y_axis: false,
            buffer_ttl: 0.15, 
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
    input_map: Res<InputMap>,
    mut input_state: ResMut<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    if !input_state.enabled {
        return;
    }

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

    // Buffer certain actions
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
    
    // Continuous Input
    input_state.crouch_pressed = check_action(InputAction::Crouch);
    input_state.sprint_pressed = check_action(InputAction::Sprint);
    input_state.aim_pressed = check_action(InputAction::Aim);
    input_state.lean_left = check_action(InputAction::LeanLeft);
    input_state.lean_right = check_action(InputAction::LeanRight);
    input_state.block_pressed = check_action(InputAction::Block);
    input_state.fire_pressed = check_action(InputAction::Fire);

    // Just Pressed Input
    input_state.jump_pressed = check_action_just_pressed(InputAction::Jump);
    input_state.interact_pressed = check_action_just_pressed(InputAction::Interact);
    input_state.lock_on_pressed = check_action_just_pressed(InputAction::LockOn);
    input_state.attack_pressed = check_action_just_pressed(InputAction::Attack);
    input_state.switch_camera_mode_pressed = check_action_just_pressed(InputAction::SwitchCameraMode);
    input_state.fire_just_pressed = check_action_just_pressed(InputAction::Fire);
    input_state.reload_pressed = check_action_just_pressed(InputAction::Reload);
    input_state.reset_camera_pressed = check_action_just_pressed(InputAction::ResetCamera);
    input_state.next_weapon_pressed = check_action_just_pressed(InputAction::NextWeapon);
    input_state.prev_weapon_pressed = check_action_just_pressed(InputAction::PrevWeapon);
    input_state.toggle_inventory_pressed = check_action_just_pressed(InputAction::ToggleInventory);
    input_state.side_switch_pressed = check_action_just_pressed(InputAction::SideSwitch);
    
    // Stealth/Advanced
    input_state.hide_pressed = check_action_just_pressed(InputAction::Hide);
    input_state.peek_pressed = check_action_just_pressed(InputAction::Peek);
    input_state.corner_lean_pressed = check_action_just_pressed(InputAction::CornerLean);
    input_state.zoom_in_pressed = check_action_just_pressed(InputAction::ZoomIn);
    input_state.zoom_out_pressed = check_action_just_pressed(InputAction::ZoomOut);

    // Weapon Selection
    input_state.select_weapon = None;
    if check_action_just_pressed(InputAction::SelectWeapon1) { input_state.select_weapon = Some(0); }
    else if check_action_just_pressed(InputAction::SelectWeapon2) { input_state.select_weapon = Some(1); }
    else if check_action_just_pressed(InputAction::SelectWeapon3) { input_state.select_weapon = Some(2); }
    else if check_action_just_pressed(InputAction::SelectWeapon4) { input_state.select_weapon = Some(3); }
    else if check_action_just_pressed(InputAction::SelectWeapon5) { input_state.select_weapon = Some(4); }
    else if check_action_just_pressed(InputAction::SelectWeapon6) { input_state.select_weapon = Some(5); }
    else if check_action_just_pressed(InputAction::SelectWeapon7) { input_state.select_weapon = Some(6); }
    else if check_action_just_pressed(InputAction::SelectWeapon8) { input_state.select_weapon = Some(7); }
    else if check_action_just_pressed(InputAction::SelectWeapon9) { input_state.select_weapon = Some(8); }
    else if check_action_just_pressed(InputAction::SelectWeapon0) { input_state.select_weapon = Some(9); }

    // Look (handled by mouse events typically, but for this system we'll need to re-enable it if needed)
    // input_state.look = ...
}

/// System to handle runtime remapping of actions
fn handle_rebinding(
    mut rebind_state: ResMut<RebindState>,
    mut input_map: ResMut<InputMap>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(action) = rebind_state.action else { return };

    let mut new_binding = None;
    if let Some(key) = keyboard.get_just_pressed().next() {
        new_binding = Some(InputBinding::Key(key.clone()));
    } else if let Some(button) = mouse_buttons.get_just_pressed().next() {
        new_binding = Some(InputBinding::Mouse(button.clone()));
    }

    if let Some(binding) = new_binding {
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

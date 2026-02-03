use bevy::prelude::*;
use std::collections::HashMap;
use super::types::{InputAction, InputBinding, BufferedAction, InputContext, ALL_INPUT_ACTIONS};
use std::collections::HashSet;

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
        bindings.insert(InputAction::AbilityUse, vec![InputBinding::Key(KeyCode::KeyG)]);
        bindings.insert(InputAction::AbilitySelect1, vec![InputBinding::Key(KeyCode::F1)]);
        bindings.insert(InputAction::AbilitySelect2, vec![InputBinding::Key(KeyCode::F2)]);
        bindings.insert(InputAction::AbilitySelect3, vec![InputBinding::Key(KeyCode::F3)]);
        bindings.insert(InputAction::AbilitySelect4, vec![InputBinding::Key(KeyCode::F4)]);
        bindings.insert(InputAction::AbilitySelect5, vec![InputBinding::Key(KeyCode::F5)]);
        bindings.insert(InputAction::AbilitySelect6, vec![InputBinding::Key(KeyCode::F6)]);
        bindings.insert(InputAction::AbilitySelect7, vec![InputBinding::Key(KeyCode::F7)]);
        bindings.insert(InputAction::AbilitySelect8, vec![InputBinding::Key(KeyCode::F8)]);
        Self { bindings }
    }
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

/// Current input context stack (top is active).
#[derive(Resource, Debug)]
pub struct InputContextStack {
    pub stack: Vec<InputContext>,
}

impl Default for InputContextStack {
    fn default() -> Self {
        Self {
            stack: vec![InputContext::Gameplay],
        }
    }
}

impl InputContextStack {
    pub fn current(&self) -> InputContext {
        *self.stack.last().unwrap_or(&InputContext::Gameplay)
    }
}

#[derive(Resource, Debug)]
pub struct InputContextRules {
    pub blocked_actions: HashMap<InputContext, HashSet<InputAction>>,
}

impl Default for InputContextRules {
    fn default() -> Self {
        let mut blocked_actions = HashMap::new();
        blocked_actions.insert(InputContext::Menu, HashSet::from([
            InputAction::MoveForward,
            InputAction::MoveBackward,
            InputAction::MoveLeft,
            InputAction::MoveRight,
            InputAction::Jump,
            InputAction::Sprint,
            InputAction::Crouch,
            InputAction::Attack,
            InputAction::Block,
            InputAction::Aim,
            InputAction::Fire,
            InputAction::Reload,
            InputAction::NextWeapon,
            InputAction::PrevWeapon,
        ]));

        blocked_actions.insert(InputContext::Vehicle, HashSet::from([
            InputAction::Jump,
            InputAction::Crouch,
            InputAction::AbilityUse,
        ]));

        Self { blocked_actions }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionValue {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub value: f32,
}

#[derive(Resource, Debug)]
pub struct ActionState {
    pub actions: HashMap<InputAction, ActionValue>,
}

impl Default for ActionState {
    fn default() -> Self {
        let mut actions = HashMap::new();
        for action in ALL_INPUT_ACTIONS {
            actions.insert(action, ActionValue::default());
        }
        Self { actions }
    }
}

impl ActionState {
    pub fn get(&self, action: InputAction) -> ActionValue {
        *self.actions.get(&action).unwrap_or(&ActionValue::default())
    }
}

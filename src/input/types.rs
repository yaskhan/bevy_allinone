use bevy::prelude::*;

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
    AbilityUse,
    AbilitySelect1,
    AbilitySelect2,
    AbilitySelect3,
    AbilitySelect4,
    AbilitySelect5,
    AbilitySelect6,
    AbilitySelect7,
    AbilitySelect8,
}

pub const ALL_INPUT_ACTIONS: [InputAction; 46] = [
    InputAction::MoveForward,
    InputAction::MoveBackward,
    InputAction::MoveLeft,
    InputAction::MoveRight,
    InputAction::Jump,
    InputAction::Sprint,
    InputAction::Crouch,
    InputAction::Interact,
    InputAction::Aim,
    InputAction::LeanLeft,
    InputAction::LeanRight,
    InputAction::Attack,
    InputAction::Block,
    InputAction::SwitchCameraMode,
    InputAction::Fire,
    InputAction::Reload,
    InputAction::NextWeapon,
    InputAction::PrevWeapon,
    InputAction::ToggleInventory,
    InputAction::SelectWeapon1,
    InputAction::SelectWeapon2,
    InputAction::SelectWeapon3,
    InputAction::SelectWeapon4,
    InputAction::SelectWeapon5,
    InputAction::SelectWeapon6,
    InputAction::SelectWeapon7,
    InputAction::SelectWeapon8,
    InputAction::SelectWeapon9,
    InputAction::SelectWeapon0,
    InputAction::Hide,
    InputAction::Peek,
    InputAction::CornerLean,
    InputAction::ResetCamera,
    InputAction::LockOn,
    InputAction::ZoomIn,
    InputAction::ZoomOut,
    InputAction::SideSwitch,
    InputAction::AbilityUse,
    InputAction::AbilitySelect1,
    InputAction::AbilitySelect2,
    InputAction::AbilitySelect3,
    InputAction::AbilitySelect4,
    InputAction::AbilitySelect5,
    InputAction::AbilitySelect6,
    InputAction::AbilitySelect7,
    InputAction::AbilitySelect8,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum InputContext {
    Gameplay,
    Menu,
    Vehicle,
}

/// Input binding types
#[derive(Debug, Clone, Reflect)]
pub enum InputBinding {
    Key(KeyCode),
    Mouse(MouseButton),
}

/// A buffered action that was recently pressed
#[derive(Debug, Clone)]
pub struct BufferedAction {
    pub action: InputAction,
    pub timestamp: f32,
}

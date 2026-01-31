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

use bevy::prelude::*;
use super::types::InputAction;

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

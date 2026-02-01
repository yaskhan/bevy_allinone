pub mod types;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use types::*;
use components::*;
use systems::*;

pub use types::{HideState, CoverType, CoverObject};
pub use components::{StealthController, StealthState, CoverDetection, VisibilityMeter};
pub use systems::*;

pub struct StealthPlugin;

impl Plugin for StealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<StealthController>()
            .register_type::<StealthState>()
            .register_type::<CoverDetection>()
            .register_type::<VisibilityMeter>()
            .add_systems(Update, (
                handle_stealth_input,
                update_stealth_state,
                update_visibility_meter,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_cover_objects,
                check_line_of_sight,
                update_hide_states,
            ));
    }
}

/// Public API for stealth system
pub mod api {
    use super::*;
    use crate::character::CharacterMovementState;
    
    /// Check if character is hidden
    pub fn is_hidden(state: &StealthState) -> bool {
        state.is_hidden
    }
    
    /// Check if character is detected
    pub fn is_detected(state: &StealthState) -> bool {
        state.is_detected
    }
    
    /// Get current hide state
    pub fn get_hide_state(state: &StealthState) -> HideState {
        state.hide_state
    }
    
    /// Get visibility level (0.0 = hidden, 1.0 = visible)
    pub fn get_visibility_level(visibility: &VisibilityMeter) -> f32 {
        visibility.current_visibility
    }
    
    /// Get detection level (0.0 = not detected, 1.0 = fully detected)
    pub fn get_detection_level(visibility: &VisibilityMeter) -> f32 {
        visibility.detection_level
    }
    
    /// Get sound level (0.0 = silent, 1.0 = very loud)
    pub fn get_sound_level(visibility: &VisibilityMeter) -> f32 {
        visibility.sound_level
    }
    
    /// Check if character is in cover
    pub fn is_in_cover(cover: &CoverDetection) -> bool {
        cover.is_in_cover
    }
    
    /// Get current cover type
    pub fn get_cover_type(cover: &CoverDetection) -> CoverType {
        cover.cover_type
    }
    
    /// Toggle hide state (external call)
    pub fn toggle_hide(
        stealth: &StealthController,
        state: &mut StealthState,
        movement: &mut CharacterMovementState,
    ) {
        toggle_hide_state(stealth, state, movement);
    }
    
    /// Toggle peek state (external call)
    pub fn toggle_peek(state: &mut StealthState) {
        toggle_peek_state(state);
    }
    
    /// Toggle corner lean state (external call)
    pub fn toggle_corner_lean(state: &mut StealthState) {
        toggle_corner_lean_state(state);
    }
    
    /// Reset camera (external call)
    pub fn reset_camera_external(state: &mut StealthState) {
        reset_camera(state);
    }
}

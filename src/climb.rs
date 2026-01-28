//! # Climb System
//!
//! Wall climbing mechanics for vertical traversal, inspired by games like Assassin's Creed or Mirror's Edge.
//!
//! ## Features
//!
//! - **Wall Detection**: Raycasting to detect climbable surfaces
//! - **Climb States**:
//!   - **Approach**: Moving toward wall
//!   - **Hang**: Hanging from ledge
//!   - **Climb Up**: Ascending
//!   - **Climb Down**: Descending
//!   - **Climb Left/Right**: Horizontal movement
//!   - **Vault**: Quick climb over low obstacles
//! - **Climb Speed**: Configurable ascent/descent rates
//! - **Stamina System**: Limited climbing time
//! - **Edge Detection**: Detect ledges and handholds
//! - **Surface Types**: Different climb speeds for different materials
//! - **Animation Integration**: Climbing animations
//! - **Camera Adjustments**: Camera follows climb path
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_allinone::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(GameControllerPlugin)
//!         .run();
//! }
//! ```

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::physics::{GroundDetection, GroundDetectionSettings};
use crate::character::{CharacterController, Player};
use crate::input::{InputState, InputAction};
use crate::interaction::InteractionEventQueue;

/// Force mode for applying forces to rigidbodies
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum ForceMode {
    #[default]
    Impulse,
    VelocityChange,
    Force,
}

pub struct ClimbPlugin;

impl Plugin for ClimbPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ClimbLedgeSystem>()
            .register_type::<LedgeZone>()
            .add_systems(Update, (
                handle_climb_input,
                update_climb_state,
                update_climb_visuals,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_ledge,
                detect_ledge_below,
                update_climb_movement,
                handle_auto_hang,
            ));
    }
}

/// Main climb ledge system component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbLedgeSystem {
    // Main Settings
    pub climb_ledge_active: bool,
    pub use_hang_from_ledge_icon: bool,
    pub use_fixed_device_icon_position: bool,
    pub keep_weapons_on_ledge_detected: bool,
    pub draw_weapons_after_climb_ledge_if_previously_carried: bool,

    // Climb Animator Settings
    pub climb_ledge_action_id: i32,
    pub hold_on_ledge_action_name: String,
    pub action_active_animator_name: String,
    pub action_id_animator_name: String,
    pub match_start_value: f32,
    pub match_end_value: f32,
    pub match_mask_value: Vec3,
    pub match_mask_rotation_value: f32,
    pub base_layer_index: i32,

    // Raycast Ledge Detection Settings
    pub climb_ledge_ray_forward_distance: f32,
    pub climb_ledge_ray_down_distance: f32,
    pub layer_mask_to_check: u32,

    // Other Settings
    pub only_grab_ledge_if_moving_forward: bool,
    pub adjust_to_hold_on_ledge_position_speed: f32,
    pub adjust_to_hold_on_ledge_rotation_speed: f32,
    pub hold_on_ledge_offset: Vec3,
    pub climb_ledge_target_position_offset_third_person: Vec3,
    pub climb_ledge_target_position_offset_first_person: Vec3,
    pub hand_offset: f32,
    pub time_to_climb_ledge_third_person: f32,
    pub time_to_climb_ledge_first_person: f32,
    pub climb_ledge_speed_first_person: f32,
    pub climb_if_surface_found_below_player: bool,
    pub check_for_ledge_zones_active: bool,

    // Ledge Below Check Settings
    pub check_for_hang_from_ledge_on_ground: bool,
    pub check_ledge_zone_detected_by_raycast: bool,
    pub raycast_radius_to_check_surface_below_player: f32,
    pub check_for_hang_from_ledge_on_ground_raycast_distance: f32,
    pub only_hang_from_ledge_if_player_is_not_moving: bool,
    pub time_to_cancel_hang_from_ledge_if_not_found: f32,
    pub can_cancel_hang_from_ledge: bool,
    pub has_to_look_at_ledge_position_on_first_person: bool,
    pub use_max_distance_to_camera_center: bool,
    pub max_distance_to_camera_center: f32,

    // Auto Climb Ledge Settings
    pub auto_climb_in_third_person: bool,
    pub auto_climb_in_first_person: bool,

    // Jump Settings
    pub can_jump_when_hold_ledge: bool,
    pub jump_force_when_hold_ledge: f32,
    pub jump_force_mode: ForceMode,

    // Grab Surface On Air Settings
    pub can_grab_any_surface_on_air: bool,
    pub use_grab_surface_amount_limit: bool,
    pub grab_surface_amount_limit: i32,
    pub current_grab_surface_amount: i32,

    // Debug State
    pub avoid_player_grab_ledge: bool,
    pub ledge_zone_found: bool,
    pub activate_climb_action: bool,
    pub can_start_to_climb_ledge: bool,
    pub climbing_ledge: bool,
    pub can_use_climb_ledge: bool,
    pub can_climb_current_ledge_zone: bool,
    pub stop_grab_ledge: bool,
    pub direction_angle: f32,
    pub surface_to_hang_on_ground_found: bool,
    pub moving_toward_surface_to_hang: bool,
    pub previously_moving_toward_surface_to_hang: bool,
    pub on_air_while_searching_ledge_to_hang: bool,
    pub ledge_zone_close_enough_to_screen_center: bool,
    pub current_distance_to_target: f32,
    pub can_check_for_hang_from_ledge_on_ground: bool,
    pub climb_ledge_action_activated: bool,
    pub lose_ledge_action_activated: bool,
    pub grabbing_surface: bool,
    pub climb_ledge_action_paused: bool,
}

impl Default for ClimbLedgeSystem {
    fn default() -> Self {
        Self {
            climb_ledge_active: true,
            use_hang_from_ledge_icon: false,
            use_fixed_device_icon_position: false,
            keep_weapons_on_ledge_detected: false,
            draw_weapons_after_climb_ledge_if_previously_carried: false,

            climb_ledge_action_id: 1,
            hold_on_ledge_action_name: "Hold On Ledge".to_string(),
            action_active_animator_name: "Action Active".to_string(),
            action_id_animator_name: "Action ID".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,

            climb_ledge_ray_forward_distance: 1.0,
            climb_ledge_ray_down_distance: 1.0,
            layer_mask_to_check: 1,

            only_grab_ledge_if_moving_forward: false,
            adjust_to_hold_on_ledge_position_speed: 3.0,
            adjust_to_hold_on_ledge_rotation_speed: 10.0,
            hold_on_ledge_offset: Vec3::ZERO,
            climb_ledge_target_position_offset_third_person: Vec3::ZERO,
            climb_ledge_target_position_offset_first_person: Vec3::ZERO,
            hand_offset: 0.2,
            time_to_climb_ledge_third_person: 2.0,
            time_to_climb_ledge_first_person: 1.0,
            climb_ledge_speed_first_person: 1.0,
            climb_if_surface_found_below_player: false,
            check_for_ledge_zones_active: true,

            check_for_hang_from_ledge_on_ground: false,
            check_ledge_zone_detected_by_raycast: true,
            raycast_radius_to_check_surface_below_player: 0.5,
            check_for_hang_from_ledge_on_ground_raycast_distance: 2.0,
            only_hang_from_ledge_if_player_is_not_moving: true,
            time_to_cancel_hang_from_ledge_if_not_found: 3.0,
            can_cancel_hang_from_ledge: true,
            has_to_look_at_ledge_position_on_first_person: false,
            use_max_distance_to_camera_center: false,
            max_distance_to_camera_center: 100.0,

            auto_climb_in_third_person: false,
            auto_climb_in_first_person: false,

            can_jump_when_hold_ledge: false,
            jump_force_when_hold_ledge: 10.0,
            jump_force_mode: ForceMode::Impulse,

            can_grab_any_surface_on_air: false,
            use_grab_surface_amount_limit: false,
            grab_surface_amount_limit: 3,
            current_grab_surface_amount: 0,

            avoid_player_grab_ledge: false,
            ledge_zone_found: false,
            activate_climb_action: false,
            can_start_to_climb_ledge: false,
            climbing_ledge: false,
            can_use_climb_ledge: true,
            can_climb_current_ledge_zone: true,
            stop_grab_ledge: false,
            direction_angle: 0.0,
            surface_to_hang_on_ground_found: false,
            moving_toward_surface_to_hang: false,
            previously_moving_toward_surface_to_hang: false,
            on_air_while_searching_ledge_to_hang: false,
            ledge_zone_close_enough_to_screen_center: false,
            current_distance_to_target: 0.0,
            can_check_for_hang_from_ledge_on_ground: true,
            climb_ledge_action_activated: false,
            lose_ledge_action_activated: false,
            grabbing_surface: false,
            climb_ledge_action_paused: false,
        }
    }
}

/// Ledge zone component for configuring climbable surfaces
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeZone {
    pub tag_to_check: String,
    pub ledge_zone_active: bool,
    pub check_down_raycast_offset: Vec3,
    pub climb_ledge_forward_ray_distance: f32,
    pub climb_ledge_down_ray_distance: f32,
    pub ledge_zone_can_be_climbed: bool,
    pub avoid_player_grab_ledge: bool,
    pub can_check_for_hang_from_ledge_on_ground: bool,
    pub only_hang_from_ledge_if_player_is_not_moving: bool,
    pub can_grab_any_surface_on_air_active: bool,
}

impl Default for LedgeZone {
    fn default() -> Self {
        Self {
            tag_to_check: "Player".to_string(),
            ledge_zone_active: true,
            check_down_raycast_offset: Vec3::ZERO,
            climb_ledge_forward_ray_distance: 1.0,
            climb_ledge_down_ray_distance: 1.0,
            ledge_zone_can_be_climbed: true,
            avoid_player_grab_ledge: false,
            can_check_for_hang_from_ledge_on_ground: true,
            only_hang_from_ledge_if_player_is_not_moving: true,
            can_grab_any_surface_on_air_active: true,
        }
    }
}

/// Climb state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum ClimbState {
    None,
    Approaching,
    Hanging,
    ClimbingUp,
    ClimbingDown,
    ClimbingLeft,
    ClimbingRight,
    Vaulting,
    Falling,
}

impl Default for ClimbState {
    fn default() -> Self {
        Self::None
    }
}

/// Component to track current climb state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbStateTracker {
    pub current_state: ClimbState,
    pub previous_state: ClimbState,
    pub state_timer: f32,
    pub climb_speed: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub stamina_drain_rate: f32,
    pub stamina_regen_rate: f32,
    pub is_stamina_depleted: bool,
}

impl Default for ClimbStateTracker {
    fn default() -> Self {
        Self {
            current_state: ClimbState::None,
            previous_state: ClimbState::None,
            state_timer: 0.0,
            climb_speed: 3.0,
            stamina: 100.0,
            max_stamina: 100.0,
            stamina_drain_rate: 10.0,
            stamina_regen_rate: 5.0,
            is_stamina_depleted: false,
        }
    }
}

/// Component for ledge detection results
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeDetection {
    pub ledge_found: bool,
    pub ledge_position: Vec3,
    pub ledge_normal: Vec3,
    pub ledge_distance: f32,
    pub ledge_height: f32,
    pub is_hangable: bool,
    pub is_climbable: bool,
    pub surface_type: SurfaceType,
    pub raycast_hit_point: Vec3,
    pub raycast_hit_normal: Vec3,
}

impl Default for LedgeDetection {
    fn default() -> Self {
        Self {
            ledge_found: false,
            ledge_position: Vec3::ZERO,
            ledge_normal: Vec3::ZERO,
            ledge_distance: 0.0,
            ledge_height: 0.0,
            is_hangable: false,
            is_climbable: false,
            surface_type: SurfaceType::Default,
            raycast_hit_point: Vec3::ZERO,
            raycast_hit_normal: Vec3::ZERO,
        }
    }
}

/// Surface type for different climb speeds
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum SurfaceType {
    #[default]
    Default,
    Stone,
    Wood,
    Metal,
    Ice,
    Rope,
    Custom(f32), // Custom climb speed multiplier
}

impl SurfaceType {
    pub fn climb_speed_multiplier(&self) -> f32 {
        match self {
            SurfaceType::Default => 1.0,
            SurfaceType::Stone => 1.0,
            SurfaceType::Wood => 0.9,
            SurfaceType::Metal => 0.8,
            SurfaceType::Ice => 1.2,
            SurfaceType::Rope => 0.7,
            SurfaceType::Custom(multiplier) => *multiplier,
        }
    }
}

/// Component for auto-hang functionality
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AutoHang {
    pub active: bool,
    pub moving_toward_ledge: bool,
    pub target_ledge_position: Vec3,
    pub target_ledge_normal: Vec3,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub timeout: f32,
    pub timer: f32,
    pub only_when_not_moving: bool,
    pub look_at_ledge_on_first_person: bool,
    pub max_distance_to_camera_center: f32,
}

impl Default for AutoHang {
    fn default() -> Self {
        Self {
            active: false,
            moving_toward_ledge: false,
            target_ledge_position: Vec3::ZERO,
            target_ledge_normal: Vec3::ZERO,
            move_speed: 3.0,
            rotation_speed: 10.0,
            timeout: 3.0,
            timer: 0.0,
            only_when_not_moving: true,
            look_at_ledge_on_first_person: false,
            max_distance_to_camera_center: 100.0,
        }
    }
}

/// Component for climb animation control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbAnimation {
    pub is_active: bool,
    pub action_id: i32,
    pub action_name: String,
    pub match_start_value: f32,
    pub match_end_value: f32,
    pub match_mask_value: Vec3,
    pub match_mask_rotation_value: f32,
    pub base_layer_index: i32,
    pub is_first_person: bool,
    pub time_to_climb_third_person: f32,
    pub time_to_climb_first_person: f32,
}

impl Default for ClimbAnimation {
    fn default() -> Self {
        Self {
            is_active: false,
            action_id: 1,
            action_name: "Hold On Ledge".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,
            is_first_person: false,
            time_to_climb_third_person: 2.0,
            time_to_climb_first_person: 1.0,
        }
    }
}

/// Component for climb movement control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbMovement {
    pub is_active: bool,
    pub target_position: Vec3,
    pub target_rotation: Quat,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub hand_offset: f32,
    pub is_first_person: bool,
    pub climb_speed_first_person: f32,
    pub adjust_position_speed: f32,
    pub adjust_rotation_speed: f32,
}

impl Default for ClimbMovement {
    fn default() -> Self {
        Self {
            is_active: false,
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            move_speed: 3.0,
            rotation_speed: 10.0,
            hand_offset: 0.2,
            is_first_person: false,
            climb_speed_first_person: 1.0,
            adjust_position_speed: 3.0,
            adjust_rotation_speed: 10.0,
        }
    }
}

/// Component for jump from ledge
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeJump {
    pub can_jump: bool,
    pub jump_force: f32,
    pub jump_force_mode: ForceMode,
    pub is_jumping: bool,
    pub jump_timer: f32,
}

impl Default for LedgeJump {
    fn default() -> Self {
        Self {
            can_jump: false,
            jump_force: 10.0,
            jump_force_mode: ForceMode::Impulse,
            is_jumping: false,
            jump_timer: 0.0,
        }
    }
}

/// Component for grab surface on air
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrabSurfaceOnAir {
    pub can_grab: bool,
    pub use_amount_limit: bool,
    pub amount_limit: i32,
    pub current_amount: i32,
    pub is_grabbing: bool,
    pub grab_timer: f32,
}

impl Default for GrabSurfaceOnAir {
    fn default() -> Self {
        Self {
            can_grab: false,
            use_amount_limit: false,
            amount_limit: 3,
            current_amount: 0,
            is_grabbing: false,
            grab_timer: 0.0,
        }
    }
}

/// Event for when a ledge is grabbed
#[derive(Event, Debug, Reflect)]
pub struct LedgeGrabbedEvent {
    pub entity: Entity,
    pub ledge_position: Vec3,
    pub ledge_normal: Vec3,
    pub surface_type: SurfaceType,
}

/// Event for when a ledge is climbed
#[derive(Event, Debug, Reflect)]
pub struct LedgeClimbedEvent {
    pub entity: Entity,
    pub climb_time: f32,
    pub surface_type: SurfaceType,
}

/// Event for when a ledge is lost
#[derive(Event, Debug, Reflect)]
pub struct LedgeLostEvent {
    pub entity: Entity,
    pub reason: LedgeLostReason,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum LedgeLostReason {
    PlayerMoved,
    StaminaDepleted,
    SurfaceBecameUnclimbable,
    ManualRelease,
    JumpedOff,
}

/// Event for when a ledge jump occurs
#[derive(Event, Debug, Reflect)]
pub struct LedgeJumpEvent {
    pub entity: Entity,
    pub jump_force: f32,
    pub jump_direction: Vec3,
}

/// System to handle climb input
pub fn handle_climb_input(
    input_state: Res<InputState>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut AutoHang,
        &mut GrabSurfaceOnAir,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut state_tracker,
        mut auto_hang,
        mut grab_surface,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active || !climb_system.can_use_climb_ledge {
            continue;
        }

        // Check if player is dead or in special states
        if character.is_dead || character.zero_gravity_mode || character.free_floating_mode {
            continue;
        }

        // Handle jump from ledge
        if climb_system.can_jump_when_hold_ledge &&
           (state_tracker.current_state == ClimbState::Hanging || climb_system.grabbing_surface) &&
           !climb_system.activate_climb_action {
            if input_state.jump_pressed {
                // Trigger jump from ledge
                // TODO: Implement jump physics
            }
        }

        // Handle grab surface on air
        if climb_system.can_grab_any_surface_on_air &&
           !character.is_dead &&
           !climb_system.climbing_ledge &&
           !climb_system.climb_ledge_action_paused {
            if input_state.interact_pressed {
                // Try to grab surface
                // TODO: Implement grab surface logic
            }
        }

        // Handle auto hang from ledge
        if climb_system.check_for_hang_from_ledge_on_ground &&
           climb_system.surface_to_hang_on_ground_found &&
           !climb_system.moving_toward_surface_to_hang &&
           climb_system.only_hang_from_ledge_if_player_is_not_moving {

            // Check if player is on ground and not moving
            // TODO: Implement auto hang detection
        }
    }
}

/// System to update climb state
pub fn update_climb_state(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut LedgeDetection,
        &mut AutoHang,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut state_tracker,
        mut ledge_detection,
        mut auto_hang,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active {
            continue;
        }

        // Update state timer
        state_tracker.state_timer += time.delta_secs();

        // Update stamina
        if state_tracker.current_state != ClimbState::None &&
           state_tracker.current_state != ClimbState::Falling {
            // Drain stamina while climbing
            state_tracker.stamina -= state_tracker.stamina_drain_rate * time.delta_secs();
            if state_tracker.stamina <= 0.0 {
                state_tracker.stamina = 0.0;
                state_tracker.is_stamina_depleted = true;
                // Trigger stamina depleted event
                // TODO: Implement stamina depleted logic
            }
        } else {
            // Regenerate stamina when not climbing
            if state_tracker.stamina < state_tracker.max_stamina {
                state_tracker.stamina += state_tracker.stamina_regen_rate * time.delta_secs();
                if state_tracker.stamina >= state_tracker.max_stamina {
                    state_tracker.stamina = state_tracker.max_stamina;
                    state_tracker.is_stamina_depleted = false;
                }
            }
        }

        // Update auto-hang timer
        if auto_hang.active && auto_hang.moving_toward_ledge {
            auto_hang.timer += time.delta_secs();
            if auto_hang.timer >= auto_hang.timeout {
                // Timeout - cancel auto hang
                auto_hang.active = false;
                auto_hang.moving_toward_ledge = false;
                auto_hang.timer = 0.0;
            }
        }

        // Update climb action activation
        if climb_system.activate_climb_action {
            if climb_system.can_start_to_climb_ledge {
                // Climbing in progress
                // TODO: Implement climbing logic
            }
        }

        // Update ledge detection state
        if climb_system.ledge_zone_found {
            // Ledge zone is active
            // TODO: Implement ledge zone logic
        }
    }
}

/// System to update climb visuals (UI, icons, etc.)
pub fn update_climb_visuals(
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut AutoHang,
        &Transform,
    ), With<Player>>,
) {
    for (mut climb_system, mut auto_hang, transform) in query.iter_mut() {
        if !climb_system.climb_ledge_active {
            continue;
        }

        // Update hang from ledge icon
        if climb_system.use_hang_from_ledge_icon &&
           climb_system.check_for_hang_from_ledge_on_ground &&
           climb_system.surface_to_hang_on_ground_found &&
           !climb_system.moving_toward_surface_to_hang &&
           climb_system.only_hang_from_ledge_if_player_is_not_moving {

            // TODO: Update icon position based on ledge position
            // This would involve camera projection and UI positioning
        }
    }
}

/// System to detect ledge in front of player
pub fn detect_ledge(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut LedgeDetection,
        &mut ClimbStateTracker,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut ledge_detection,
        mut state_tracker,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active ||
           !climb_system.can_use_climb_ledge ||
           character.is_dead ||
           character.zero_gravity_mode ||
           character.free_floating_mode {
            continue;
        }

        // Skip if climbing or hanging
        if climb_system.climbing_ledge || climb_system.grabbing_surface {
            continue;
        }

        // Skip if on ground and not checking for air grab
        // TODO: Check if player is on ground

        // Skip if checking for ledge zones and no zone found
        if climb_system.check_for_ledge_zones_active && !climb_system.ledge_zone_found {
            continue;
        }

        // Skip if only grabbing when moving forward
        if climb_system.only_grab_ledge_if_moving_forward {
            // TODO: Check if player is moving forward
        }

        // Perform raycast to detect ledge
        // TODO: Implement raycast logic using avian3d physics
        // This would involve:
        // 1. Getting raycast position from climb_ledge_ray_position
        // 2. Getting raycast direction from climb_ledge_ray_position.forward
        // 3. Performing raycast with climb_ledge_ray_forward_distance
        // 4. Checking if hit surface is climbable
        // 5. Performing second raycast to check for ledge surface
        // 6. Updating ledge_detection component

        // If ledge detected, activate grab
        if ledge_detection.ledge_found && ledge_detection.is_climbable {
            climb_system.activate_climb_action = true;
            climb_system.climbing_ledge = true;

            // Update state tracker
            state_tracker.current_state = ClimbState::Hanging;
            state_tracker.state_timer = 0.0;
        }
    }
}

/// System to detect ledge below player (for auto-hang)
pub fn detect_ledge_below(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut LedgeDetection,
        &mut AutoHang,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut ledge_detection,
        mut auto_hang,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active ||
           !climb_system.check_for_hang_from_ledge_on_ground ||
           !climb_system.can_check_for_hang_from_ledge_on_ground {
            continue;
        }

        // Skip if player is not on ground
        // TODO: Check if player is on ground

        // Skip if ledge already found
        if climb_system.surface_to_hang_on_ground_found {
            continue;
        }

        // Skip if checking for ledge zones and no zone found
        if climb_system.check_for_ledge_zones_active && !climb_system.ledge_zone_found {
            continue;
        }

        // Skip if only hanging when not moving and player is moving
        if climb_system.only_hang_from_ledge_if_player_is_not_moving {
            // TODO: Check if player is moving
        }

        // Perform raycast to detect ledge below
        // TODO: Implement raycast logic using avian3d physics
        // This would involve:
        // 1. Getting raycast position from hang_from_ledge_on_ground_raycast_position
        // 2. Getting raycast direction from hang_from_ledge_on_ground_raycast_position.forward
        // 3. Performing raycast with check_for_hang_from_ledge_on_ground_raycast_distance
        // 4. If no surface found, perform downward raycast to find ledge
        // 5. Check if there's enough space above ledge for player
        // 6. Update ledge_detection and auto_hang components

        // If ledge detected, set up auto-hang
        if ledge_detection.ledge_found && ledge_detection.is_hangable {
            climb_system.surface_to_hang_on_ground_found = true;
            auto_hang.active = true;
            auto_hang.target_ledge_position = ledge_detection.ledge_position;
            auto_hang.target_ledge_normal = ledge_detection.ledge_normal;
            auto_hang.timer = 0.0;
        }
    }
}

/// System to update climb movement
pub fn update_climb_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut ClimbMovement,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut state_tracker,
        mut climb_movement,
        mut transform,
        mut character,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active || !climb_movement.is_active {
            continue;
        }

        // Update climb movement based on current state
        match state_tracker.current_state {
            ClimbState::Hanging => {
                // Adjust position and rotation to hold on ledge
                // TODO: Implement position/rotation adjustment
            }
            ClimbState::ClimbingUp | ClimbState::ClimbingDown | 
            ClimbState::ClimbingLeft | ClimbState::ClimbingRight => {
                // Move toward target position
                // TODO: Implement climbing movement
            }
            ClimbState::Vaulting => {
                // Quick vault over obstacle
                // TODO: Implement vaulting movement
            }
            _ => {}
        }

        // Update movement timer
        // TODO: Implement movement timing
    }
}

/// System to handle auto-hang movement
pub fn handle_auto_hang(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut AutoHang,
        &mut Transform,
        &mut CharacterController,
        &InputState,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut auto_hang,
        mut transform,
        mut character,
        input_state,
    ) in query.iter_mut() {
        if !auto_hang.active || !auto_hang.moving_toward_ledge {
            continue;
        }

        // Move player toward ledge position
        // TODO: Implement movement toward ledge
        // This would involve:
        // 1. Calculating direction to ledge
        // 2. Moving player in that direction
        // 3. Rotating player to face ledge
        // 4. Checking if player reached ledge
        // 5. If reached, trigger hang state

        // Check if player reached ledge
        let distance_to_ledge = transform.translation.distance(auto_hang.target_ledge_position);
        if distance_to_ledge < 0.5 {
            // Reached ledge - trigger hang
            climb_system.moving_toward_surface_to_hang = false;
            climb_system.surface_to_hang_on_ground_found = true;
            climb_system.grabbing_surface = true;
            auto_hang.active = false;
            auto_hang.moving_toward_ledge = false;
            auto_hang.timer = 0.0;
        }
    }
}

/// System to handle ledge jump
pub fn handle_ledge_jump(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut LedgeJump,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut state_tracker,
        mut ledge_jump,
        mut transform,
        mut character,
    ) in query.iter_mut() {
        if !ledge_jump.can_jump || !ledge_jump.is_jumping {
            continue;
        }

        // Apply jump force
        // TODO: Implement jump physics using avian3d
        // This would involve:
        // 1. Calculating jump direction (up from ledge)
        // 2. Applying force to character rigidbody
        // 3. Transitioning from climb state to falling state

        // Update jump timer
        ledge_jump.jump_timer += time.delta_secs();
        if ledge_jump.jump_timer > 0.5 {
            // Jump complete
            ledge_jump.is_jumping = false;
            ledge_jump.jump_timer = 0.0;

            // Transition to falling state
            state_tracker.current_state = ClimbState::Falling;
            state_tracker.state_timer = 0.0;

            // Release from ledge
            climb_system.climbing_ledge = false;
            climb_system.grabbing_surface = false;
            climb_system.activate_climb_action = false;
        }
    }
}

/// System to handle grab surface on air
pub fn handle_grab_surface_on_air(
    time: Res<Time>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut GrabSurfaceOnAir,
        &mut LedgeDetection,
        &mut ClimbStateTracker,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut grab_surface,
        mut ledge_detection,
        mut state_tracker,
        character,
        transform,
    ) in query.iter_mut() {
        if !grab_surface.can_grab || !grab_surface.is_grabbing {
            continue;
        }

        // Check if already grabbed
        if climb_system.grabbing_surface {
            continue;
        }

        // Check amount limit
        if grab_surface.use_amount_limit && grab_surface.current_amount >= grab_surface.amount_limit {
            continue;
        }

        // Perform raycast to detect surface
        // TODO: Implement raycast logic using avian3d physics
        // This would involve:
        // 1. Getting raycast position and direction
        // 2. Performing raycast
        // 3. Checking if surface is grabbable
        // 4. Updating ledge_detection

        // If surface detected, grab it
        if ledge_detection.ledge_found && ledge_detection.is_climbable {
            climb_system.grabbing_surface = true;
            climb_system.climbing_ledge = false;

            // Update state tracker
            state_tracker.current_state = ClimbState::Hanging;
            state_tracker.state_timer = 0.0;

            // Increment grab amount
            if grab_surface.use_amount_limit {
                grab_surface.current_amount += 1;
            }

            // Reset grab timer
            grab_surface.grab_timer = 0.0;
            grab_surface.is_grabbing = false;
        }

        // Update grab timer
        grab_surface.grab_timer += time.delta_secs();
        if grab_surface.grab_timer > 1.0 {
            // Timeout - cancel grab
            grab_surface.is_grabbing = false;
            grab_surface.grab_timer = 0.0;
        }
    }
}

/// System to handle ledge zone triggers
pub fn handle_ledge_zone_trigger(
    mut commands: Commands,
    mut ledge_zone_query: Query<(&LedgeZone, &Transform)>,
    mut player_query: Query<(&mut ClimbLedgeSystem, &Transform), With<Player>>,
) {
    // TODO: Implement collision detection logic
    // This would involve using avian3d's collision events
}

/// Utility function to calculate climb speed based on surface type
pub fn calculate_climb_speed(surface_type: SurfaceType, base_speed: f32) -> f32 {
    base_speed * surface_type.climb_speed_multiplier()
}

/// Utility function to check if surface is climbable
pub fn is_surface_climbable(normal: Vec3, max_angle: f32) -> bool {
    let angle = normal.angle_between(Vec3::Y).to_degrees();
    angle >= max_angle
}

/// Utility function to check if surface is hangable
pub fn is_surface_hangable(normal: Vec3, max_angle: f32) -> bool {
    let angle = normal.angle_between(Vec3::Y).to_degrees();
    angle >= max_angle && angle <= 90.0
}

/// Utility function to calculate ledge height
pub fn calculate_ledge_height(start_pos: Vec3, end_pos: Vec3) -> f32 {
    (end_pos.y - start_pos.y).abs()
}

/// Utility function to calculate ledge distance
pub fn calculate_ledge_distance(start_pos: Vec3, end_pos: Vec3) -> f32 {
    start_pos.distance(end_pos)
}

/// Utility function to calculate climb direction
pub fn calculate_climb_direction(_player_forward: Vec3, ledge_normal: Vec3) -> Vec3 {
    // Climb direction is opposite to ledge normal
    -ledge_normal
}

/// Utility function to calculate vault direction
pub fn calculate_vault_direction(player_forward: Vec3, _ledge_normal: Vec3) -> Vec3 {
    // Vault direction is forward from player
    player_forward
}

/// Utility function to check if player is moving toward ledge
pub fn is_moving_toward_ledge(player_velocity: Vec3, ledge_direction: Vec3) -> bool {
    let dot = player_velocity.dot(ledge_direction);
    dot > 0.0
}

/// Utility function to calculate jump direction from ledge
pub fn calculate_jump_direction(ledge_normal: Vec3) -> Vec3 {
    // Jump direction is up from ledge
    Vec3::Y
}

/// Utility function to calculate hand position for climbing
pub fn calculate_hand_position(ledge_position: Vec3, ledge_normal: Vec3, hand_offset: f32) -> Vec3 {
    ledge_position + ledge_normal * hand_offset
}

/// Utility function to calculate target rotation for climbing
pub fn calculate_target_rotation(ledge_normal: Vec3, up_vector: Vec3) -> Quat {
    Quat::from_rotation_arc(Vec3::Z, -ledge_normal)
}

/// Utility function to check if player has line of sight to ledge
pub fn has_line_of_sight_to_ledge(_start_pos: Vec3, _end_pos: Vec3, _layer_mask: u32) -> bool {
    // TODO: Implement raycast logic
    true
}

/// Utility function to check if there's space above ledge
pub fn has_space_above_ledge(_ledge_position: Vec3, _player_height: f32, _layer_mask: u32) -> bool {
    // TODO: Implement raycast logic
    true
}

/// Utility function to check if there's space below ledge
pub fn has_space_below_ledge(_ledge_position: Vec3, _player_height: f32, _layer_mask: u32) -> bool {
    // TODO: Implement raycast logic
    true
}

/// Utility function to calculate stamina cost for climb action
pub fn calculate_stamina_cost(climb_distance: f32, surface_type: SurfaceType) -> f32 {
    let base_cost = climb_distance * 2.0;
    base_cost * surface_type.climb_speed_multiplier()
}

/// Utility function to check if stamina is sufficient
pub fn has_sufficient_stamina(stamina: f32, required: f32) -> bool {
    stamina >= required
}

/// Utility function to calculate climb time
pub fn calculate_climb_time(climb_distance: f32, climb_speed: f32) -> f32 {
    climb_distance / climb_speed.max(0.1)
}

/// Utility function to calculate vault time
pub fn calculate_vault_time(vault_distance: f32, vault_speed: f32) -> f32 {
    vault_distance / vault_speed.max(0.1)
}

/// Utility function to calculate jump force
pub fn calculate_jump_force(base_force: f32, stamina_multiplier: f32, surface_multiplier: f32) -> f32 {
    base_force * stamina_multiplier * surface_multiplier
}

/// Utility function to check if player can climb based on current state
pub fn can_climb(
    character: &CharacterController,
    climb_system: &ClimbLedgeSystem,
    state_tracker: &ClimbStateTracker,
) -> bool {
    if !climb_system.climb_ledge_active {
        return false;
    }
    if !climb_system.can_use_climb_ledge {
        return false;
    }
    if character.is_dead {
        return false;
    }
    if character.zero_gravity_mode || character.free_floating_mode {
        return false;
    }
    if state_tracker.is_stamina_depleted {
        return false;
    }
    if climb_system.climb_ledge_action_paused {
        return false;
    }
    true
}

/// Utility function to check if player can grab surface on air
pub fn can_grab_surface_on_air(
    character: &CharacterController,
    climb_system: &ClimbLedgeSystem,
    grab_surface: &GrabSurfaceOnAir,
) -> bool {
    if !climb_system.can_grab_any_surface_on_air {
        return false;
    }
    if !grab_surface.can_grab {
        return false;
    }
    if character.is_dead {
        return false;
    }
    if climb_system.climbing_ledge {
        return false;
    }
    if climb_system.climb_ledge_action_paused {
        return false;
    }
    if grab_surface.use_amount_limit && grab_surface.current_amount >= grab_surface.amount_limit {
        return false;
    }
    true
}

/// Utility function to check if player can auto-hang
pub fn can_auto_hang(
    character: &CharacterController,
    climb_system: &ClimbLedgeSystem,
    auto_hang: &AutoHang,
) -> bool {
    if !climb_system.check_for_hang_from_ledge_on_ground {
        return false;
    }
    if !auto_hang.active {
        return false;
    }
    if character.is_dead {
        return false;
    }
    if climb_system.climbing_ledge {
        return false;
    }
    if climb_system.grabbing_surface {
        return false;
    }
    true
}

/// Utility function to check if player is on ground
pub fn is_player_on_ground(character: &CharacterController, ground_detection: &GroundDetection) -> bool {
    ground_detection.is_grounded && !character.zero_gravity_mode
}

/// Utility function to check if player is moving
pub fn is_player_moving(velocity: Vec3, threshold: f32) -> bool {
    velocity.length() > threshold
}

/// Utility function to check if player is moving forward
pub fn is_player_moving_forward(velocity: Vec3, forward: Vec3, threshold: f32) -> bool {
    let dot = velocity.dot(forward);
    dot > threshold
}

/// Utility function to check if player is looking at ledge
pub fn is_player_looking_at_ledge(player_forward: Vec3, ledge_direction: Vec3, max_angle: f32) -> bool {
    let angle = player_forward.angle_between(ledge_direction).to_degrees();
    angle <= max_angle
}

/// Utility function to calculate screen position for UI icon
pub fn calculate_screen_position(_world_position: Vec3, _camera: &Camera, _transform: &Transform) -> Option<Vec2> {
    // TODO: Implement camera projection logic
    None
}

/// Utility function to check if target is on screen
pub fn is_target_on_screen(screen_position: Vec2, screen_size: Vec2) -> bool {
    screen_position.x >= 0.0 && screen_position.x <= screen_size.x &&
    screen_position.y >= 0.0 && screen_position.y <= screen_size.y
}

/// Utility function to calculate distance to camera center
pub fn distance_to_camera_center(screen_position: Vec2, screen_center: Vec2) -> f32 {
    screen_position.distance(screen_center)
}

/// Utility function to check if target is close enough to camera center
pub fn is_close_to_camera_center(
    screen_position: Vec2,
    screen_center: Vec2,
    max_distance: f32,
) -> bool {
    distance_to_camera_center(screen_position, screen_center) <= max_distance
}

/// Utility function to calculate rotation toward target
pub fn calculate_rotation_toward_target(current_rotation: Quat, target_direction: Vec3, up_vector: Vec3) -> Quat {
    Quat::from_rotation_arc(current_rotation * Vec3::Z, target_direction)
}

/// Utility function to interpolate position
pub fn interpolate_position(
    current_position: Vec3,
    target_position: Vec3,
    t: f32,
) -> Vec3 {
    current_position.lerp(target_position, t.clamp(0.0, 1.0))
}

/// Utility function to interpolate rotation
pub fn interpolate_rotation(
    current_rotation: Quat,
    target_rotation: Quat,
    t: f32,
) -> Quat {
    current_rotation.slerp(target_rotation, t.clamp(0.0, 1.0))
}

/// Utility function to check if position is close enough
pub fn is_position_close_enough(current: Vec3, target: Vec3, threshold: f32) -> bool {
    current.distance(target) <= threshold
}

/// Utility function to check if rotation is close enough
pub fn is_rotation_close_enough(current: Quat, target: Quat, threshold_degrees: f32) -> bool {
    let angle = current.angle_between(target).to_degrees();
    angle <= threshold_degrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_type_climb_speed_multiplier() {
        assert_eq!(SurfaceType::Default.climb_speed_multiplier(), 1.0);
        assert_eq!(SurfaceType::Stone.climb_speed_multiplier(), 1.0);
        assert_eq!(SurfaceType::Wood.climb_speed_multiplier(), 0.9);
        assert_eq!(SurfaceType::Metal.climb_speed_multiplier(), 0.8);
        assert_eq!(SurfaceType::Ice.climb_speed_multiplier(), 1.2);
        assert_eq!(SurfaceType::Rope.climb_speed_multiplier(), 0.7);
        assert_eq!(SurfaceType::Custom(1.5).climb_speed_multiplier(), 1.5);
    }

    #[test]
    fn test_is_surface_climbable() {
        let vertical_normal = Vec3::UP;
        let wall_normal = Vec3::X;
        let slope_normal = Vec3::new(0.5, 0.5, 0.0).normalize();

        assert!(!is_surface_climbable(vertical_normal, 45.0));
        assert!(is_surface_climbable(wall_normal, 45.0));
        assert!(is_surface_climbable(slope_normal, 45.0));
    }

    #[test]
    fn test_is_surface_hangable() {
        let vertical_normal = Vec3::UP;
        let wall_normal = Vec3::X;
        let ceiling_normal = Vec3::DOWN;

        assert!(!is_surface_hangable(vertical_normal, 45.0));
        assert!(is_surface_hangable(wall_normal, 45.0));
        assert!(!is_surface_hangable(ceiling_normal, 45.0));
    }

    #[test]
    fn test_calculate_ledge_height() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(0.0, 2.0, 0.0);
        assert_eq!(calculate_ledge_height(start, end), 2.0);
    }

    #[test]
    fn test_calculate_ledge_distance() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(calculate_ledge_distance(start, end), 5.0);
    }

    #[test]
    fn test_calculate_climb_direction() {
        let player_forward = Vec3::FORWARD;
        let ledge_normal = Vec3::X;
        let direction = calculate_climb_direction(player_forward, ledge_normal);
        assert_eq!(direction, -ledge_normal);
    }

    #[test]
    fn test_calculate_jump_direction() {
        let ledge_normal = Vec3::X;
        let direction = calculate_jump_direction(ledge_normal);
        assert_eq!(direction, Vec3::UP);
    }

    #[test]
    fn test_calculate_stamina_cost() {
        let cost = calculate_stamina_cost(5.0, SurfaceType::Default);
        assert_eq!(cost, 10.0);

        let cost_ice = calculate_stamina_cost(5.0, SurfaceType::Ice);
        assert_eq!(cost_ice, 12.0);
    }

    #[test]
    fn test_has_sufficient_stamina() {
        assert!(has_sufficient_stamina(100.0, 50.0));
        assert!(!has_sufficient_stamina(30.0, 50.0));
    }

    #[test]
    fn test_calculate_climb_time() {
        let time = calculate_climb_time(10.0, 2.0);
        assert_eq!(time, 5.0);
    }

    #[test]
    fn test_calculate_jump_force() {
        let force = calculate_jump_force(10.0, 1.0, 1.0);
        assert_eq!(force, 10.0);

        let force_with_multiplier = calculate_jump_force(10.0, 1.2, 0.8);
        assert_eq!(force_with_multiplier, 9.6);
    }

    #[test]
    fn test_is_position_close_enough() {
        let pos1 = Vec3::new(0.0, 0.0, 0.0);
        let pos2 = Vec3::new(0.1, 0.0, 0.0);
        assert!(is_position_close_enough(pos1, pos2, 0.2));
        assert!(!is_position_close_enough(pos1, pos2, 0.05));
    }

    #[test]
    fn test_is_rotation_close_enough() {
        let rot1 = Quat::IDENTITY;
        let rot2 = Quat::from_rotation_y(0.1);
        assert!(is_rotation_close_enough(rot1, rot2, 10.0));
        assert!(!is_rotation_close_enough(rot1, rot2, 1.0));
    }

    #[test]
    fn test_interpolate_position() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let result = interpolate_position(start, end, 0.5);
        assert_eq!(result, Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_interpolate_rotation() {
        let start = Quat::IDENTITY;
        let end = Quat::from_rotation_y(std::f32::consts::PI);
        let result = interpolate_rotation(start, end, 0.5);
        let angle = result.angle_between(Quat::from_rotation_y(std::f32::consts::PI / 2.0));
        assert!(angle < 0.01);
    }

    #[test]
    fn test_is_moving_toward_ledge() {
        let velocity = Vec3::new(1.0, 0.0, 0.0);
        let ledge_direction = Vec3::new(1.0, 0.0, 0.0);
        assert!(is_moving_toward_ledge(velocity, ledge_direction));

        let velocity = Vec3::new(-1.0, 0.0, 0.0);
        assert!(!is_moving_toward_ledge(velocity, ledge_direction));
    }

    #[test]
    fn test_is_player_looking_at_ledge() {
        let player_forward = Vec3::FORWARD;
        let ledge_direction = Vec3::FORWARD;
        assert!(is_player_looking_at_ledge(player_forward, ledge_direction, 45.0));

        let ledge_direction = Vec3::BACK;
        assert!(!is_player_looking_at_ledge(player_forward, ledge_direction, 45.0));
    }

    #[test]
    fn test_distance_to_camera_center() {
        let screen_pos = Vec2::new(100.0, 100.0);
        let screen_center = Vec2::new(0.0, 0.0);
        let distance = distance_to_camera_center(screen_pos, screen_center);
        assert_eq!(distance, 100.0 * 2.0_f32.sqrt());
    }

    #[test]
    fn test_is_close_to_camera_center() {
        let screen_pos = Vec2::new(10.0, 10.0);
        let screen_center = Vec2::new(0.0, 0.0);
        assert!(is_close_to_camera_center(screen_pos, screen_center, 20.0));
        assert!(!is_close_to_camera_center(screen_pos, screen_center, 10.0));
    }

    #[test]
    fn test_is_target_on_screen() {
        let screen_pos = Vec2::new(100.0, 100.0);
        let screen_size = Vec2::new(1920.0, 1080.0);
        assert!(is_target_on_screen(screen_pos, screen_size));

        let screen_pos = Vec2::new(-10.0, 100.0);
        assert!(!is_target_on_screen(screen_pos, screen_size));
    }

    #[test]
    fn test_calculate_rotation_toward_target() {
        let current_rotation = Quat::IDENTITY;
        let target_direction = Vec3::X;
        let up_vector = Vec3::UP;
        let result = calculate_rotation_toward_target(current_rotation, target_direction, up_vector);
        let angle = result.angle_between(Quat::from_rotation_y(std::f32::consts::PI / 2.0));
        assert!(angle < 0.01);
    }

    #[test]
    fn test_calculate_hand_position() {
        let ledge_position = Vec3::new(0.0, 2.0, 0.0);
        let ledge_normal = Vec3::X;
        let hand_offset = 0.5;
        let result = calculate_hand_position(ledge_position, ledge_normal, hand_offset);
        assert_eq!(result, Vec3::new(0.5, 2.0, 0.0));
    }

    #[test]
    fn test_calculate_target_rotation() {
        let ledge_normal = Vec3::X;
        let up_vector = Vec3::UP;
        let result = calculate_target_rotation(ledge_normal, up_vector);
        let expected = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        let angle = result.angle_between(expected);
        assert!(angle < 0.01);
    }

    #[test]
    fn test_can_climb() {
        let character = CharacterController::default();
        let climb_system = ClimbLedgeSystem::default();
        let state_tracker = ClimbStateTracker::default();

        assert!(can_climb(&character, &climb_system, &state_tracker));

        let mut character_dead = character.clone();
        character_dead.is_dead = true;
        assert!(!can_climb(&character_dead, &climb_system, &state_tracker));

        let mut climb_system_paused = climb_system.clone();
        climb_system_paused.climb_ledge_action_paused = true;
        assert!(!can_climb(&character, &climb_system_paused, &state_tracker));

        let mut state_tracker_depleted = state_tracker.clone();
        state_tracker_depleted.is_stamina_depleted = true;
        assert!(!can_climb(&character, &climb_system, &state_tracker_depleted));
    }

    #[test]
    fn test_can_grab_surface_on_air() {
        let character = CharacterController::default();
        let climb_system = ClimbLedgeSystem::default();
        let grab_surface = GrabSurfaceOnAir::default();

        assert!(!can_grab_surface_on_air(&character, &climb_system, &grab_surface));

        let mut climb_system_grab = climb_system.clone();
        climb_system_grab.can_grab_any_surface_on_air = true;

        let mut grab_surface_grab = grab_surface.clone();
        grab_surface_grab.can_grab = true;

        assert!(can_grab_surface_on_air(&character, &climb_system_grab, &grab_surface_grab));

        let mut character_dead = character.clone();
        character_dead.is_dead = true;
        assert!(!can_grab_surface_on_air(&character_dead, &climb_system_grab, &grab_surface_grab));
    }

    #[test]
    fn test_can_auto_hang() {
        let character = CharacterController::default();
        let climb_system = ClimbLedgeSystem::default();
        let auto_hang = AutoHang::default();

        assert!(!can_auto_hang(&character, &climb_system, &auto_hang));

        let mut climb_system_hang = climb_system.clone();
        climb_system_hang.check_for_hang_from_ledge_on_ground = true;

        let mut auto_hang_active = auto_hang.clone();
        auto_hang_active.active = true;

        assert!(can_auto_hang(&character, &climb_system_hang, &auto_hang_active));

        let mut character_dead = character.clone();
        character_dead.is_dead = true;
        assert!(!can_auto_hang(&character_dead, &climb_system_hang, &auto_hang_active));
    }
}

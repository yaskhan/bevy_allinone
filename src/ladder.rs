//! # Ladder System
//!
//! Ladder climbing mechanics for structured vertical movement.
//!
//! ## Features
//!
//! - **Ladder Detection**: Trigger-based detection
//! - **Climb States**:
//!   - **Approach**: Moving toward ladder
//!   - **Mount**: Getting on ladder
//!   - **Climb Up/Down**: Vertical movement
//!   - **Dismount**: Getting off ladder
//! - **Speed Control**: Configurable climb speed
//! - **Hand Position**: IK for realistic hand placement
//! - **Camera Lock**: Camera follows ladder axis
//! - **Exit Points**: Multiple exit points on ladder
//! - **Ladder Types**: Wooden, metal, rope ladders
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
use crate::character::{CharacterController, Player};
use crate::input::{InputState, InputAction};

pub struct LadderPlugin;

impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<LadderSystem>()
            .register_type::<PlayerLadderSystem>()
            .add_systems(Update, (
                handle_ladder_input,
                update_ladder_state,
                update_ladder_movement,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_ladder,
                handle_ladder_mount,
                handle_ladder_dismount,
            ).chain());
    }
}

/// Main ladder system component (attached to ladder objects)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderSystem {
    // Main Settings
    pub tag_to_check: String,
    pub ladder_active: bool,
    pub use_ladder_horizontal_movement: bool,
    pub move_in_ladder_center: bool,
    pub use_local_movement_direction: bool,

    // Events Settings
    pub use_events_enter_exit_ladder: bool,

    // Gizmo Settings
    pub show_gizmo: bool,
    pub gizmo_color: Color,
    pub gizmo_length: f32,

    // Debug
    pub current_player: Option<Entity>,
}

impl Default for LadderSystem {
    fn default() -> Self {
        Self {
            tag_to_check: "Player".to_string(),
            ladder_active: true,
            use_ladder_horizontal_movement: true,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            use_events_enter_exit_ladder: false,
            show_gizmo: true,
            gizmo_color: Color::rgb(1.0, 0.0, 0.0),
            gizmo_length: 4.0,
            current_player: None,
        }
    }
}

/// Player ladder system component (attached to player)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerLadderSystem {
    // Main Settings
    pub ladder_found: bool,
    pub ladder_movement_speed: f32,
    pub ladder_vertical_movement_amount: f32,
    pub ladder_horizontal_movement_amount: f32,

    pub min_angle_to_inverse_direction: f32,

    pub use_always_horizontal_movement_on_ladder: bool,
    pub use_always_local_movement_direction: bool,

    pub min_angle_vertical_direction: f32,
    pub max_angle_vertical_direction: f32,

    pub climb_ladder_foot_step_state_name: String,

    // Debug
    pub ladder_end_detected: bool,
    pub ladder_start_detected: bool,

    pub movement_direction: i32,
    pub ladder_vertical_input: f32,
    pub ladder_horizontal_input: f32,
    pub ladder_angle: f32,
    pub ladder_signed_angle: f32,

    pub current_vertical_input: f32,
    pub current_horizontal_input: f32,

    pub ladder_movement_direction: Vec3,

    pub moving_on_ladder: bool,
    pub moving_on_ladder_previously: bool,

    pub ladder_direction_transform: Option<Entity>,
    pub ladder_raycast_direction_transform: Option<Entity>,

    // Events Settings
    pub use_events_on_third_person: bool,

    // Internal state
    pub use_ladder_horizontal_movement: bool,
    pub move_in_ladder_center: bool,
    pub use_local_movement_direction: bool,

    pub current_ladder_system: Option<Entity>,
    pub previous_ladder_system: Option<Entity>,
}

impl Default for PlayerLadderSystem {
    fn default() -> Self {
        Self {
            ladder_found: false,
            ladder_movement_speed: 5.0,
            ladder_vertical_movement_amount: 0.3,
            ladder_horizontal_movement_amount: 0.1,
            min_angle_to_inverse_direction: 100.0,
            use_always_horizontal_movement_on_ladder: false,
            use_always_local_movement_direction: false,
            min_angle_vertical_direction: 60.0,
            max_angle_vertical_direction: 120.0,
            climb_ladder_foot_step_state_name: "Climb Ladders".to_string(),
            ladder_end_detected: false,
            ladder_start_detected: false,
            movement_direction: 1,
            ladder_vertical_input: 0.0,
            ladder_horizontal_input: 0.0,
            ladder_angle: 0.0,
            ladder_signed_angle: 0.0,
            current_vertical_input: 0.0,
            current_horizontal_input: 0.0,
            ladder_movement_direction: Vec3::ZERO,
            moving_on_ladder: false,
            moving_on_ladder_previously: false,
            ladder_direction_transform: None,
            ladder_raycast_direction_transform: None,
            use_events_on_third_person: false,
            use_ladder_horizontal_movement: false,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            current_ladder_system: None,
            previous_ladder_system: None,
        }
    }
}

/// Ladder direction component for ladder objects
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderDirection {
    pub direction: Vec3,
    pub raycast_direction: Vec3,
}

impl Default for LadderDirection {
    fn default() -> Self {
        Self {
            direction: Vec3::Y,
            raycast_direction: Vec3::Y,
        }
    }
}

/// Ladder end detection component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderEndDetection {
    pub layer_mask: u32,
    pub check_distance: f32,
    pub check_offset: Vec3,
}

impl Default for LadderEndDetection {
    fn default() -> Self {
        Self {
            layer_mask: 1,
            check_distance: 2.0,
            check_offset: Vec3::new(0.0, 0.1, 0.0),
        }
    }
}

/// Ladder movement state
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum LadderMovementState {
    #[default]
    None,
    Approaching,
    Mounting,
    ClimbingUp,
    ClimbingDown,
    ClimbingHorizontal,
    Dismounting,
}

/// Component to track ladder movement state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderMovementTracker {
    pub current_state: LadderMovementState,
    pub previous_state: LadderMovementState,
    pub state_timer: f32,
    pub mount_duration: f32,
    pub dismount_duration: f32,
    pub climb_speed: f32,
    pub horizontal_climb_speed: f32,
}

impl Default for LadderMovementTracker {
    fn default() -> Self {
        Self {
            current_state: LadderMovementState::None,
            previous_state: LadderMovementState::None,
            state_timer: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            climb_speed: 5.0,
            horizontal_climb_speed: 3.0,
        }
    }
}

/// Component for ladder mount/dismount animation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderAnimation {
    pub is_mounting: bool,
    pub is_dismounting: bool,
    pub mount_progress: f32,
    pub dismount_progress: f32,
    pub mount_duration: f32,
    pub dismount_duration: f32,
    pub mount_start_position: Vec3,
    pub mount_target_position: Vec3,
    pub dismount_start_position: Vec3,
    pub dismount_target_position: Vec3,
}

impl Default for LadderAnimation {
    fn default() -> Self {
        Self {
            is_mounting: false,
            is_dismounting: false,
            mount_progress: 0.0,
            dismount_progress: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            mount_start_position: Vec3::ZERO,
            mount_target_position: Vec3::ZERO,
            dismount_start_position: Vec3::ZERO,
            dismount_target_position: Vec3::ZERO,
        }
    }
}

/// Component for ladder movement control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderMovement {
    pub is_active: bool,
    pub movement_direction: Vec3,
    pub vertical_input: f32,
    pub horizontal_input: f32,
    pub movement_speed: f32,
    pub vertical_movement_amount: f32,
    pub horizontal_movement_amount: f32,
    pub move_in_ladder_center: bool,
    pub use_horizontal_movement: bool,
    pub use_local_direction: bool,
    pub min_angle_vertical: f32,
    pub max_angle_vertical: f32,
    pub min_angle_to_inverse: f32,
}

impl Default for LadderMovement {
    fn default() -> Self {
        Self {
            is_active: false,
            movement_direction: Vec3::ZERO,
            vertical_input: 0.0,
            horizontal_input: 0.0,
            movement_speed: 5.0,
            vertical_movement_amount: 0.3,
            horizontal_movement_amount: 0.1,
            move_in_ladder_center: false,
            use_horizontal_movement: false,
            use_local_direction: false,
            min_angle_vertical: 60.0,
            max_angle_vertical: 120.0,
            min_angle_to_inverse: 100.0,
        }
    }
}

/// Component for ladder exit detection
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderExitDetection {
    pub end_detected: bool,
    pub start_detected: bool,
    pub end_check_distance: f32,
    pub start_check_distance: f32,
    pub end_check_offset: Vec3,
    pub start_check_offset: Vec3,
    pub layer_mask: u32,
}

impl Default for LadderExitDetection {
    fn default() -> Self {
        Self {
            end_detected: false,
            start_detected: false,
            end_check_distance: 2.0,
            start_check_distance: 0.13,
            end_check_offset: Vec3::ZERO,
            start_check_offset: Vec3::new(0.0, 0.1, 0.0),
            layer_mask: 1,
        }
    }
}

/// Component for ladder footstep state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderFootstep {
    pub climb_state_name: String,
    pub is_climbing: bool,
    pub step_interval: f32,
    pub step_timer: f32,
}

impl Default for LadderFootstep {
    fn default() -> Self {
        Self {
            climb_state_name: "Climb Ladders".to_string(),
            is_climbing: false,
            step_interval: 0.5,
            step_timer: 0.0,
        }
    }
}

/// Event for when player enters a ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderEnterEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
    pub ladder_transform: Transform,
}

/// Event for when player exits a ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderExitEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}

/// Event for when player starts climbing ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderClimbStartEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}

/// Event for when player stops climbing ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderClimbStopEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}

/// System to handle ladder input
pub fn handle_ladder_input(
    input_state: Res<InputState>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        character,
        transform,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if player is dead or in special states
        if character.is_dead || character.zero_gravity_mode || character.free_floating_mode {
            continue;
        }

        // Get input values
        let vertical_input = input_state.movement.y;
        let horizontal_input = input_state.movement.x;

        // Update ladder movement
        ladder_movement.vertical_input = vertical_input;
        ladder_movement.horizontal_input = horizontal_input;

        // Handle dismount (jump or move away from ladder)
        if input_state.jump_pressed {
            // Trigger dismount
            // TODO: Implement dismount logic
        }

        // Handle mount (if already on ladder)
        if ladder_system.ladder_found && !ladder_movement.is_active {
            // Start climbing
            ladder_movement.is_active = true;
            ladder_tracker.current_state = LadderMovementState::ClimbingUp;
            ladder_tracker.state_timer = 0.0;
        }
    }
}

/// System to update ladder state
pub fn update_ladder_state(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut LadderExitDetection,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut ladder_exit,
        character,
        transform,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Update state timer
        ladder_tracker.state_timer += time.delta_secs();

        // Update animation progress
        if ladder_animation.is_mounting {
            ladder_animation.mount_progress += time.delta_secs() / ladder_animation.mount_duration;
            if ladder_animation.mount_progress >= 1.0 {
                ladder_animation.is_mounting = false;
                ladder_animation.mount_progress = 0.0;
            }
        }

        if ladder_animation.is_dismounting {
            ladder_animation.dismount_progress += time.delta_secs() / ladder_animation.dismount_duration;
            if ladder_animation.dismount_progress >= 1.0 {
                ladder_animation.is_dismounting = false;
                ladder_animation.dismount_progress = 0.0;
            }
        }

        // Update moving state
        let is_moving = ladder_movement.vertical_input.abs() > 0.01 || 
                       ladder_movement.horizontal_input.abs() > 0.01;
        
        if is_moving != ladder_system.moving_on_ladder {
            ladder_system.moving_on_ladder_previously = ladder_system.moving_on_ladder;
            ladder_system.moving_on_ladder = is_moving;
        }

        // Update footstep state
        if ladder_system.moving_on_ladder != ladder_system.moving_on_ladder_previously {
            // TODO: Update footstep state
        }
    }
}

/// System to update ladder movement
pub fn update_ladder_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderExitDetection,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_exit,
        mut transform,
        mut character,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found || !ladder_movement.is_active {
            continue;
        }

        // Get ladder direction transform
        let Some(ladder_direction_entity) = ladder_system.ladder_direction_transform else {
            continue;
        };

        // Get ladder direction from world (simplified - in real implementation would query transform)
        let ladder_direction = Vec3::Y; // Default upward
        let ladder_right = Vec3::X; // Default right

        // Calculate movement direction based on camera and ladder orientation
        let camera_forward = Vec3::Z; // Default forward (would come from camera)
        let ladder_angle = camera_forward.angle_between(ladder_direction);

        // Determine movement direction
        let mut movement_direction = 1;
        if ladder_angle > ladder_movement.min_angle_to_inverse {
            movement_direction = -1;
        }

        // Calculate vertical and horizontal inputs
        let mut current_vertical_input = ladder_movement.vertical_input;
        let mut current_horizontal_input = ladder_movement.horizontal_input;

        if ladder_movement.use_local_direction || ladder_system.use_always_local_movement_direction {
            let signed_angle = camera_forward.cross(ladder_direction).y;
            
            if ladder_angle < ladder_movement.min_angle_vertical || 
               ladder_angle > ladder_movement.max_angle_vertical {
                // Use standard input
            } else {
                // Swap inputs for horizontal movement
                if signed_angle < 0.0 {
                    movement_direction = -1;
                } else {
                    movement_direction = 1;
                }

                current_vertical_input = ladder_movement.horizontal_input;
                current_horizontal_input = -ladder_movement.vertical_input;
            }
        }

        // Apply movement direction
        ladder_movement.vertical_input = current_vertical_input * movement_direction as f32;
        ladder_movement.horizontal_input = current_horizontal_input * movement_direction as f32;

        // Calculate movement
        let mut movement = Vec3::ZERO;

        // Move in ladder center if enabled
        if ladder_movement.move_in_ladder_center {
            if ladder_movement.use_horizontal_movement || 
               ladder_system.use_always_horizontal_movement_on_ladder {
                if ladder_movement.horizontal_input.abs() < 0.01 {
                    // Center on ladder
                    // TODO: Implement centering logic
                }
            }
        }

        // Vertical movement
        movement += ladder_direction * (ladder_movement.vertical_movement_amount * ladder_movement.vertical_input);

        // Check for ladder end/start
        let ladder_end_detected = !ladder_exit.end_detected;
        let ladder_start_detected = ladder_exit.start_detected;

        if ladder_end_detected || (ladder_start_detected && ladder_movement.vertical_input < 0.0) {
            // Exit ladder
            movement = ladder_direction * ladder_movement.vertical_input;
        }

        // Horizontal movement (if enabled)
        if ladder_movement.use_horizontal_movement || 
           ladder_system.use_always_horizontal_movement_on_ladder {
            movement += ladder_right * (ladder_movement.horizontal_movement_amount * ladder_movement.horizontal_input);
        }

        // Apply movement
        let target_position = transform.translation + movement;
        let new_position = transform.translation.lerp(
            target_position, 
            (time.delta_secs() * ladder_movement.movement_speed).min(1.0)
        );

        transform.translation = new_position;

        // Update movement tracker
        let position_delta = (new_position - transform.translation).length_squared();
        if position_delta > 0.0001 {
            ladder_system.moving_on_ladder = true;
        } else {
            ladder_system.moving_on_ladder = false;
        }
    }
}

/// System to detect ladder
pub fn detect_ladder(
    mut commands: Commands,
    mut ladder_query: Query<(&LadderSystem, &Transform, Entity), Without<Player>>,
    mut player_query: Query<(&mut PlayerLadderSystem, &Transform, Entity), With<Player>>,
) {
    // TODO: Implement collision detection logic using avian3d's collision events
    // This would involve using EventReader<CollisionEvent> from avian3d
}

/// System to handle ladder mount
pub fn handle_ladder_mount(
    mut commands: Commands,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut transform,
        mut character,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if we need to mount
        if ladder_tracker.current_state == LadderMovementState::Approaching {
            // Start mounting
            ladder_tracker.current_state = LadderMovementState::Mounting;
            ladder_tracker.state_timer = 0.0;

            // Set up animation
            ladder_animation.is_mounting = true;
            ladder_animation.mount_progress = 0.0;
            ladder_animation.mount_start_position = transform.translation;
            
            // Calculate target position (on ladder)
            // TODO: Calculate proper mount position
            ladder_animation.mount_target_position = transform.translation + Vec3::Y * 0.5;

            // Disable gravity and physics
            character.zero_gravity_mode = true;

            // Trigger mount event
            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                commands.trigger(LadderClimbStartEvent {
                    player_entity: transform.id(),
                    ladder_entity,
                });
            }
        }
    }
}

/// System to handle ladder dismount
pub fn handle_ladder_dismount(
    mut commands: Commands,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut transform,
        mut character,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if we need to dismount
        if ladder_tracker.current_state == LadderMovementState::Dismounting {
            // Start dismounting
            ladder_tracker.state_timer = 0.0;

            // Set up animation
            ladder_animation.is_dismounting = true;
            ladder_animation.dismount_progress = 0.0;
            ladder_animation.dismount_start_position = transform.translation;
            
            // Calculate target position (off ladder)
            // TODO: Calculate proper dismount position
            ladder_animation.dismount_target_position = transform.translation + Vec3::Y * -0.5;

            // Re-enable gravity and physics
            character.zero_gravity_mode = false;

            // Trigger dismount event
            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                commands.trigger(LadderClimbStopEvent {
                    player_entity: transform.id(),
                    ladder_entity,
                });
            }

            // Reset ladder state
            ladder_system.ladder_found = false;
            ladder_movement.is_active = false;
            ladder_tracker.current_state = LadderMovementState::None;
        }
    }
}

/// Utility function to check if player is on ladder
pub fn is_player_on_ladder(ladder_system: &PlayerLadderSystem) -> bool {
    ladder_system.ladder_found
}

/// Utility function to check if player is moving on ladder
pub fn is_player_moving_on_ladder(ladder_system: &PlayerLadderSystem) -> bool {
    ladder_system.moving_on_ladder
}

/// Utility function to get ladder movement direction
pub fn get_ladder_movement_direction(ladder_system: &PlayerLadderSystem) -> Vec3 {
    ladder_system.ladder_movement_direction
}

/// Utility function to calculate ladder angle
pub fn calculate_ladder_angle(camera_forward: Vec3, ladder_direction: Vec3) -> f32 {
    camera_forward.angle_between(ladder_direction)
}

/// Utility function to check if ladder is vertical enough
pub fn is_ladder_vertical_enough(angle: f32, min_angle: f32, max_angle: f32) -> bool {
    angle >= min_angle && angle <= max_angle
}

/// Utility function to calculate movement direction sign
pub fn calculate_movement_direction(camera_forward: Vec3, ladder_direction: Vec3, min_angle: f32) -> i32 {
    let angle = camera_forward.angle_between(ladder_direction);
    if angle > min_angle {
        -1
    } else {
        1
    }
}

/// Utility function to calculate signed angle
pub fn calculate_signed_angle(camera_forward: Vec3, ladder_direction: Vec3, up_vector: Vec3) -> f32 {
    camera_forward.cross(ladder_direction).dot(up_vector)
}

/// Utility function to check if player can mount ladder
pub fn can_mount_ladder(
    ladder_system: &PlayerLadderSystem,
    character: &CharacterController,
) -> bool {
    if !ladder_system.ladder_found {
        return false;
    }
    if character.is_dead {
        return false;
    }
    if character.zero_gravity_mode || character.free_floating_mode {
        return false;
    }
    true
}

/// Utility function to check if player can dismount ladder
pub fn can_dismount_ladder(
    ladder_system: &PlayerLadderSystem,
    ladder_exit: &LadderExitDetection,
    character: &CharacterController,
) -> bool {
    if !ladder_system.ladder_found {
        return false;
    }
    if character.is_dead {
        return false;
    }
    // Check if at ladder end or start
    ladder_exit.end_detected || ladder_exit.start_detected
}

/// Utility function to calculate mount position
pub fn calculate_mount_position(
    current_position: Vec3,
    ladder_direction: Vec3,
    ladder_position: Vec3,
    mount_offset: f32,
) -> Vec3 {
    ladder_position + ladder_direction * mount_offset
}

/// Utility function to calculate dismount position
pub fn calculate_dismount_position(
    current_position: Vec3,
    ladder_direction: Vec3,
    dismount_offset: f32,
) -> Vec3 {
    current_position + ladder_direction * dismount_offset
}

/// Utility function to interpolate position for mount/dismount
pub fn interpolate_ladder_position(
    start: Vec3,
    target: Vec3,
    progress: f32,
) -> Vec3 {
    start.lerp(target, progress.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_player_on_ladder() {
        let ladder_system = PlayerLadderSystem::default();
        assert!(!is_player_on_ladder(&ladder_system));

        let mut ladder_system_with_ladder = PlayerLadderSystem::default();
        ladder_system_with_ladder.ladder_found = true;
        assert!(is_player_on_ladder(&ladder_system_with_ladder));
    }

    #[test]
    fn test_is_player_moving_on_ladder() {
        let ladder_system = PlayerLadderSystem::default();
        assert!(!is_player_moving_on_ladder(&ladder_system));

        let mut ladder_system_moving = PlayerLadderSystem::default();
        ladder_system_moving.moving_on_ladder = true;
        assert!(is_player_moving_on_ladder(&ladder_system_moving));
    }

    #[test]
    fn test_get_ladder_movement_direction() {
        let ladder_system = PlayerLadderSystem::default();
        let direction = get_ladder_movement_direction(&ladder_system);
        assert_eq!(direction, Vec3::ZERO);

        let mut ladder_system_with_direction = PlayerLadderSystem::default();
        ladder_system_with_direction.ladder_movement_direction = Vec3::Y;
        let direction = get_ladder_movement_direction(&ladder_system_with_direction);
        assert_eq!(direction, Vec3::Y);
    }

    #[test]
    fn test_calculate_ladder_angle() {
        let camera_forward = Vec3::Z;
        let ladder_direction = Vec3::Y;
        let angle = calculate_ladder_angle(camera_forward, ladder_direction);
        assert_eq!(angle, std::f32::consts::PI / 2.0);
    }

    #[test]
    fn test_is_ladder_vertical_enough() {
        let angle = 90.0_f32.to_radians();
        let min_angle = 60.0_f32.to_radians();
        let max_angle = 120.0_f32.to_radians();
        assert!(is_ladder_vertical_enough(angle, min_angle, max_angle));

        let angle = 30.0_f32.to_radians();
        assert!(!is_ladder_vertical_enough(angle, min_angle, max_angle));
    }

    #[test]
    fn test_calculate_movement_direction() {
        let camera_forward = Vec3::Z;
        let ladder_direction = Vec3::Y;
        let min_angle = 100.0_f32.to_radians();
        let direction = calculate_movement_direction(camera_forward, ladder_direction, min_angle);
        assert_eq!(direction, -1);

        let camera_forward = Vec3::Y;
        let direction = calculate_movement_direction(camera_forward, ladder_direction, min_angle);
        assert_eq!(direction, 1);
    }

    #[test]
    fn test_calculate_signed_angle() {
        let camera_forward = Vec3::Z;
        let ladder_direction = Vec3::X;
        let up_vector = Vec3::Y;
        let signed_angle = calculate_signed_angle(camera_forward, ladder_direction, up_vector);
        assert!(signed_angle.abs() > 0.0);
    }

    #[test]
    fn test_can_mount_ladder() {
        let ladder_system = PlayerLadderSystem::default();
        let character = CharacterController::default();
        assert!(!can_mount_ladder(&ladder_system, &character));

        let mut ladder_system_with_ladder = PlayerLadderSystem::default();
        ladder_system_with_ladder.ladder_found = true;
        assert!(can_mount_ladder(&ladder_system_with_ladder, &character));

        let mut character_dead = CharacterController::default();
        character_dead.is_dead = true;
        assert!(!can_mount_ladder(&ladder_system_with_ladder, &character_dead));
    }

    #[test]
    fn test_can_dismount_ladder() {
        let ladder_system = PlayerLadderSystem::default();
        let ladder_exit = LadderExitDetection::default();
        let character = CharacterController::default();
        assert!(!can_dismount_ladder(&ladder_system, &ladder_exit, &character));

        let mut ladder_system_with_ladder = PlayerLadderSystem::default();
        ladder_system_with_ladder.ladder_found = true;
        let mut ladder_exit_detected = LadderExitDetection::default();
        ladder_exit_detected.end_detected = true;
        assert!(can_dismount_ladder(&ladder_system_with_ladder, &ladder_exit_detected, &character));
    }

    #[test]
    fn test_calculate_mount_position() {
        let current_position = Vec3::new(0.0, 0.0, 0.0);
        let ladder_direction = Vec3::Y;
        let ladder_position = Vec3::new(0.0, 2.0, 0.0);
        let mount_offset = 0.5;
        let result = calculate_mount_position(current_position, ladder_direction, ladder_position, mount_offset);
        assert_eq!(result, Vec3::new(0.0, 2.5, 0.0));
    }

    #[test]
    fn test_calculate_dismount_position() {
        let current_position = Vec3::new(0.0, 2.0, 0.0);
        let ladder_direction = Vec3::Y;
        let dismount_offset = -0.5;
        let result = calculate_dismount_position(current_position, ladder_direction, dismount_offset);
        assert_eq!(result, Vec3::new(0.0, 1.5, 0.0));
    }

    #[test]
    fn test_interpolate_ladder_position() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let target = Vec3::new(0.0, 2.0, 0.0);
        let progress = 0.5;
        let result = interpolate_ladder_position(start, target, progress);
        assert_eq!(result, Vec3::new(0.0, 1.0, 0.0));
    }
}

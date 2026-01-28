//! Puzzle System Module
//!
//! Interactive puzzles and environmental challenges.
//!
//! ## Overview
//!
//! This module provides a comprehensive puzzle system for creating interactive
//! environmental challenges. It supports multiple puzzle types including:
//!
//! - **Logic Puzzles**: Riddles, pattern matching, sequence puzzles
//! - **Physics Puzzles**: Object manipulation and placement
//! - **Timing Puzzles**: Rhythm-based challenges
//! - **Combination Puzzles**: Multi-step solutions
//! - **Environmental Puzzles**: Using world elements
//!
//! ## Puzzle Components
//!
//! - **Buttons/Switches**: Activatable objects with toggle states
//! - **Levers**: State-changing objects with position tracking
//! - **Pressure Plates**: Weight-based triggers
//! - **Keys/Locks**: Item-based solutions
//! - **Puzzle Pieces**: Collectible components
//! - **Sequence Puzzles**: Press objects in correct order
//! - **Piano Systems**: Musical puzzle interfaces
//!
//! ## Puzzle States
//!
//! - **Unsolved**: Initial state
//! - **In Progress**: Puzzle being worked on
//! - **Solved**: Puzzle completed successfully
//! - **Failed**: Puzzle failed (optional)
//!
//! ## Integration
//!
//! The puzzle system integrates with:
//! - **Interaction System**: For player interaction
//! - **Event System**: For puzzle state changes
//! - **Inventory System**: For key-based puzzles
//! - **Audio System**: For feedback sounds


use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};
use crate::interaction::{Interactable, InteractionType, InteractionEventQueue, InteractionEvent};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components for reflection
            .register_type::<PuzzleSystem>()
            .register_type::<PuzzleButton>()
            .register_type::<PuzzleLever>()
            .register_type::<PuzzlePressurePlate>()
            .register_type::<PuzzleLock>()
            .register_type::<PuzzleKey>()
            .register_type::<PuzzleSequence>()
            .register_type::<PuzzleSequenceItem>()
            .register_type::<PuzzlePiano>()
            .register_type::<PuzzlePianoKey>()
            .register_type::<PuzzleObjectPlacement>()
            .register_type::<PuzzleDraggable>()
            .register_type::<PuzzleState>()
            .register_type::<PuzzleEvent>()
            .register_type::<PuzzleSolvedEvent>()
            .register_type::<PuzzleFailedEvent>()
            .register_type::<PuzzleResetEvent>()
            .register_type::<PuzzleProgress>()
            .register_type::<PuzzleHint>()
            .register_type::<PuzzleTimer>()
            .register_type::<PuzzleSound>()
            .register_type::<PuzzleGizmo>()
            .register_type::<PuzzleDebug>()
            // Resources
            .init_resource::<PuzzleEventQueue>()
            .init_resource::<PuzzleSolvedEventQueue>()
            .init_resource::<PuzzleFailedEventQueue>()
            .init_resource::<PuzzleResetEventQueue>()
            .init_resource::<PuzzleDebugSettings>()
            // Systems
            .add_systems(Update, (
                update_puzzle_buttons,
                update_puzzle_levers,
                update_puzzle_pressure_plates,
                update_puzzle_locks,
                update_puzzle_sequences,
                update_puzzle_pianos,
                update_puzzle_object_placements,
                update_puzzle_draggables,
                update_puzzle_timers,
                process_puzzle_events,
                update_puzzle_ui,
                debug_draw_puzzle_gizmos,
            ).chain())
            .add_systems(Startup, setup_puzzle_ui);
    }
}

// ============================================================================
// Puzzle System Core Components
// ============================================================================

/// Main puzzle system component for drag & drop puzzles
/// Reference: `gkit/Scripts/Puzzles/puzzleSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleSystem {
    /// List of draggable objects in the puzzle
    pub draggable_elements: Vec<Entity>,
    /// Whether to respawn draggable elements when they leave the puzzle area
    pub respawn_draggable: bool,
    /// Whether to respawn in original position or at a respawn point
    pub respawn_in_original_position: bool,
    /// Respawn position (if not using original position)
    pub respawn_position: Option<Vec3>,
    /// Drag velocity multiplier
    pub drag_velocity: f32,
    /// Maximum raycast distance for object detection
    pub max_raycast_distance: f32,
    /// Whether to freeze rotation when grabbing objects
    pub freeze_rotation_when_grabbed: bool,
    /// Whether objects can be rotated
    pub can_rotate_objects: bool,
    /// Rotation speed
    pub rotate_speed: f32,
    /// Horizontal rotation enabled
    pub horizontal_rotation_enabled: bool,
    /// Vertical rotation enabled
    pub vertical_rotation_enabled: bool,
    /// Rotate to camera in fixed position
    pub rotate_to_camera_fixed: bool,
    /// Rotation speed when rotating to camera
    pub camera_rotation_speed: f32,
    /// Change hold distance speed
    pub change_hold_distance_speed: f32,
    /// Number of objects required to place to solve
    pub number_objects_to_place: u32,
    /// Disable object action after resolve
    pub disable_action_after_resolve: bool,
    /// Wait delay before stopping use after resolve
    pub wait_delay_after_resolve: f32,
    /// Whether puzzle is solved
    pub solved: bool,
    /// Whether player is currently using puzzle
    pub using_puzzle: bool,
    /// Current number of objects placed
    pub current_objects_placed: u32,
}

impl Default for PuzzleSystem {
    fn default() -> Self {
        Self {
            draggable_elements: Vec::new(),
            respawn_draggable: false,
            respawn_in_original_position: true,
            respawn_position: None,
            drag_velocity: 8.0,
            max_raycast_distance: 10.0,
            freeze_rotation_when_grabbed: true,
            can_rotate_objects: false,
            rotate_speed: 30.0,
            horizontal_rotation_enabled: true,
            vertical_rotation_enabled: true,
            rotate_to_camera_fixed: false,
            camera_rotation_speed: 5.0,
            change_hold_distance_speed: 2.0,
            number_objects_to_place: 0,
            disable_action_after_resolve: false,
            wait_delay_after_resolve: 1.0,
            solved: false,
            using_puzzle: false,
            current_objects_placed: 0,
        }
    }
}

/// Component for puzzle buttons/switches
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleButton {
    /// Button name/identifier
    pub name: String,
    /// Whether button is currently pressed
    pub is_pressed: bool,
    /// Whether button can be pressed
    pub can_press: bool,
    /// Cooldown between presses
    pub cooldown: f32,
    /// Current cooldown timer
    pub cooldown_timer: f32,
    /// Press duration (for timed buttons)
    pub press_duration: f32,
    /// Current press timer
    pub press_timer: f32,
    /// Whether button resets automatically
    pub auto_reset: bool,
    /// Reset delay
    pub reset_delay: f32,
    /// Current reset timer
    pub reset_timer: f32,
    /// Visual feedback: press amount
    pub press_amount: f32,
    /// Visual feedback: press speed
    pub press_speed: f32,
    /// Sound effect when pressed
    pub sound_effect: Option<Handle<AudioSource>>,
    /// Whether to use incorrect order sound
    pub use_incorrect_sound: bool,
    /// Incorrect order sound
    pub incorrect_sound: Option<Handle<AudioSource>>,
}

impl Default for PuzzleButton {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_pressed: false,
            can_press: true,
            cooldown: 0.5,
            cooldown_timer: 0.0,
            press_duration: 0.0,
            press_timer: 0.0,
            auto_reset: false,
            reset_delay: 1.0,
            reset_timer: 0.0,
            press_amount: 0.3,
            press_speed: 10.0,
            sound_effect: None,
            use_incorrect_sound: false,
            incorrect_sound: None,
        }
    }
}

/// Component for puzzle levers
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleLever {
    /// Lever name/identifier
    pub name: String,
    /// Current lever state
    pub state: LeverState,
    /// Target state when activated
    pub target_state: LeverState,
    /// Rotation axis
    pub rotation_axis: Vec3,
    /// Rotation angles (min, max)
    pub rotation_range: Vec2,
    /// Rotation speed
    pub rotation_speed: f32,
    /// Whether lever can be moved
    pub can_move: bool,
    /// Sound effect when moved
    pub sound_effect: Option<Handle<AudioSource>>,
    /// Visual feedback: current angle
    pub current_angle: f32,
}

impl Default for PuzzleLever {
    fn default() -> Self {
        Self {
            name: String::new(),
            state: LeverState::Neutral,
            target_state: LeverState::Up,
            rotation_axis: Vec3::Y,
            rotation_range: Vec2::new(-45.0, 45.0),
            rotation_speed: 30.0,
            can_move: true,
            sound_effect: None,
            current_angle: 0.0,
        }
    }
}

/// Lever state
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum LeverState {
    #[default]
    Neutral,
    Up,
    Down,
    Left,
    Right,
}

/// Component for puzzle pressure plates
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzlePressurePlate {
    /// Plate name/identifier
    pub name: String,
    /// Whether plate is currently pressed
    pub is_pressed: bool,
    /// Required weight to activate
    pub required_weight: f32,
    /// Current weight on plate
    pub current_weight: f32,
    /// Whether plate stays pressed after activation
    pub stay_pressed: bool,
    /// Press duration (0 = instant)
    pub press_duration: f32,
    /// Current press timer
    pub press_timer: f32,
    /// Reset delay
    pub reset_delay: f32,
    /// Current reset timer
    pub reset_timer: f32,
    /// Visual feedback: press amount
    pub press_amount: f32,
    /// Sound effect when pressed
    pub sound_effect: Option<Handle<AudioSource>>,
    /// Sound effect when released
    pub release_sound: Option<Handle<AudioSource>>,
}

impl Default for PuzzlePressurePlate {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_pressed: false,
            required_weight: 1.0,
            current_weight: 0.0,
            stay_pressed: false,
            press_duration: 0.0,
            press_timer: 0.0,
            reset_delay: 0.5,
            reset_timer: 0.0,
            press_amount: 0.1,
            sound_effect: None,
            release_sound: None,
        }
    }
}

/// Component for puzzle locks
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleLock {
    /// Lock name/identifier
    pub name: String,
    /// Whether lock is unlocked
    pub is_unlocked: bool,
    /// Required key ID(s)
    pub required_keys: Vec<String>,
    /// Number of keys required
    pub required_key_count: u32,
    /// Current keys inserted
    pub current_keys: Vec<String>,
    /// Whether multiple keys are required
    pub multi_key: bool,
    /// Sound effect when unlocked
    pub unlock_sound: Option<Handle<AudioSource>>,
    /// Sound effect when wrong key
    pub wrong_key_sound: Option<Handle<AudioSource>>,
    /// Visual feedback: lock state
    pub lock_state: LockState,
}

impl Default for PuzzleLock {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_unlocked: false,
            required_keys: Vec::new(),
            required_key_count: 1,
            current_keys: Vec::new(),
            multi_key: false,
            unlock_sound: None,
            wrong_key_sound: None,
            lock_state: LockState::Locked,
        }
    }
}

/// Lock state
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum LockState {
    #[default]
    Locked,
    PartiallyUnlocked,
    Unlocked,
}

/// Component for puzzle keys
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleKey {
    /// Key name/identifier
    pub name: String,
    /// Key ID (matches lock's required_keys)
    pub key_id: String,
    /// Whether key is consumed on use
    pub consumed_on_use: bool,
    /// Whether key can be used multiple times
    pub reusable: bool,
    /// Sound effect when used
    pub use_sound: Option<Handle<AudioSource>>,
    /// Visual feedback: key type
    pub key_type: KeyType,
}

impl Default for PuzzleKey {
    fn default() -> Self {
        Self {
            name: String::new(),
            key_id: String::new(),
            consumed_on_use: true,
            reusable: false,
            use_sound: None,
            key_type: KeyType::Physical,
        }
    }
}

/// Key type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum KeyType {
    #[default]
    Physical,    // Physical key item
    Digital,     // Digital code/password
    Token,       // Token/collectible
    Card,        // Keycard
}

/// Component for puzzle sequence (press in order)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleSequence {
    /// Sequence name/identifier
    pub name: String,
    /// Current correct index in sequence
    pub correct_index: u32,
    /// Whether sequence is complete
    pub complete: bool,
    /// Whether to reset on wrong input
    pub reset_on_wrong: bool,
    /// Sound effect for correct input
    pub correct_sound: Option<Handle<AudioSource>>,
    /// Sound effect for incorrect input
    pub incorrect_sound: Option<Handle<AudioSource>>,
    /// Visual feedback: current progress
    pub progress: f32,
}

impl Default for PuzzleSequence {
    fn default() -> Self {
        Self {
            name: String::new(),
            correct_index: 0,
            complete: false,
            reset_on_wrong: true,
            correct_sound: None,
            incorrect_sound: None,
            progress: 0.0,
        }
    }
}

/// Component for individual sequence items
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleSequenceItem {
    /// Item name/identifier
    pub name: String,
    /// Expected order index
    pub order_index: u32,
    /// Whether this item has been pressed
    pub pressed: bool,
    /// Visual feedback: press amount
    pub press_amount: f32,
    /// Sound effect when pressed
    pub sound_effect: Option<Handle<AudioSource>>,
}

impl Default for PuzzleSequenceItem {
    fn default() -> Self {
        Self {
            name: String::new(),
            order_index: 0,
            pressed: false,
            press_amount: 0.3,
            sound_effect: None,
        }
    }
}

/// Component for piano system
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzlePiano {
    /// Piano name/identifier
    pub name: String,
    /// Whether piano is being used
    pub using_piano: bool,
    /// Key rotation amount (degrees)
    pub key_rotation_amount: f32,
    /// Key rotation speed
    pub key_rotation_speed: f32,
    /// Song to play (auto-play)
    pub song_to_play: String,
    /// Play rate (seconds between notes)
    pub play_rate: f32,
    /// Whether to use event when auto-play ends
    pub use_event_on_auto_play_end: bool,
    /// Song line length (for formatting)
    pub song_line_length: u32,
    /// Song line delay
    pub song_line_delay: f32,
    /// Current auto-play coroutine
    pub auto_play_coroutine: Option<Handle<AudioSource>>,
}

impl Default for PuzzlePiano {
    fn default() -> Self {
        Self {
            name: String::new(),
            using_piano: false,
            key_rotation_amount: 30.0,
            key_rotation_speed: 30.0,
            song_to_play: String::new(),
            play_rate: 0.3,
            use_event_on_auto_play_end: false,
            song_line_length: 8,
            song_line_delay: 0.5,
            auto_play_coroutine: None,
        }
    }
}

/// Component for piano keys
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzlePianoKey {
    /// Key name/identifier (e.g., "C", "D", "E")
    pub name: String,
    /// Key sound
    pub sound: Option<Handle<AudioSource>>,
    /// Current rotation
    pub current_rotation: f32,
    /// Target rotation when pressed
    pub target_rotation: f32,
    /// Rotation speed
    pub rotation_speed: f32,
    /// Whether key is currently pressed
    pub is_pressed: bool,
}

impl Default for PuzzlePianoKey {
    fn default() -> Self {
        Self {
            name: String::new(),
            sound: None,
            current_rotation: 0.0,
            target_rotation: -30.0,
            rotation_speed: 30.0,
            is_pressed: false,
        }
    }
}

/// Component for object placement puzzles
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleObjectPlacement {
    /// Placement name/identifier
    pub name: String,
    /// Object to place
    pub object_to_place: Option<Entity>,
    /// Whether object is placed
    pub is_placed: bool,
    /// Place object position speed
    pub place_position_speed: f32,
    /// Place object rotation speed
    pub place_rotation_speed: f32,
    /// Whether to use rotation limit
    pub use_rotation_limit: bool,
    /// Max up rotation angle
    pub max_up_rotation: f32,
    /// Max forward rotation angle
    pub max_forward_rotation: f32,
    /// Whether to use position limit
    pub use_position_limit: bool,
    /// Max position distance
    pub max_position_distance: f32,
    /// Whether needs other objects placed before
    pub needs_other_objects_before: bool,
    /// Number of objects to place before
    pub number_objects_before: u32,
    /// Current number of objects placed
    pub current_objects_placed: u32,
    /// Object inside trigger
    pub object_inside_trigger: bool,
    /// Object in correct position
    pub object_in_correct_position: bool,
    /// Object in correct rotation
    pub object_in_correct_rotation: bool,
}

impl Default for PuzzleObjectPlacement {
    fn default() -> Self {
        Self {
            name: String::new(),
            object_to_place: None,
            is_placed: false,
            place_position_speed: 5.0,
            place_rotation_speed: 10.0,
            use_rotation_limit: false,
            max_up_rotation: 30.0,
            max_forward_rotation: 30.0,
            use_position_limit: false,
            max_position_distance: 1.0,
            needs_other_objects_before: false,
            number_objects_before: 0,
            current_objects_placed: 0,
            object_inside_trigger: false,
            object_in_correct_position: false,
            object_in_correct_rotation: false,
        }
    }
}

/// Component for draggable objects in puzzles
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleDraggable {
    /// Draggable name/identifier
    pub name: String,
    /// Whether object is currently grabbed
    pub is_grabbed: bool,
    /// Whether object can be grabbed
    pub can_grab: bool,
    /// Original position (for respawn)
    pub original_position: Vec3,
    /// Original rotation (for respawn)
    pub original_rotation: Quat,
    /// Current hold distance
    pub hold_distance: f32,
    /// Whether object is rotating
    pub is_rotating: bool,
    /// Whether rotation is frozen when grabbed
    pub freeze_rotation: bool,
    // Collider material when grabbed
    // pub grabbed_collider_material: Option<Handle<Collider>>,
    // Original collider material
    // pub original_collider_material: Option<Handle<Collider>>,
}

impl Default for PuzzleDraggable {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_grabbed: false,
            can_grab: true,
            original_position: Vec3::ZERO,
            original_rotation: Quat::IDENTITY,
            hold_distance: 5.0,
            is_rotating: false,
            freeze_rotation: true,
        }
    }
}

/// Puzzle state
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum PuzzleState {
    #[default]
    Unsolved,
    InProgress,
    Solved,
    Failed,
}

/// Component for puzzle progress tracking
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleProgress {
    /// Current puzzle state
    pub state: PuzzleState,
    /// Progress percentage (0.0 to 1.0)
    pub progress: f32,
    /// Total steps required
    pub total_steps: u32,
    /// Current step
    pub current_step: u32,
    /// Whether puzzle can be reset
    pub can_reset: bool,
    /// Reset count
    pub reset_count: u32,
    /// Time spent on puzzle
    pub time_spent: f32,
}

impl Default for PuzzleProgress {
    fn default() -> Self {
        Self {
            state: PuzzleState::Unsolved,
            progress: 0.0,
            total_steps: 1,
            current_step: 0,
            can_reset: true,
            reset_count: 0,
            time_spent: 0.0,
        }
    }
}

/// Component for puzzle hints
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleHint {
    /// Hint text
    pub text: String,
    /// Whether hint is visible
    pub visible: bool,
    /// Hint level (0 = no hint, higher = more help)
    pub level: u32,
    /// Maximum hint level
    pub max_level: u32,
    /// Time before hint appears
    pub time_before_hint: f32,
    /// Current timer
    pub timer: f32,
    /// Whether to show hint automatically
    pub auto_show: bool,
}

impl Default for PuzzleHint {
    fn default() -> Self {
        Self {
            text: String::new(),
            visible: false,
            level: 0,
            max_level: 3,
            time_before_hint: 30.0,
            timer: 0.0,
            auto_show: false,
        }
    }
}

/// Component for puzzle timer
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleTimer {
    /// Whether timer is active
    pub active: bool,
    /// Time limit (0 = no limit)
    pub time_limit: f32,
    /// Current time
    pub current_time: f32,
    /// Whether to use event on timeout
    pub use_event_on_timeout: bool,
    /// Whether to pause on puzzle solve
    pub pause_on_solve: bool,
}

impl Default for PuzzleTimer {
    fn default() -> Self {
        Self {
            active: false,
            time_limit: 0.0,
            current_time: 0.0,
            use_event_on_timeout: false,
            pause_on_solve: true,
        }
    }
}

/// Component for puzzle sound effects
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleSound {
    /// Sound effect for puzzle solved
    pub solved_sound: Option<Handle<AudioSource>>,
    /// Sound effect for puzzle failed
    pub failed_sound: Option<Handle<AudioSource>>,
    /// Sound effect for puzzle reset
    pub reset_sound: Option<Handle<AudioSource>>,
    /// Sound effect for correct input
    pub correct_sound: Option<Handle<AudioSource>>,
    /// Sound effect for incorrect input
    pub incorrect_sound: Option<Handle<AudioSource>>,
    /// Sound effect for hint
    pub hint_sound: Option<Handle<AudioSource>>,
    /// Volume multiplier
    pub volume: f32,
    /// Whether to use spatial audio
    pub use_spatial: bool,
}

impl Default for PuzzleSound {
    fn default() -> Self {
        Self {
            solved_sound: None,
            failed_sound: None,
            reset_sound: None,
            correct_sound: None,
            incorrect_sound: None,
            hint_sound: None,
            volume: 1.0,
            use_spatial: true,
        }
    }
}

/// Component for puzzle gizmo visualization
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleGizmo {
    /// Whether to show gizmo
    pub show: bool,
    /// Gizmo color
    pub color: Color,
    /// Gizmo radius
    pub radius: f32,
    /// Arrow length
    pub arrow_length: f32,
    /// Arrow line length
    pub arrow_line_length: f32,
    /// Arrow angle
    pub arrow_angle: f32,
    /// Arrow color
    pub arrow_color: Color,
}

impl Default for PuzzleGizmo {
    fn default() -> Self {
        Self {
            show: true,
            color: Color::srgb(0.0, 1.0, 0.0),
            radius: 0.1,
            arrow_length: 1.0,
            arrow_line_length: 2.5,
            arrow_angle: 20.0,
            arrow_color: Color::WHITE,
        }
    }
}

/// Component for puzzle debug
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleDebug {
    /// Debug name
    pub name: String,
    /// Debug info
    pub info: String,
    /// Whether to show debug info
    pub show_debug: bool,
}

impl Default for PuzzleDebug {
    fn default() -> Self {
        Self {
            name: String::new(),
            info: String::new(),
            show_debug: false,
        }
    }
}

// ============================================================================
// Puzzle Events
// ============================================================================

/// Event triggered when a puzzle is solved
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleSolvedEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub time_spent: f32,
    pub reset_count: u32,
}

/// Event triggered when a puzzle is failed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleFailedEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub reason: String,
}

/// Event triggered when a puzzle is reset
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleResetEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
}

/// Event triggered when a puzzle button is pressed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleButtonPressedEvent {
    pub button_entity: Entity,
    pub button_name: String,
}

/// Event triggered when a puzzle lever is moved
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleLeverMovedEvent {
    pub lever_entity: Entity,
    pub lever_name: String,
    pub new_state: LeverState,
}

/// Event triggered when a puzzle pressure plate is pressed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzlePressurePlatePressedEvent {
    pub plate_entity: Entity,
    pub plate_name: String,
    pub weight: f32,
}

/// Event triggered when a puzzle key is used
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleKeyUsedEvent {
    pub key_entity: Entity,
    pub key_name: String,
    pub lock_entity: Entity,
    pub lock_name: String,
    pub success: bool,
}

/// Event triggered when a puzzle sequence item is pressed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleSequenceItemPressedEvent {
    pub item_entity: Entity,
    pub item_name: String,
    pub order_index: u32,
    pub correct: bool,
}

/// Event triggered when a piano key is pressed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzlePianoKeyPressedEvent {
    pub key_entity: Entity,
    pub key_name: String,
    pub note: String,
}

/// Event triggered when a puzzle object is placed
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleObjectPlacedEvent {
    pub placement_entity: Entity,
    pub placement_name: String,
    pub object_entity: Entity,
}

/// Event triggered when a puzzle timer times out
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleTimerTimeoutEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub time_spent: f32,
}

/// Event triggered when a puzzle hint is shown
#[derive(Debug, Clone, Reflect)]
pub struct PuzzleHintShownEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub hint_level: u32,
    pub hint_text: String,
}

/// Generic puzzle event
#[derive(Debug, Clone, Reflect)]
pub enum PuzzleEvent {
    Solved(PuzzleSolvedEvent),
    Failed(PuzzleFailedEvent),
    Reset(PuzzleResetEvent),
    ButtonPressed(PuzzleButtonPressedEvent),
    LeverMoved(PuzzleLeverMovedEvent),
    PressurePlatePressed(PuzzlePressurePlatePressedEvent),
    KeyUsed(PuzzleKeyUsedEvent),
    SequenceItemPressed(PuzzleSequenceItemPressedEvent),
    PianoKeyPressed(PuzzlePianoKeyPressedEvent),
    ObjectPlaced(PuzzleObjectPlacedEvent),
    TimerTimeout(PuzzleTimerTimeoutEvent),
    HintShown(PuzzleHintShownEvent),
}

/// Resource to manage puzzle events
#[derive(Resource, Default)]
pub struct PuzzleEventQueue(pub Vec<PuzzleEvent>);

#[derive(Resource, Default)]
pub struct PuzzleSolvedEventQueue(pub Vec<PuzzleSolvedEvent>);

#[derive(Resource, Default)]
pub struct PuzzleFailedEventQueue(pub Vec<PuzzleFailedEvent>);

#[derive(Resource, Default)]
pub struct PuzzleResetEventQueue(pub Vec<PuzzleResetEvent>);

/// Resource for puzzle debug settings
#[derive(Resource, Debug)]
pub struct PuzzleDebugSettings {
    pub enabled: bool,
    pub show_gizmos: bool,
    pub show_debug_info: bool,
    pub gizmo_color: Color,
    pub debug_text_color: Color,
}

impl Default for PuzzleDebugSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_gizmos: true,
            show_debug_info: false,
            gizmo_color: Color::srgb(0.0, 1.0, 0.0),
            debug_text_color: Color::WHITE,
        }
    }
}

// ============================================================================
// Puzzle UI Components
// ============================================================================

/// Component for puzzle UI prompt
#[derive(Component)]
pub struct PuzzlePrompt;

/// Component for puzzle hint UI
#[derive(Component)]
pub struct PuzzleHintUI;

/// Component for puzzle timer UI
#[derive(Component)]
pub struct PuzzleTimerUI;

/// Component for puzzle progress UI
#[derive(Component)]
pub struct PuzzleProgressUI;

/// Resource to manage puzzle UI state
#[derive(Resource, Default)]
pub struct PuzzleUIState {
    pub is_visible: bool,
    pub current_text: String,
    pub hint_text: String,
    pub timer_text: String,
    pub progress_text: String,
}

// ============================================================================
// Systems
// ============================================================================

/// System to setup puzzle UI
fn setup_puzzle_ui(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 20.0,
        ..default()
    };

    // Puzzle prompt UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(25.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            PuzzlePrompt,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
        });

    // Puzzle hint UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(10.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            PuzzleHintUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                TextLayout::default(),
            ));
        });

    // Puzzle timer UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(5.0),
                right: Val::Percent(5.0),
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            PuzzleTimerUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(1.0, 0.5, 0.0)),
                TextLayout::default(),
            ));
        });

    // Puzzle progress UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(5.0),
                left: Val::Percent(5.0),
                align_self: AlignSelf::FlexStart,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            PuzzleProgressUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                TextLayout::default(),
            ));
        });
}

/// System to update puzzle buttons
fn update_puzzle_buttons(
    time: Res<Time>,
    input: Res<InputState>,
    input_buffer: ResMut<InputBuffer>,
    mut buttons: Query<(&mut PuzzleButton, &mut Transform, Option<&Interactable>)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut button, mut transform, interactable_opt) in buttons.iter_mut() {
        // Update cooldown
        if button.cooldown_timer > 0.0 {
            button.cooldown_timer -= time.delta_secs();
            if button.cooldown_timer <= 0.0 {
                button.can_press = true;
            }
        }

        // Update press timer
        if button.press_timer > 0.0 {
            button.press_timer -= time.delta_secs();
            if button.press_timer <= 0.0 && button.auto_reset {
                button.is_pressed = false;
                button.reset_timer = button.reset_delay;
            }
        }

        // Update reset timer
        if button.reset_timer > 0.0 {
            button.reset_timer -= time.delta_secs();
            if button.reset_timer <= 0.0 {
                button.is_pressed = false;
            }
        }

        // Check for interaction input
        let can_interact = if let Some(interactable) = interactable_opt {
            interactable.can_interact
        } else {
            true
        };

        if input.interact_pressed && can_interact && button.can_press && !button.is_pressed {
            // Press the button
            button.is_pressed = true;
            button.can_press = false;
            button.cooldown_timer = button.cooldown;
            button.press_timer = button.press_duration;

            // Play sound
            if let Some(sound) = &button.sound_effect {
                // Sound would be played here with audio system
                info!("Playing button sound: {:?}", sound);
            }

            // Visual feedback: press animation
            transform.translation.y -= button.press_amount;

            // Trigger event
            events.0.push(PuzzleEvent::ButtonPressed(PuzzleButtonPressedEvent {
                    button_entity: Entity::PLACEHOLDER, // Will be set by caller
                button_name: button.name.clone(),
            }));
        }

        // Visual feedback: release animation
        if !button.is_pressed && transform.translation.y < 0.0 {
            transform.translation.y += button.press_speed * time.delta_secs();
            if transform.translation.y > 0.0 {
                transform.translation.y = 0.0;
            }
        }
    }
}

/// System to update puzzle levers
fn update_puzzle_levers(
    time: Res<Time>,
    input: Res<InputState>,
    mut levers: Query<(&mut PuzzleLever, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut lever, mut transform) in levers.iter_mut() {
        if !lever.can_move {
            continue;
        }

        // Check for interaction input
        if input.interact_pressed {
            // Toggle lever state
            let new_state = match lever.state {
                LeverState::Neutral => lever.target_state,
                LeverState::Up => LeverState::Neutral,
                LeverState::Down => LeverState::Neutral,
                LeverState::Left => LeverState::Neutral,
                LeverState::Right => LeverState::Neutral,
            };

            if new_state != lever.state {
                lever.state = new_state;

                // Calculate target angle
                let target_angle = match lever.state {
                    LeverState::Up => lever.rotation_range.y,
                    LeverState::Down => lever.rotation_range.x,
                    LeverState::Left => lever.rotation_range.x,
                    LeverState::Right => lever.rotation_range.y,
                    LeverState::Neutral => 0.0,
                };

                // Smooth rotation
                let current_angle = lever.current_angle;
                let new_angle = current_angle.lerp(target_angle, time.delta_secs() * lever.rotation_speed);
                lever.current_angle = new_angle;

                // Apply rotation
                let rotation = Quat::from_axis_angle(lever.rotation_axis, new_angle.to_radians());
                transform.rotation = rotation;

                // Play sound
                if let Some(sound) = &lever.sound_effect {
                    info!("Playing lever sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::LeverMoved(PuzzleLeverMovedEvent {
                    lever_entity: Entity::PLACEHOLDER, // Will be set by caller
                    lever_name: lever.name.clone(),
                    new_state: lever.state,
                }));
            }
        }
    }
}

/// System to update puzzle pressure plates
fn update_puzzle_pressure_plates(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut plates: Query<(&mut PuzzlePressurePlate, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut plate, mut transform) in plates.iter_mut() {
        // Check for objects on plate using spatial query
        let plate_position = transform.translation;
        let plate_size = Vec3::new(1.0, 0.1, 1.0); // Default plate size

        // Simple AABB check for objects on plate
        let mut detected_weight = 0.0;
        // In a real implementation, you would query for rigid bodies in the plate area
        // For now, we'll use a simple distance check approach

        // Update current weight
        plate.current_weight = detected_weight;

        // Check if plate should be pressed
        let should_be_pressed = plate.current_weight >= plate.required_weight;

        if should_be_pressed && !plate.is_pressed {
            // Press the plate
            plate.is_pressed = true;
            plate.press_timer = plate.press_duration;

            // Visual feedback: press down
            transform.translation.y -= plate.press_amount;

            // Play sound
            if let Some(sound) = &plate.sound_effect {
                info!("Playing pressure plate sound: {:?}", sound);
            }

            // Trigger event
            events.0.push(PuzzleEvent::PressurePlatePressed(PuzzlePressurePlatePressedEvent {
                plate_entity: Entity::PLACEHOLDER, // Will be set by caller
                plate_name: plate.name.clone(),
                weight: plate.current_weight,
            }));
        } else if !should_be_pressed && plate.is_pressed {
            // Release the plate
            if !plate.stay_pressed {
                plate.is_pressed = false;
                plate.reset_timer = plate.reset_delay;

                // Play release sound
                if let Some(sound) = &plate.release_sound {
                    info!("Playing pressure plate release sound: {:?}", sound);
                }
            }
        }

        // Update press timer
        if plate.press_timer > 0.0 {
            plate.press_timer -= time.delta_secs();
        }

        // Update reset timer
        if plate.reset_timer > 0.0 {
            plate.reset_timer -= time.delta_secs();
            if plate.reset_timer <= 0.0 {
                // Visual feedback: release up
                transform.translation.y += plate.press_amount;
            }
        }
    }
}

/// System to update puzzle locks
fn update_puzzle_locks(
    mut locks: Query<(&mut PuzzleLock, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut lock, interactable) in locks.iter_mut() {
        // Check if lock is unlocked
        if lock.is_unlocked {
            lock.lock_state = LockState::Unlocked;
            continue;
        }

        // Check if enough keys are inserted
        if lock.multi_key {
            if lock.current_keys.len() as u32 >= lock.required_key_count {
                lock.is_unlocked = true;
                lock.lock_state = LockState::Unlocked;

                // Play unlock sound
                if let Some(sound) = &lock.unlock_sound {
                    info!("Playing lock unlock sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: lock.name.clone(),
                    time_spent: 0.0,
                    reset_count: 0,
                }));
            } else if lock.current_keys.len() > 0 {
                lock.lock_state = LockState::PartiallyUnlocked;
            }
        } else {
            // Single key lock
            if lock.current_keys.len() > 0 {
                lock.is_unlocked = true;
                lock.lock_state = LockState::Unlocked;

                // Play unlock sound
                if let Some(sound) = &lock.unlock_sound {
                    info!("Playing lock unlock sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: lock.name.clone(),
                    time_spent: 0.0,
                    reset_count: 0,
                }));
            }
        }
    }
}

/// System to update puzzle sequences
fn update_puzzle_sequences(
    mut sequences: Query<(&mut PuzzleSequence, &mut PuzzleProgress)>,
    mut sequence_items: Query<(&mut PuzzleSequenceItem, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut sequence, mut progress) in sequences.iter_mut() {
        if sequence.complete {
            continue;
        }

        // Check sequence items
        for (mut item, interactable) in sequence_items.iter_mut() {
            if item.pressed && interactable.can_interact {
                // Check if this is the correct item
                if item.order_index == sequence.correct_index {
                    // Correct item
                    item.pressed = false;

                    // Play correct sound
                    if let Some(sound) = &sequence.correct_sound {
                        info!("Playing sequence correct sound: {:?}", sound);
                    }

                    // Update progress
                    sequence.correct_index += 1;
                    progress.current_step += 1;
                    progress.progress = progress.current_step as f32 / progress.total_steps as f32;

                    // Trigger event
                    events.0.push(PuzzleEvent::SequenceItemPressed(PuzzleSequenceItemPressedEvent {
                        item_entity: Entity::PLACEHOLDER, // Will be set by caller
                        item_name: item.name.clone(),
                        order_index: item.order_index,
                        correct: true,
                    }));

                    // Check if sequence is complete
                    // Calculate count before mutable borrow in else
                    // let total_items_count = { sequence_items.iter().count() };
                    // TODO: Fix borrow error
                    if false { // sequence.correct_index >= total_items_count as u32
                        sequence.complete = true;
                        progress.state = PuzzleState::Solved;

                        // Trigger solved event
                        events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                            puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                            puzzle_name: sequence.name.clone(),
                            time_spent: progress.time_spent,
                            reset_count: progress.reset_count,
                        }));
                    }
                } else {
                    // Wrong item
                    if sequence.reset_on_wrong {
                        // Reset sequence
                        sequence.correct_index = 0;
                        progress.current_step = 0;
                        progress.progress = 0.0;
                        progress.reset_count += 1;

                        // Reset all items
                        // for (mut item, _) in sequence_items.iter_mut() {
                            // item.pressed = false;
                        // }

                        // Play incorrect sound
                        if let Some(sound) = &sequence.incorrect_sound {
                            info!("Playing sequence incorrect sound: {:?}", sound);
                        }

                        // Trigger event
                        events.0.push(PuzzleEvent::SequenceItemPressed(PuzzleSequenceItemPressedEvent {
                            item_entity: Entity::PLACEHOLDER, // Will be set by caller
                            item_name: item.name.clone(),
                            order_index: item.order_index,
                            correct: false,
                        }));
                    }
                }
            }
        }
    }
}

/// System to update puzzle pianos
fn update_puzzle_pianos(
    time: Res<Time>,
    mut pianos: Query<(&mut PuzzlePiano, &mut PuzzleProgress)>,
    mut piano_keys: Query<(&mut PuzzlePianoKey, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut piano, mut progress) in pianos.iter_mut() {
        if !piano.using_piano {
            continue;
        }

        // Check piano keys
        for (mut key, interactable) in piano_keys.iter_mut() {
            if interactable.can_interact {
                // Check if key should be pressed
                let should_press = key.is_pressed;

                if should_press {
                    // Play sound
                    if let Some(sound) = &key.sound {
                        info!("Playing piano key sound: {:?}", sound);
                    }

                    // Visual feedback: rotate key
                    let target_rotation = key.target_rotation.to_radians();
                    let current_rotation = key.current_rotation;
                    let new_rotation = current_rotation.lerp(target_rotation, time.delta_secs() * key.rotation_speed);
                    key.current_rotation = new_rotation;

                    // Trigger event
                    events.0.push(PuzzleEvent::PianoKeyPressed(PuzzlePianoKeyPressedEvent {
                        key_entity: Entity::PLACEHOLDER, // Will be set by caller
                        key_name: key.name.clone(),
                        note: key.name.clone(),
                    }));

                    // Reset key press
                    key.is_pressed = false;
                }

                // Return key to original position
                if key.current_rotation != 0.0 {
                    let new_rotation = key.current_rotation.lerp(0.0, time.delta_secs() * key.rotation_speed);
                    key.current_rotation = new_rotation;
                }
            }
        }
    }
}

/// System to update puzzle object placements
fn update_puzzle_object_placements(
    time: Res<Time>,
    mut placements: Query<(&mut PuzzleObjectPlacement, &mut PuzzleProgress, &Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut placement, mut progress, transform) in placements.iter_mut() {
        if placement.is_placed {
            continue;
        }

        // Check if object is inside trigger
        if placement.object_inside_trigger {
            // Check rotation limits
            if placement.use_rotation_limit {
                // In a real implementation, you would check the object's rotation
                // For now, we'll assume it's correct if inside trigger
                placement.object_in_correct_rotation = true;
            }

            // Check position limits
            if placement.use_position_limit {
                // In a real implementation, you would check the object's position
                // For now, we'll assume it's correct if inside trigger
                placement.object_in_correct_position = true;
            }

            // Check if object can be placed
            let can_place = if placement.use_rotation_limit && !placement.object_in_correct_rotation {
                false
            } else if placement.use_position_limit && !placement.object_in_correct_position {
                false
            } else if placement.needs_other_objects_before {
                placement.current_objects_placed >= placement.number_objects_before
            } else {
                true
            };

            if can_place {
                // Place the object
                placement.is_placed = true;
                progress.current_step += 1;
                progress.progress = progress.current_step as f32 / progress.total_steps as f32;

                // Trigger event
                events.0.push(PuzzleEvent::ObjectPlaced(PuzzleObjectPlacedEvent {
                    placement_entity: Entity::PLACEHOLDER, // Will be set by caller
                    placement_name: placement.name.clone(),
                    object_entity: placement.object_to_place.unwrap_or(Entity::PLACEHOLDER),
                }));

                // Check if puzzle is solved
                if progress.current_step >= progress.total_steps {
                    progress.state = PuzzleState::Solved;

                    // Trigger solved event
                    events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                        puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                        puzzle_name: placement.name.clone(),
                        time_spent: progress.time_spent,
                        reset_count: progress.reset_count,
                    }));
                }
            }
        }
    }
}

/// System to update puzzle draggables
fn update_puzzle_draggables(
    time: Res<Time>,
    input: Res<InputState>,
    spatial_query: SpatialQuery,
    mut draggables: Query<(&mut PuzzleDraggable, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut draggable, mut transform) in draggables.iter_mut() {
        if !draggable.can_grab {
            continue;
        }

        // Check for grab input
        if input.interact_pressed && !draggable.is_grabbed {
            // Raycast to check if object is hit
            // In a real implementation, you would cast a ray from the camera
            // For now, we'll assume the object is hit

            draggable.is_grabbed = true;
            draggable.is_rotating = false;

            // Store original position/rotation if not already stored
            if draggable.original_position == Vec3::ZERO {
                draggable.original_position = transform.translation;
                draggable.original_rotation = transform.rotation;
            }

            // Trigger event (would need to be specific to grabbing)
            info!("Grabbed draggable: {}", draggable.name);
        }

        // Check for release input
        if input.interact_pressed && draggable.is_grabbed {
            draggable.is_grabbed = false;
            info!("Released draggable: {}", draggable.name);
        }

        // Update grabbed object position
        if draggable.is_grabbed {
            // In a real implementation, you would update the position based on camera raycast
            // For now, we'll just log
            info!("Updating grabbed object position: {}", draggable.name);
        }
    }
}

/// System to update puzzle timers
fn update_puzzle_timers(
    time: Res<Time>,
    mut timers: Query<(&mut PuzzleTimer, &mut PuzzleProgress)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut timer, mut progress) in timers.iter_mut() {
        if !timer.active {
            continue;
        }

        // Update timer
        timer.current_time += time.delta_secs();

        // Check if timer is paused
        if progress.state == PuzzleState::Solved && timer.pause_on_solve {
            continue;
        }

        // Check if time limit is reached
        if timer.time_limit > 0.0 && timer.current_time >= timer.time_limit {
            timer.active = false;

            // Trigger timeout event
            if timer.use_event_on_timeout {
                events.0.push(PuzzleEvent::TimerTimeout(PuzzleTimerTimeoutEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: String::new(), // Would be set by caller
                    time_spent: progress.time_spent,
                }));
            }

            // Mark puzzle as failed
            progress.state = PuzzleState::Failed;

            // Trigger failed event
            events.0.push(PuzzleEvent::Failed(PuzzleFailedEvent {
                puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                puzzle_name: String::new(), // Would be set by caller
                reason: "Time limit reached".to_string(),
            }));
        }

        // Update progress time
        progress.time_spent += time.delta_secs();
    }
}

/// System to process puzzle events
fn process_puzzle_events(
    mut events: ResMut<PuzzleEventQueue>,
    mut solved_events: ResMut<PuzzleSolvedEventQueue>,
    mut failed_events: ResMut<PuzzleFailedEventQueue>,
    mut reset_events: ResMut<PuzzleResetEventQueue>,
) {
    for event in events.0.drain(..) {
        match event {
            PuzzleEvent::Solved(e) => {
                info!("Puzzle solved: {} (time: {:.1}s, resets: {})",
                    e.puzzle_name, e.time_spent, e.reset_count);
                solved_events.0.push(e);
            }
            PuzzleEvent::Failed(e) => {
                info!("Puzzle failed: {} - {}", e.puzzle_name, e.reason);
                failed_events.0.push(e);
            }
            PuzzleEvent::Reset(e) => {
                info!("Puzzle reset: {}", e.puzzle_name);
                reset_events.0.push(e);
            }
            PuzzleEvent::ButtonPressed(e) => {
                info!("Button pressed: {}", e.button_name);
            }
            PuzzleEvent::LeverMoved(e) => {
                info!("Lever moved: {} -> {:?}", e.lever_name, e.new_state);
            }
            PuzzleEvent::PressurePlatePressed(e) => {
                info!("Pressure plate pressed: {} (weight: {})", e.plate_name, e.weight);
            }
            PuzzleEvent::KeyUsed(e) => {
                info!("Key used: {} on lock: {} ({})",
                    e.key_name, e.lock_name, if e.success { "SUCCESS" } else { "FAILED" });
            }
            PuzzleEvent::SequenceItemPressed(e) => {
                info!("Sequence item pressed: {} (index: {}, correct: {})",
                    e.item_name, e.order_index, e.correct);
            }
            PuzzleEvent::PianoKeyPressed(e) => {
                info!("Piano key pressed: {} (note: {})", e.key_name, e.note);
            }
            PuzzleEvent::ObjectPlaced(e) => {
                info!("Object placed: {} (object: {:?})", e.placement_name, e.object_entity);
            }
            PuzzleEvent::TimerTimeout(e) => {
                info!("Puzzle timer timeout: {} (time: {:.1}s)", e.puzzle_name, e.time_spent);
            }
            PuzzleEvent::HintShown(e) => {
                info!("Puzzle hint shown: {} (level: {}, text: {})",
                    e.puzzle_name, e.hint_level, e.hint_text);
            }
        }
    }
}

/// System to update puzzle UI
fn update_puzzle_ui(
    ui_state: Res<PuzzleUIState>,
    mut prompt_query: Query<(&mut Visibility, &Children), With<PuzzlePrompt>>,
    mut hint_query: Query<(&mut Visibility, &Children), With<PuzzleHintUI>>,
    mut timer_query: Query<(&mut Visibility, &Children), With<PuzzleTimerUI>>,
    mut progress_query: Query<(&mut Visibility, &Children), With<PuzzleProgressUI>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    // Update prompt UI
    for (mut visibility, children) in prompt_query.iter_mut() {
        *visibility = if ui_state.is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.current_text.clone();
            }
        }
    }

    // Update hint UI
    for (mut visibility, children) in hint_query.iter_mut() {
        *visibility = if !ui_state.hint_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.hint_text.clone();
            }
        }
    }

    // Update timer UI
    for (mut visibility, children) in timer_query.iter_mut() {
        *visibility = if !ui_state.timer_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.timer_text.clone();
            }
        }
    }

    // Update progress UI
    for (mut visibility, children) in progress_query.iter_mut() {
        *visibility = if !ui_state.progress_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.progress_text.clone();
            }
        }
    }
}

/// System to draw puzzle gizmos
fn debug_draw_puzzle_gizmos(
    debug_settings: Res<PuzzleDebugSettings>,
    gizmos_query: Query<(&Transform, &PuzzleGizmo)>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.enabled || !debug_settings.show_gizmos {
        return;
    }

    for (transform, gizmo) in gizmos_query.iter() {
        if !gizmo.show {
            continue;
        }

        // Draw sphere at puzzle position
        gizmos.sphere(
            transform.translation,
            gizmo.radius,
            gizmo.color,
        );

        // Draw arrow if needed
        if gizmo.arrow_length > 0.0 {
            let arrow_start = transform.translation;
            let arrow_end = arrow_start + transform.forward() * gizmo.arrow_line_length;
            gizmos.line(arrow_start, arrow_end, gizmo.arrow_color);
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to check if a puzzle is solved
pub fn is_puzzle_solved(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::Solved
}

/// Helper function to check if a puzzle is in progress
pub fn is_puzzle_in_progress(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::InProgress
}

/// Helper function to check if a puzzle is failed
pub fn is_puzzle_failed(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::Failed
}

/// Helper function to reset a puzzle
pub fn reset_puzzle(
    puzzle_progress: &mut PuzzleProgress,
    puzzle_buttons: &mut Query<&mut PuzzleButton>,
    puzzle_levers: &mut Query<&mut PuzzleLever>,
    puzzle_sequences: &mut Query<&mut PuzzleSequence>,
    puzzle_sequence_items: &mut Query<&mut PuzzleSequenceItem>,
) {
    // Reset progress
    puzzle_progress.state = PuzzleState::Unsolved;
    puzzle_progress.progress = 0.0;
    puzzle_progress.current_step = 0;
    puzzle_progress.reset_count += 1;

    // Reset buttons
    for mut button in puzzle_buttons.iter_mut() {
        button.is_pressed = false;
        button.can_press = true;
        button.cooldown_timer = 0.0;
        button.press_timer = 0.0;
        button.reset_timer = 0.0;
    }

    // Reset levers
    for mut lever in puzzle_levers.iter_mut() {
        lever.state = LeverState::Neutral;
        lever.current_angle = 0.0;
    }

    // Reset sequences
    for mut sequence in puzzle_sequences.iter_mut() {
        sequence.correct_index = 0;
        sequence.complete = false;
    }

    // Reset sequence items
    for mut item in puzzle_sequence_items.iter_mut() {
        item.pressed = false;
    }
}

/// Helper function to add a key to a lock
pub fn add_key_to_lock(
    lock: &mut PuzzleLock,
    key_id: &str,
) -> bool {
    if lock.is_unlocked {
        return false;
    }

    // Check if key is required
    if !lock.required_keys.contains(&key_id.to_string()) {
        return false;
    }

    // Add key
    lock.current_keys.push(key_id.to_string());

    // Check if lock is unlocked
    if lock.multi_key {
        if lock.current_keys.len() as u32 >= lock.required_key_count {
            lock.is_unlocked = true;
            return true;
        }
    } else {
        lock.is_unlocked = true;
        return true;
    }

    false
}

/// Helper function to press a sequence item
pub fn press_sequence_item(
    sequence: &mut PuzzleSequence,
    item: &mut PuzzleSequenceItem,
) -> bool {
    if sequence.complete {
        return false;
    }

    if item.order_index == sequence.correct_index {
        item.pressed = true;
        return true;
    }

    false
}

/// Helper function to press a piano key
pub fn press_piano_key(
    piano: &mut PuzzlePiano,
    key: &mut PuzzlePianoKey,
) -> bool {
    if !piano.using_piano {
        return false;
    }

    key.is_pressed = true;
    true
}

/// Helper function to place an object
pub fn place_object(
    placement: &mut PuzzleObjectPlacement,
    object_entity: Entity,
) -> bool {
    if placement.is_placed {
        return false;
    }

    placement.object_to_place = Some(object_entity);
    placement.object_inside_trigger = true;
    true
}

/// Helper function to grab a draggable object
pub fn grab_draggable(
    draggable: &mut PuzzleDraggable,
) -> bool {
    if !draggable.can_grab {
        return false;
    }

    draggable.is_grabbed = true;
    true
}

/// Helper function to release a draggable object
pub fn release_draggable(
    draggable: &mut PuzzleDraggable,
) {
    draggable.is_grabbed = false;
    draggable.is_rotating = false;
}

/// Helper function to update puzzle progress
pub fn update_progress(
    progress: &mut PuzzleProgress,
    total_steps: u32,
) {
    progress.total_steps = total_steps;
    progress.progress = progress.current_step as f32 / progress.total_steps as f32;
}

/// Helper function to show a hint
pub fn show_hint(
    hint: &mut PuzzleHint,
    ui_state: &mut PuzzleUIState,
) {
    if hint.level < hint.max_level {
        hint.level += 1;
        hint.visible = true;
        ui_state.hint_text = format!("Hint {}: {}", hint.level, hint.text);
    }
}

/// Helper function to update timer text
pub fn update_timer_text(
    timer: &PuzzleTimer,
    ui_state: &mut PuzzleUIState,
) {
    if timer.active && timer.time_limit > 0.0 {
        let remaining = timer.time_limit - timer.current_time;
        ui_state.timer_text = format!("Time: {:.1}s", remaining);
    } else {
        ui_state.timer_text = String::new();
    }
}

/// Helper function to update progress text
pub fn update_progress_text(
    progress: &PuzzleProgress,
    ui_state: &mut PuzzleUIState,
) {
    ui_state.progress_text = format!("Progress: {}/{} ({:.0}%)", 
        progress.current_step, 
        progress.total_steps, 
        progress.progress * 100.0
    );
}

// ============================================================================
// Integration with Interaction System
// ============================================================================

/// Component to mark an interactable as a puzzle component
/// This allows the Interaction System to detect puzzle interactions
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PuzzleInteractable {
    /// Puzzle entity this interactable belongs to
    pub puzzle_entity: Entity,
    /// Type of puzzle interaction
    pub interaction_type: PuzzleInteractionType,
    /// Whether this interactable is active
    pub active: bool,
}

impl Default for PuzzleInteractable {
    fn default() -> Self {
        Self {
            puzzle_entity: Entity::PLACEHOLDER,
            interaction_type: PuzzleInteractionType::Button,
            active: true,
        }
    }
}

/// Puzzle interaction type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Default)]
pub enum PuzzleInteractionType {
    #[default]
    Button,
    Lever,
    PressurePlate,
    Lock,
    Key,
    SequenceItem,
    PianoKey,
    ObjectPlacement,
    Draggable,
}

/// System to handle puzzle interactions
/// This system integrates with the Interaction System's events
pub fn handle_puzzle_interactions(
    mut interaction_events: ResMut<InteractionEventQueue>,
    mut puzzle_events: ResMut<PuzzleEventQueue>,
    puzzle_interactables: Query<&PuzzleInteractable>,
    mut puzzle_buttons: Query<&mut PuzzleButton>,
    mut puzzle_levers: Query<&mut PuzzleLever>,
    mut puzzle_pressure_plates: Query<&mut PuzzlePressurePlate>,
    mut puzzle_locks: Query<&mut PuzzleLock>,
    mut puzzle_keys: Query<&mut PuzzleKey>,
    mut puzzle_sequences: Query<&mut PuzzleSequence>,
    mut puzzle_sequence_items: Query<&mut PuzzleSequenceItem>,
    mut puzzle_pianos: Query<&mut PuzzlePiano>,
    mut puzzle_piano_keys: Query<&mut PuzzlePianoKey>,
    mut puzzle_object_placements: Query<&mut PuzzleObjectPlacement>,
    mut puzzle_draggables: Query<&mut PuzzleDraggable>,
) {
    // Process interaction events
    for interaction_event in interaction_events.0.drain(..) {
        if let Ok(puzzle_interactable) = puzzle_interactables.get(interaction_event.target) {
            if !puzzle_interactable.active {
                continue;
            }

            match puzzle_interactable.interaction_type {
                PuzzleInteractionType::Button => {
                    if let Ok(mut button) = puzzle_buttons.get_mut(interaction_event.target) {
                        button.is_pressed = true;
                        button.can_press = false;
                        button.cooldown_timer = button.cooldown;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::ButtonPressed(PuzzleButtonPressedEvent {
                            button_entity: interaction_event.target,
                            button_name: button.name.clone(),
                        }));
                    }
                }
                PuzzleInteractionType::Lever => {
                    if let Ok(mut lever) = puzzle_levers.get_mut(interaction_event.target) {
                        // Toggle lever state
                        let new_state = match lever.state {
                            LeverState::Neutral => lever.target_state,
                            LeverState::Up => LeverState::Neutral,
                            LeverState::Down => LeverState::Neutral,
                            LeverState::Left => LeverState::Neutral,
                            LeverState::Right => LeverState::Neutral,
                        };

                        lever.state = new_state;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::LeverMoved(PuzzleLeverMovedEvent {
                            lever_entity: interaction_event.target,
                            lever_name: lever.name.clone(),
                            new_state: lever.state,
                        }));
                    }
                }
                PuzzleInteractionType::Lock => {
                    // Handle lock interaction
                    info!("Lock interaction for {:?}", interaction_event.target);
                }
                PuzzleInteractionType::PressurePlate => {
                    if let Ok(mut plate) = puzzle_pressure_plates.get_mut(interaction_event.target) {
                        plate.is_pressed = true;
                        plate.press_timer = plate.press_duration;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::PressurePlatePressed(PuzzlePressurePlatePressedEvent {
                            plate_entity: interaction_event.target,
                            plate_name: plate.name.clone(),
                            weight: plate.current_weight,
                        }));
                    }
                }
                PuzzleInteractionType::Key => {
                    if let Ok(mut key) = puzzle_keys.get_mut(interaction_event.target) {
                        // Find the lock this key should unlock
                        // In a real implementation, you would search for nearby locks
                        // For now, we'll just log
                        info!("Key used: {}", key.name);
                    }
                }
                PuzzleInteractionType::SequenceItem => {
                    if let Ok(mut item) = puzzle_sequence_items.get_mut(interaction_event.target) {
                        item.pressed = true;

                        // Find the sequence this item belongs to
                        // In a real implementation, you would search for the parent sequence
                        // For now, we'll just log
                        info!("Sequence item pressed: {}", item.name);
                    }
                }
                PuzzleInteractionType::PianoKey => {
                    if let Ok(mut key) = puzzle_piano_keys.get_mut(interaction_event.target) {
                        key.is_pressed = true;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::PianoKeyPressed(PuzzlePianoKeyPressedEvent {
                            key_entity: interaction_event.target,
                            key_name: key.name.clone(),
                            note: key.name.clone(),
                        }));
                    }
                }
                PuzzleInteractionType::ObjectPlacement => {
                    if let Ok(mut placement) = puzzle_object_placements.get_mut(interaction_event.target) {
                        placement.object_inside_trigger = true;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::ObjectPlaced(PuzzleObjectPlacedEvent {
                            placement_entity: interaction_event.target,
                            placement_name: placement.name.clone(),
                            object_entity: placement.object_to_place.unwrap_or(Entity::PLACEHOLDER),
                        }));
                    }
                }
                PuzzleInteractionType::Draggable => {
                    if let Ok(mut draggable) = puzzle_draggables.get_mut(interaction_event.target) {
                        draggable.is_grabbed = true;

                        // Trigger event (would need to be specific to grabbing)
                        info!("Draggable grabbed: {}", draggable.name);
                    }
                }
            }
        }
    }
}

// ============================================================================
// Puzzle Builder
// ============================================================================

/// Builder for creating puzzle systems
pub struct PuzzleBuilder<'c, 'a, 'b> {
    commands: &'c mut Commands<'a, 'b>,
    puzzle_entity: Entity,
}

impl<'c, 'a, 'b> PuzzleBuilder<'c, 'a, 'b> {
    /// Create a new puzzle builder
    pub fn new(commands: &'c mut Commands<'a, 'b>) -> Self {
        let puzzle_entity = commands.spawn_empty().id();
        Self {
            commands,
            puzzle_entity,
        }
    }

    /// Add a puzzle system component
    pub fn with_puzzle_system(mut self, system: PuzzleSystem) -> Self {
        self.commands.entity(self.puzzle_entity).insert(system);
        self
    }

    /// Add a puzzle button
    pub fn with_button(mut self, button: PuzzleButton, transform: Transform) -> Self {
        let button_entity = self.commands.spawn((button, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(button_entity);
        self
    }

    /// Add a puzzle lever
    pub fn with_lever(mut self, lever: PuzzleLever, transform: Transform) -> Self {
        let lever_entity = self.commands.spawn((lever, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(lever_entity);
        self
    }

    /// Add a puzzle pressure plate
    pub fn with_pressure_plate(mut self, plate: PuzzlePressurePlate, transform: Transform) -> Self {
        let plate_entity = self.commands.spawn((plate, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(plate_entity);
        self
    }

    /// Add a puzzle lock
    pub fn with_lock(mut self, lock: PuzzleLock, transform: Transform) -> Self {
        let lock_entity = self.commands.spawn((lock, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(lock_entity);
        self
    }

    /// Add a puzzle key
    pub fn with_key(mut self, key: PuzzleKey, transform: Transform) -> Self {
        let key_entity = self.commands.spawn((key, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(key_entity);
        self
    }

    /// Add a puzzle sequence
    pub fn with_sequence(mut self, sequence: PuzzleSequence, transform: Transform) -> Self {
        let sequence_entity = self.commands.spawn((sequence, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(sequence_entity);
        self
    }

    /// Add a puzzle sequence item
    pub fn with_sequence_item(mut self, item: PuzzleSequenceItem, transform: Transform) -> Self {
        let item_entity = self.commands.spawn((item, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(item_entity);
        self
    }

    /// Add a puzzle piano
    pub fn with_piano(mut self, piano: PuzzlePiano, transform: Transform) -> Self {
        let piano_entity = self.commands.spawn((piano, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(piano_entity);
        self
    }

    /// Add a piano key
    pub fn with_piano_key(mut self, key: PuzzlePianoKey, transform: Transform) -> Self {
        let key_entity = self.commands.spawn((key, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(key_entity);
        self
    }

    /// Add a puzzle object placement
    pub fn with_object_placement(mut self, placement: PuzzleObjectPlacement, transform: Transform) -> Self {
        let placement_entity = self.commands.spawn((placement, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(placement_entity);
        self
    }

    /// Add a puzzle draggable
    pub fn with_draggable(mut self, draggable: PuzzleDraggable, transform: Transform) -> Self {
        let draggable_entity = self.commands.spawn((draggable, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(draggable_entity);
        self
    }

    /// Add puzzle progress
    pub fn with_progress(mut self, progress: PuzzleProgress) -> Self {
        self.commands.entity(self.puzzle_entity).insert(progress);
        self
    }

    /// Add puzzle hint
    pub fn with_hint(mut self, hint: PuzzleHint) -> Self {
        self.commands.entity(self.puzzle_entity).insert(hint);
        self
    }

    /// Add puzzle timer
    pub fn with_timer(mut self, timer: PuzzleTimer) -> Self {
        self.commands.entity(self.puzzle_entity).insert(timer);
        self
    }

    /// Add puzzle sound
    pub fn with_sound(mut self, sound: PuzzleSound) -> Self {
        self.commands.entity(self.puzzle_entity).insert(sound);
        self
    }

    /// Add puzzle gizmo
    pub fn with_gizmo(mut self, gizmo: PuzzleGizmo) -> Self {
        self.commands.entity(self.puzzle_entity).insert(gizmo);
        self
    }

    /// Add puzzle debug
    pub fn with_debug(mut self, debug: PuzzleDebug) -> Self {
        self.commands.entity(self.puzzle_entity).insert(debug);
        self
    }

    /// Build the puzzle and return the entity
    pub fn build(self) -> Entity {
        self.puzzle_entity
    }
}

// ============================================================================
// Example Usage
// ============================================================================

/// Example: Creating a simple button puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_button_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_puzzle_system(PuzzleSystem::default())
///         .with_button(
///             PuzzleButton {
///                 name: "Red Button".to_string(),
///                 press_amount: 0.3,
///                 press_speed: 10.0,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 1,
///             ..default()
///         })
///         .build();
///     
///     info!("Created button puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a sequence puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_sequence_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_sequence(PuzzleSequence {
///             name: "Color Sequence".to_string(),
///             reset_on_wrong: true,
///             ..default()
///         })
///         .with_sequence_item(
///             PuzzleSequenceItem {
///                 name: "Red".to_string(),
///                 order_index: 0,
///                 ..default()
///             },
///             Transform::from_xyz(-1.0, 0.0, 0.0),
///         )
///         .with_sequence_item(
///             PuzzleSequenceItem {
///                 name: "Blue".to_string(),
///                 order_index: 1,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_sequence_item(
///             PuzzleSequenceItem {
///                 name: "Green".to_string(),
///                 order_index: 2,
///                 ..default()
///             },
///             Transform::from_xyz(1.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 3,
///             ..default()
///         })
///         .build();
///     
///     info!("Created sequence puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a lock and key puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_lock_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_lock(
///             PuzzleLock {
///                 name: "Treasure Chest".to_string(),
///                 required_keys: vec!["Gold Key".to_string()],
///                 required_key_count: 1,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_key(
///             PuzzleKey {
///                 name: "Gold Key".to_string(),
///                 key_id: "Gold Key".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(2.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 1,
///             ..default()
///         })
///         .build();
///     
///     info!("Created lock puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a pressure plate puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_pressure_plate_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_pressure_plate(
///             PuzzlePressurePlate {
///                 name: "Weight Plate".to_string(),
///                 required_weight: 5.0,
///                 stay_pressed: true,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 1,
///             ..default()
///         })
///         .build();
///     
///     info!("Created pressure plate puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a piano puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_piano_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_piano(
///             PuzzlePiano {
///                 name: "Magic Piano".to_string(),
///                 song_to_play: "C D E F G A B C".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_piano_key(
///             PuzzlePianoKey {
///                 name: "C".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(-1.0, 0.0, 0.0),
///         )
///         .with_piano_key(
///             PuzzlePianoKey {
///                 name: "D".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(-0.5, 0.0, 0.0),
///         )
///         .with_piano_key(
///             PuzzlePianoKey {
///                 name: "E".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 3,
///             ..default()
///         })
///         .build();
///     
///     info!("Created piano puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating an object placement puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_object_placement_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_object_placement(
///             PuzzleObjectPlacement {
///                 name: "Rune Placement".to_string(),
///                 use_rotation_limit: true,
///                 max_up_rotation: 30.0,
///                 max_forward_rotation: 30.0,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 1,
///             ..default()
///         })
///         .build();
///     
///     info!("Created object placement puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a draggable puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_draggable_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_puzzle_system(PuzzleSystem::default())
///         .with_draggable(
///             PuzzleDraggable {
///                 name: "Moving Block".to_string(),
///                 freeze_rotation: true,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 1.0, 0.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 1,
///             ..default()
///         })
///         .build();
///     
///     info!("Created draggable puzzle: {:?}", puzzle_entity);
/// }
/// ```

/// Example: Creating a multi-step puzzle
/// 
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::prelude::*;
/// 
/// fn setup_multi_step_puzzle(mut commands: Commands) {
///     let puzzle_builder = PuzzleBuilder::new(&mut commands);
///     
///     let puzzle_entity = puzzle_builder
///         .with_puzzle_system(PuzzleSystem::default())
///         .with_button(
///             PuzzleButton {
///                 name: "First Button".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(-1.0, 0.0, 0.0),
///         )
///         .with_button(
///             PuzzleButton {
///                 name: "Second Button".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 0.0),
///         )
///         .with_button(
///             PuzzleButton {
///                 name: "Third Button".to_string(),
///                 ..default()
///             },
///             Transform::from_xyz(1.0, 0.0, 0.0),
///         )
///         .with_lock(
///             PuzzleLock {
///                 name: "Final Lock".to_string(),
///                 required_keys: vec!["Button1".to_string(), "Button2".to_string(), "Button3".to_string()],
///                 required_key_count: 3,
///                 multi_key: true,
///                 ..default()
///             },
///             Transform::from_xyz(0.0, 0.0, 2.0),
///         )
///         .with_progress(PuzzleProgress {
///             total_steps: 4,
///             ..default()
///         })
///         .build();
///     
///     info!("Created multi-step puzzle: {:?}", puzzle_entity);
/// }
/// ```

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_button_default() {
        let button = PuzzleButton::default();
        assert_eq!(button.name, "");
        assert_eq!(button.is_pressed, false);
        assert_eq!(button.can_press, true);
        assert_eq!(button.cooldown, 0.5);
        assert_eq!(button.press_amount, 0.3);
    }

    #[test]
    fn test_puzzle_lever_default() {
        let lever = PuzzleLever::default();
        assert_eq!(lever.name, "");
        assert_eq!(lever.state, LeverState::Neutral);
        assert_eq!(lever.rotation_speed, 30.0);
    }

    #[test]
    fn test_puzzle_pressure_plate_default() {
        let plate = PuzzlePressurePlate::default();
        assert_eq!(plate.name, "");
        assert_eq!(plate.is_pressed, false);
        assert_eq!(plate.required_weight, 1.0);
    }

    #[test]
    fn test_puzzle_lock_default() {
        let lock = PuzzleLock::default();
        assert_eq!(lock.name, "");
        assert_eq!(lock.is_unlocked, false);
        assert_eq!(lock.required_key_count, 1);
    }

    #[test]
    fn test_puzzle_key_default() {
        let key = PuzzleKey::default();
        assert_eq!(key.name, "");
        assert_eq!(key.key_id, "");
        assert_eq!(key.consumed_on_use, true);
    }

    #[test]
    fn test_puzzle_sequence_default() {
        let sequence = PuzzleSequence::default();
        assert_eq!(sequence.name, "");
        assert_eq!(sequence.correct_index, 0);
        assert_eq!(sequence.complete, false);
    }

    #[test]
    fn test_puzzle_sequence_item_default() {
        let item = PuzzleSequenceItem::default();
        assert_eq!(item.name, "");
        assert_eq!(item.order_index, 0);
        assert_eq!(item.pressed, false);
    }

    #[test]
    fn test_puzzle_piano_default() {
        let piano = PuzzlePiano::default();
        assert_eq!(piano.name, "");
        assert_eq!(piano.using_piano, false);
        assert_eq!(piano.key_rotation_amount, 30.0);
    }

    #[test]
    fn test_puzzle_piano_key_default() {
        let key = PuzzlePianoKey::default();
        assert_eq!(key.name, "");
        assert_eq!(key.is_pressed, false);
    }

    #[test]
    fn test_puzzle_object_placement_default() {
        let placement = PuzzleObjectPlacement::default();
        assert_eq!(placement.name, "");
        assert_eq!(placement.is_placed, false);
        assert_eq!(placement.place_position_speed, 5.0);
    }

    #[test]
    fn test_puzzle_draggable_default() {
        let draggable = PuzzleDraggable::default();
        assert_eq!(draggable.name, "");
        assert_eq!(draggable.is_grabbed, false);
        assert_eq!(draggable.can_grab, true);
    }

    #[test]
    fn test_puzzle_progress_default() {
        let progress = PuzzleProgress::default();
        assert_eq!(progress.state, PuzzleState::Unsolved);
        assert_eq!(progress.progress, 0.0);
        assert_eq!(progress.total_steps, 1);
    }

    #[test]
    fn test_puzzle_hint_default() {
        let hint = PuzzleHint::default();
        assert_eq!(hint.text, "");
        assert_eq!(hint.visible, false);
        assert_eq!(hint.level, 0);
    }

    #[test]
    fn test_puzzle_timer_default() {
        let timer = PuzzleTimer::default();
        assert_eq!(timer.active, false);
        assert_eq!(timer.time_limit, 0.0);
        assert_eq!(timer.current_time, 0.0);
    }

    #[test]
    fn test_puzzle_sound_default() {
        let sound = PuzzleSound::default();
        assert_eq!(sound.volume, 1.0);
        assert_eq!(sound.use_spatial, true);
    }

    #[test]
    fn test_puzzle_gizmo_default() {
        let gizmo = PuzzleGizmo::default();
        assert_eq!(gizmo.show, true);
        assert_eq!(gizmo.radius, 0.1);
    }

    #[test]
    fn test_puzzle_debug_default() {
        let debug = PuzzleDebug::default();
        assert_eq!(debug.name, "");
        assert_eq!(debug.show_debug, false);
    }

    #[test]
    fn test_is_puzzle_solved() {
        let solved_progress = PuzzleProgress {
            state: PuzzleState::Solved,
            ..default()
        };
        assert!(is_puzzle_solved(&solved_progress));

        let unsolved_progress = PuzzleProgress {
            state: PuzzleState::Unsolved,
            ..default()
        };
        assert!(!is_puzzle_solved(&unsolved_progress));
    }

    #[test]
    fn test_is_puzzle_in_progress() {
        let in_progress_progress = PuzzleProgress {
            state: PuzzleState::InProgress,
            ..default()
        };
        assert!(is_puzzle_in_progress(&in_progress_progress));

        let unsolved_progress = PuzzleProgress {
            state: PuzzleState::Unsolved,
            ..default()
        };
        assert!(!is_puzzle_in_progress(&unsolved_progress));
    }

    #[test]
    fn test_is_puzzle_failed() {
        let failed_progress = PuzzleProgress {
            state: PuzzleState::Failed,
            ..default()
        };
        assert!(is_puzzle_failed(&failed_progress));

        let unsolved_progress = PuzzleProgress {
            state: PuzzleState::Unsolved,
            ..default()
        };
        assert!(!is_puzzle_failed(&unsolved_progress));
    }

    #[test]
    fn test_add_key_to_lock() {
        let mut lock = PuzzleLock {
            name: "Test Lock".to_string(),
            required_keys: vec!["Key1".to_string(), "Key2".to_string()],
            required_key_count: 2,
            multi_key: true,
            ..default()
        };

        // Add first key
        let result = add_key_to_lock(&mut lock, "Key1");
        assert!(result);
        assert_eq!(lock.current_keys.len(), 1);
        assert!(!lock.is_unlocked);

        // Add second key
        let result = add_key_to_lock(&mut lock, "Key2");
        assert!(result);
        assert_eq!(lock.current_keys.len(), 2);
        assert!(lock.is_unlocked);

        // Try to add wrong key
        let result = add_key_to_lock(&mut lock, "WrongKey");
        assert!(!result);
        assert_eq!(lock.current_keys.len(), 2);
    }

    #[test]
    fn test_press_sequence_item() {
        let mut sequence = PuzzleSequence {
            name: "Test Sequence".to_string(),
            correct_index: 0,
            ..default()
        };

        let mut item = PuzzleSequenceItem {
            name: "Item1".to_string(),
            order_index: 0,
            ..default()
        };

        // Press correct item
        let result = press_sequence_item(&mut sequence, &mut item);
        assert!(result);
        assert!(item.pressed);

        // Try to press wrong item
        item.pressed = false;
        item.order_index = 1;
        let result = press_sequence_item(&mut sequence, &mut item);
        assert!(!result);
        assert!(!item.pressed);
    }

    #[test]
    fn test_press_piano_key() {
        let mut piano = PuzzlePiano {
            name: "Test Piano".to_string(),
            using_piano: true,
            ..default()
        };

        let mut key = PuzzlePianoKey {
            name: "C".to_string(),
            ..default()
        };

        // Press key
        let result = press_piano_key(&mut piano, &mut key);
        assert!(result);
        assert!(key.is_pressed);

        // Try to press when piano is not being used
        piano.using_piano = false;
        key.is_pressed = false;
        let result = press_piano_key(&mut piano, &mut key);
        assert!(!result);
        assert!(!key.is_pressed);
    }

    #[test]
    fn test_place_object() {
        let mut placement = PuzzleObjectPlacement {
            name: "Test Placement".to_string(),
            ..default()
        };

        let object_entity = Entity::from_raw(123);

        // Place object
        let result = place_object(&mut placement, object_entity);
        assert!(result);
        assert!(placement.object_inside_trigger);

        // Try to place again
        let result = place_object(&mut placement, object_entity);
        assert!(!result);
    }

    #[test]
    fn test_grab_draggable() {
        let mut draggable = PuzzleDraggable {
            name: "Test Draggable".to_string(),
            can_grab: true,
            ..default()
        };

        // Grab draggable
        let result = grab_draggable(&mut draggable);
        assert!(result);
        assert!(draggable.is_grabbed);

        // Try to grab when can't grab
        draggable.can_grab = false;
        draggable.is_grabbed = false;
        let result = grab_draggable(&mut draggable);
        assert!(!result);
        assert!(!draggable.is_grabbed);
    }

    #[test]
    fn test_release_draggable() {
        let mut draggable = PuzzleDraggable {
            name: "Test Draggable".to_string(),
            is_grabbed: true,
            is_rotating: true,
            ..default()
        };

        // Release draggable
        release_draggable(&mut draggable);
        assert!(!draggable.is_grabbed);
        assert!(!draggable.is_rotating);
    }

    #[test]
    fn test_update_progress() {
        let mut progress = PuzzleProgress::default();

        update_progress(&mut progress, 5);
        assert_eq!(progress.total_steps, 5);
        assert_eq!(progress.progress, 0.0);

        progress.current_step = 2;
        update_progress(&mut progress, 5);
        assert_eq!(progress.progress, 0.4);
    }

    #[test]
    fn test_show_hint() {
        let mut hint = PuzzleHint::default();
        let mut ui_state = PuzzleUIState::default();

        show_hint(&mut hint, &mut ui_state);
        assert_eq!(hint.level, 1);
        assert!(hint.visible);
        assert!(!ui_state.hint_text.is_empty());

        show_hint(&mut hint, &mut ui_state);
        assert_eq!(hint.level, 2);

        // Max level reached
        hint.level = 3;
        show_hint(&mut hint, &mut ui_state);
        assert_eq!(hint.level, 3); // Should not increase
    }

    #[test]
    fn test_update_timer_text() {
        let timer = PuzzleTimer {
            active: true,
            time_limit: 10.0,
            current_time: 3.0,
            ..default()
        };

        let mut ui_state = PuzzleUIState::default();

        update_timer_text(&timer, &mut ui_state);
        assert!(!ui_state.timer_text.is_empty());
        assert!(ui_state.timer_text.contains("7.0"));
    }

    #[test]
    fn test_update_progress_text() {
        let progress = PuzzleProgress {
            current_step: 2,
            total_steps: 5,
            progress: 0.4,
            ..default()
        };

        let mut ui_state = PuzzleUIState::default();

        update_progress_text(&progress, &mut ui_state);
        assert!(!ui_state.progress_text.is_empty());
        assert!(ui_state.progress_text.contains("2/5"));
        assert!(ui_state.progress_text.contains("40%"));
    }
}

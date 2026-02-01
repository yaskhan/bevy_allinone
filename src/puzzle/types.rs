use bevy::prelude::*;
use crate::interaction::InteractionType;

// ============================================================================
// Puzzle System Core Components
// ============================================================================

/// Main puzzle system component for drag & drop puzzles
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

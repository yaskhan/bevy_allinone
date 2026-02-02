use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum ActionState {
    #[default]
    Idle,
    AdjustingTransform,
    PlayingAnimation,
    Finished,
}

/// Main component for an interactive action (e.g., sitting, vaulting)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ActionSystem {
    pub action_name: String,
    pub is_active: bool,
    
    // Activation Conditions
    pub use_min_distance: bool,
    pub min_distance: f32,
    pub use_min_angle: bool,
    pub min_angle: f32,
    
    // Player Adjustment
    pub use_position_to_adjust_player: bool,
    // Target transform to match player against
    pub match_target_transform: Option<Transform>, 
    pub adjust_player_position_speed: f32,
    pub rotate_player_to_face_target: bool,

    pub duration: f32,
    pub animation_speed: f32,
    pub animation_clip: Option<Handle<AnimationClip>>, // Placeholder for animation asset
    
    // State Overrides
    pub disable_physics: bool,
    pub disable_gravity: bool,
    pub disable_input: bool,
    
    // Internal state
    pub player_detected: bool,
}

impl Default for ActionSystem {
    fn default() -> Self {
        Self {
            action_name: "Action".to_string(),
            is_active: true,
            use_min_distance: true,
            min_distance: 2.0,
            use_min_angle: true,
            min_angle: 45.0,
            use_position_to_adjust_player: false,
            match_target_transform: None,
            adjust_player_position_speed: 5.0,
            rotate_player_to_face_target: true,
            duration: 1.0,
            animation_speed: 1.0,
            animation_clip: None,
            disable_physics: true,
            disable_gravity: true,
            disable_input: true,
            player_detected: false,
        }
    }
}

/// Component on the player to manage active actions
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerActionSystem {
    pub current_action: Option<Entity>,
    pub is_action_active: bool,
    
    pub state: ActionState,
    pub action_timer: f32,
    
    // State backup to restore after action
    pub previous_gravity_state: bool,
    pub previous_physics_state: bool,
}

impl Default for PlayerActionSystem {
    fn default() -> Self {
        Self {
            current_action: None,
            is_action_active: false,
            state: ActionState::Idle,
            action_timer: 0.0,
            previous_gravity_state: true,
            previous_physics_state: true,
        }
    }
}

/// Event to trigger an action
#[derive(Debug, Clone, Copy, Event)]
pub struct StartActionEvent {
    pub action_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct StartActionEventQueue(pub Vec<StartActionEvent>);

/// Event when an action ends
#[derive(Debug, Clone, Copy, Event)]
pub struct EndActionEvent {
    pub action_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct EndActionEventQueue(pub Vec<EndActionEvent>);

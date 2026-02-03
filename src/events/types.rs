use bevy::prelude::*;

/// Parameters that can be passed with an event
#[derive(Debug, Clone, Reflect)]
pub enum EventParameter {
    None,
    Float(f32),
    Bool(bool),
    String(String),
}

/// A "Remote Event" that can be triggered by name.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RemoteEventReceiver {
    /// List of event names this entity listens for
    // In a real Bevy app, we might just use specific components, 
    // but we keep the string-based mapping.
    pub events: Vec<String>,
}

impl Default for RemoteEventReceiver {
    fn default() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}

/// A generic event sent to trigger logic by name
#[derive(Event, Debug, Clone)]
pub struct RemoteEvent {
    pub name: String,
    pub target: Option<Entity>,
    pub source: Option<Entity>,
    pub parameter: EventParameter,
}

#[derive(Resource, Default)]
pub struct RemoteEventQueue(pub Vec<RemoteEvent>);

/// Defines a trigger area that fires events on intersection
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EventTrigger {
    pub on_enter: bool,
    pub on_exit: bool,
    pub on_stay: bool,
    
    pub tags: Vec<String>, // Tags required to trigger
    pub target_entity: Option<Entity>, // Only specific entity triggers
    
    // Actions to perform
    pub enter_events: Vec<TriggerEventInfo>,
    pub exit_events: Vec<TriggerEventInfo>,
    
    // State
    pub is_active: bool,
    pub trigger_limit: Option<u32>, // Max times to trigger
    pub times_triggered: u32,
    pub delay: f32,
}

#[derive(Component, Debug, Default)]
pub struct PreviousCollisions(pub std::collections::HashSet<Entity>);

impl Default for EventTrigger {
    fn default() -> Self {
        Self {
            on_enter: true,
            on_exit: false,
            on_stay: false,
            tags: vec!["Player".to_string()],
            target_entity: None,
            enter_events: Vec::new(),
            exit_events: Vec::new(),
            is_active: true,
            trigger_limit: None,
            times_triggered: 0,
            delay: 0.0,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct TriggerEventInfo {
    pub event_name: String,
    pub use_remote_event: bool, // If true, sends RemoteEvent. If false, might do something else (placeholder)
    pub parameter: EventParameter,
    pub delay: f32,
}

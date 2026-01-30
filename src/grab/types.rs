use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component for objects that can be grabbed.
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Grabbable {
    /// Whether the object uses weight for grab limits.
    pub use_weight: bool,
    /// Weight of the object.
    pub weight: f32,
    /// Extra distance allowed for grabbing this specific object.
    pub extra_grab_distance: f32,
    /// Whether to fire events on grab/drop.
    pub use_events: bool,
}

impl Default for Grabbable {
    fn default() -> Self {
        Self {
            use_weight: true,
            weight: 1.0,
            extra_grab_distance: 0.0,
            use_events: false,
        }
    }
}

/// Grabbing modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum GrabMode {
    /// Physics-based follow (spring/velocity).
    Powers,
    /// Fixed position relative to grabber.
    Realistic,
}

/// Component for entities that can grab objects.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Grabber {
    /// Currently held object.
    pub held_object: Option<Entity>,
    /// Target distance to hold the object.
    pub hold_distance: f32,
    /// Max distance before force-dropping.
    pub max_hold_distance: f32,
    /// Speed of movement follow.
    pub hold_speed: f32,
    /// Current rotation sensitivity.
    pub rotation_speed: f32,
    /// Current throw force.
    pub throw_force: f32,
    /// Max throw force.
    pub max_throw_force: f32,
    /// Current grab mode.
    pub mode: GrabMode,
    /// Is currently rotating the object?
    pub is_rotating: bool,
    /// Is charging a throw?
    pub is_charging_throw: bool,
}

impl Default for Grabber {
    fn default() -> Self {
        Self {
            held_object: None,
            hold_distance: 2.0,
            max_hold_distance: 4.0,
            hold_speed: 10.0,
            rotation_speed: 5.0,
            throw_force: 500.0,
            max_throw_force: 2000.0,
            mode: GrabMode::Powers,
            is_rotating: false,
            is_charging_throw: false,
        }
    }
}

/// Grab action events.
#[derive(Event, Debug, Clone)]
pub enum GrabEvent {
    Grab(Entity, Entity), // Grabber, Grabbable
    Drop(Entity, Entity), // Grabber, Grabbable
    Throw(Entity, Entity, Vec3, f32), // Grabber, Grabbable, Direction, Force
}

#[derive(Resource, Default)]
pub struct GrabEventQueue(pub Vec<GrabEvent>);

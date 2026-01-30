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
    /// Reference to a specific object parent to grab instead of this one.
    pub parent_to_grab: Option<Entity>,
}

impl Default for Grabbable {
    fn default() -> Self {
        Self {
            use_weight: true,
            weight: 1.0,
            extra_grab_distance: 0.0,
            use_events: false,
            parent_to_grab: None,
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

/// Component to redirect grab action to a parent or another entity.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GrabObjectParent {
    pub object_to_grab: Entity,
}

/// Component for handling events on grab/drop.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GrabObjectEventSystem {
    pub use_event_on_grab: bool,
    pub use_event_on_drop: bool,
    // Note: In Bevy we'll use actual events or specialized systems
}

/// Component for an object that can be placed into a slot.
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ObjectToPlace {
    pub object_name: String,
    pub is_placed: bool,
    pub can_call_placed_event: bool,
    pub can_call_removed_event: bool,
}

impl Default for ObjectToPlace {
    fn default() -> Self {
        Self {
            object_name: "Default".to_string(),
            is_placed: false,
            can_call_placed_event: true,
            can_call_removed_event: true,
        }
    }
}

/// Component for a slot where objects can be placed.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PutObjectSystem {
    pub use_certain_object: bool,
    pub certain_object_to_place: Option<Entity>,
    pub object_name_to_place: String,
    pub place_to_put: Option<Entity>, // Usually a child transform
    pub current_object_placed: Option<Entity>,
    pub position_speed: f32,
    pub rotation_speed: f32,
    pub is_object_placed: bool,
    pub max_distance_to_place: f32,
    pub disable_object_on_place: bool,
}

impl Default for PutObjectSystem {
    fn default() -> Self {
        Self {
            use_certain_object: false,
            certain_object_to_place: None,
            object_name_to_place: "Default".to_string(),
            place_to_put: None,
            current_object_placed: None,
            position_speed: 10.0,
            rotation_speed: 10.0,
            is_object_placed: false,
            max_distance_to_place: 0.5,
            disable_object_on_place: false,
        }
    }
}

/// Component for entities that can grab multiple objects with "powers".
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GrabPowerer {
    pub is_enabled: bool,
    pub grab_radius: f32,
    pub held_objects: Vec<Entity>,
    pub launch_force: f32,
    pub max_launch_force: f32,
    pub launch_speed: f32,
    pub is_charging: bool,
    pub last_grab_time: f32,
}

impl Default for GrabPowerer {
    fn default() -> Self {
        Self {
            is_enabled: true,
            grab_radius: 10.0,
            held_objects: Vec::new(),
            launch_force: 500.0,
            max_launch_force: 3500.0,
            launch_speed: 1200.0,
            is_charging: false,
            last_grab_time: 0.0,
        }
    }
}

/// Settings for object outlining.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct OutlineSettings {
    pub enabled: bool,
    pub width: f32,
    pub color: Color,
    pub active: bool,
}

impl Default for OutlineSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            width: 0.05,
            color: Color::srgba(1.0, 1.0, 0.0, 1.0), // srgba for Yellow
            active: false,
        }
    }
}

/// Information for a specific melee attack.
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct GrabAttackInfo {
    pub name: String,
    pub damage: f32,
    pub attack_type: String, // e.g., "Slash", "Bash"
    pub stamina_cost: f32,
    pub duration: f32,
}

/// Component to allow using a grabbed object as a melee weapon.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GrabMeleeWeapon {
    pub attacks: Vec<GrabAttackInfo>,
    pub can_block: bool,
    pub block_protection: f32,
    pub can_throw_return: bool,
    pub throw_speed: f32,
    pub return_speed: f32,
    pub damage_type_id: i32,
}

impl Default for GrabMeleeWeapon {
    fn default() -> Self {
        Self {
            attacks: Vec::new(),
            can_block: true,
            block_protection: 0.5,
            can_throw_return: false,
            throw_speed: 20.0,
            return_speed: 30.0,
            damage_type_id: 0,
        }
    }
}

/// Extended physics settings for grab objects.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GrabPhysicalObjectSettings {
    pub grab_physically: bool,
    pub set_mass: bool,
    pub mass_value: f32,
    pub tag_when_active: String,
    pub tag_when_inactive: String,
    pub disable_collider_on_grab: bool,
}

impl Default for GrabPhysicalObjectSettings {
    fn default() -> Self {
        Self {
            grab_physically: true,
            set_mass: false,
            mass_value: 1.0,
            tag_when_active: "Grabbable".to_string(),
            tag_when_inactive: "Default".to_string(),
            disable_collider_on_grab: false,
        }
    }
}

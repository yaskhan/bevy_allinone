//! Weapon types and components
//!
//! Core data structures for weapons, projectiles, and related components.

use bevy::prelude::*;

/// Weapon component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub weapon_name: String,
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
    pub current_fire_timer: f32,
    pub ammo_capacity: i32,
    pub current_ammo: i32,
    pub reload_time: f32,
    pub current_reload_timer: f32,
    pub is_reloading: bool,
    pub is_automatic: bool,
    pub spread: f32,
    pub base_spread: f32,
    pub aim_spread_mult: f32,
    pub projectiles_per_shot: u32,
    pub projectile_speed: f32,
    pub weapon_type: WeaponType,
    pub attachments: Vec<Attachment>,
    // Ballistic properties for projectiles fired from this weapon
    pub projectile_mass: f32,           // kg
    pub projectile_drag_coeff: f32,     // Cd
    pub projectile_area: f32,           // m^2
    pub projectile_penetration: f32,    // Joules or arbitrary units
    // Zeroing distance (meters)
    pub zeroing_distance: f32,
    // Weapon pocket system
    pub pocket_id: Option<String>,      // Which pocket this weapon belongs to
    pub key_number: u8,                 // Quick access slot number (1-10)
    pub enabled: bool,                  // Whether weapon is available
    pub equipped: bool,                 // Whether weapon is currently equipped
    pub carrying: bool,                 // Whether weapon is being carried
    pub is_dual: bool,                  // Whether this is a dual-wield weapon
    pub linked_dual_weapon: Option<String>, // For dual weapons, the linked weapon name
    pub using_right_hand: bool,         // For dual wield, which hand
    pub can_be_dropped: bool,           // Whether weapon can be dropped
    pub last_fired_time: f32,           // Last time weapon was fired
    pub last_reloaded_time: f32,        // Last time weapon was reloaded
    pub last_drawn_time: f32,           // Last time weapon was drawn
    // Base stats (for attachment modifiers)
    pub base_damage: f32,
    pub base_spread_value: f32,
    pub base_fire_rate: f32,
    pub base_reload_time: f32,
    pub base_ammo_capacity: i32,
    pub base_range: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_name: "Generic Gun".to_string(),
            damage: 10.0,
            range: 50.0,
            fire_rate: 0.1, // 10 shots/sec
            current_fire_timer: 0.0,
            ammo_capacity: 30,
            current_ammo: 30,
            reload_time: 1.5,
            current_reload_timer: 0.0,
            is_reloading: false,
            is_automatic: false,
            spread: 0.0,
            base_spread: 2.0, // Degrees
            aim_spread_mult: 0.2, // Tighter when aiming
            projectiles_per_shot: 1,
            projectile_speed: 0.0, // 0 = hitscan
            weapon_type: WeaponType::Pistol,
            attachments: Vec::new(),
            projectile_mass: 0.008, // 9mm approx 8g
            projectile_drag_coeff: 0.3,
            projectile_area: 0.000005, // 9mm diameter area
            projectile_penetration: 500.0,
            zeroing_distance: 50.0,
            pocket_id: None,
            key_number: 0,
            enabled: true,
            equipped: false,
            carrying: false,
            is_dual: false,
            linked_dual_weapon: None,
            using_right_hand: false,
            can_be_dropped: true,
            last_fired_time: 0.0,
            last_reloaded_time: 0.0,
            last_drawn_time: 0.0,
            // Base stats (for attachment modifiers)
            base_damage: 10.0,
            base_spread_value: 2.0,
            base_fire_rate: 0.1,
            base_reload_time: 1.5,
            base_ammo_capacity: 30,
            base_range: 50.0,
        }
    }
}

/// Weapon type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum WeaponType {
    Melee,
    #[default]
    Pistol,
    Rifle,
    Shotgun,
    Bow,
    Thrown,
}

/// Weapon attachment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Attachment {
    Silencer,
    ExtendedMag,
    Scope,
    HeavyBarrel,
    LaserSight,
}

/// Projectile component for ballistics
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub damage: f32,
    pub lifetime: f32,
    pub owner: Entity,
    // Physical properties
    pub mass: f32,
    pub drag_coeff: f32,
    pub reference_area: f32,
    pub penetration_power: f32,
}

/// Visual tracer component for interpolation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BulletTracer {
    pub target_entity: Entity,
    pub current_pos: Vec3,
    pub target_pos: Vec3,
    pub speed: f32,
}

/// Accuracy component for dynamic spread
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Accuracy {
    pub current_bloom: f32,
    pub base_spread: f32,
    pub max_spread: f32,
    pub bloom_per_shot: f32,
    pub recovery_rate: f32,
    // Modifiers
    pub movement_penalty: f32,
    pub ads_modifier: f32,
    pub airborne_multiplier: f32,
}

/// Global Ballistics Environment Resource
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct BallisticsEnvironment {
    pub gravity: Vec3,
    pub air_density: f32,
    pub wind: Vec3,
}

impl Default for BallisticsEnvironment {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_density: 1.225,
            wind: Vec3::ZERO,
        }
    }
}

/// Object Pool for visual effects (Sparks, Decals)
#[derive(Resource, Debug, Default)]
pub struct VisualEffectPool {
    pub available_sparks: Vec<Entity>,
    pub available_decals: Vec<Entity>,
}

// ============================================================================
// Weapon Pocket System
// ============================================================================

/// Represents a weapon pocket for organized storage
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct WeaponPocket {
    /// Pocket identifier
    pub id: String,
    /// Pocket display name
    pub name: String,
    /// Maximum number of weapons in this pocket
    pub capacity: usize,
    /// Current weapons in this pocket
    pub weapon_ids: Vec<String>,
    /// Whether this pocket is active
    pub active: bool,
    /// Pocket type (e.g., "primary", "secondary", "melee", "special")
    pub pocket_type: PocketType,
}

/// Types of weapon pockets
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Default)]
pub enum PocketType {
    #[default]
    Primary,
    Secondary,
    Melee,
    Special,
    Grenade,
    Custom(String),
}

impl PocketType {
    pub fn as_str(&self) -> &str {
        match self {
            PocketType::Primary => "primary",
            PocketType::Secondary => "secondary",
            PocketType::Melee => "melee",
            PocketType::Special => "special",
            PocketType::Grenade => "grenade",
            PocketType::Custom(name) => name,
        }
    }
}

impl Default for WeaponPocket {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            capacity: 3,
            weapon_ids: Vec::new(),
            active: true,
            pocket_type: PocketType::Primary,
        }
    }
}

impl WeaponPocket {
    /// Create a new weapon pocket
    pub fn new(id: &str, name: &str, capacity: usize, pocket_type: PocketType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            capacity,
            weapon_ids: Vec::new(),
            active: true,
            pocket_type,
        }
    }

    /// Check if pocket has room for another weapon
    pub fn has_room(&self) -> bool {
        self.weapon_ids.len() < self.capacity
    }

    /// Add a weapon to the pocket
    pub fn add_weapon(&mut self, weapon_id: &str) -> bool {
        if self.has_room() && !self.weapon_ids.contains(&weapon_id.to_string()) {
            self.weapon_ids.push(weapon_id.to_string());
            true
        } else {
            false
        }
    }

    /// Remove a weapon from the pocket
    pub fn remove_weapon(&mut self, weapon_id: &str) -> bool {
        if let Some(index) = self.weapon_ids.iter().position(|id| id == weapon_id) {
            self.weapon_ids.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if pocket contains a weapon
    pub fn contains_weapon(&self, weapon_id: &str) -> bool {
        self.weapon_ids.iter().any(|id| id == weapon_id)
    }

    /// Get number of weapons in pocket
    pub fn weapon_count(&self) -> usize {
        self.weapon_ids.len()
    }
}

/// List of weapons in a pocket (for weapon list manager)
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct WeaponListOnPocket {
    pub name: String,
    pub weapon_list: Vec<Entity>,
}

impl Default for WeaponListOnPocket {
    fn default() -> Self {
        Self {
            name: String::new(),
            weapon_list: Vec::new(),
        }
    }
}

use bevy::prelude::*;

/// Explosive barrel pickup/container.
///
/// GKC reference: `explosiveBarrel.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExplosiveBarrel {
    pub explosion_sound: String,
    pub explosion_damage: f32,
    pub ignore_shield: bool,
    pub damage_radius: f32,
    pub min_velocity_to_explode: f32,
    pub explosion_delay: f32,
    pub explosion_force: f32,
    pub break_in_pieces: bool,
    pub can_damage_to_owner: bool,
    pub damage_type_id: i32,
    pub explosion_force_to_pieces: f32,
    pub explosion_radius_to_pieces: f32,
    pub push_characters: bool,
    pub kill_objects_in_radius: bool,
    pub apply_explosion_force_to_vehicles: bool,
    pub explosion_force_to_vehicles_multiplier: f32,
    pub use_remote_event_on_objects_found: bool,
    pub remote_event_name: String,
    pub can_explode: bool,
    pub show_gizmo: bool,
    pub broken_barrel_prefab: String,
    pub explosion_particles_prefab: String,
    pub transparent_shader: String,
    pub exploded: bool,
}

impl Default for ExplosiveBarrel {
    fn default() -> Self {
        Self {
            explosion_sound: String::new(),
            explosion_damage: 0.0,
            ignore_shield: false,
            damage_radius: 0.0,
            min_velocity_to_explode: 0.0,
            explosion_delay: 0.0,
            explosion_force: 300.0,
            break_in_pieces: false,
            can_damage_to_owner: true,
            damage_type_id: -1,
            explosion_force_to_pieces: 5.0,
            explosion_radius_to_pieces: 30.0,
            push_characters: true,
            kill_objects_in_radius: false,
            apply_explosion_force_to_vehicles: true,
            explosion_force_to_vehicles_multiplier: 0.2,
            use_remote_event_on_objects_found: false,
            remote_event_name: String::new(),
            can_explode: true,
            show_gizmo: false,
            broken_barrel_prefab: String::new(),
            explosion_particles_prefab: String::new(),
            transparent_shader: String::new(),
            exploded: false,
        }
    }
}

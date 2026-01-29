//! Jetpack System
//!
//! Manages jetpack mechanics including equipping, fuel consumption/regeneration, and thrust physics.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct JetpackPlugin;

impl Plugin for JetpackPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Jetpack>()
            .add_event::<EquipJetpackEvent>()
            .add_event::<ToggleJetpackEvent>()
            .add_event::<JetpackTurboEvent>()
            .add_systems(Update, (
                handle_jetpack_events,
                update_jetpack_physics,
                manage_jetpack_fuel,
            ).chain());
    }
}

/// Component to configure and manage jetpack state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Jetpack {
    pub equipped: bool,
    pub active: bool, // Currently thrusting
    pub turbo_active: bool,
    
    pub max_fuel: f32,
    pub current_fuel: f32,
    pub fuel_burn_rate: f32,
    pub fuel_recharge_rate: f32,
    pub recharge_delay: f32,
    pub last_used_time: f32,

    pub force: f32,
    pub turbo_force_multiplier: f32,
    pub air_control: f32,
    pub max_velocity: f32,
    
    pub velocity: Vec3, // Simulated velocity
}

impl Default for Jetpack {
    fn default() -> Self {
        Self {
            equipped: false,
            active: false,
            turbo_active: false,
            max_fuel: 100.0,
            current_fuel: 100.0,
            fuel_burn_rate: 10.0,
            fuel_recharge_rate: 5.0,
            recharge_delay: 2.0,
            last_used_time: 0.0,
            force: 15.0,
            turbo_force_multiplier: 1.5,
            air_control: 5.0,
            max_velocity: 60.0,
            velocity: Vec3::ZERO,
        }
    }
}

/// Event to toggle equipped state
#[derive(Event)]
pub struct EquipJetpackEvent {
    pub entity: Entity,
    pub equip: bool,
}

/// Event to toggle active thrusting state
#[derive(Event)]
pub struct ToggleJetpackEvent {
    pub entity: Entity,
    pub active: bool,
}

/// Event to toggle turbo mode
#[derive(Event)]
pub struct JetpackTurboEvent {
    pub entity: Entity,
    pub active: bool,
}

/// System to handle jetpack state events
pub fn handle_jetpack_events(
    mut equip_events: EventReader<EquipJetpackEvent>,
    mut toggle_events: EventReader<ToggleJetpackEvent>,
    mut turbo_events: EventReader<JetpackTurboEvent>,
    mut query: Query<&mut Jetpack>,
    time: Res<Time>,
) {
    for event in equip_events.read() {
        if let Ok(mut jetpack) = query.get_mut(event.entity) {
            jetpack.equipped = event.equip;
            if !jetpack.equipped {
                jetpack.active = false; // Auto deactivate if unequipped
            }
            info!("Jetpack: Equipped state set to {} for {:?}", jetpack.equipped, event.entity);
        }
    }

    for event in toggle_events.read() {
        if let Ok(mut jetpack) = query.get_mut(event.entity) {
            if jetpack.equipped && jetpack.current_fuel > 0.0 {
                jetpack.active = event.active;
                if jetpack.active {
                     // Reset last used time will happen when stopping? 
                     // Or maybe tracking active state handles it.
                } else {
                     jetpack.last_used_time = time.elapsed_secs();
                }
                info!("Jetpack: Active state set to {} for {:?}", jetpack.active, event.entity);
            } else if !jetpack.equipped {
                warn!("Jetpack: Cannot activate, not equipped!");
            } else {
                 warn!("Jetpack: Cannot activate, no fuel!");
            }
        }
    }
    
    for event in turbo_events.read() {
        if let Ok(mut jetpack) = query.get_mut(event.entity) {
            jetpack.turbo_active = event.active;
            info!("Jetpack: Turbo state set to {} for {:?}", jetpack.turbo_active, event.entity);
        }
    }
}

/// System to manage fuel consumption and regeneration
pub fn manage_jetpack_fuel(
    mut query: Query<&mut Jetpack>,
    time: Res<Time>,
) {
    for mut jetpack in query.iter_mut() {
        if jetpack.active {
            // Consume fuel
            let consumption = jetpack.fuel_burn_rate * time.delta_secs();
            jetpack.current_fuel = (jetpack.current_fuel - consumption).max(0.0);
            
            // Auto-cutoff if out of fuel
            if jetpack.current_fuel <= 0.0 {
                jetpack.active = false;
                jetpack.last_used_time = time.elapsed_secs();
                info!("Jetpack: Out of fuel!");
            }
        } else {
            // Regenerate fuel if enough time passed since use
            if time.elapsed_secs() > jetpack.last_used_time + jetpack.recharge_delay {
                if jetpack.current_fuel < jetpack.max_fuel {
                    let recharge = jetpack.fuel_recharge_rate * time.delta_secs();
                    jetpack.current_fuel = (jetpack.current_fuel + recharge).min(jetpack.max_fuel);
                }
            }
        }
    }
}

/// System to apply jetpack physics
pub fn update_jetpack_physics(
    mut query: Query<(&mut Jetpack, &GlobalTransform, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (mut jetpack, global_transform, transform_opt) in query.iter_mut() {
        if !jetpack.active {
            continue;
        }

        // Calculate force direction (Up + Horizontal Input)
        let up = Vec3::Y; // Global Up
        let move_input = input_state.move_direction;
        let forward = global_transform.forward();
        let right = global_transform.right();
        
        let mut force_dir = up; // Primary thrust is up
        
        // Add horizontal control
        force_dir += forward * move_input.z * 0.5; // Scale horizontal influence
        force_dir += right * move_input.x * 0.5;
        
        force_dir = force_dir.normalize();

        let mut force_magnitude = jetpack.force;
        if jetpack.turbo_active {
            force_magnitude *= jetpack.turbo_force_multiplier;
        }

        // Apply to velocity (Simulated)
        let acceleration = force_dir * force_magnitude;
        jetpack.velocity += acceleration * time.delta_secs();
        
         // Clamp max velocity
        if jetpack.velocity.length() > jetpack.max_velocity {
            jetpack.velocity = jetpack.velocity.normalize() * jetpack.max_velocity;
        }

        // Apply velocity to transform (Manual integration)
        if let Some(mut transform) = transform_opt {
             transform.translation += jetpack.velocity * time.delta_secs();
        }
    }
}

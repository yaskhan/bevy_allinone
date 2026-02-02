use bevy::prelude::*;
use crate::character::types::*;
use crate::physics::GroundDetection;
use crate::combat::{DamageEventQueue, DamageEvent, DamageType}; // Import combat types
use avian3d::prelude::*;

pub fn handle_falling_damage(
    time: Res<Time>,
    mut damage_queue: ResMut<DamageEventQueue>, // Use Queue instead of EventWriter
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &LinearVelocity, &GroundDetection)>,
) {
    for (entity, controller, mut state, velocity, ground) in query.iter_mut() {
        if !controller.fall_damage_enabled { continue; }

        if !ground.is_grounded {
            state.last_vertical_velocity = velocity.y;
            state.air_time += time.delta_secs();
        } else if state.last_vertical_velocity < -controller.min_velocity_for_damage {
            let impact_speed = state.last_vertical_velocity.abs();
            // Damage formula: (impact + duration) * multiplier
            let damage = (impact_speed - controller.min_velocity_for_damage + state.air_time * 2.0) * controller.falling_damage_multiplier;
            
            // Push to Damage Queue
            damage_queue.0.push(DamageEvent {
                target: entity,
                amount: damage,
                damage_type: DamageType::Fall,
                source: None,
                position: Some(Vec3::ZERO), // Or player position?
                direction: Some(Vec3::Y),
                ignore_shield: true, // Typically fall damage ignores shields? GKit behavior? Let's assume yes or make it config.
            });
            
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        } else {
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        }
    }
}

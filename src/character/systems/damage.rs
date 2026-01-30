use bevy::prelude::*;
use crate::character::types::*;
use crate::physics::GroundDetection;
use avian3d::prelude::*;

pub fn handle_falling_damage(
    time: Res<Time>,
    // mut damage_events: EventWriter<DamageEvent>,
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &LinearVelocity, &GroundDetection)>,
) {
    for (_entity, controller, mut state, velocity, ground) in query.iter_mut() {
        if !controller.fall_damage_enabled { continue; }

        if !ground.is_grounded {
            state.last_vertical_velocity = velocity.y;
            state.air_time += time.delta_secs();
        } else if state.last_vertical_velocity < -controller.min_velocity_for_damage {
            let impact_speed = state.last_vertical_velocity.abs();
            // Damage formula: (impact + duration) * multiplier
            let _damage = (impact_speed - controller.min_velocity_for_damage + state.air_time * 2.0) * controller.falling_damage_multiplier;
            
            // Commented out as per instruction:
            // damage_events.send(DamageEvent {
            //     target: entity,
            //     amount: damage,
            //     damage_type: DamageType::Fall,
            //     source: None,
            // });
            
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        } else {
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        }
    }
}

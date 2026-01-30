use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::input::InputState;

pub fn vehicle_input_system(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut InputState, &Children)>,
    driver_query: Query<&InputState, (With<VehicleDriver>, Without<Vehicle>)>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut v_input, children) in vehicle_query.iter_mut() {
        let mut found_driver = false;
        for child in children.iter() {
            if let Ok(input) = driver_query.get(child) {
                v_input.movement = input.movement;
                v_input.jump_pressed = input.jump_pressed;
                v_input.interact_pressed = input.interact_pressed;
                found_driver = true;
                break;
            }
        }

        if !found_driver {
            v_input.movement = Vec2::ZERO;
            v_input.jump_pressed = false;
            vehicle.is_boosting = false;
            vehicle.is_jumping = false;
        }

        // Update vehicle input state
        vehicle.is_driving = found_driver;

        // Handle boost input
        if vehicle.can_use_boost && vehicle.is_driving && vehicle.is_turned_on {
            if v_input.jump_pressed && !vehicle.is_jumping {
                vehicle.is_boosting = true;
                vehicle.boost_input = vehicle.boost_multiplier;
            } else {
                vehicle.is_boosting = false;
                vehicle.boost_input = 1.0;
            }
        } else {
            vehicle.is_boosting = false;
            vehicle.boost_input = 1.0;
        }

        // Handle jump input
        if vehicle.can_jump && vehicle.is_driving && vehicle.is_turned_on && v_input.jump_pressed && !vehicle.is_jumping {
            vehicle.is_jumping = true;
        }

        // Update motor and steer inputs with smoothing
        let target_motor = v_input.movement.y;
        let target_steer = v_input.movement.x;

        // Smooth steering
        vehicle.steer_input += (target_steer - vehicle.steer_input) * 10.0 * delta;
        vehicle.steer_input = vehicle.steer_input.clamp(-1.0, 1.0);

        // Smooth motor input
        if !vehicle.is_reversing {
            vehicle.motor_input += (target_motor - vehicle.motor_input) * 5.0 * delta;
        } else {
            vehicle.motor_input += (target_motor - vehicle.motor_input) * 3.0 * delta;
        }
        vehicle.motor_input = vehicle.motor_input.clamp(-1.0, 1.0);

        // Handle braking
        vehicle.is_braking = vehicle.motor_input.abs() < 0.05 && vehicle.current_speed > 0.5;
    }
}

use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::input::InputState;
use crate::character::{CharacterController, CharacterMovementState};

pub fn handle_vehicle_interaction(
    _time: Res<Time>,
    mut commands: Commands,
    mut character_query: Query<(Entity, &GlobalTransform, &InputState, &mut CharacterMovementState, Has<ChildOf>), With<CharacterController>>,
    mut seat_query: Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
) {
    for (entity, gt, input, mut state, is_in_vehicle) in character_query.iter_mut() {
        if input.interact_pressed {
            if is_in_vehicle {
                // If already in a vehicle/seat, try to exit
                for (_seat_entity, mut seat, seat_gt) in seat_query.iter_mut() {
                    if seat.occupied_by == Some(entity) {
                        // Exit
                        info!("Exiting vehicle...");
                        seat.occupied_by = None;
                        commands.entity(entity).remove_parent_in_place();
                        commands.entity(entity).remove::<VehicleDriver>();

                        // Update character state
                        state.is_in_vehicle = false;
                        state.vehicle_entity = None;

                        // Teleport slightly outside
                        let exit_pos = seat_gt.translation() + seat_gt.right() * 2.0 + Vec3::Y * 0.5;
                        commands.entity(entity).insert(Transform::from_translation(exit_pos));

                        break;
                    }
                }
                continue;
            }

            // If not in a vehicle, look for one to enter
            let char_pos: Vec3 = gt.translation();
            let mut closest_seat_entity = None;
            let mut min_dist = 4.0;

            for (seat_entity, seat, seat_gt) in seat_query.iter() {
                if seat.occupied_by.is_none() {
                    let dist = seat_gt.translation().distance(char_pos);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_seat_entity = Some(seat_entity);
                    }
                }
            }

            if let Some(seat_entity) = closest_seat_entity {
                info!("Entering vehicle seat...");
                if let Ok((_, mut seat, _)) = seat_query.get_mut(seat_entity) {
                    seat.occupied_by = Some(entity);
                    let is_driver = seat.is_driver_seat;

                    commands.entity(entity).set_parent_in_place(seat_entity);
                    if is_driver {
                        commands.entity(entity).insert(VehicleDriver);
                    }

                    // Update character state
                    state.is_in_vehicle = true;
                    state.vehicle_entity = Some(seat_entity);
                }
            }
        }
    }
}

use bevy::prelude::*;
use crate::vehicles::types::*;
use avian3d::prelude::*;
use crate::character::CharacterMovementState;

pub fn character_vehicle_sync_system(
    vehicle_query: Query<(&GlobalTransform, &Children, &Vehicle)>,
    seat_query: Query<&VehicleSeat>,
    mut occupant_query: Query<(&mut Transform, &mut LinearVelocity, &mut CharacterMovementState)>,
) {
    for (v_gt, children, _vehicle) in vehicle_query.iter() {
        for child in children.iter() {
            if let Ok(seat) = seat_query.get(child) {
                if let Some(occupant_entity) = seat.occupied_by {
                    if let Ok((mut trans, mut vel, mut state)) = occupant_query.get_mut(occupant_entity) {
                        // Position character in seat with offset
                        let seat_world_pos = v_gt.translation() + v_gt.rotation() * seat.offset;
                        trans.translation = seat_world_pos;

                        // Rotate character to face forward
                        trans.rotation = v_gt.rotation();

                        // Disable character movement
                        vel.0 = Vec3::ZERO;

                        // Update character state to indicate in vehicle
                        state.is_in_vehicle = true;
                        state.vehicle_entity = Some(occupant_entity);
                    }
                }
            }
        }
    }
}

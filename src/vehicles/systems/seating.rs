use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::character::{CharacterController, CharacterMovementState};
use avian3d::prelude::*;

pub fn manage_vehicle_passengers(
    mut commands: Commands,
    mut seat_query: Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
    mut manager_query: Query<(Entity, &VehicleSeatingManager, &Vehicle)>,
    mut character_query: Query<(Entity, &mut CharacterMovementState, &mut LinearVelocity)>,
) {
    // This system could handle periodic updates or specific transitions
}

pub fn eject_passengers(
    commands: &mut Commands,
    vehicle_entity: Entity,
    manager: &VehicleSeatingManager,
    seat_query: &mut Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
    character_query: &mut Query<(Entity, &mut CharacterMovementState, &mut LinearVelocity)>,
) {
    for seat_entity in &manager.seats {
        if let Ok((_e, mut seat, seat_gt)) = seat_query.get_mut(*seat_entity) {
            if let Some(passenger) = seat.occupied_by.take() {
                if let Ok((p_entity, mut state, mut velocity)) = character_query.get_mut(passenger) {
                    info!("Ejecting passenger from seat...");
                    
                    commands.entity(p_entity).remove_parent_in_place();
                    commands.entity(p_entity).remove::<VehicleDriver>();

                    state.is_in_vehicle = false;
                    state.vehicle_entity = None;

                    // Apply ejection force
                    let exit_pos = seat_gt.translation() + seat.exit_position;
                    commands.entity(p_entity).insert(Transform::from_translation(exit_pos));
                    
                    if manager.eject_on_destroy {
                        let eject_dir = (seat_gt.up().as_vec3() + seat_gt.right().as_vec3()).normalize();
                        velocity.0 += eject_dir * manager.eject_force;
                    }
                }
            }
        }
    }
}

pub fn handle_seating_interaction(
    time: Res<Time>,
    mut commands: Commands,
    mut character_query: Query<(Entity, &GlobalTransform, &crate::input::InputState, &mut CharacterMovementState, &mut LinearVelocity, Has<ChildOf>)>,
    mut seat_query: Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
    mut vehicle_query: Query<(Entity, &Vehicle, &VehicleSeatingManager)>,
) {
    // We'll call this from the main interaction system or replace it
}

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::{InputState, InputAction};
use super::types::*;

/// System to handle grab/drop input.
pub fn handle_grab_input(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &mut Grabber)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber) in grabber_query.iter_mut() {
        if let Some(held) = grabber.held_object {
            // Drop input
            if input.interact_pressed {
                 event_queue.0.push(GrabEvent::Drop(entity, held));
            }
        }
    }
}

/// System to process Grab events.
pub fn process_grab_events(
    mut event_queue: ResMut<GrabEventQueue>,
    mut grabber_query: Query<&mut Grabber>,
    mut grabbable_query: Query<(&Grabbable, &mut LinearVelocity)>,
) {
    let events: Vec<GrabEvent> = event_queue.0.drain(..).collect();

    for event in events {
        match event {
            GrabEvent::Grab(grabber_entity, target_entity) => {
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object.is_none() {
                        grabber.held_object = Some(target_entity);
                        info!("Object grabbed: {:?}", target_entity);
                    }
                }
            }
            GrabEvent::Drop(grabber_entity, target_entity) => {
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object == Some(target_entity) {
                        grabber.held_object = None;
                        grabber.is_rotating = false;
                        info!("Object dropped: {:?}", target_entity);
                    }
                }
            }
            GrabEvent::Throw(grabber_entity, target_entity, direction, force) => {
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object == Some(target_entity) {
                        grabber.held_object = None;
                        grabber.is_rotating = false;
                        
                        if let Ok((_grabbable, mut velocity)) = grabbable_query.get_mut(target_entity) {
                            // Apply throw as velocity change for simplicity if Impulse is missing
                            velocity.0 += direction * (force * 0.1); 
                            info!("Object thrown with force: {}", force);
                        }
                    }
                }
            }
        }
    }
}

/// System to update the position of the held object.
pub fn update_held_object(
    grabber_query: Query<(&Grabber, &GlobalTransform)>,
    mut held_query: Query<(&GlobalTransform, &mut LinearVelocity), With<Grabbable>>,
) {
    for (grabber, transform) in grabber_query.iter() {
        let Some(held_entity) = grabber.held_object else { continue };
        
        if let Ok((held_transform, mut velocity)) = held_query.get_mut(held_entity) {
            let target_pos = transform.translation() + transform.forward() * grabber.hold_distance;
            let current_pos = held_transform.translation();
            
            let dir = target_pos - current_pos;
            let distance = dir.length();
            
            if distance > grabber.max_hold_distance {
                // Should force-drop here. For now just dampen.
            }

            // Power-based follow
            velocity.0 = dir * grabber.hold_speed;
        }
    }
}

/// System to handle object rotation while held.
pub fn handle_rotation(
    input: Res<InputState>,
    mut grabber_query: Query<(&mut Grabber, &GlobalTransform)>,
    mut held_query: Query<&mut Transform, With<Grabbable>>,
) {
    for (mut grabber, grabber_xf) in grabber_query.iter_mut() {
        let Some(held_entity) = grabber.held_object else { continue };
        
        if input.aim_pressed {
            grabber.is_rotating = true;
            if let Ok(mut transform) = held_query.get_mut(held_entity) {
                let axis = input.look;
                
                let sensitivity = 0.1;
                let rot_y = Quat::from_rotation_y(-axis.x * grabber.rotation_speed * sensitivity);
                let rot_x = Quat::from_rotation_x(-axis.y * grabber.rotation_speed * sensitivity);
                
                let rotation = rot_y * rot_x;
                let pivot = transform.translation;
                
                // transform.rotate_around(pivot, rotation) manual implementation
                transform.rotation = rotation * transform.rotation;
            }
        } else {
            grabber.is_rotating = false;
        }
    }
}

/// System to handle throwing logic.
pub fn handle_throwing(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &mut Grabber, &GlobalTransform)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber, transform) in grabber_query.iter_mut() {
        let Some(held) = grabber.held_object else { continue };

        if input.attack_pressed {
            grabber.is_charging_throw = true;
            grabber.throw_force = (grabber.throw_force + 10.0).min(grabber.max_throw_force);
        } else if grabber.is_charging_throw {
            let dir = transform.forward();
            event_queue.0.push(GrabEvent::Throw(entity, held, *dir, grabber.throw_force));
            grabber.is_charging_throw = false;
            grabber.throw_force = 500.0;
        }
    }
}

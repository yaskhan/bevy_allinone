use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::{InputState, InputAction};
use super::types::*;

/// System to handle grab/drop input.
pub fn handle_grab_input(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &mut Grabber)>,
    mut powerer_query: Query<(Entity, &mut GrabPowerer)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber) in grabber_query.iter_mut() {
        if let Some(held) = grabber.held_object {
            if input.interact_pressed {
                 event_queue.0.push(GrabEvent::Drop(entity, held));
            }
        }
    }

    for (entity, mut powerer) in powerer_query.iter_mut() {
        if !powerer.held_objects.is_empty() {
            if input.interact_pressed {
                for held in powerer.held_objects.clone() {
                    event_queue.0.push(GrabEvent::Drop(entity, held));
                }
                powerer.held_objects.clear();
            }
        }
    }
}

/// System to process Grab events.
pub fn process_grab_events(
    mut event_queue: ResMut<GrabEventQueue>,
    mut grabber_query: Query<&mut Grabber>,
    mut powerer_query: Query<&mut GrabPowerer>,
    mut grabbable_query: Query<(&Grabbable, &mut LinearVelocity, Option<&mut Mass>, Option<&mut Collider>)>,
    parent_redirect_query: Query<&GrabObjectParent>,
    event_system_query: Query<&GrabObjectEventSystem>,
    physical_settings_query: Query<&GrabPhysicalObjectSettings>,
) {
    let events: Vec<GrabEvent> = event_queue.0.drain(..).collect();

    for event in events {
        match event {
            GrabEvent::Grab(grabber_entity, mut target_entity) => {
                // Check for redirection
                if let Ok(redirect) = parent_redirect_query.get(target_entity) {
                    target_entity = redirect.object_to_grab;
                }

                // Handle single grabber
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object.is_none() {
                        grabber.held_object = Some(target_entity);
                        info!("Object grabbed: {:?}", target_entity);

                        // Apply physical settings
                        if let Ok(settings) = physical_settings_query.get(target_entity) {
                            if let Ok((_g, _v, mut mass, mut _collider)) = grabbable_query.get_mut(target_entity) {
                                if settings.set_mass {
                                    if let Some(ref mut m) = mass {
                                        m.0 = settings.mass_value;
                                    }
                                }
                                // Collider toggle usually needs to be deferred or handled via commands
                            }
                        }
                    }
                }
                
                // Handle power powerer
                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    if !powerer.held_objects.contains(&target_entity) {
                        powerer.held_objects.push(target_entity);
                        info!("Object grabbed by power: {:?}", target_entity);
                    }
                }

                // Trigger event
                if let Ok(_event_sys) = event_system_query.get(target_entity) {
                    info!("Grab event triggered for object: {:?}", target_entity);
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

                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    powerer.held_objects.retain(|&e| e != target_entity);
                    info!("Object dropped by power: {:?}", target_entity);
                }

                // Trigger event
                if let Ok(_event_sys) = event_system_query.get(target_entity) {
                    info!("Drop event triggered for object: {:?}", target_entity);
                }
            }
            GrabEvent::Throw(grabber_entity, target_entity, direction, force) => {
                let mut thrown = false;
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object == Some(target_entity) {
                        grabber.held_object = None;
                        grabber.is_rotating = false;
                        thrown = true;
                    }
                }

                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    if powerer.held_objects.contains(&target_entity) {
                        powerer.held_objects.retain(|&e| e != target_entity);
                        thrown = true;
                    }
                }
                
                if thrown {
                    if let Ok((_grabbable, mut velocity, _, _)) = grabbable_query.get_mut(target_entity) {
                        velocity.0 += direction * (force * 0.1); 
                        info!("Object thrown with force: {}", force);

                        // Trigger event
                        if let Ok(_event_sys) = event_system_query.get(target_entity) {
                            info!("Throw event triggered for object: {:?}", target_entity);
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
/// System to handle object placement in slots.
pub fn update_put_object_systems(
    mut _commands: Commands,
    _time: Res<Time>,
    mut put_query: Query<(Entity, &mut PutObjectSystem, &GlobalTransform)>,
    mut to_place_query: Query<(Entity, &mut ObjectToPlace, &mut Transform, &mut LinearVelocity)>,
    _event_queue: ResMut<GrabEventQueue>,
    grabber_query: Query<(Entity, &Grabber)>,
) {
    for (_put_entity, mut put_system, put_xf) in put_query.iter_mut() {
        if put_system.is_object_placed {
            // Check if object is removed
            if let Some(placed_entity) = put_system.current_object_placed {
                if let Ok((_entity, mut to_place, transform, _vel)) = to_place_query.get_mut(placed_entity) {
                    let target_pos = put_xf.translation();

                    let dist = transform.translation.distance(target_pos);
                    if dist > put_system.max_distance_to_place * 1.5 {
                        // Object was moved away (e.g. grabbed by player)
                        info!("Object removed from slot: {:?}", placed_entity);
                        put_system.is_object_placed = false;
                        put_system.current_object_placed = None;
                        to_place.is_placed = false;
                    }
                } else {
                    // Object was deleted or similar
                    put_system.is_object_placed = false;
                    put_system.current_object_placed = None;
                }
                continue;
            }
        }

        // Logic to detect and place object
        for (to_place_entity, mut to_place, mut transform, mut velocity) in to_place_query.iter_mut() {
            if to_place.is_placed || to_place.object_name != put_system.object_name_to_place {
                continue;
            }

            if put_system.use_certain_object {
                if Some(to_place_entity) != put_system.certain_object_to_place {
                    continue;
                }
            }

            let target_pos = put_xf.translation();
            let dist = transform.translation.distance(target_pos);

            if dist < put_system.max_distance_to_place {
                // Check if currently held
                let mut is_held = false;
                for (_grabber_entity, grabber) in grabber_query.iter() {
                    if grabber.held_object == Some(to_place_entity) {
                        is_held = true;
                        break;
                    }
                }

                if !is_held {
                    // Place it!
                    info!("Object placed in slot: {:?}", to_place_entity);
                    put_system.is_object_placed = true;
                    put_system.current_object_placed = Some(to_place_entity);
                    to_place.is_placed = true;

                    // Magnetize to center
                    transform.translation = target_pos;
                    velocity.0 = Vec3::ZERO;
                }
            }
        }
    }
}
/// System to handle radius-based power grabbing.
pub fn handle_power_grabbing(
    input: Res<InputState>,
    mut powerer_query: Query<(Entity, &mut GrabPowerer, &GlobalTransform)>,
    grabbable_query: Query<(Entity, &GlobalTransform), With<Grabbable>>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut powerer, transform) in powerer_query.iter_mut() {
        if !powerer.is_enabled { continue; }

        if input.interact_pressed && powerer.held_objects.is_empty() {
             let center = transform.translation();
             for (target_entity, target_xf) in grabbable_query.iter() {
                 if target_xf.translation().distance(center) < powerer.grab_radius {
                     event_queue.0.push(GrabEvent::Grab(entity, target_entity));
                 }
             }
        }
    }
}

/// System to update positions of multiple held objects (Power mode).
pub fn update_power_held_objects(
    mut powerer_query: Query<(&mut GrabPowerer, &GlobalTransform)>,
    mut held_query: Query<(&GlobalTransform, &mut LinearVelocity), With<Grabbable>>,
) {
    for (mut powerer, transform) in powerer_query.iter_mut() {
        let mut to_remove = Vec::new();
        let forward = transform.forward();
        let right = transform.right();
        
        let held_count = powerer.held_objects.len();
        for (i, &held_entity) in powerer.held_objects.iter().enumerate() {
            if let Ok((held_xf, mut velocity)) = held_query.get_mut(held_entity) {
                // Arrange objects in a fan or arc in front of the player
                let angle = (i as f32 - (held_count as f32 - 1.0) / 2.0) * 0.5;
                let offset = (*forward * 3.0) + (*right * angle * 2.0);
                let target_pos = transform.translation() + offset;
                
                let dir = target_pos - held_xf.translation();
                velocity.0 = dir * 5.0; // Slow follow for "powers"
            } else {
                to_remove.push(held_entity);
            }
        }
        
        powerer.held_objects.retain(|e| !to_remove.contains(e));
    }
}

/// System to update object outlines.
pub fn update_outlines(
    mut _objects: Query<(&mut OutlineSettings, &Children)>,
    mut _materials: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    // Placeholder for visual outline logic
}

/// System to handle melee attack input for grabbed objects.
pub fn handle_grab_melee(
    input: Res<InputState>,
    grabber_query: Query<&Grabber>,
    weapon_query: Query<&GrabMeleeWeapon>,
) {
    for grabber in grabber_query.iter() {
        let Some(held) = grabber.held_object else { continue };
        if let Ok(_weapon) = weapon_query.get(held) {
            if input.attack_pressed {
                info!("Melee attack with grabbed object: {:?}", held);
                // Trigger animation and damage systems
            }
        }
    }
}

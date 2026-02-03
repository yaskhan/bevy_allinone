use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};
use super::types::*;
use super::components::*;
use super::events::*;
use super::resources::*;

/// System to setup the interaction UI
pub fn setup_interaction_ui(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 24.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(20.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            InteractionPrompt,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Interact"),
                text_style,
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
        });
}

/// System to update the interaction UI based on current detection
pub fn update_interaction_ui(
    current_interactable: Res<CurrentInteractable>,
    interactables: Query<&Interactable>,
    player_query: Query<&UsingDevicesSystem>,
    mut ui_query: Query<(&mut Visibility, &Children), With<InteractionPrompt>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut visibility, children) in ui_query.iter_mut() {
        let mut target_label = None;
        let mut target_is_in_range = false;

        // Try to get info from UsingDevicesSystem first
        if let Some(player_system) = player_query.iter().next() {
            if player_system.current_device_index >= 0 {
                if let Some(device) = player_system.device_list.get(player_system.current_device_index as usize) {
                    if let Ok(interactable) = interactables.get(device.entity) {
                        target_label = Some((interactable.interaction_type, interactable.interaction_text.clone()));
                        target_is_in_range = true;
                    }
                }
            }
        }

        // Fallback to CurrentInteractable
        if target_label.is_none() {
            if let Some(entity) = current_interactable.entity {
                if let Ok(interactable) = interactables.get(entity) {
                    target_label = Some((interactable.interaction_type, interactable.interaction_text.clone()));
                    target_is_in_range = current_interactable.is_in_range;
                }
            }
        }

        if let Some((interaction_type, interaction_text)) = target_label {
            *visibility = Visibility::Visible;
            
            for child in children.iter() {
                if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                    let key_text = "E"; 

                    let (color, suffix) = if target_is_in_range {
                        (Color::WHITE, "")
                    } else {
                        (Color::srgb(1.0, 0.2, 0.2), " (Too Far)")
                    };

                    text_color.0 = color;
                    
                    text.0 = format!("Press {} to {} {}{}", 
                        key_text, 
                        match interaction_type {
                            InteractionType::Pickup => "pick up",
                            InteractionType::Use => "use",
                            InteractionType::Talk => "talk to",
                            InteractionType::Open => "open",
                            InteractionType::Activate => "activate",
                            InteractionType::Examine => "examine",
                            InteractionType::Toggle => "toggle",
                            InteractionType::Grab => "grab",
                            InteractionType::Device => "use device",
                        },
                        interaction_text,
                        suffix
                    );
                }
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System to validate interactions (cooldowns, states)
pub fn validate_interactions(
    time: Res<Time>,
    mut interactables: Query<(&mut Interactable, Option<&mut InteractionData>)>,
) {
    for (mut interactable, data_opt) in interactables.iter_mut() {
        if let Some(mut data) = data_opt {
            // Update cooldown
            if data.cooldown_timer > 0.0 {
                data.cooldown_timer -= time.delta_secs();
                if data.cooldown_timer <= 0.0 {
                    interactable.can_interact = true;
                } else {
                    interactable.can_interact = false;
                }
            }
        }
    }
}

/// System to detect interactables using raycasting
pub fn detect_interactables(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut current_interactable: ResMut<CurrentInteractable>,
    mut detectors: Query<(
        &GlobalTransform,
        &mut InteractionDetector,
    )>,
    interactables: Query<&Interactable>,
) {
    // Clear current interactable at the start
    current_interactable.entity = None;
    current_interactable.distance = 0.0;
    current_interactable.is_in_range = false;

    for (transform, mut detector) in detectors.iter_mut() {
        // Update timer
        detector.time_since_update += time.delta_secs();
        
        // Check if we should update this frame
        if detector.time_since_update < detector.update_interval {
            continue;
        }
        
        // Reset timer
        detector.time_since_update = 0.0;

        // Calculate ray origin and direction
        let ray_origin = transform.translation() + detector.ray_offset;
        let ray_direction = transform.forward();

        // Perform raycast
        if let Some(hit) = spatial_query.cast_ray(
            ray_origin,
            ray_direction.into(),
            detector.max_distance,
            true, // ignore_origin_penetration
            &SpatialQueryFilter::default(),
        ) {
            // Check if hit entity has Interactable component
            if let Ok(interactable) = interactables.get(hit.entity) {
                // Always update detection if we hit an interactable (within max_ray_distance)
                // But mark if it's within interaction_distance
                current_interactable.entity = Some(hit.entity);
                current_interactable.distance = hit.distance;
                current_interactable.interaction_point = ray_origin + *ray_direction * hit.distance;
                current_interactable.is_in_range = hit.distance <= interactable.interaction_distance && interactable.can_interact;
            }
        }
    }
}

/// System to process interaction inputs
pub fn process_interactions(
    input: Res<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
    current_interactable: Res<CurrentInteractable>,
    mut events: ResMut<InteractionEventQueue>,
    mut pickup_events: ResMut<crate::pickups::PickupEventQueue>,
    mut interactables: Query<(&mut Interactable, Option<&mut InteractionData>, Option<&mut UsableDevice>)>,
    mut player_query: Query<(Entity, &mut UsingDevicesSystem), With<InteractionDetector>>,
    mut electronic_device_activation_queue: ResMut<crate::devices::electronic_device::ElectronicDeviceActivationEventQueue>,
    mut grab_queue: ResMut<crate::grab::GrabEventQueue>,
) {
    if !input.interact_pressed && !input_buffer.is_buffered(InputAction::Interact) {
        return;
    }

    // Determine target entity
    let mut target_entity = current_interactable.entity;
    let mut is_in_range = current_interactable.is_in_range;

    // Preference for UsingDevicesSystem
    let mut source_entity = Entity::PLACEHOLDER;
    if let Some((player_entity, player_system)) = player_query.iter_mut().next() {
        source_entity = player_entity;
        if player_system.current_device_index >= 0 {
            if let Some(device) = player_system.device_list.get(player_system.current_device_index as usize) {
                target_entity = Some(device.entity);
                is_in_range = true; // Devices in the list are already checked for range
            }
        }
    }

    if let Some(entity) = target_entity {
        if !is_in_range {
            return;
        }

        if let Ok((mut interactable, data_opt, device_opt)) = interactables.get_mut(entity) {
            if !interactable.can_interact {
                return;
            }

            // Consume input
            input_buffer.consume(InputAction::Interact);

            // Handle Cooldown
            if let Some(mut data) = data_opt {
                data.cooldown_timer = data.cooldown;
                interactable.can_interact = false; // Prevent immediate re-interaction
            }

            // Helper to print interaction
            info!("Interacted with {:?} - Type: {:?}", entity, interactable.interaction_type);

            // Handle Device Logic
            if let Some(mut device) = device_opt {
                device.is_active = !device.is_active;
                interactable.interaction_text = if device.is_active {
                    device.active_text.clone()
                } else {
                    device.inactive_text.clone()
                };
                info!("Device state toggled: {}", device.is_active);
            }
            
            // Trigger Event
            if source_entity != Entity::PLACEHOLDER {
                events.0.push(InteractionEvent {
                    source: source_entity,
                    target: entity,
                    interaction_type: interactable.interaction_type,
                });

                if interactable.interaction_type == InteractionType::Pickup {
                    pickup_events.0.push(crate::pickups::PickupEvent {
                        source: source_entity,
                        target: entity,
                    });
                }

                // Specifically trigger Electronic Device activation if applicable
                electronic_device_activation_queue.0.push(crate::devices::electronic_device::ElectronicDeviceActivationEvent {
                    device_entity: entity,
                    player_entity: source_entity,
                });

                // Specifically trigger Grab if applicable
                if interactable.interaction_type == InteractionType::Grab {
                    grab_queue.0.push(crate::grab::GrabEvent::Grab(source_entity, entity));
                }
            }
        }
    }
}

/// Debug system to visualize interaction rays
pub fn debug_draw_interaction_rays(
    debug_settings: Res<InteractionDebugSettings>,
    current_interactable: Res<CurrentInteractable>,
    detectors: Query<(&GlobalTransform, &InteractionDetector)>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.enabled {
        return;
    }

    for (transform, detector) in detectors.iter() {
        let ray_origin = transform.translation() + detector.ray_offset;
        let ray_direction = transform.forward();
        let ray_end = ray_origin + ray_direction * detector.max_distance;

        // Choose color based on whether we hit something
        let color = if current_interactable.entity.is_some() {
            debug_settings.hit_color
        } else {
            debug_settings.miss_color
        };

        // Draw the ray
        gizmos.line(ray_origin, ray_end, color);

        // Draw a sphere at the hit point if we have one
        if let Some(_entity) = current_interactable.entity {
            gizmos.sphere(
                current_interactable.interaction_point,
                0.1,
                debug_settings.hit_color,
            );
        }
    }
}

/// System to update the player's device list based on events
pub fn update_device_list(
    mut add_queue: ResMut<AddDeviceQueue>,
    mut remove_queue: ResMut<RemoveDeviceQueue>,
    mut players: Query<&mut UsingDevicesSystem>,
    devices: Query<(&DeviceStringAction, &GlobalTransform)>,
) {
    for event in add_queue.0.drain(..) {
        if let Ok(mut player_system) = players.get_mut(event.player) {
            if let Ok((device_action, _transform)) = devices.get(event.device) {
                // Check if already in list
                if !player_system.device_list.iter().any(|d| d.entity == event.device) {
                    player_system.device_list.push(DeviceInfo {
                        name: device_action.device_name.clone(),
                        entity: event.device,
                        action_offset: device_action.action_offset,
                        use_local_offset: device_action.use_local_offset,
                        use_custom_min_distance: device_action.use_custom_min_distance,
                        custom_min_distance: device_action.custom_min_distance,
                        use_custom_min_angle: device_action.use_custom_min_angle,
                        custom_min_angle: device_action.custom_min_angle,
                        use_relative_direction: device_action.use_relative_direction,
                        ignore_use_only_if_visible: device_action.ignore_use_only_if_visible,
                        check_if_obstacle: device_action.check_if_obstacle,
                    });
                }
            }
        }
    }

    for event in remove_queue.0.drain(..) {
        if let Ok(mut player_system) = players.get_mut(event.player) {
            player_system.device_list.retain(|d| d.entity != event.device);
        }
    }
}

/// System to select the closest device from the player's list
pub fn select_closest_device(
    mut players: Query<(Entity, &GlobalTransform, &mut UsingDevicesSystem)>,
    devices: Query<&GlobalTransform>,
) {
    for (player_entity, player_transform, mut player_system) in players.iter_mut() {
        if !player_system.can_use_devices || player_system.device_list.is_empty() {
            player_system.current_device_index = -1;
            continue;
        }

        let mut min_dist = f32::INFINITY;
        let mut best_index = -1;
        let player_pos = player_transform.translation();

        for (i, device_info) in player_system.device_list.iter().enumerate() {
            if let Ok(device_transform) = devices.get(device_info.entity) {
                let dist = player_pos.distance(device_transform.translation());
                
                // Check distance limits
                let max_dist = if device_info.use_custom_min_distance {
                    device_info.custom_min_distance
                } else {
                    player_system.min_distance_to_use_devices
                };

                if dist < max_dist && dist < min_dist {
                    min_dist = dist;
                    best_index = i as i32;
                }
            }
        }

        player_system.current_device_index = best_index;
    }
}

/// System to automatically add/remove devices based on proximity
pub fn detect_devices_in_proximity(
    player_query: Query<(Entity, &GlobalTransform, &UsingDevicesSystem)>,
    device_query: Query<(Entity, &GlobalTransform), With<DeviceStringAction>>,
    mut add_queue: ResMut<AddDeviceQueue>,
    mut remove_queue: ResMut<RemoveDeviceQueue>,
) {
    for (player_entity, player_transform, player_system) in player_query.iter() {
        let player_pos = player_transform.translation();
        
        for (device_entity, device_transform) in device_query.iter() {
            let dist = player_pos.distance(device_transform.translation());
            
            let is_in_list = player_system.device_list.iter().any(|d| d.entity == device_entity);
            
            if dist < player_system.raycast_distance {
                if !is_in_list {
                    add_queue.0.push(AddDeviceEvent {
                        player: player_entity,
                        device: device_entity,
                    });
                }
            } else if is_in_list {
                remove_queue.0.push(RemoveDeviceEvent {
                    player: player_entity,
                    device: device_entity,
                });
            }
        }
    }
}

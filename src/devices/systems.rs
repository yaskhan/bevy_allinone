use bevy::prelude::*;
use bevy::ui::{PositionType, Val, AlignSelf, JustifyContent, AlignItems, UiRect};
use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};
use crate::character::CharacterController;
use crate::camera::{CameraController, CameraMode};
use super::types::*;

/// System to setup the device UI
pub fn setup_device_ui(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 24.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(15.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            DevicePrompt,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Use Device"),
                text_style,
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
        });
}

/// System to update device UI
pub fn update_device_ui(
    device_ui_state: Res<DeviceUIState>,
    mut ui_query: Query<(&mut Visibility, &Children), With<DevicePrompt>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut visibility, children) in ui_query.iter_mut() {
        if device_ui_state.is_visible {
            *visibility = Visibility::Visible;

            for child in children.iter() {
                if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                    text_color.0 = Color::WHITE;
                    text.0 = format!("Press {} to {} {}",
                        device_ui_state.current_key_text,
                        device_ui_state.current_action_text,
                        device_ui_state.current_object_name
                    );
                }
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System to detect devices
pub fn detect_devices(
    _time: Res<Time>,
    spatial_query: SpatialQuery,
    mut device_list: ResMut<DeviceList>,
    mut using_devices_systems: Query<(
        &mut UsingDevicesSystem,
        &GlobalTransform,
        &CharacterController,
        &CameraController,
    )>,
    device_string_actions: Query<&DeviceStringAction>,
) {
    for (mut using_devices_system, transform, _character, camera) in using_devices_systems.iter_mut() {
        // Update screen dimensions
        using_devices_system.screen_width = 1920.0; // Placeholder
        using_devices_system.screen_height = 1080.0; // Placeholder

        // Update first person state
        using_devices_system.first_person_active = camera.mode == CameraMode::FirstPerson;

        // Clear device list if needed
        if !using_devices_system.device_list_contains_elements {
            device_list.devices.clear();
            device_list.device_entities.clear();
        }

        // Perform raycast if searching with raycast
        if using_devices_system.searching_devices_with_raycast {
            let ray_origin = transform.translation();
            let ray_direction = transform.forward();

            if let Some(hit) = spatial_query.cast_ray(
                ray_origin,
                ray_direction.into(),
                using_devices_system.raycast_distance,
                true,
                &SpatialQueryFilter::default(),
            ) {
                // Check if hit entity has DeviceStringAction
                if let Ok(device_string_action) = device_string_actions.get(hit.entity) {
                    // Check if device is in tags list
                    if using_devices_system.tags_for_devices.contains(&device_string_action.device_name) {
                        // Add device to list
                        add_device_to_list(
                            hit.entity,
                            &mut device_list,
                            &mut using_devices_system,
                            &device_string_action,
                        );
                    }
                }
            }
        }
    }
}

/// Helper function to add device to list
fn add_device_to_list(
    device_entity: Entity,
    device_list: &mut DeviceList,
    using_devices_system: &mut UsingDevicesSystem,
    device_string_action: &DeviceStringAction,
) {
    if device_list.device_entities.contains(&device_entity) {
        return;
    }

    let mut device_info = DeviceInfo::default();
    device_info.device_entity = Some(device_entity);
    device_info.device_transform = None; // Will be set by transform query
    device_info.position_to_icon = None;
    device_info.use_transform_for_string_action = device_string_action.use_transform_for_string_action;
    device_info.use_separated_transform_for_every_view = device_string_action.use_separated_transform_for_every_view;
    device_info.action_offset = device_string_action.action_offset;
    device_info.use_local_offset = device_string_action.use_local_offset;
    device_info.use_custom_min_distance_to_use_device = device_string_action.use_custom_min_distance_to_use_device;
    device_info.custom_min_distance_to_use_device = device_string_action.custom_min_distance_to_use_device;
    device_info.use_custom_min_angle_to_use = device_string_action.use_custom_min_angle_to_use;
    device_info.custom_min_angle_to_use_device = device_string_action.custom_min_angle_to_use_device;
    device_info.use_relative_direction_between_player_and_object = device_string_action.use_relative_direction_between_player_and_object;
    device_info.ignore_use_only_device_if_visible_on_camera = device_string_action.ignore_use_only_device_if_visible_on_camera;
    device_info.use_custom_device_transform_position = device_string_action.use_custom_device_transform_position;
    device_info.custom_device_transform_position = device_string_action.custom_device_transform_position;
    device_info.use_fixed_device_icon_position = device_string_action.use_fixed_device_icon_position;
    device_info.check_if_obstacle_between_device_and_player = device_string_action.check_if_obstacle_between_device_and_player;

    device_list.devices.push(device_info);
    device_list.device_entities.push(device_entity);
    using_devices_system.device_list_contains_elements = true;
    using_devices_system.device_list_count = device_list.devices.len() as i32;
}

/// System to process device interaction
pub fn process_device_interaction(
    input: Res<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
    mut using_devices_systems: Query<&mut UsingDevicesSystem>,
    mut electronic_devices: Query<&mut ElectronicDevice>,
    mut device_string_actions: Query<&mut DeviceStringAction>,
) {
    if !input.interact_pressed && !input_buffer.is_buffered(InputAction::Interact) {
        return;
    }

    for mut using_devices_system in using_devices_systems.iter_mut() {
        if !using_devices_system.can_use_devices || using_devices_system.driving || using_devices_system.examining_object {
            continue;
        }

        if !using_devices_system.use_device_button_enabled {
            continue;
        }

        if let Some(object_to_use) = using_devices_system.object_to_use {
            // Consume input
            input_buffer.consume(InputAction::Interact);

            // Handle pickup logic
            if using_devices_system.current_device_is_pickup && using_devices_system.hold_button_to_take_pickups_around {
                using_devices_system.holding_button = true;
                using_devices_system.last_time_pressed_button = 0.0; // Will be set by time resource
            } else {
                // Use device
                use_device(object_to_use, &mut using_devices_system, &mut electronic_devices, &mut device_string_actions);
            }
        }
    }
}

/// Helper function to use device
fn use_device(
    device_entity: Entity,
    using_devices_system: &mut UsingDevicesSystem,
    electronic_devices: &mut Query<&mut ElectronicDevice>,
    device_string_actions: &mut Query<&mut DeviceStringAction>,
) {
    if let Ok(mut electronic_device) = electronic_devices.get_mut(device_entity) {
        // Toggle device state
        electronic_device.using_device = !electronic_device.using_device;

        // Update device string action
        if let Ok(mut device_string_action) = device_string_actions.get_mut(device_entity) {
            device_string_action.using_device = electronic_device.using_device;

            if device_string_action.set_using_device_state {
                device_string_action.check_set_using_device_state(electronic_device.using_device);
            }

            if device_string_action.hide_icon_on_use_device {
                using_devices_system.icon_button_can_be_shown = false;
            } else if device_string_action.show_icon_on_stop_use_device && !electronic_device.using_device {
                using_devices_system.icon_button_can_be_shown = true;
            }
        }

        info!("Device {} toggled: {}", device_entity.index(), electronic_device.using_device);
    }
}

/// System to update device icons
pub fn update_device_icons(
    _time: Res<Time>,
    mut using_devices_systems: Query<(&mut UsingDevicesSystem, &GlobalTransform)>,
    device_string_actions: Query<&DeviceStringAction>,
    device_list: Res<DeviceList>,
) {
    for (mut using_devices_system, transform) in using_devices_systems.iter_mut() {
        if !using_devices_system.device_list_contains_elements || !using_devices_system.show_use_device_icon_enabled {
            continue;
        }

        if using_devices_system.examining_object || using_devices_system.driving {
            continue;
        }

        // Get closest device
        let closest_device_index = get_closest_device(&using_devices_system, &device_list, transform);

        if closest_device_index != -1 {
            using_devices_system.current_target_index = closest_device_index;

            if let Some(device_info) = device_list.devices.get(closest_device_index as usize) {
                if let Some(device_entity) = device_info.device_entity {
                    using_devices_system.object_to_use = Some(device_entity);

                    // Update UI state
                    if let Ok(device_string_action) = device_string_actions.get(device_entity) {
                        using_devices_system.current_device_action_text = device_string_action.device_action.clone();
                        using_devices_system.current_device_action_name = device_string_action.device_name.clone();
                        using_devices_system.current_device_name_text_font_size = device_string_action.text_font_size;
                    }
                }
            }
        } else {
            using_devices_system.object_to_use = None;
            using_devices_system.current_target_index = -1;
        }
    }
}

/// Helper function to get closest device
fn get_closest_device(
    using_devices_system: &UsingDevicesSystem,
    device_list: &DeviceList,
    transform: &GlobalTransform,
) -> i32 {
    if device_list.devices.is_empty() {
        return -1;
    }

    let mut current_target_index = -1;
    let mut min_distance_to_target = f32::INFINITY;
    let current_position = transform.translation();

    for (i, device_info) in device_list.devices.iter().enumerate() {
        if let Some(device_transform) = device_info.device_transform {
            let device_position = device_transform.translation();

            let distance = current_position.distance(device_position);

            if distance < min_distance_to_target {
                min_distance_to_target = distance;
                current_target_index = i as i32;
            }
        }
    }

    // Check min distance
    if using_devices_system.use_min_distance_to_use_devices {
        if let Some(device_info) = device_list.devices.get(current_target_index as usize) {
            let mut use_custom_min_distance = device_info.use_custom_min_distance_to_use_device;
            let mut custom_min_distance = device_info.custom_min_distance_to_use_device;

            if !use_custom_min_distance {
                use_custom_min_distance = using_devices_system.use_custom_min_distance_to_use_device;
                custom_min_distance = using_devices_system.custom_min_distance_to_use_device;
            }

            let min_distance = if use_custom_min_distance {
                custom_min_distance
            } else {
                using_devices_system.min_distance_to_use_devices
            };

            if min_distance_to_target > min_distance {
                return -1;
            }
        }
    }

    current_target_index
}

/// Debug system to draw device info
pub fn debug_draw_device_info(
    debug_settings: Res<DeviceDebugSettings>,
    device_list: Res<DeviceList>,
    using_devices_systems: Query<&UsingDevicesSystem>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.enabled || !debug_settings.show_device_list {
        return;
    }

    for using_devices_system in using_devices_systems.iter() {
        if !using_devices_system.device_list_contains_elements {
            continue;
        }

        for (i, device_info) in device_list.devices.iter().enumerate() {
            if let Some(device_transform) = device_info.device_transform {
                let color = if i as i32 == using_devices_system.current_target_index {
                    debug_settings.closest_device_color
                } else {
                    debug_settings.device_color
                };

                gizmos.sphere(
                    device_transform.translation(),
                    0.2,
                    color,
                );

                if debug_settings.show_device_info {
                    // Note: text_2d is not available in Bevy 0.18
                    // Use gizmos.line or other debug methods instead
                    gizmos.sphere(
                        device_transform.translation(),
                        0.1,
                        color,
                    );
                }
            }
        }
    }
}

//! Interaction system module
//!
//! Object interaction, pickups, and usable devices.

use bevy::prelude::*;

use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InteractionEventQueue>()
            .init_resource::<CurrentInteractable>()
            .init_resource::<InteractionDebugSettings>()
            .add_event::<AddDeviceEvent>()
            .add_event::<RemoveDeviceEvent>()
            .add_systems(Update, (
                detect_interactables,
                detect_devices_in_proximity,
                update_device_list,
                select_closest_device,
                validate_interactions,
                process_interactions,
                update_interaction_ui,
                debug_draw_interaction_rays,
            ).chain())
            .add_systems(Startup, setup_interaction_ui);
    }
}

/// Event to add a device to the player's list
#[derive(Event)]
pub struct AddDeviceEvent {
    pub player: Entity,
    pub device: Entity,
}

/// Event to remove a device from the player's list
#[derive(Event)]
pub struct RemoveDeviceEvent {
    pub player: Entity,
    pub device: Entity,
}

/// Component for entities that can detect and interact with objects
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionDetector {
    /// Maximum distance for interaction detection
    pub max_distance: f32,
    /// Ray offset from entity position (usually forward from camera/eyes)
    pub ray_offset: Vec3,
    /// How often to update detection (in seconds, 0 = every frame)
    pub update_interval: f32,
    /// Time since last update
    pub time_since_update: f32,
    /// Layer mask for raycasting
    pub interaction_layers: u32,
}

impl Default for InteractionDetector {
    fn default() -> Self {
        Self {
            max_distance: 3.0,
            ray_offset: Vec3::ZERO,
            update_interval: 0.1, // Update 10 times per second
            time_since_update: 0.0,
            interaction_layers: 0xFFFFFFFF, // All layers by default
        }
    }
}

/// Resource tracking the currently detected interactable
#[derive(Resource, Debug, Default)]
pub struct CurrentInteractable {
    pub entity: Option<Entity>,
    pub distance: f32,
    pub interaction_point: Vec3,
    pub is_in_range: bool,
}

/// Settings for debug visualization
#[derive(Resource, Debug)]
pub struct InteractionDebugSettings {
    pub enabled: bool,
    pub ray_color: Color,
    pub hit_color: Color,
    pub miss_color: Color,
}

impl Default for InteractionDebugSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            ray_color: Color::srgb(0.0, 1.0, 0.0),
            hit_color: Color::srgb(1.0, 0.5, 0.0),
            miss_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

/// Interactable component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Interactable {
    pub interaction_text: String,
    pub interaction_distance: f32,
    pub can_interact: bool,
    pub interaction_type: InteractionType,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            interaction_text: "Interact".to_string(),
            interaction_distance: 3.0,
            can_interact: true,
            interaction_type: InteractionType::Use,
        }
    }
}

/// Interaction type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum InteractionType {
    Pickup,
    Use,
    Talk,
    Open,
    Activate,
    Examine,
    Toggle,
    Grab,
    Device,
}

/// Information about a detected device, matching the original project structure.
#[derive(Debug, Clone, Reflect)]
pub struct DeviceInfo {
    pub name: String,
    pub entity: Entity,
    pub action_offset: f32,
    pub use_local_offset: bool,
    pub use_custom_min_distance: bool,
    pub custom_min_distance: f32,
    pub use_custom_min_angle: bool,
    pub custom_min_angle: f32,
    pub use_relative_direction: bool,
    pub ignore_use_only_if_visible: bool,
    pub check_if_obstacle: bool,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            entity: Entity::PLACEHOLDER,
            action_offset: 1.0,
            use_local_offset: true,
            use_custom_min_distance: false,
            custom_min_distance: 0.0,
            use_custom_min_angle: false,
            custom_min_angle: 0.0,
            use_relative_direction: false,
            ignore_use_only_if_visible: false,
            check_if_obstacle: false,
        }
    }
}

/// Component for the player to manage nearby devices.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UsingDevicesSystem {
    pub can_use_devices: bool,
    pub device_list: Vec<DeviceInfo>,
    pub current_device_index: i32,
    pub use_device_action_name: String,
    pub raycast_distance: f32,
    pub layer_mask: u32,
    pub searching_devices_with_raycast: bool,
    pub show_use_device_icon_enabled: bool,
    pub use_min_distance_to_use_devices: bool,
    pub min_distance_to_use_devices: f32,
    pub use_only_device_if_visible_on_camera: bool,
    pub driving: bool,
}

impl Default for UsingDevicesSystem {
    fn default() -> Self {
        Self {
            can_use_devices: true,
            device_list: Vec::new(),
            current_device_index: -1,
            use_device_action_name: "Activate Device".to_string(),
            raycast_distance: 5.0,
            layer_mask: 0xFFFFFFFF,
            searching_devices_with_raycast: false,
            show_use_device_icon_enabled: true,
            use_min_distance_to_use_devices: true,
            min_distance_to_use_devices: 4.0,
            use_only_device_if_visible_on_camera: false,
            driving: false,
        }
    }
}

/// Component for metadata about device interaction, matching the original project's architecture.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeviceStringAction {
    pub device_name: String,
    pub device_action: String,
    pub secondary_device_action: String,
    pub show_icon: bool,
    pub action_offset: f32,
    pub use_local_offset: bool,
    pub use_custom_min_distance: bool,
    pub custom_min_distance: f32,
    pub use_custom_min_angle: bool,
    pub custom_min_angle: f32,
    pub use_relative_direction: bool,
    pub ignore_use_only_if_visible: bool,
    pub check_if_obstacle: bool,
    pub icon_enabled: bool,
}

impl Default for DeviceStringAction {
    fn default() -> Self {
        Self {
            device_name: "Device".to_string(),
            device_action: "Activate".to_string(),
            secondary_device_action: String::new(),
            show_icon: true,
            action_offset: 1.0,
            use_local_offset: true,
            use_custom_min_distance: false,
            custom_min_distance: 0.0,
            use_custom_min_angle: false,
            custom_min_angle: 0.0,
            use_relative_direction: false,
            ignore_use_only_if_visible: false,
            check_if_obstacle: true,
            icon_enabled: true,
        }
    }
}

/// Component for the interaction UI prompt text
#[derive(Component)]
pub struct InteractionPrompt;

/// Resource to manage interaction UI state
#[derive(Resource, Default)]
pub struct InteractionUIState {
    pub is_visible: bool,
    pub current_text: String,
}

/// System to setup the interaction UI
fn setup_interaction_ui(mut commands: Commands) {
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
fn update_interaction_ui(
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
        if let Ok(player_system) = player_query.get_single() {
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

/// Data specific to the interaction
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionData {
    /// Duration for the interaction (0.0 for instant)
    pub duration: f32,
    /// Cooldown after interaction
    pub cooldown: f32,
    /// Current cooldown timer
    pub cooldown_timer: f32,
    /// Whether the interaction triggers automatically when in range
    pub auto_trigger: bool,
    /// Custom data string (e.g., item ID, door key, dialogue ID)
    pub data: String,
}

impl Default for InteractionData {
    fn default() -> Self {
        Self {
            duration: 0.0,
            cooldown: 0.5,
            cooldown_timer: 0.0,
            auto_trigger: false,
            data: String::new(),
        }
    }
}

/// Component for usable devices (doors, switches, etc.)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UsableDevice {
    pub is_active: bool,
    pub requires_key: bool,
    pub key_id: String,
    pub active_text: String,
    pub inactive_text: String,
}

impl Default for UsableDevice {
    fn default() -> Self {
        Self {
            is_active: false,
            requires_key: false,
            key_id: String::new(),
            active_text: "Turn Off".to_string(),
            inactive_text: "Turn On".to_string(),
        }
    }
}

/// Event triggered when a valid interaction occurs
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

/// Custom queue for interaction events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct InteractionEventQueue(pub Vec<InteractionEvent>);

/// System to validate interactions (cooldowns, states)
fn validate_interactions(
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
fn detect_interactables(
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
fn process_interactions(
    input: Res<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
    current_interactable: Res<CurrentInteractable>,
    mut events: ResMut<InteractionEventQueue>,
    mut interactables: Query<(&mut Interactable, Option<&mut InteractionData>, Option<&mut UsableDevice>)>,
    mut player_query: Query<(Entity, &mut UsingDevicesSystem), With<InteractionDetector>>,
    mut electronic_device_activation_queue: ResMut<crate::devices::electronic_device::ElectronicDeviceActivationEventQueue>,
) {
    if !input.interact_pressed && !input_buffer.is_buffered(InputAction::Interact) {
        return;
    }

    // Determine target entity
    let mut target_entity = current_interactable.entity;
    let mut is_in_range = current_interactable.is_in_range;

    // Preference for UsingDevicesSystem
    let mut source_entity = Entity::PLACEHOLDER;
    if let Ok((player_entity, player_system)) = player_query.get_single_mut() {
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

                // Specifically trigger Electronic Device activation if applicable
                electronic_device_activation_queue.0.push(crate::devices::electronic_device::ElectronicDeviceActivationEvent {
                    device_entity: entity,
                    player_entity: source_entity,
                });
            }
        }
    }
}

/// Debug system to visualize interaction rays
fn debug_draw_interaction_rays(
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
    mut add_events: EventReader<AddDeviceEvent>,
    mut remove_events: EventReader<RemoveDeviceEvent>,
    mut players: Query<&mut UsingDevicesSystem>,
    devices: Query<(&DeviceStringAction, &GlobalTransform)>,
) {
    for event in add_events.read() {
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

    for event in remove_events.read() {
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
    mut add_events: EventWriter<AddDeviceEvent>,
    mut remove_events: EventWriter<RemoveDeviceEvent>,
) {
    for (player_entity, player_transform, player_system) in player_query.iter() {
        let player_pos = player_transform.translation();
        
        for (device_entity, device_transform) in device_query.iter() {
            let dist = player_pos.distance(device_transform.translation());
            
            let is_in_list = player_system.device_list.iter().any(|d| d.entity == device_entity);
            
            if dist < player_system.raycast_distance {
                if !is_in_list {
                    add_events.send(AddDeviceEvent {
                        player: player_entity,
                        device: device_entity,
                    });
                }
            } else if is_in_list {
                remove_events.send(RemoveDeviceEvent {
                    player: player_entity,
                    device: device_entity,
                });
            }
        }
    }
}

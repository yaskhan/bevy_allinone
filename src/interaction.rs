//! Interaction system module
//!
//! Object interaction, pickups, and usable devices.

use bevy::prelude::*;
use bevy::ecs::event::Event;
use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<InteractionEvent>()
            .init_resource::<CurrentInteractable>()
            .init_resource::<InteractionDebugSettings>()
            .add_systems(Update, (
                detect_interactables,
                validate_interactions,
                process_interactions,
                update_interaction_ui,
                debug_draw_interaction_rays,
            ).chain())
            .add_systems(Startup, setup_interaction_ui);
    }
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
    mut ui_query: Query<(&mut Visibility, &Children), With<InteractionPrompt>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut visibility, children) in ui_query.iter_mut() {
        if let Some(entity) = current_interactable.entity {
            if let Ok(interactable) = interactables.get(entity) {
                *visibility = Visibility::Visible;
                
                // Update text
                for child in children.iter() {
                    if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                        // In real implementation, get keybinding from InputMap
                        let key_text = "E"; 

                        let (color, suffix) = if current_interactable.is_in_range {
                            (Color::WHITE, "")
                        } else {
                            (Color::srgb(1.0, 0.2, 0.2), " (Too Far)")
                        };

                        text_color.0 = color;
                        
                        text.0 = format!("Press {} to {} {}{}", 
                            key_text, 
                            match interactable.interaction_type {
                                InteractionType::Pickup => "pick up",
                                InteractionType::Use => "use",
                                InteractionType::Talk => "talk to",
                                InteractionType::Open => "open",
                                InteractionType::Activate => "activate",
                                InteractionType::Examine => "examine",
                                InteractionType::Toggle => "toggle",
                                InteractionType::Grab => "grab",
                            },
                            interactable.interaction_text,
                            suffix
                        );
                    }
                }
            } else {
                *visibility = Visibility::Hidden;
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
#[derive(Event, Debug, Clone)]
pub struct InteractionEvent {
    pub source: Entity,
    pub target: Entity,
    pub interaction_type: InteractionType,
}

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
    mut interactables: Query<(&mut Interactable, Option<&mut InteractionData>, Option<&mut UsableDevice>)>,
) {
    if !input.interact_pressed && !input_buffer.is_buffered(InputAction::Interact) {
        return;
    }

    if let Some(entity) = current_interactable.entity {
        if !current_interactable.is_in_range {
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
            
            // In a real implementation, we would send an event here, 
            // but we are skipping it for now due to API issues.
            // commands.trigger(InteractionEvent { ... });
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

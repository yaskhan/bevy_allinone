//! Interaction system module
//!
//! Object interaction, pickups, and usable devices.

use bevy::prelude::*;
use avian3d::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentInteractable>()
            .init_resource::<InteractionDebugSettings>()
            .add_systems(Update, (
                detect_interactables,
                process_interactions,
                debug_draw_interaction_rays,
            ).chain());
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
                // Check if within interaction distance
                if hit.distance <= interactable.interaction_distance && interactable.can_interact {
                    // Update current interactable
                    current_interactable.entity = Some(hit.entity);
                    current_interactable.distance = hit.distance;
                    current_interactable.interaction_point = ray_origin + *ray_direction * hit.distance;
                }
            }
        }
    }
}

/// System to process interaction inputs
fn process_interactions(
    // TODO: Add input handling
    // TODO: Trigger interaction events
) {
    // Will be implemented in later steps
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

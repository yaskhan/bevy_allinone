use bevy::prelude::*;
use avian3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            apply_custom_gravity,
            detect_ground,
            handle_slopes,
        ).chain());
    }
}

/// Custom Gravity component
///
/// Allows for entities to have their own gravity force and direction.
/// Useful for wall-walking, planetary gravity, or zero-g sectors.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CustomGravity {
    /// The gravity acceleration vector. If None, uses global gravity.
    pub gravity: Option<Vec3>,
    /// Multiplier for the gravity force.
    pub multiplier: f32,
}

impl Default for CustomGravity {
    fn default() -> Self {
        Self {
            gravity: None,
            multiplier: 1.0,
        }
    }
}

/// Ground detection component
///
/// Tracks whether the entity is touching the ground, the distance to it,
/// and the normal of the surface.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct GroundDetection {
    pub is_grounded: bool,
    pub ground_normal: Vec3,
    pub ground_distance: f32,
    pub ground_angle: f32,
    pub entity_below: Option<Entity>,
}

/// Settings for ground detection
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GroundDetectionSettings {
    pub ray_radius: f32,
    pub ray_length: f32,
    pub max_slope_angle: f32,
    pub collision_mask: LayerMask,
}

impl Default for GroundDetectionSettings {
    fn default() -> Self {
        Self {
            ray_radius: 0.2,
            ray_length: 0.1,
            max_slope_angle: 45.0,
            collision_mask: LayerMask::ALL,
        }
    }
}

/// System to apply custom gravity to entities with an Avian3D Rigidbody.
fn apply_custom_gravity(
    mut query: Query<(&CustomGravity, &mut ExternalForce)>,
    global_gravity: Res<avian3d::prelude::Gravity>,
) {
    for (custom, mut force) in query.iter_mut() {
        let g = custom.gravity.unwrap_or(global_gravity.0);
        force.apply_force(g * custom.multiplier);
    }
}

/// System to detect ground using Avian3D's SpatialQuery.
fn detect_ground(
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &GlobalTransform, &GroundDetectionSettings, &mut GroundDetection)>,
) {
    for (entity, transform, settings, mut detection) in query.iter_mut() {
        let ray_pos = transform.translation();
        let ray_dir = Dir3::NEG_Y;

        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        
        if let Some(hit) = spatial_query.cast_ray(
            ray_pos,
            ray_dir,
            settings.ray_length + 0.5,
            true,
            filter,
        ) {
            detection.is_grounded = hit.distance <= settings.ray_length + 0.05;
            detection.ground_normal = hit.normal;
            detection.ground_distance = hit.distance;
            detection.ground_angle = hit.normal.angle_between(Vec3::Y).to_degrees();
            detection.entity_below = Some(hit.entity);
        } else {
            detection.is_grounded = false;
            detection.ground_normal = Vec3::Y;
            detection.ground_distance = f32::MAX;
            detection.ground_angle = 0.0;
            detection.entity_below = None;
        }
    }
}

/// Handles slope calculations based on ground normal.
fn handle_slopes(
    mut query: Query<(&GroundDetection, &GroundDetectionSettings)>,
) {
    for (detection, settings) in query.iter_mut() {
        if detection.is_grounded {
            let angle = detection.ground_normal.angle_between(Vec3::Y).to_degrees();
            if angle > settings.max_slope_angle {
                // TODO: Inform character controller to slip or prevent movement
            }
        }
    }
}

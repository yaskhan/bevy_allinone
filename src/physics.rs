use bevy::prelude::*;
use avian3d::prelude::*;
// use avian3d::external_force::ExternalForce;

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
    pub last_is_grounded: bool,
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
    pub max_step_height: f32,
    pub step_check_distance: f32,
    pub collision_mask: LayerMask,
}

impl Default for GroundDetectionSettings {
    fn default() -> Self {
        Self {
            ray_radius: 0.2,
            ray_length: 0.1,
            max_slope_angle: 45.0,
            max_step_height: 0.3,
            step_check_distance: 0.2,
            collision_mask: LayerMask::ALL,
        }
    }
}

/// System to apply custom gravity to entities with an Avian3D Rigidbody.
fn apply_custom_gravity(
    time: Res<Time>,
    mut query: Query<(&CustomGravity, &mut LinearVelocity)>,
    global_gravity: Res<avian3d::prelude::Gravity>, // Keep global gravity for Option::unwrap_or
) {
    for (custom, mut velocity) in query.iter_mut() {
        // Apply gravity acceleration: v += g * dt
        // custom.gravity is Vec3 (acceleration) or Force? name implies gravity vector (acceleration typically)
        // Standard gravity is 9.81.
        // Assuming CustomGravity holds the acceleration vector itself or direction?
        // Let's assume it's acceleration or force/mass.
        let g = custom.gravity.unwrap_or(global_gravity.0); // Use unwrap_or to handle Option<Vec3>
        
        let accel = g * custom.multiplier;
        velocity.0 += accel * time.delta_secs();
    }
}

/// System to detect ground using Avian3D's SpatialQuery.
fn detect_ground(
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &GlobalTransform, &GroundDetectionSettings, &mut GroundDetection)>,
) {
    for (entity, transform, settings, mut detection) in query.iter_mut() {
        detection.last_is_grounded = detection.is_grounded;
        
        let ray_pos = transform.translation();
        let ray_dir = Dir3::NEG_Y;

        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        
        if let Some(hit) = spatial_query.cast_ray(
            ray_pos,
            ray_dir,
            settings.ray_length + settings.max_step_height + 0.1, // Check deeper for snapping
            true,
            &filter,
        ) {
            let threshold = settings.ray_length + 0.05;
            detection.is_grounded = hit.distance <= threshold;
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

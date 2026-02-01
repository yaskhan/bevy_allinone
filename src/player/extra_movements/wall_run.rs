//! Wall Run System
//!
//! Manages wall running mechanics including detection and physics overrides.

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use crate::physics::GroundDetection;

pub struct WallRunPlugin;

impl Plugin for WallRunPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WallRun>()
            .add_systems(Update, (
                handle_wall_run_check,
                update_wall_run_physics,
            ).chain());
    }
}

/// Wall side detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum WallSide {
    Left,
    Right,
}

/// Component to configure and manage wall running state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WallRun {
    pub active: bool,
    pub wall_run_speed: f32,
    pub wall_sprint_speed: f32,
    pub detection_distance: f32,
    pub min_height_above_ground: f32,
    pub min_vertical_speed: f32,
    pub wall_stick_force: f32,
    pub max_wall_angle: f32,

    // State
    pub wall_side: Option<WallSide>,
    pub last_active_time: f32,
    pub wall_run_timer: f32,
    pub max_wall_run_duration: f32,

    // Physics
    pub last_wall_normal: Vec3,
    pub last_wall_entity: Option<Entity>,
    pub current_wall_entity: Option<Entity>,
    pub wall_hit_position: Vec3,
}

impl Default for WallRun {
    fn default() -> Self {
        Self {
            active: false,
            wall_run_speed: 8.0,
            wall_sprint_speed: 12.0,
            detection_distance: 1.2,
            min_height_above_ground: 0.5,
            min_vertical_speed: -10.0,
            wall_stick_force: 5.0,
            max_wall_angle: 85.0,
            wall_side: None,
            last_active_time: 0.0,
            wall_run_timer: 0.0,
            max_wall_run_duration: 3.0,
            last_wall_normal: Vec3::Z,
            last_wall_entity: None,
            current_wall_entity: None,
            wall_hit_position: Vec3::ZERO,
        }
    }
}

/// System to detect walls and enable/disable wall running using physics raycasting
pub fn handle_wall_run_check(
    mut query: Query<(Entity, &mut WallRun, &GlobalTransform, &GroundDetection, &LinearVelocity)>,
    time: Res<Time>,
    input_state: Res<InputState>,
    spatial_query: SpatialQuery,
) {
    for (entity, mut wall_run, global_transform, ground, velocity) in query.iter_mut() {
        wall_run.last_active_time += time.delta_secs();

        // Check preconditions for wall running:
        // 1. Must be in air (not grounded)
        // 2. Must be moving forward with sufficient intent
        // 3. Must meet minimum height above ground
        // 4. Vertical velocity must be above minimum threshold (not falling too fast)
        let can_start_wall_run = !ground.is_grounded
            && input_state.movement.y > 0.1
            && ground.ground_distance >= wall_run.min_height_above_ground
            && velocity.y >= wall_run.min_vertical_speed;

        if !can_start_wall_run && !wall_run.active {
            wall_run.wall_side = None;
            continue;
        }

        // If already wall running, check if we should continue
        if wall_run.active {
            wall_run.wall_run_timer += time.delta_secs();

            // End wall run if timer exceeded or no longer pressing forward
            if wall_run.wall_run_timer >= wall_run.max_wall_run_duration
                || input_state.movement.y <= 0.0 {
                wall_run.active = false;
                wall_run.last_wall_entity = wall_run.current_wall_entity;
                wall_run.current_wall_entity = None;
                wall_run.wall_side = None;
                continue;
            }
        }

        let ray_origin = global_transform.translation() + Vec3::Y * 0.5;
        let right_dir = global_transform.right();
        let left_dir = -right_dir;
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);

        // Cast rays to detect walls on both sides
        let hit_right = spatial_query.cast_ray(
            ray_origin,
            Dir3::new(*right_dir).unwrap_or(Dir3::X),
            wall_run.detection_distance,
            true,
            &filter,
        );

        let hit_left = spatial_query.cast_ray(
            ray_origin,
            Dir3::new(*left_dir).unwrap_or(Dir3::NEG_X),
            wall_run.detection_distance,
            true,
            &filter,
        );

        // Process wall hit
        let wall_hit = match (hit_right, hit_left) {
            (Some(right_hit), Some(left_hit)) => {
                // Both sides have walls, pick the closer one
                if right_hit.distance < left_hit.distance {
                    Some((WallSide::Right, right_hit))
                } else {
                    Some((WallSide::Left, left_hit))
                }
            }
            (Some(right_hit), None) => Some((WallSide::Right, right_hit)),
            (None, Some(left_hit)) => Some((WallSide::Left, left_hit)),
            (None, None) => None,
        };

        if let Some((side, hit)) = wall_hit {
            // Validate wall angle (must be roughly vertical)
            let wall_angle = hit.normal.angle_between(Vec3::Y).to_degrees();
            if wall_angle > wall_run.max_wall_angle {
                // Wall is too sloped, can't run on it
                if wall_run.active {
                    wall_run.active = false;
                    wall_run.wall_side = None;
                }
                continue;
            }

            // Check if this is a different wall than last time (chain wall runs)
            let is_new_wall = wall_run.last_wall_entity.map_or(true, |e| e != hit.entity);

            if !wall_run.active && can_start_wall_run && is_new_wall {
                // Start wall run
                wall_run.active = true;
                wall_run.wall_run_timer = 0.0;
                wall_run.last_wall_entity = None;
            }

            if wall_run.active {
                wall_run.wall_side = Some(side);
                wall_run.current_wall_entity = Some(hit.entity);
                wall_run.last_wall_normal = hit.normal;
                wall_run.wall_hit_position = ray_origin + match side {
                    WallSide::Right => *right_dir * hit.distance,
                    WallSide::Left => *left_dir * hit.distance,
                };
            }
        } else {
            // No wall detected
            if wall_run.active {
                wall_run.active = false;
                wall_run.last_wall_entity = wall_run.current_wall_entity;
                wall_run.current_wall_entity = None;
                wall_run.wall_side = None;
            }
        }
    }
}

/// System to apply physics for wall running (override gravity, stick to wall)
pub fn update_wall_run_physics(
    mut query: Query<(&mut WallRun, &GlobalTransform, &mut LinearVelocity, &mut AngularVelocity)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (wall_run, global_transform, mut velocity, mut angular_vel) in query.iter_mut() {
        if !wall_run.active {
            continue;
        }

        let forward = global_transform.forward();
        let up = global_transform.up();
        let wall_normal = wall_run.last_wall_normal;

        // Project forward direction onto wall plane
        // v_parallel = v - (v . n) * n
        let forward_proj = *forward - forward.dot(wall_normal) * wall_normal;
        let forward_proj = forward_proj.normalize_or_zero();

        if forward_proj.length_squared() < 0.001 {
            // Forward is parallel to wall normal, can't run
            continue;
        }

        // Determine speed based on sprint input
        let speed = if input_state.sprint_pressed {
            wall_run.wall_sprint_speed
        } else {
            wall_run.wall_run_speed
        };

        // Calculate target velocity along wall
        let target_vel = forward_proj * speed;

        // Blend current horizontal velocity with target
        velocity.x = velocity.x.lerp(target_vel.x, dt * 5.0);
        velocity.z = velocity.z.lerp(target_vel.z, dt * 5.0);

        // Counteract gravity (maintain or slightly reduce vertical velocity)
        // This gives the "wall running" effect where you don't fall
        velocity.y = velocity.y.lerp(0.0, dt * 3.0);

        // Apply "stick" force towards wall to prevent drifting away
        let stick_dir = -wall_normal;
        velocity.0 += stick_dir * wall_run.wall_stick_force * dt;

        // Optional: Apply slight rotation to align with wall
        let target_right = wall_normal.cross(Vec3::Y).normalize_or_zero();
        if target_right.length_squared() > 0.001 {
            let current_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
            let angle_diff = current_forward.angle_between(Vec3::new(forward_proj.x, 0.0, forward_proj.z));

            if angle_diff > 0.01 {
                let rot_dir = if wall_run.wall_side == Some(WallSide::Right) { -1.0 } else { 1.0 };
                angular_vel.y += rot_dir * angle_diff * 2.0 * dt;
            }
        }
    }
}

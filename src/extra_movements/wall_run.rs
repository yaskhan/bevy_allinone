//! Wall Run System
//!
//! Manages wall running mechanics including detection and physics overrides.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

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

/// Component to configure and manage wall running state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WallRun {
    pub active: bool,
    pub wall_run_speed: f32,
    pub wall_sprint_speed: f32,
    pub detection_distance: f32,
    pub min_height: f32, // Minimum height above ground to start
    
    // State
    pub is_running_right: bool,
    pub is_running_left: bool,
    pub last_active_time: f32,
    
    // Physics
    pub last_wall_normal: Vec3,
}

impl Default for WallRun {
    fn default() -> Self {
        Self {
            active: false,
            wall_run_speed: 8.0,
            wall_sprint_speed: 12.0,
            detection_distance: 1.0,
            min_height: 1.5,
            is_running_right: false,
            is_running_left: false,
            last_active_time: 0.0,
            last_wall_normal: Vec3::Z,
        }
    }
}

/// System to detect walls and enable/disable wall running
pub fn handle_wall_run_check(
    mut query: Query<(&mut WallRun, &GlobalTransform)>,
    // physics_query: Query<&RapierContext>, // Start of physics integration
    time: Res<Time>,
    input_state: Res<InputState>,
) {
    for (mut wall_run, global_transform) in query.iter_mut() {
        // Only check if moving forward and in air (simplified condition)
        if input_state.move_direction.z <= 0.0 { // Assuming Z is forward in local space
             // Not moving forward, cannot wall run
             if wall_run.active {
                 wall_run.active = false;
                 wall_run.is_running_right = false;
                 wall_run.is_running_left = false;
             }
             continue;
        }

        let right = global_transform.right();
        let left = -right;
        let origin = global_transform.translation(); // Adjust for height usually
        
        let mut found_wall = false;
        
        // Placeholder for raycasting (would use physics context)
        // Check Right
        // if let Some((entity, toi)) = physics.cast_ray(origin, right, wall_run.detection_distance, true, QueryFilter::default()) {
        //      wall_run.active = true;
        //      wall_run.is_running_right = true;
        //      wall_run.is_running_left = false;
        //      wall_run.last_wall_normal = ...;
        //      found_wall = true;
        // }
        // Check Left
        // ...

        // Simulation for testing without physics engine hookup yet
        // Assume active if active (set by debug event or other logic)
        if wall_run.active {
             // Keep running logic
        } else {
             // Clear state
             wall_run.is_running_right = false;
             wall_run.is_running_left = false;
        }
    }
}

/// System to apply physics for wall running (override gravity, stick to wall)
pub fn update_wall_run_physics(
    mut query: Query<(&mut WallRun, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (wall_run, transform_opt) in query.iter_mut() {
        if !wall_run.active {
            continue;
        }

        if let Some(mut transform) = transform_opt {
             // Move forward along the wall
             // In a real system, we project the forward vector onto the wall plane using the normal
             let forward_speed = if input_state.sprint { wall_run.wall_sprint_speed } else { wall_run.wall_run_speed };
             
             // Placeholder movement
             // transform.translation += transform.forward() * forward_speed * time.delta_secs();
             
             // "Stick" logic would apply velocity towards the wall (-normal) to counter-act drifting off
             
             // Gravity should be disabled or heavily reduced here usually
        }
    }
}

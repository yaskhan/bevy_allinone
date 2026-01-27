//! Camera system module
//!
//! Advanced camera management for 3rd/1st person views, locked cameras, and more.

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_camera_follow,
                update_camera_rotation,
                update_camera_zoom,
                handle_camera_collision,
            ).chain());
    }
}

/// Camera controller component
///
/// Manages camera behavior for character following, rotation, and special modes.
#[derive(Component, Debug)]
pub struct GameCamera {
    // Follow settings
    pub follow_target: Option<Entity>,
    pub pivot_offset: Vec3,
    pub camera_offset: Vec3,
    
    // Rotation settings
    pub rotation_speed: f32,
    pub vertical_rotation_speed: f32,
    pub horizontal_rotation_speed: f32,
    
    // Limits
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    // Zoom settings
    pub current_distance: f32,
    pub zoom_speed: f32,
    pub smooth_zoom: bool,
    
    // FOV settings
    pub default_fov: f32,
    pub aim_fov: f32,
    pub sprint_fov: f32,
    pub fov_change_speed: f32,
    
    // Collision settings
    pub check_collision: bool,
    pub collision_layers: u32,
    pub collision_radius: f32,
    
    // State
    pub is_locked: bool,
    pub is_first_person: bool,
    pub shake_active: bool,
    
    // TODO: Add more fields
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            follow_target: None,
            pivot_offset: Vec3::new(0.0, 1.5, 0.0),
            camera_offset: Vec3::new(0.0, 0.0, -3.0),
            
            rotation_speed: 5.0,
            vertical_rotation_speed: 5.0,
            horizontal_rotation_speed: 5.0,
            
            min_vertical_angle: -60.0,
            max_vertical_angle: 60.0,
            min_distance: 1.0,
            max_distance: 10.0,
            
            current_distance: 3.0,
            zoom_speed: 2.0,
            smooth_zoom: true,
            
            default_fov: 60.0,
            aim_fov: 45.0,
            sprint_fov: 70.0,
            fov_change_speed: 5.0,
            
            check_collision: true,
            collision_layers: 1,
            collision_radius: 0.2,
            
            is_locked: false,
            is_first_person: false,
            shake_active: false,
        }
    }
}

/// Camera state component
///
/// Tracks current camera angles and position
#[derive(Component, Debug, Default)]
pub struct CameraState {
    pub current_horizontal_angle: f32,
    pub current_vertical_angle: f32,
    pub target_distance: f32,
    pub current_fov: f32,
    
    // TODO: Add more state tracking
}

/// Camera shake effect
///
/// TODO: Implement camera shake system
#[derive(Component, Debug)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub frequency: f32,
    pub elapsed: f32,
}

/// Locked camera mode
///
/// For fixed camera angles (like classic survival horror games)
///
/// TODO: Implement locked camera system
#[derive(Component, Debug)]
pub struct LockedCamera {
    pub camera_position: Vec3,
    pub look_at_target: Vec3,
    pub transition_speed: f32,
}

/// Camera waypoint system
///
/// For cinematic camera paths
///
/// TODO: Implement waypoint system
#[derive(Component, Debug)]
pub struct CameraWaypoint {
    pub waypoints: Vec<Vec3>,
    pub current_waypoint: usize,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Update camera to follow target
///
/// TODO: Implement follow logic
fn update_camera_follow(
    time: Res<Time>,
    mut camera_query: Query<(
        &GameCamera,
        &CameraState,
        &mut Transform,
    )>,
    target_query: Query<&Transform, Without<GameCamera>>,
) {
    for (camera, state, mut camera_transform) in camera_query.iter_mut() {
        if let Some(target_entity) = camera.follow_target {
            if let Ok(target_transform) = target_query.get(target_entity) {
                // TODO: Calculate pivot position
                // TODO: Apply camera offset
                // TODO: Smooth camera movement
                // TODO: Handle first person mode
                
                let _delta = time.delta_secs();
                let _target_pos = target_transform.translation;
                let _pivot_pos = _target_pos + camera.pivot_offset;
                
                // Placeholder
                camera_transform.translation = _pivot_pos + camera.camera_offset;
            }
        }
    }
}

/// Update camera rotation based on input
///
/// TODO: Implement rotation logic
fn update_camera_rotation(
    time: Res<Time>,
    mut query: Query<(
        &GameCamera,
        &mut CameraState,
        &mut Transform,
    )>,
) {
    for (camera, mut state, mut transform) in query.iter_mut() {
        if camera.is_locked {
            continue;
        }
        
        // TODO: Read mouse/gamepad input
        // TODO: Apply rotation speed
        // TODO: Clamp vertical angle
        // TODO: Update transform rotation
        
        let _delta = time.delta_secs();
        
        // Clamp vertical angle
        state.current_vertical_angle = state.current_vertical_angle
            .clamp(camera.min_vertical_angle, camera.max_vertical_angle);
        
        // TODO: Apply rotation to transform
    }
}

/// Update camera zoom
///
/// TODO: Implement zoom logic
fn update_camera_zoom(
    time: Res<Time>,
    mut query: Query<(
        &mut GameCamera,
        &mut CameraState,
    )>,
) {
    for (mut camera, mut state) in query.iter_mut() {
        // TODO: Read zoom input
        // TODO: Smooth zoom transition
        // TODO: Clamp distance
        
        let _delta = time.delta_secs();
        
        // Clamp distance
        camera.current_distance = camera.current_distance
            .clamp(camera.min_distance, camera.max_distance);
        
        state.target_distance = camera.current_distance;
    }
}

/// Handle camera collision with environment
///
/// TODO: Implement collision logic
fn handle_camera_collision(
    mut query: Query<(
        &GameCamera,
        &mut Transform,
    )>,
) {
    for (camera, mut _transform) in query.iter_mut() {
        if !camera.check_collision {
            continue;
        }
        
        // TODO: Raycast from pivot to camera position
        // TODO: Adjust camera distance on collision
        // TODO: Smooth collision response
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Spawn a game camera
///
/// TODO: Add more configuration options
pub fn spawn_camera(
    commands: &mut Commands,
    target: Entity,
    position: Vec3,
) -> Entity {
    commands.spawn((
        Camera3d::default(),
        GameCamera {
            follow_target: Some(target),
            ..default()
        },
        CameraState::default(),
        Transform::from_translation(position),
        GlobalTransform::default(),
    ))
    .id()
}

/// Set camera target
pub fn set_camera_target(
    camera: &mut GameCamera,
    target: Entity,
) {
    camera.follow_target = Some(target);
}

/// Toggle first person mode
///
/// TODO: Implement first person transition
pub fn toggle_first_person(
    camera: &mut GameCamera,
) {
    camera.is_first_person = !camera.is_first_person;
    
    // TODO: Adjust camera offset
    // TODO: Adjust FOV
    // TODO: Trigger transition animation
}

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum WaypointRotationMode {
    #[default]
    UseWaypointRotation,
    FaceMovement,
    LookAtTarget,
}

/// Individual waypoint in a track
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraWaypoint {
    pub wait_time: f32,
    pub movement_speed: Option<f32>,
    pub rotation_speed: Option<f32>,
    pub rotation_mode: WaypointRotationMode,
    pub look_at_target: Option<Entity>,
}

impl Default for CameraWaypoint {
    fn default() -> Self {
        Self {
            wait_time: 0.0,
            movement_speed: None,
            rotation_speed: None,
            rotation_mode: WaypointRotationMode::UseWaypointRotation,
            look_at_target: None,
        }
    }
}

/// A track composed of multiple waypoints
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraWaypointTrack {
    pub waypoints: Vec<Entity>,
    pub loop_track: bool,
}

/// State for a camera following a waypoint track
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraWaypointFollower {
    pub current_track: Option<Entity>,
    pub current_waypoint_index: usize,
    pub waiting_timer: f32,
    pub is_moving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum CameraMode {
    #[default]
    ThirdPerson,
    FirstPerson,
    Locked,
    SideScroller,
}

/// Camera controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraController {
    pub follow_target: Option<Entity>,
    pub mode: CameraMode,
    
    // Sensitivity
    pub rot_sensitivity_3p: f32,
    pub rot_sensitivity_1p: f32,
    pub aim_zoom_sensitivity_mult: f32,
    
    // Limits
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    
    // Zoom/Distance
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    // Smoothing
    pub smooth_follow_speed: f32,
    pub smooth_rotation_speed: f32,
    pub pivot_smooth_speed: f32,
    
    // Offsets (Dynamic)
    pub default_pivot_offset: Vec3,
    pub aim_pivot_offset: Vec3,
    pub crouch_pivot_offset: Vec3,
    
    // Leaning
    pub lean_amount: f32,
    pub lean_angle: f32,
    pub lean_speed: f32,
    
    // FOV
    pub default_fov: f32,
    pub aim_fov: f32,
    pub fov_speed: f32,
    
    // Collision
    pub use_collision: bool,
    pub collision_radius: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            follow_target: None,
            mode: CameraMode::ThirdPerson,
            
            rot_sensitivity_3p: 0.15,
            rot_sensitivity_1p: 0.1,
            aim_zoom_sensitivity_mult: 0.5,
            
            min_vertical_angle: -80.0,
            max_vertical_angle: 80.0,
            
            distance: 4.0,
            min_distance: 1.0,
            max_distance: 10.0,
            
            smooth_follow_speed: 15.0,
            smooth_rotation_speed: 20.0,
            pivot_smooth_speed: 10.0,
            
            default_pivot_offset: Vec3::new(0.0, 1.6, 0.0),
            aim_pivot_offset: Vec3::new(0.5, 1.5, 0.0),
            crouch_pivot_offset: Vec3::new(0.0, 1.0, 0.0),
            
            lean_amount: 0.4,
            lean_angle: 15.0,
            lean_speed: 8.0,
            
            default_fov: 60.0,
            aim_fov: 40.0,
            fov_speed: 10.0,
            
            use_collision: true,
            collision_radius: 0.2,
        }
    }
}

/// Camera state tracking
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub current_pivot: Vec3,
    pub current_lean: f32,
    pub noise_offset: Vec2,
    pub bob_offset: Vec3,
    pub is_aiming: bool,
    pub is_crouching: bool,
    pub lock_on_target: Option<Entity>,
    pub fov_override: Option<f32>,
    pub fov_override_speed: Option<f32>,
}

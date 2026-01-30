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
    TopDown,
}

#[derive(Debug, Clone, Reflect)]
pub struct CameraZoneSettings {
    pub mode: CameraMode,
    pub distance: Option<f32>,
    pub pivot_offset: Option<Vec3>,
    pub fov: Option<f32>,
    pub fixed_yaw: Option<f32>,
    pub fixed_pitch: Option<f32>,
    pub follow_rotation: bool,
    pub look_at_player: bool,
    pub transition_speed: f32,
}

impl Default for CameraZoneSettings {
    fn default() -> Self {
        Self {
            mode: CameraMode::ThirdPerson,
            distance: None,
            pivot_offset: None,
            fov: None,
            fixed_yaw: None,
            fixed_pitch: None,
            follow_rotation: true,
            look_at_player: true,
            transition_speed: 5.0,
        }
    }
}

/// A component for a trigger volume that changes camera settings
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraZone {
    pub settings: CameraZoneSettings,
    pub priority: i32,
}

impl Default for CameraZone {
    fn default() -> Self {
        Self {
            settings: CameraZoneSettings::default(),
            priority: 0,
        }
    }
}

/// Tracks the current camera zone for an entity
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraZoneTracker {
    pub current_zone: Option<Entity>,
    pub active_zones: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum CameraSide {
    #[default]
    Right,
    Left,
}

/// Camera side preference

#[derive(Debug, Clone, Reflect)]
pub struct TargetLockSettings {
    pub enabled: bool,
    pub max_distance: f32,
    pub fov_threshold: f32, // Angle threshold to maintain lock
    pub scan_radius: f32,   // Radius of the "sticky" area at screen center
    pub lock_smooth_speed: f32,
}

impl Default for TargetLockSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_distance: 30.0,
            fov_threshold: 45.0,
            scan_radius: 2.0,
            lock_smooth_speed: 10.0,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct CameraStateInfo {
    pub name: String,
    pub cam_position_offset: Vec3,
    pub pivot_position_offset: Vec3,
    pub pivot_parent_position_offset: Vec3,
    pub y_limits: Vec2,
    pub initial_fov: f32,
    pub fov_transition_speed: f32,
    pub max_fov: f32,
    pub min_fov: f32,
    pub lean_enabled: bool,
    pub max_lean_angle: f32,
    pub lean_speed: f32,
    pub head_track_active: bool,
    pub look_in_player_direction: bool,
    pub look_in_player_direction_speed: f32,
    pub camera_collision_active: bool,
}

impl Default for CameraStateInfo {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            cam_position_offset: Vec3::ZERO,
            pivot_position_offset: Vec3::ZERO,
            pivot_parent_position_offset: Vec3::ZERO,
            y_limits: Vec2::new(-80.0, 80.0),
            initial_fov: 60.0,
            fov_transition_speed: 10.0,
            max_fov: 90.0,
            min_fov: 15.0,
            lean_enabled: true,
            max_lean_angle: 15.0,
            lean_speed: 8.0,
            head_track_active: true,
            look_in_player_direction: false,
            look_in_player_direction_speed: 5.0,
            camera_collision_active: true,
        }
    }
}

/// Camera controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraController {
    pub follow_target: Option<Entity>,
    pub mode: CameraMode,
    pub current_side: CameraSide,
    
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
    pub distance_smooth_speed: f32,
    
    // Offsets (Dynamic)
    pub side_offset: f32,
    pub default_pivot_offset: Vec3,
    pub aim_pivot_offset: Vec3,
    pub crouch_pivot_offset: Vec3,
    
    // Leaning
    pub lean_amount: f32,
    pub lean_angle: f32,
    pub lean_speed: f32,
    pub lean_raycast_dist: f32,
    pub lean_wall_angle: f32,
    
    // FOV
    pub default_fov: f32,
    pub aim_fov: f32,
    pub fov_speed: f32,
    
    // Collision
    pub use_collision: bool,
    pub collision_radius: f32,

    // Target Lock
    pub target_lock: TargetLockSettings,

    // States list and current state tracking
    pub states: Vec<CameraStateInfo>,
    pub current_state_name: String,

    // Baseline settings (for smooth restoration after zones)
    pub base_mode: CameraMode,
    pub base_distance: f32,
    pub base_fov: f32,
    pub base_pivot_offset: Vec3,
    pub base_transition_speed: f32,
    pub enabled: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            follow_target: None,
            mode: CameraMode::ThirdPerson,
            current_side: CameraSide::Right,
            
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
            distance_smooth_speed: 8.0,
            
            side_offset: 0.5,
            default_pivot_offset: Vec3::new(0.0, 1.6, 0.0),
            aim_pivot_offset: Vec3::new(0.5, 1.5, 0.0),
            crouch_pivot_offset: Vec3::new(0.0, 1.0, 0.0),
            
            lean_amount: 0.4,
            lean_angle: 15.0,
            lean_speed: 8.0,
            lean_raycast_dist: 0.8,
            lean_wall_angle: 5.0,
            
            default_fov: 60.0,
            aim_fov: 40.0,
            fov_speed: 10.0,
            
            use_collision: true,
            collision_radius: 0.2,

            target_lock: TargetLockSettings::default(),

            states: Vec::new(),
            current_state_name: "Third Person".to_string(),

            base_mode: CameraMode::ThirdPerson,
            base_distance: 4.0,
            base_fov: 60.0,
            base_pivot_offset: Vec3::new(0.0, 1.6, 0.0),
            base_transition_speed: 5.0,
            enabled: true,
        }
    }
}

/// Target state for a camera
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraTargetState {
    pub marked_target: Option<Entity>,
    pub locked_target: Option<Entity>,
    pub is_locking: bool,
}

/// Camera state tracking
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub current_pivot: Vec3,
    pub current_side_interpolator: f32, // -1.0 (Left) to 1.0 (Right)
    pub current_lean: f32,
    pub noise_offset: Vec2,
    pub bob_offset: Vec3,
    pub is_aiming: bool,
    pub is_crouching: bool,
    pub fov_override: Option<f32>,
    pub fov_override_speed: Option<f32>,
}
#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct TransparencySettings {
    pub enabled: bool,
    pub alpha_target: f32,
    pub fade_speed: f32,
    pub ray_radius: f32,
}

impl Default for TransparencySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            alpha_target: 0.2,
            fade_speed: 10.0,
            ray_radius: 0.2,
        }
    }
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerCullingSettings {
    pub enabled: bool,
    pub min_dist: f32,
    pub fade_speed: f32,
    pub min_alpha: f32,
}

impl Default for PlayerCullingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_dist: 1.0,
            fade_speed: 8.0,
            min_alpha: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct TransparentSurface {
    pub target_alpha: f32,
    pub current_alpha: f32,
    pub active_this_frame: bool,
}

impl Default for TransparentSurface {
    fn default() -> Self {
        Self {
            target_alpha: 1.0,
            current_alpha: 1.0,
            active_this_frame: false,
        }
    }
}

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Main component for head tracking.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct HeadTrack {
    pub enabled: bool,
    
    // Weights (0.0 to 1.0)
    pub head_weight: f32,
    pub body_weight: f32,
    
    // Limits (degrees)
    pub range_angle_x: Vec2, // Pitch (Vertical)
    pub range_angle_y: Vec2, // Yaw (Horizontal)
    
    // Speed
    pub rotation_speed: f32,
    pub weight_change_speed: f32,
    
    // Targets
    pub look_in_camera_direction: bool,
    pub active_target: Option<Entity>,
    
    // Bone Entities (Cached)
    pub head_bone: Option<Entity>,
    pub neck_bone: Option<Entity>,
    
    // Current Internal State
    pub current_head_weight: f32,
    pub current_body_weight: f32,
}

impl Default for HeadTrack {
    fn default() -> Self {
        Self {
            enabled: true,
            head_weight: 1.0,
            body_weight: 0.4,
            range_angle_x: Vec2::new(-90.0, 90.0),
            range_angle_y: Vec2::new(-90.0, 90.0),
            rotation_speed: 3.0,
            weight_change_speed: 2.0,
            look_in_camera_direction: true,
            active_target: None,
            head_bone: None,
            neck_bone: None,
            current_head_weight: 0.0,
            current_body_weight: 0.0,
        }
    }
}

/// Component for objects that can be tracked by the head.
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct HeadTrackTarget {
    pub enabled: bool,
    pub min_distance: f32,
    pub priority: i32,
}

impl Default for HeadTrackTarget {
    fn default() -> Self {
        Self {
            enabled: true,
            min_distance: 10.0,
            priority: 0,
        }
    }
}

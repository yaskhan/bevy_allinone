use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum PointAndClickElementType {
    #[default]
    Device,
    Vehicle,
    Friend,
    Enemy,
}

/// Component for objects that can be clicked on
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PointAndClickElement {
    pub element_type: PointAndClickElementType,
    pub interaction_offset: Vec3,
    pub enabled: bool,
}

impl Default for PointAndClickElement {
    fn default() -> Self {
        Self {
            element_type: PointAndClickElementType::Device,
            interaction_offset: Vec3::new(0.0, 0.0, 1.0), // Default 1 unit forward
            enabled: true,
        }
    }
}

/// Component for the agent/player controlled by point and click
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PointAndClickController {
    pub enabled: bool,
    pub current_target: Option<Vec3>,
    pub move_speed: f32,
    pub stopping_distance: f32,
    pub rotation_speed: f32,
}

impl Default for PointAndClickController {
    fn default() -> Self {
        Self {
            enabled: true,
            current_target: None,
            move_speed: 5.0,
            stopping_distance: 0.1,
            rotation_speed: 10.0,
        }
    }
}

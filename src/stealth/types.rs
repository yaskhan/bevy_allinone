use bevy::prelude::*;

/// Hide state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum HideState {
    #[default]
    Visible,
    CrouchHide,
    ProneHide,
    Peek,
    CornerLean,
    FixedPlaceHide,
}

/// Cover type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CoverType {
    #[default]
    Low,
    Medium,
    High,
    Corner,
    Full,
}

/// Cover object information
#[derive(Debug, Clone, Reflect)]
pub struct CoverObject {
    pub entity: Entity,
    pub position: Vec3,
    pub normal: Vec3,
    pub height: f32,
    pub cover_type: CoverType,
    pub is_corner: bool,
}

impl Default for CoverObject {
    fn default() -> Self {
        Self {
            entity: Entity::from_bits(0),
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            height: 0.0,
            cover_type: CoverType::Low,
            is_corner: false,
        }
    }
}

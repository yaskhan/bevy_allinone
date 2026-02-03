use bevy::prelude::*;

/// Collision type categories.
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum CollisionType {
    Unknown,
    Ground,
    Wall,
    Ceiling,
}

/// Stores last collision classification.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CheckCollisionType {
    pub last_type: CollisionType,
}

impl Default for CheckCollisionType {
    fn default() -> Self {
        Self {
            last_type: CollisionType::Unknown,
        }
    }
}

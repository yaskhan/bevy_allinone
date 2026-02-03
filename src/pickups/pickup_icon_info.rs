use bevy::prelude::*;

/// Pickup icon metadata.
///
/// GKC reference: `pickUpIconInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpIconInfo {
    pub id: i32,
    pub name: String,
    pub icon_object: Option<Entity>,
    pub texture_object: Option<Entity>,
    pub target: Option<Entity>,
    pub icon_active: bool,
    pub paused: bool,
    pub icon_size: Vec2,
}

impl Default for PickUpIconInfo {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            icon_object: None,
            texture_object: None,
            target: None,
            icon_active: false,
            paused: false,
            icon_size: Vec2::new(32.0, 32.0),
        }
    }
}

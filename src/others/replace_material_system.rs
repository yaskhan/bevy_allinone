use bevy::prelude::*;

/// Replaces material properties (simplified to color).
///
/// GKC reference: `replaceMaterialSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ReplaceMaterialSystem {
    pub color: Color,
    pub enabled: bool,
}

impl Default for ReplaceMaterialSystem {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            enabled: false,
        }
    }
}

pub fn update_replace_material_system(
    mut query: Query<(&ReplaceMaterialSystem, Option<&mut Sprite>, Option<&mut BackgroundColor>)>,
) {
    for (settings, sprite, bg) in query.iter_mut() {
        if !settings.enabled {
            continue;
        }
        if let Some(mut sprite) = sprite {
            sprite.color = settings.color;
        }
        if let Some(mut bg) = bg {
            bg.0 = settings.color;
        }
    }
}

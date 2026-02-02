use bevy::prelude::*;

/// Changes object colors over time.
///
/// GKC reference: `changeObjectColors.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ChangeObjectColors {
    pub target: Color,
    pub speed: f32,
    pub enabled: bool,
}

impl Default for ChangeObjectColors {
    fn default() -> Self {
        Self {
            target: Color::WHITE,
            speed: 1.0,
            enabled: true,
        }
    }
}

pub fn update_change_object_colors(
    time: Res<Time>,
    mut query: Query<(&ChangeObjectColors, Option<&mut Sprite>, Option<&mut BackgroundColor>)>,
) {
    let delta = time.delta_seconds();
    for (settings, sprite, bg) in query.iter_mut() {
        if !settings.enabled {
            continue;
        }
        let t = (settings.speed * delta).clamp(0.0, 1.0);
        if let Some(mut sprite) = sprite {
            sprite.color = sprite.color.lerp(settings.target, t);
        }
        if let Some(mut bg) = bg {
            bg.0 = bg.0.lerp(settings.target, t);
        }
    }
}

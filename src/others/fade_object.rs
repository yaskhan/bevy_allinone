use bevy::prelude::*;

/// Fade object visibility over time.
///
/// GKC reference: `fadeObject.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FadeObject {
    pub target_alpha: f32,
    pub speed: f32,
    pub enabled: bool,
}

impl Default for FadeObject {
    fn default() -> Self {
        Self {
            target_alpha: 0.0,
            speed: 1.0,
            enabled: false,
        }
    }
}

pub fn update_fade_object(
    time: Res<Time>,
    mut query: Query<(&FadeObject, Option<&mut Sprite>, Option<&mut BackgroundColor>)>,
) {
    let delta = time.delta_seconds();
    for (settings, sprite, bg) in query.iter_mut() {
        if !settings.enabled {
            continue;
        }
        let step = (settings.speed * delta).clamp(0.0, 1.0);
        if let Some(mut sprite) = sprite {
            let mut color = sprite.color;
            color.set_alpha(color.alpha().lerp(settings.target_alpha, step));
            sprite.color = color;
        }
        if let Some(mut bg) = bg {
            let mut color = bg.0;
            color.set_alpha(color.alpha().lerp(settings.target_alpha, step));
            bg.0 = color;
        }
    }
}

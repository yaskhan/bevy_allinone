use bevy::prelude::*;

/// Hoverboard animation settings.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HoverBoardAnimationSystem {
    pub enabled: bool,
    pub bob_amplitude: f32,
    pub bob_speed: f32,
    pub base_offset: Vec3,
}

impl Default for HoverBoardAnimationSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            bob_amplitude: 0.05,
            bob_speed: 2.0,
            base_offset: Vec3::ZERO,
        }
    }
}

pub fn update_hoverboard_animation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &HoverBoardAnimationSystem)>,
) {
    for (mut transform, anim) in query.iter_mut() {
        if !anim.enabled {
            continue;
        }
        let bob = (time.elapsed_secs() * anim.bob_speed).sin() * anim.bob_amplitude;
        transform.translation = anim.base_offset + Vec3::Y * bob;
    }
}

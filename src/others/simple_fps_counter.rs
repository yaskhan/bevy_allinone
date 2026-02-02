use bevy::prelude::*;

/// Simple FPS counter.
///
/// GKC reference: `SimpleFPSCounter.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleFpsCounter {
    pub fps: f32,
}

impl Default for SimpleFpsCounter {
    fn default() -> Self {
        Self { fps: 0.0 }
    }
}

pub fn update_simple_fps_counter(
    time: Res<Time>,
    mut query: Query<&mut SimpleFpsCounter>,
) {
    let delta = time.delta_seconds();
    if delta <= 0.0 {
        return;
    }
    let fps = 1.0 / delta;
    for mut counter in query.iter_mut() {
        counter.fps = fps;
    }
}

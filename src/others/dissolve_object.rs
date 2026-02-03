use bevy::prelude::*;

/// Dissolve effect controller.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DissolveObject {
    pub amount: f32,
    pub speed: f32,
    pub enabled: bool,
}

impl Default for DissolveObject {
    fn default() -> Self {
        Self {
            amount: 0.0,
            speed: 1.0,
            enabled: false,
        }
    }
}

pub fn update_dissolve_object(
    time: Res<Time>,
    mut query: Query<(&mut DissolveObject, Option<&mut Visibility>)>,
) {
    let delta = time.delta_seconds();
    for (mut dissolve, visibility) in query.iter_mut() {
        if !dissolve.enabled {
            continue;
        }
        dissolve.amount = (dissolve.amount + dissolve.speed * delta).clamp(0.0, 1.0);
        if dissolve.amount >= 1.0 {
            if let Some(mut visibility) = visibility {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

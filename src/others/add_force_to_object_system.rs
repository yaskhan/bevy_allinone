use bevy::prelude::*;
use avian3d::prelude::LinearVelocity;

/// Adds force to an object using LinearVelocity as a fallback.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AddForceToObjectSystem {
    pub force: Vec3,
    pub impulse: bool,
    pub apply_every_frame: bool,
    pub enabled: bool,
}

impl Default for AddForceToObjectSystem {
    fn default() -> Self {
        Self {
            force: Vec3::ZERO,
            impulse: true,
            apply_every_frame: false,
            enabled: true,
        }
    }
}

pub fn update_add_force_to_object_system(
    time: Res<Time>,
    mut query: Query<(&mut AddForceToObjectSystem, &mut LinearVelocity)>,
) {
    let delta = time.delta_seconds();
    if delta == 0.0 {
        return;
    }

    for (mut system, mut velocity) in query.iter_mut() {
        if !system.enabled {
            continue;
        }
        if system.force == Vec3::ZERO {
            continue;
        }

        if system.impulse {
            velocity.0 += system.force;
        } else {
            velocity.0 += system.force * delta;
        }

        if !system.apply_every_frame {
            system.enabled = false;
        }
    }
}

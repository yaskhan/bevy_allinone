use bevy::prelude::*;
use avian3d::prelude::{LinearVelocity, AngularVelocity};

/// Enables or disables rigidbody motion.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SetRigidbodyStateSystem {
    pub frozen: bool,
}

impl Default for SetRigidbodyStateSystem {
    fn default() -> Self {
        Self { frozen: false }
    }
}

pub fn update_set_rigidbody_state_system(
    mut query: Query<(&SetRigidbodyStateSystem, Option<&mut LinearVelocity>, Option<&mut AngularVelocity>)>,
) {
    for (state, lin, ang) in query.iter_mut() {
        if !state.frozen {
            continue;
        }
        if let Some(mut lin) = lin {
            lin.0 = Vec3::ZERO;
        }
        if let Some(mut ang) = ang {
            ang.0 = Vec3::ZERO;
        }
    }
}

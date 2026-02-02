use bevy::prelude::*;
use crate::physics::LinearVelocity;
use super::ability_info::AbilityInfo;

/// Dash ability controller.
///
/// GKC reference: `dashSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DashAbility {
    pub ability_name: String,
    pub dash_speed: f32,
    pub dash_duration: f32,
    pub active: bool,
    pub timer: f32,
}

impl Default for DashAbility {
    fn default() -> Self {
        Self {
            ability_name: "Dash".to_string(),
            dash_speed: 12.0,
            dash_duration: 0.2,
            active: false,
            timer: 0.0,
        }
    }
}

/// Start a dash when the matching ability is active.
pub fn start_dash_from_ability(
    mut query: Query<(&AbilityInfo, &mut DashAbility, &GlobalTransform, &mut LinearVelocity)>,
) {
    for (ability, mut dash, transform, mut velocity) in query.iter_mut() {
        if ability.name != dash.ability_name {
            continue;
        }

        if ability.active && !dash.active {
            dash.active = true;
            dash.timer = dash.dash_duration;
            let forward = transform.forward().as_vec3();
            velocity.0 = forward * dash.dash_speed;
        }
    }
}

/// Update dash timers and stop when finished.
pub fn update_dash_ability(
    time: Res<Time>,
    mut query: Query<(&mut DashAbility, &mut LinearVelocity)>,
) {
    for (mut dash, mut velocity) in query.iter_mut() {
        if !dash.active {
            continue;
        }

        dash.timer -= time.delta_secs();
        if dash.timer <= 0.0 {
            dash.active = false;
            velocity.0 = Vec3::ZERO;
        }
    }
}

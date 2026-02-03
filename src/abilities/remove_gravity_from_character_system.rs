use bevy::prelude::*;
use crate::physics::CustomGravity;
use avian3d::prelude::LinearVelocity;

#[derive(Debug, Clone)]
pub struct RemoveGravityEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct RemoveGravityEventQueue(pub Vec<RemoveGravityEvent>);

/// Temporarily remove gravity from character.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RemoveGravityFromCharacterSystem {
    pub remove_gravity_duration: f32,
    pub extra_gravity_force: f32,
    pub pause_extra_force_after_delay: bool,
    pub pause_extra_force_delay: f32,
    pub pause_extra_force_speed: f32,
    pub active_timer: f32,
    pub active: bool,
    pub cached_multiplier: f32,
    pub current_force: Vec3,
}

impl Default for RemoveGravityFromCharacterSystem {
    fn default() -> Self {
        Self {
            remove_gravity_duration: 5.0,
            extra_gravity_force: 5.0,
            pause_extra_force_after_delay: false,
            pause_extra_force_delay: 0.5,
            pause_extra_force_speed: 3.0,
            active_timer: 0.0,
            active: false,
            cached_multiplier: 1.0,
            current_force: Vec3::ZERO,
        }
    }
}

pub fn activate_remove_gravity(
    mut events: ResMut<RemoveGravityEventQueue>,
    mut query: Query<(&mut RemoveGravityFromCharacterSystem, Option<&mut CustomGravity>, Option<&mut LinearVelocity>)>,
) {
    for event in events.0.drain(..) {
        if let Ok((mut system, custom_gravity, velocity)) = query.get_mut(event.entity) {
            system.active = true;
            system.active_timer = system.remove_gravity_duration;
            system.current_force = Vec3::Y * system.extra_gravity_force;

            if let Some(mut gravity) = custom_gravity {
                system.cached_multiplier = gravity.multiplier;
                gravity.multiplier = 0.0;
            }
            if let Some(mut vel) = velocity {
                vel.0 += system.current_force;
            }
        }
    }
}

pub fn update_remove_gravity(
    time: Res<Time>,
    mut query: Query<(&mut RemoveGravityFromCharacterSystem, Option<&mut CustomGravity>, Option<&mut LinearVelocity>)>,
) {
    for (mut system, custom_gravity, velocity) in query.iter_mut() {
        if !system.active {
            continue;
        }

        system.active_timer -= time.delta_secs();
        if system.pause_extra_force_after_delay && system.active_timer <= system.remove_gravity_duration - system.pause_extra_force_delay {
            let step = system.pause_extra_force_speed * time.delta_secs();
            let len = system.current_force.length();
            if len > 0.0 {
                let new_len = (len - step).max(0.0);
                system.current_force = system.current_force.normalize_or_zero() * new_len;
            }
            if let Some(mut vel) = velocity {
                vel.0 += system.current_force * time.delta_secs();
            }
        }

        if system.active_timer <= 0.0 {
            system.active = false;
            if let Some(mut gravity) = custom_gravity {
                gravity.multiplier = system.cached_multiplier;
            }
        }
    }
}

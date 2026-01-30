use bevy::prelude::*;
use crate::vehicles::types::*;
use avian3d::prelude::*;
use bevy::ecs::system::EntityCommands;

pub fn update_vehicle_stats(
    time: Res<Time>,
    mut query: Query<&mut VehicleStats>,
) {
    let delta = time.delta_secs();
    let current_time = time.elapsed_secs();

    for mut stats in query.iter_mut() {
        if current_time - stats.last_damage_time < stats.regen_delay {
            continue;
        }

        if stats.health_regen_enabled && stats.health < stats.max_health {
            stats.health = (stats.health + stats.health_regen_speed * delta).min(stats.max_health);
        }

        if stats.booster_regen_enabled && stats.booster < stats.max_booster {
            stats.booster = (stats.booster + stats.booster_regen_speed * delta).min(stats.max_booster);
        }

        if stats.fuel_regen_enabled && stats.fuel < stats.max_fuel {
            stats.fuel = (stats.fuel + stats.fuel_regen_speed * delta).min(stats.max_fuel);
        }
    }
}

pub fn handle_vehicle_collisions(
    // mut collision_events: EventReader<Collision>, // Disabling to fix build if not found
    mut vehicle_query: Query<(Entity, &mut VehicleStats, &LinearVelocity)>,
    receiver_query: Query<(&VehicleDamageReceiver, &ChildOf)>,
    time: Res<Time>,
) {
    // For now, this is a placeholder until we confirm the Collision event type in this Avian version
}

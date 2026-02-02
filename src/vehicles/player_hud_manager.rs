use bevy::prelude::*;
use super::types::{Vehicle, VehicleStats};

/// Player HUD manager for vehicle-related UI.
///
/// GKC reference: `playerHUDManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerHudManager {
    pub enabled: bool,
    pub current_vehicle: Option<Entity>,
}

impl Default for PlayerHudManager {
    fn default() -> Self {
        Self {
            enabled: true,
            current_vehicle: None,
        }
    }
}

pub fn update_player_hud_manager(
    mut query: Query<&mut PlayerHudManager>,
    vehicle_query: Query<(Entity, &Vehicle, &VehicleStats)>,
) {
    for mut hud in query.iter_mut() {
        if !hud.enabled {
            continue;
        }
        if hud.current_vehicle.is_none() {
            // pick first active vehicle if any
            if let Some((entity, _, _)) = vehicle_query.iter().next() {
                hud.current_vehicle = Some(entity);
            }
        }
    }
}

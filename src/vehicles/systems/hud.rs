use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_hud(
    vehicle_query: Query<(&Vehicle, &VehicleStats, &VehicleWeaponSystem)>,
    driver_query: Query<&VehicleDriver>,
    mut speed_ui: Query<&mut Text, (With<VehicleHudSpeed>, Without<VehicleHudHealth>, Without<VehicleHudFuel>, Without<VehicleHudAmmo>)>,
    mut health_ui: Query<&mut Text, (With<VehicleHudHealth>, Without<VehicleHudSpeed>, Without<VehicleHudFuel>, Without<VehicleHudAmmo>)>,
    mut fuel_ui: Query<&mut Text, (With<VehicleHudFuel>, Without<VehicleHudSpeed>, Without<VehicleHudHealth>, Without<VehicleHudAmmo>)>,
    mut ammo_ui: Query<&mut Text, (With<VehicleHudAmmo>, Without<VehicleHudSpeed>, Without<VehicleHudHealth>, Without<VehicleHudFuel>)>,
) {
    // Usually there's only one player driver at a time
    // We nested the driver under the seat, but here we can just find anyone with VehicleDriver component
    // though it's better to find the vehicle they are driving.
    
    // For now, let's assume we update the HUD with the first driven vehicle found
    for (vehicle, stats, weapon_sys) in vehicle_query.iter() {
        if vehicle.is_driving {
            // Update Speed
            for mut text in speed_ui.iter_mut() {
                text.0 = format!("{:.0} KM/H", vehicle.current_speed * 3.6);
            }

            // Update Health
            for mut text in health_ui.iter_mut() {
                text.0 = format!("HP: {:.0}/{:.0}", stats.health, stats.max_health);
            }

            // Update Fuel
            for mut text in fuel_ui.iter_mut() {
                if stats.use_fuel {
                    text.0 = format!("FUEL: {:.0}%", (stats.fuel / stats.max_fuel) * 100.0);
                } else {
                    text.0 = "FUEL: N/A".to_string();
                }
            }

            // Update Ammo
            for mut text in ammo_ui.iter_mut() {
                if let Some(weapon) = weapon_sys.weapons.get(weapon_sys.current_weapon_index) {
                    text.0 = format!("AMMO: {} / {}", weapon.ammo_in_clip, weapon.total_ammo);
                } else {
                    text.0 = "AMMO: 0".to_string();
                }
            }
            
            // Only update HUD for the primary driven vehicle
            break;
        }
    }
}

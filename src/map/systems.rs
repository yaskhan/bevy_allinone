use bevy::prelude::*;
use bevy::ui::{Node, Val, UiRect, Display, FlexDirection, AlignItems, JustifyContent, PositionType};
use avian3d::prelude::*;
use crate::map::types::*;

// ============================================================================
// MAP SYSTEMS
// ============================================================================

/// System to update the compass UI rotation based on the camera/player.
pub fn update_compass(
    player_query: Query<&Transform, With<crate::character::Player>>,
    mut compass_query: Query<&mut Transform, (With<CompassUI>, Without<crate::character::Player>)>,
) {
    let Some(player_transform) = player_query.iter().next() else { return };
    let (_, rotation, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
    
    for mut compass_transform in compass_query.iter_mut() {
        // Rotate the compass UI in the opposite direction of the player's yaw
        compass_transform.rotation = Quat::from_rotation_z(rotation);
    }
}

/// System to connect MapObjectInformation changes to MapMarkers (Sync)
pub fn update_map_object_information(
    mut commands: Commands,
    query: Query<(Entity, &MapObjectInformation), Changed<MapObjectInformation>>,
) {
    for (entity, info) in query.iter() {
        // Sync MapObjectInformation to MapMarker for the UI system to pick it up
        // Or we could have the UI system read MapObjectInformation directly.
        // For now, let's keep MapMarker as the "renderable" component for simpler migration.
        commands.entity(entity).insert(MapMarker {
            name: info.name.clone(),
            icon_type: info.icon_type,
            visible_in_minimap: info.is_visible,
            visible_in_full_map: info.is_visible,
        });
    }
}

/// System to update visible map elements based on building and floor
pub fn update_visible_map_elements(
    mut icons: Query<(&mut Node, &MapMarkerIcon)>,
    markers: Query<&MapObjectInformation>,
    global_state: Res<MapGlobalState>,
) {
    for (mut node, icon) in icons.iter_mut() {
        if let Ok(info) = markers.get(icon.marker_entity) {
            // Check if object belongs to current hierarchy context
            let same_building = info.building_index == global_state.current_building_index;
            let same_floor = info.floor_index == global_state.current_floor_index;
            
            // Allow global objects (index -1) or specific matches
            let is_visible = info.building_index == -1 || (same_building && same_floor);
            
            if !is_visible {
                node.display = Display::None;
            } 
            // If visible, we don't interfere; update_minimap_positions handles placement
        }
    }
}

/// System to handle quick travel interaction with raycast validation.
/// Uses proximity check combined with line-of-sight verification for more accurate interaction.
pub fn handle_quick_travel(
    mut player_query: Query<(Entity, &mut Transform, &GlobalTransform, &crate::character::Player)>,
    stations: Query<(Entity, &Transform, &GlobalTransform, &QuickTravelStation), (Without<crate::character::Player>)>,
    input: Res<ButtonInput<KeyCode>>,
    spatial_query: SpatialQuery,
) {
    let Some((player_entity, mut player_transform, player_global, _)) = player_query.iter_mut().next() else { return };

    if input.just_pressed(KeyCode::KeyE) {
        let player_pos = player_global.translation();
        let mut best_station: Option<(Vec3, &QuickTravelStation)> = None;
        let mut best_distance = f32::MAX;

        for (station_entity, _station_transform, station_global, station) in stations.iter() {
            if !station.is_active {
                continue;
            }

            let station_pos = station_global.translation();
            let distance = player_pos.distance(station_pos);

            // Check proximity threshold
            if distance > station.interaction_distance {
                continue;
            }

            // Verify line of sight using raycast (skip if station has allow_through_walls)
            if !station.allow_through_walls {
                let dir_to_station = (station_pos - player_pos).normalize();
                let filter = SpatialQueryFilter::from_excluded_entities([player_entity, station_entity]);

                if let Some(hit) = spatial_query.cast_ray(
                    player_pos + Vec3::Y * 1.5, // Eye level
                    Dir3::new(dir_to_station).unwrap_or(Dir3::Y),
                    distance,
                    true,
                    &filter,
                ) {
                    // If we hit something before the station, it's blocked
                    if hit.distance < distance - 0.5 {
                        continue;
                    }
                }
            }

            // Select closest valid station
            if distance < best_distance {
                best_distance = distance;
                best_station = Some((station_pos, station));
            }
        }

        if let Some((station_pos, station)) = best_station {
            info!("Quick Travel to: {} (from {:?})", station.destination_name, station_pos);
            player_transform.translation = station.destination;
            // TODO: Trigger teleport effect/sound
        }
    }
}

/// System to connect ObjectiveIcon changes to MapMarkers
pub fn update_objective_icons(
    mut commands: Commands,
    query: Query<(Entity, &ObjectiveIcon), Changed<ObjectiveIcon>>,
) {
    for (entity, info) in query.iter() {
         commands.entity(entity).insert(MapMarker {
            name: info.description.clone(),
            icon_type: info.icon_type,
            // Objectives usually visible everywhere
            visible_in_minimap: true,
            visible_in_full_map: true,
        });
    }
}

// Integrated logic into existing (placeholder) check_map_zones
pub fn check_map_zones(
    player_query: Query<&Transform, With<crate::character::Player>>,
    mut zones: Query<(&Transform, &mut MapZone)>,
    mut global_state: ResMut<MapGlobalState>,
) {
    let Some(player_transform) = player_query.iter().next() else { return };
    let player_pos = player_transform.translation;

    for (transform, mut zone) in zones.iter_mut() {
        if zone.is_discovered {
            continue;
        }

        // Simple distance check (e.g. 10 units)
        if player_pos.distance(transform.translation) < 10.0 {
            zone.is_discovered = true;
            if !global_state.discovered_zones.contains(&zone.zone_id) {
                global_state.discovered_zones.push(zone.zone_id);
                info!("Map Zone Discovered: {}", zone.zone_name);
            }
        }
    }
}

/// System to handle map view toggling
pub fn handle_map_system_input(
    input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<MapSettings>,
    mut global_state: ResMut<MapGlobalState>,
) {
    if input.just_pressed(KeyCode::KeyM) {
        global_state.map_menu_opened = !global_state.map_menu_opened;
        settings.full_map_enabled = global_state.map_menu_opened;
        info!("Toggled Map: {}", global_state.map_menu_opened);
    }
}

use bevy::prelude::*;
use bevy::ui::{Node, Val, UiRect, Display, FlexDirection, AlignItems, JustifyContent, PositionType};
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

/// System to check for map zone discovery
pub fn check_map_zones(
    player_query: Query<&Transform, With<crate::character::Player>>,
    mut zones: Query<&mut MapZone>,
    mut global_state: ResMut<MapGlobalState>,
) {
    let Some(player_transform) = player_query.iter().next() else { return };
    let player_pos = player_transform.translation;

    for mut zone in zones.iter_mut() {
        if zone.is_discovered {
            continue;
        }

        // Logic to discovery (e.g., distance or volumes)
        // Placeholder for now
        // if player_pos.distance(zone_center) < radius { ... }
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

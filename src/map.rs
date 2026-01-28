use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Type of icon to display on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum MapIconType {
    Npc,
    Quest,
    PointOfInterest,
    Player,
    Enemy,
}

/// Component to mark an entity as visible on the map.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MapMarker {
    pub name: String,
    pub icon_type: MapIconType,
    pub visible_in_minimap: bool,
    pub visible_in_full_map: bool,
}

impl Default for MapMarker {
    fn default() -> Self {
        Self {
            name: "Marker".to_string(),
            icon_type: MapIconType::PointOfInterest,
            visible_in_minimap: true,
            visible_in_full_map: true,
        }
    }
}

/// Settings for the map system.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct MapSettings {
    pub minimap_zoom: f32,
    pub full_map_zoom: f32,
    pub follow_player_rotation: bool,
    pub compass_enabled: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            minimap_zoom: 1.0,
            full_map_zoom: 1.0,
            follow_player_rotation: true,
            compass_enabled: true,
        }
    }
}

/// Component for the compass UI element.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CompassUI;

/// Component for map marker icons in the UI.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MapMarkerIcon {
    pub marker_entity: Entity,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapSettings>()
            .register_type::<MapMarker>()
            .register_type::<MapSettings>()
            .register_type::<CompassUI>()
            .register_type::<MapMarkerIcon>()
            .add_systems(Update, (
                update_minimap_positions,
                update_compass,
            ));
    }
}

/// System to update the compass UI rotation based on the camera/player.
fn update_compass(
    player_query: Query<&Transform, With<crate::character::Player>>,
    mut compass_query: Query<&mut Transform, (With<CompassUI>, Without<crate::character::Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let (_, rotation, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
    
    for mut compass_transform in compass_query.iter_mut() {
        // Rotate the compass UI in the opposite direction of the player's yaw
        compass_transform.rotation = Quat::from_rotation_z(rotation);
    }
}

/// System to update marker icons in the UI.
fn update_minimap_positions(
    player_query: Query<&Transform, With<crate::character::Player>>,
    markers: Query<(Entity, &GlobalTransform, &MapMarker)>,
    mut icons: Query<(&mut Node, &MapMarkerIcon)>,
    settings: Res<MapSettings>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation;

    for (mut node, icon) in icons.iter_mut() {
        if let Ok((_entity, marker_transform, marker)) = markers.get(icon.marker_entity) {
            if !marker.visible_in_minimap {
                node.display = Display::None;
                continue;
            }
            
            node.display = Display::Flex;
            
            let marker_pos = marker_transform.translation();
            let delta = marker_pos - player_pos;
            
            // Project 3D world delta to 2D UI space (XZ plane to XY plane)
            // Apply zoom and settings here
            let ui_x = delta.x * settings.minimap_zoom * 10.0;
            let ui_y = -delta.z * settings.minimap_zoom * 10.0;
            
            node.left = Val::Px(ui_x);
            node.top = Val::Px(ui_y);
        }
    }
}

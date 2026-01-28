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

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapSettings>()
            .register_type::<MapMarker>()
            .register_type::<MapSettings>()
            .add_systems(Update, (
                update_minimap_positions,
            ));
    }
}

/// Placeholder system for updating minimap positions.
fn update_minimap_positions(
    _markers: Query<(&Transform, &MapMarker)>,
    _settings: Res<MapSettings>,
) {
    // TODO: Implement coordinate transformation to UI space
}

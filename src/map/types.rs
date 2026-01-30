use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Type of icon to display on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum MapIconType {
    Npc,
    Quest,
    PointOfInterest,
    Player,
    Enemy,
    Custom(u32), // Extended for flexibility
}

/// Component to mark an entity as visible on the map.
/// (Legacy wrapper or simple marker)
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

/// Detailed Map Object Information (Porting GKit MapObjectInformation)
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MapObjectInformation {
    pub name: String,
    pub description: String,
    pub icon_type: MapIconType,
    pub is_visible: bool,
    pub building_index: i32,
    pub floor_index: i32,
    pub follow_object_position: bool,
    pub offset: Vec3,
}

impl Default for MapObjectInformation {
    fn default() -> Self {
        Self {
            name: "Map Object".to_string(),
            description: "".to_string(),
            icon_type: MapIconType::PointOfInterest,
            is_visible: true,
            building_index: 0,
            floor_index: 0,
            follow_object_position: true,
            offset: Vec3::ZERO,
        }
    }
}

/// Component for map zones
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MapZone {
    pub zone_name: String,
    pub zone_id: i32,
    pub is_discovered: bool,
    pub discovered_by_default: bool,
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

// ============================================================================
// RESOURCES
// ============================================================================

/// Settings for the map system.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct MapSettings {
    pub minimap_zoom: f32,
    pub full_map_zoom: f32,
    pub follow_player_rotation: bool,
    pub compass_enabled: bool,
    pub minimap_enabled: bool,
    pub full_map_enabled: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            minimap_zoom: 1.0,
            full_map_zoom: 1.0,
            follow_player_rotation: true,
            compass_enabled: true,
            minimap_enabled: true,
            full_map_enabled: false,
        }
    }
}

/// Component for a building in the map system
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MapBuilding {
    pub name: String,
    pub index: i32,
    pub floors: Vec<Entity>, // Entities with MapFloor component
}

/// Component for a specific floor in a building
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MapFloor {
    pub name: String,
    pub floor_number: i32,
    pub floor_index: i32, // Index in the building's floor list
    pub is_active: bool,
}

/// Tracks discovered zones and map status.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct MapGlobalState {
    pub discovered_zones: Vec<i32>, // IDs of discovered zones
    pub map_menu_opened: bool,
    pub current_building_index: i32,
    pub current_floor_index: i32,
}

/// Component for intra-level teleportation station
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct QuickTravelStation {
    pub destination: Vec3,
    pub is_active: bool,
    pub interact_message: String,
}

/// Component for map objectives (quests)
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ObjectiveIcon {
    pub off_screen_arrow: bool,
    pub icon_type: MapIconType,
    pub description: String,
}


use bevy::prelude::*;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Level Manager component
/// Defines a spawn point/entry point for a level
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LevelManager {
    /// ID of this level manager (used for linking)
    pub id: i32,
    
    /// Scene number (for reference)
    pub scene_number: i32,
    
    /// Show gizmo for spawn point
    pub show_gizmo: bool,
    
    /// Gizmo color
    pub gizmo_color: Color,
    
    /// Gizmo radius
    pub gizmo_radius: f32,
}

impl Default for LevelManager {
    fn default() -> Self {
        Self {
            id: 0,
            scene_number: 0,
            show_gizmo: true,
            gizmo_color: Color::srgb(1.0, 0.0, 0.0),
            gizmo_radius: 0.5,
        }
    }
}

/// Travel Station Information
#[derive(Debug, Clone, Reflect)]
pub struct TravelStationDestination {
    pub name: String,
    pub scene_number: i32,
    pub level_manager_id: i32,
    pub zone_found: bool,
}

/// Travel Station Component
/// Interactive object to travel between locations
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TravelStation {
    /// List of destinations available from this station
    pub destinations: Vec<TravelStationDestination>,
    
    /// Current scene number (where this station is)
    pub current_scene_number: i32,
    
    /// Current level manager ID (where this station is linked to, usually close to)
    pub current_level_manager_id: i32,
    
    /// Are all stations unlocked by default?
    pub all_stations_unlocked: bool,
    
    /// Is the station currently being used?
    pub using_station: bool,
    
    /// Name of the station (for display)
    pub station_name: String,
}

impl Default for TravelStation {
    fn default() -> Self {
        Self {
            destinations: Vec::new(),
            current_scene_number: 0,
            current_level_manager_id: 0,
            all_stations_unlocked: false,
            using_station: false,
            station_name: "Station".to_string(),
        }
    }
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Tracks discovered stations globally
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct LevelManagerGlobalState {
    pub discovered_stations: Vec<TravelStationDestination>,
}

/// Tracks current level info
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct CurrentLevelInfo {
    pub scene_number: i32,
    pub level_manager_id: i32,
}

/// Pending Level Change
#[derive(Resource, Default, Debug)]
pub struct PendingLevelChange {
    pub active: bool,
    pub target_scene: i32,
    pub target_id: i32,
    pub timer: f32,
}

// ============================================================================
// EVENTS
// ============================================================================

/// Request to change level/travel
#[derive(Event, Debug, Clone)]
pub struct RequestLevelChangeEvent {
    pub target_scene: i32,
    pub target_level_manager_id: i32,
    pub delay: f32,
}

#[derive(Resource, Default)]
pub struct RequestLevelChangeEventQueue(pub Vec<RequestLevelChangeEvent>);

/// Event when a station is discovered
#[derive(Event, Debug, Clone)]
pub struct TravelStationDiscoveredEvent {
    pub station_name: String,
    pub scene: i32,
    pub id: i32,
}

#[derive(Resource, Default)]
pub struct TravelStationDiscoveredEventQueue(pub Vec<TravelStationDiscoveredEvent>);

use bevy::prelude::*;
// use bevy::app::App; // Explicit import removed

pub mod types;
pub mod systems;
pub mod ui;

use types::*;
use systems::*;
use ui::*;

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types & Components
            .register_type::<LevelManager>()
            .register_type::<TravelStation>()
            .register_type::<TravelStationDestination>()
            .register_type::<LevelManagerGlobalState>()
            .register_type::<CurrentLevelInfo>()

            // Resources
            .init_resource::<LevelManagerGlobalState>()
            .init_resource::<CurrentLevelInfo>()
            .init_resource::<PendingLevelChange>()
            .init_resource::<RequestLevelChangeEventQueue>()
            .init_resource::<TravelStationDiscoveredEventQueue>()

            // Events
            // Events (Managed via Queues)
            // .add_event::<RequestLevelChangeEvent>()
            // .add_event::<TravelStationDiscoveredEvent>()

            // Systems
            .add_systems(Startup, setup_travel_ui)
            .add_systems(Update, (
                handle_level_change,
                spawn_player_at_level_manager,
                handle_travel_station_discovery,
                update_travel_ui,
                handle_travel_button_interactions,
            ));
    }
}

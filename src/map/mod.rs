use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod ui;

use types::*;
use systems::*;
use ui::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types & Components
            .register_type::<MapMarker>()
            .register_type::<MapIconType>()
            .register_type::<MapObjectInformation>()
            .register_type::<MapZone>()
            .register_type::<MapBuilding>()
            .register_type::<MapFloor>()
            .register_type::<QuickTravelStation>()
            .register_type::<ObjectiveIcon>()
            .register_type::<MapSettings>()
            .register_type::<MapGlobalState>()
            .register_type::<CompassUI>()
            .register_type::<MapMarkerIcon>()

            // Resources
            .init_resource::<MapSettings>()
            .init_resource::<MapGlobalState>()

            // Systems
            .add_systems(Startup, setup_map_ui)
            .add_systems(Update, (
                update_minimap_positions,
                update_compass,
                update_map_object_information,
                update_visible_map_elements,
                handle_quick_travel,
                update_objective_icons,
                check_map_zones,
                handle_map_system_input,
                update_map_visibility,
            ));
    }
}

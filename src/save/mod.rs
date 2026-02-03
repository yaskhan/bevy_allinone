pub mod types;
pub mod resources;
pub mod systems;
pub mod events;

use bevy::prelude::*;
use types::*;
use resources::*;
use systems::*;
use events::*;

pub use types::{
    SaveData, SavedInventoryItem, EquipmentData, GameProgress, CameraOrientation, 
    SaveSlotInfo, SavePlaceholderHealth, SavePlaceholderInventory, InventoryItemData
};
pub use resources::SaveManager;
pub use systems::auto_save_system;
pub use events::{RequestSaveEvent, RequestLoadEvent};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveManager>()
            .add_event::<RequestSaveEvent>()
            .add_event::<RequestLoadEvent>()
            .add_systems(Startup, systems::init_save_manager)
            .add_systems(Update, (
                auto_save_system,
                systems::handle_save_requests,
                systems::handle_load_requests,
            ));
    }
}

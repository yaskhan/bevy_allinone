pub mod types;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use types::*;
use resources::*;
use systems::*;

pub use types::{
    SaveData, SavedInventoryItem, EquipmentData, GameProgress, CameraOrientation, 
    SaveSlotInfo, SavePlaceholderHealth, SavePlaceholderInventory, InventoryItemData
};
pub use resources::SaveManager;
pub use systems::auto_save_system;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveManager>()
            .add_systems(Update, auto_save_system);
    }
}

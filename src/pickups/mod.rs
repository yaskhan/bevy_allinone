use bevy::prelude::*;

pub mod chest_system;
pub mod crate_system;
pub mod drop_pickup_system;
pub mod explosive_barrel;
pub mod pickup_element_info;

pub use chest_system::ChestSystem;
pub use crate_system::CrateSystem;
pub use drop_pickup_system::DropPickUpSystem;
pub use explosive_barrel::ExplosiveBarrel;
pub use pickup_element_info::PickUpElementInfo;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            chest_system::update_chest_system,
            drop_pickup_system::update_drop_pickup_system,
        ));
    }
}

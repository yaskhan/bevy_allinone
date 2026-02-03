use bevy::prelude::*;

pub mod chest_system;
pub mod crate_system;
pub mod drop_pickup_system;
pub mod explosive_barrel;
pub mod pickup_element_info;
pub mod pickup_icon;
pub mod pickup_icon_info;
pub mod pickup_manager;
pub mod pickup_object;
pub mod pickups_screen_info;
pub mod player_pickup_icon_manager;
pub mod pickup_type;
pub mod ammo_pickup;
pub mod energy_pickup;
pub mod experience_multiplier_pickup;
pub mod experience_pickup;
pub mod general_pickup;
pub mod grab_objects_strength_pickup;
pub mod health_pickup;

pub use chest_system::ChestSystem;
pub use crate_system::CrateSystem;
pub use drop_pickup_system::DropPickUpSystem;
pub use explosive_barrel::ExplosiveBarrel;
pub use pickup_element_info::PickUpElementInfo;
pub use pickup_icon::PickUpIcon;
pub use pickup_icon_info::PickUpIconInfo;
pub use pickup_manager::PickUpManager;
pub use pickup_object::PickUpObject;
pub use pickups_screen_info::PickUpsScreenInfo;
pub use player_pickup_icon_manager::PlayerPickupIconManager;
pub use pickup_type::PickupType;
pub use ammo_pickup::AmmoPickup;
pub use energy_pickup::EnergyPickup;
pub use experience_multiplier_pickup::ExperienceMultiplierPickup;
pub use experience_pickup::ExperiencePickup;
pub use general_pickup::GeneralPickup;
pub use grab_objects_strength_pickup::GrabObjectsStrengthPickup;
pub use health_pickup::HealthPickup;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            chest_system::update_chest_system,
            drop_pickup_system::update_drop_pickup_system,
        ));
    }
}

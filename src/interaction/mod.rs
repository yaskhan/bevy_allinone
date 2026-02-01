pub mod types;
pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use types::*;
use components::*;
use events::*;
use resources::*;
use systems::*;

pub use types::{InteractionType, DeviceInfo};
pub use components::{
    InteractionDetector, Interactable, UsingDevicesSystem, DeviceStringAction, 
    InteractionPrompt, InteractionData, UsableDevice
};
pub use events::{
    AddDeviceEvent, AddDeviceQueue, RemoveDeviceEvent, RemoveDeviceQueue, 
    InteractionEvent, InteractionEventQueue
};
pub use resources::{CurrentInteractable, InteractionDebugSettings, InteractionUIState};
pub use systems::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InteractionEventQueue>()
            .init_resource::<CurrentInteractable>()
            .init_resource::<InteractionDebugSettings>()
            .init_resource::<AddDeviceQueue>()
            .init_resource::<RemoveDeviceQueue>()
            
            // Register types
            .register_type::<InteractionDetector>()
            .register_type::<Interactable>()
            .register_type::<InteractionType>()
            .register_type::<DeviceInfo>()
            .register_type::<UsingDevicesSystem>()
            .register_type::<DeviceStringAction>()
            .register_type::<InteractionData>()
            .register_type::<UsableDevice>()
            
            .add_systems(Update, (
                detect_interactables,
                detect_devices_in_proximity,
                update_device_list,
                select_closest_device,
                validate_interactions,
                process_interactions,
                update_interaction_ui,
                debug_draw_interaction_rays,
            ).chain())
            .add_systems(Startup, setup_interaction_ui);
    }
}

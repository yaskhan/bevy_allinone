//! Devices System Module
use bevy::prelude::*;

pub mod types;
pub mod systems;

// Import submodules
pub mod door_system;
pub mod electronic_device;
pub mod move_device_to_camera;
pub mod move_camera_to_device;
pub mod hologram_door;
pub mod simple_switch;
pub mod pressure_plate;
pub mod recharger_station;
pub mod examine_object;

pub use types::*;
pub use systems::*;

pub struct DevicesPlugin;

impl Plugin for DevicesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DeviceList>()
            .init_resource::<DeviceUIState>()
            .init_resource::<DeviceDebugSettings>()
            .add_systems(Update, (
                systems::detect_devices,
                systems::update_device_ui,
                systems::process_device_interaction,
                systems::update_device_icons,
                systems::debug_draw_device_info,
            ).chain())
            .add_systems(Startup, systems::setup_device_ui)
            // Add subplugins
            .add_plugins(door_system::DoorSystemPlugin)
            .add_plugins(electronic_device::ElectronicDevicePlugin)
            .add_plugins(move_device_to_camera::MoveDeviceToCameraPlugin)
            .add_plugins(move_camera_to_device::MoveCameraToDevicePlugin)
            .add_plugins(hologram_door::HologramDoorPlugin)
            .add_plugins(simple_switch::SimpleSwitchPlugin)
            .add_plugins(pressure_plate::PressurePlatePlugin)
            .add_plugins(recharger_station::RechargerStationPlugin)
            .add_plugins(examine_object::ExamineObjectPlugin);
    }
}

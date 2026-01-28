//! # Bevy Game Controller
//!
//! A powerful 3D/2.5D game controller plugin for Bevy Engine.
//!
//! ## Features
//!
//! - **Character Controller**: Full-body awareness 3rd/1st person controller
//! - **Camera System**: Advanced camera management with multiple modes
//! - **Input System**: Flexible input handling for multiple platforms
//! - **Combat System**: Melee and ranged combat mechanics
//! - **Inventory System**: Item management and equipment
//! - **AI System**: NPC behavior and pathfinding
//! - **Save System**: Game state persistence
//! - **And much more...**
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_allinone::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(GameControllerPlugin)
//!         .run();
//! }
//! ```

use bevy::prelude::*;

pub mod ai;
pub mod camera;
pub mod character;
pub mod combat;
pub mod input;
pub mod interaction;
pub mod inventory;
pub mod physics;
pub mod save;
pub mod utils;
pub mod vehicles;
pub mod weapons;

pub mod prelude {
    //! Commonly used types and traits
    
    pub use crate::ai::*;
    pub use crate::camera::*;
    pub use crate::character::*;
    pub use crate::combat::*;
    pub use crate::input::*;
    pub use crate::interaction::*;
    pub use crate::inventory::*;
    pub use crate::physics::*;
    pub use crate::save::*;
    pub use crate::utils::*;
    pub use crate::vehicles::*;
    pub use crate::weapons::*;
    pub use crate::GameControllerPlugin;
    pub use bevy::prelude::*;
}

/// The main plugin for the game controller systems
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add sub-plugins
            .add_plugins((
                ai::AiPlugin,
                camera::CameraPlugin,
                character::CharacterPlugin,
                combat::CombatPlugin,
                input::InputPlugin,
                interaction::InteractionPlugin,
                inventory::InventoryPlugin,
                physics::PhysicsPlugin,
                save::SavePlugin,
                vehicles::VehiclesPlugin,
                weapons::WeaponsPlugin,
            ))
            // Add resources
            .init_resource::<utils::GameTime>()
            // Add startup systems
            .add_systems(Startup, setup_allinone);
    }
}

/// Initialize the game controller system
///
/// TODO: Implement initialization logic
fn setup_allinone(
    _commands: Commands,
) {
    info!("Bevy All in one Controller Plugin initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_build() {
        let mut app = App::new();
        app.add_plugins(GameControllerPlugin);
        // Plugin should build without panicking
    }
}

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
}

use bevy::prelude::*;

/// Main plugin for the Bevy game controller system
///
/// This plugin initializes all subsystems and manages their lifecycle.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_allinone::GameControllerPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(GameControllerPlugin)
///     .run();
/// ```
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add core systems
            .add_plugins((
                character::CharacterPlugin,
                camera::CameraPlugin,
                input::InputPlugin,
                physics::PhysicsPlugin,
                combat::CombatPlugin,
                weapons::WeaponsPlugin,
                inventory::InventoryPlugin,
                interaction::InteractionPlugin,
                ai::AiPlugin,
                vehicles::VehiclesPlugin,
                save::SavePlugin,
            ))
            // Add resources
            .init_resource::<utils::GameTime>()
            // Add startup systems
            .add_systems(Startup, setup_allinone);
    }
}

/// Setup system for game controller initialization
///
/// TODO: Implement initialization logic
fn setup_allinone(
    mut commands: Commands,
) {
    info!("Bevy All in one Controller Plugin initialized");
    
    // TODO: Initialize core systems
    // TODO: Load default configurations
    // TODO: Setup event handlers
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

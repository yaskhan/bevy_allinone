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

pub mod abilities;
pub mod actions;
pub mod events;
pub mod experience;
pub mod footsteps;
pub mod game_manager;
pub mod ai;
pub mod camera;
pub mod character;
pub mod climb;
pub mod combat;
pub mod currency;
pub mod devices;
pub mod dialog;
pub mod input;
pub mod interaction;
pub mod inventory;
pub mod ladder;
pub mod map;
pub mod physics;
pub mod player;
pub mod puzzle;
pub mod quest;
pub mod save;
pub mod skills;
pub mod stats;
pub mod stealth;
pub mod tutorial;
pub mod utils;
pub mod vehicles;
pub mod vendor;
pub mod weapons;

pub mod prelude {
    //! Commonly used types and traits

    pub use crate::abilities::*;
    pub use crate::actions::*;
    pub use crate::events::*;
    pub use crate::experience::*;
    pub use crate::footsteps::*;
    pub use crate::game_manager::*;
    pub use crate::ai::*;
    pub use crate::camera::*;
    pub use crate::character::*;
    pub use crate::climb::*;
    pub use crate::combat::*;
    pub use crate::currency::*;
    pub use crate::devices;
    pub use crate::dialog::*;
    pub use crate::input::*;
    pub use crate::interaction;
    pub use crate::inventory::*;
    pub use crate::ladder::*;
    pub use crate::map::*;
    pub use crate::physics::*;
    pub use crate::player::*;
    pub use crate::puzzle::*;
    pub use crate::quest::*;
    pub use crate::save::*;
    pub use crate::skills::*;
    pub use crate::stats::*;
    pub use crate::stealth::*;
    pub use crate::tutorial::*;
    pub use crate::utils::*;
    pub use crate::vehicles::*;
    pub use crate::vendor::*;
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
            .add_plugins(abilities::AbilitiesPlugin)
            .add_plugins(actions::ActionSystemPlugin)
            .add_plugins(events::EventSystemPlugin)
            .add_plugins(experience::ExperiencePlugin)
            .add_plugins(footsteps::FootstepPlugin)
            .add_plugins(game_manager::GameManagerPlugin)
            .add_plugins(ai::AiPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(character::CharacterPlugin)
            .add_plugins(climb::ClimbPlugin)
            .add_plugins(combat::CombatPlugin)
            .add_plugins(currency::CurrencyPlugin)
            .add_plugins(devices::DevicesPlugin)
            .add_plugins(dialog::DialogPlugin)
            .add_plugins(input::InputPlugin)
            .add_plugins(interaction::InteractionPlugin)
            .add_plugins(inventory::InventoryPlugin)
            .add_plugins(ladder::LadderPlugin)
            .add_plugins(map::MapPlugin)
            .add_plugins(physics::PhysicsPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(puzzle::PuzzlePlugin)
            .add_plugins(quest::QuestPlugin)
            .add_plugins(save::SavePlugin)
            .add_plugins(skills::SkillsPlugin)
            .add_plugins(stats::StatsPlugin)
            .add_plugins(stealth::StealthPlugin)
            .add_plugins(tutorial::TutorialPlugin)
            .add_plugins(vehicles::VehiclesPlugin)
            .add_plugins(vendor::VendorPlugin)
            .add_plugins(weapons::WeaponsPlugin)
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

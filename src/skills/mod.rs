pub mod types;
pub mod skill;
pub mod skill_tree;
pub mod skills_system;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use types::*;
use skill::*;
use skill_tree::*;
use skills_system::*;
use systems::*;

pub use types::{SkillType, SkillEvent, SkillSystemEvent};
pub use skill::{Skill, SkillLevel};
pub use skill_tree::{SkillCategory, SkillTree, SkillTemplate, SkillTemplateCategory, SkillTemplateInfo};
pub use types::{SkillEffect};
pub use skills_system::SkillsSystem;
pub use systems::*;

/// Skills plugin
pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        // Register types
        app.register_type::<SkillLevel>()
           .register_type::<Skill>()
           .register_type::<SkillCategory>()
           .register_type::<SkillTree>()
           .register_type::<SkillTemplate>()
           .register_type::<SkillEffect>()
           .register_type::<SkillsSystem>();

        // Add events
        app.init_resource::<SkillSystemEventQueue>()
           .register_type::<SkillSystemEvent>();

        // Add systems
        app.add_systems(Startup, ui::setup_skill_tree_ui)
           .add_systems(Update, (
            skills_system_update,
            ui::toggle_skill_tree_ui,
            ui::update_skill_tree_ui,
        ));
    }
}

/// Prelude for skills system
pub mod prelude {
    pub use super::{
        Skill, SkillCategory, SkillLevel, SkillSystemEvent, SkillTemplate, SkillTree, SkillsPlugin,
        SkillsSystem, SkillType,
    };
}

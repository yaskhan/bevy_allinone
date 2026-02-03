use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<types::PlayerExperience>()
            .register_type::<types::ExperienceSettings>()
            .register_type::<types::ObjectExperience>()
            .init_resource::<types::ExperienceSettings>()
            .init_resource::<types::ExperienceObtainedQueue>()
            .init_resource::<types::LevelUpQueue>()
            .add_systems(Startup, systems::initialize_experience_settings)
            .add_systems(Update, (
                systems::handle_experience_gain,
                systems::handle_level_up_rewards,
                systems::update_xp_multiplier,
                systems::sync_experience_to_stats,
            ));
    }
}

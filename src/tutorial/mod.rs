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

pub use types::{Tutorial, TutorialPanel};
pub use components::{TutorialLog, TutorialRoot, TutorialTitleText, TutorialDescriptionText, TutorialPanelImage, TutorialButton};
pub use events::{TutorialEvent, TutorialEventQueue};
pub use resources::TutorialManager;
pub use systems::*;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TutorialManager>()
            .init_resource::<TutorialEventQueue>()
            .register_type::<TutorialLog>()
            .add_systems(Update, (
                handle_tutorial_events,
                update_tutorial_ui,
                handle_tutorial_buttons,
                manage_tutorial_game_state,
            ));
    }
}

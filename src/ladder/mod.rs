pub mod types;
pub mod ladder_system;
pub mod player_ladder;
pub mod systems;

use bevy::prelude::*;
use types::*;
use ladder_system::*;
use player_ladder::*;
use systems::*;

pub use types::LadderMovementState;
pub use ladder_system::LadderSystem;
pub use player_ladder::PlayerLadderSystem;
pub use systems::*;

pub struct LadderPlugin;

impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<LadderSystem>()
            .register_type::<PlayerLadderSystem>()
            .register_type::<LadderDirection>()
            .register_type::<LadderEndDetection>()
            .register_type::<LadderMovementTracker>()
            .register_type::<LadderAnimation>()
            .register_type::<LadderMovement>()
            .register_type::<LadderExitDetection>()
            .register_type::<LadderFootstep>()
            .add_systems(Update, (
                handle_ladder_input,
                update_ladder_state,
                update_ladder_movement,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_ladder,
                handle_ladder_mount,
                handle_ladder_dismount,
            ).chain());
    }
}

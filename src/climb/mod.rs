pub mod types;
pub mod climb_ledge_system;
pub mod systems;

use bevy::prelude::*;
use types::*;
use climb_ledge_system::*;
use systems::*;

// Re-export specific types for cleaner imports
pub use types::ClimbState;
pub use types::LedgeZone;
pub use types::ForceMode;
pub use types::SurfaceType;
pub use types::LedgeLostReason;
pub use types::ClimbStateTracker;
pub use types::LedgeDetection;
pub use types::AutoHang;
pub use types::ClimbAnimation;
pub use types::ClimbMovement;
pub use types::LedgeJump;
pub use types::GrabSurfaceOnAir;
pub use types::LedgeGrabbedEvent;
pub use types::LedgeClimbedEvent;
pub use types::LedgeLostEvent;
pub use types::LedgeJumpEvent;
pub use climb_ledge_system::ClimbLedgeSystem;

pub struct ClimbPlugin;

impl Plugin for ClimbPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ClimbLedgeSystem>()
            .register_type::<LedgeZone>()
            .add_systems(Update, (
                handle_climb_input,
                update_climb_state,
                update_climb_visuals,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_ledge,
                detect_ledge_below,
                update_climb_movement,
                handle_auto_hang,
            ));
    }
}

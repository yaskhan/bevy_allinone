use bevy::prelude::*;
use bevy::app::App;
use crate::vehicles::types::{VehicleAI, WaypointPath};

pub mod types;
mod systems;
mod faction;
mod patrol;
mod perception;
mod turret;
mod combat;
mod behavior;
mod hiding;
mod movement;
mod vehicle_ai;

pub use types::*;
pub use systems::*;
pub use faction::*;
pub use patrol::*;
pub use perception::*;
pub use turret::*;
pub use combat::*;
pub use behavior::*;
pub use hiding::*;
pub use movement::*;
pub use vehicle_ai::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AiController>()
            .register_type::<AiPerception>()
            .register_type::<FriendManager>()
            .register_type::<AiVisionVisualizer>()
            .register_type::<AiStateVisuals>()
            .register_type::<AiCombatBrain>()
            .register_type::<AiCombatStrategy>()
            .register_type::<AiRangedCombatSettings>()
            .register_type::<AiMeleeCombatSettings>()
            .register_type::<AiCloseCombatSettings>()
            .register_type::<AiPowersCombatSettings>()
            .register_type::<AiBehaviorState>()
            .register_type::<CharacterFaction>()
            .register_type::<HidePosition>()
            .register_type::<FactionRelation>()
            .register_type::<Turret>()
            .register_type::<TurretCombat>()
            .register_type::<TurretLaser>()
            .register_type::<AiCombatSettings>()
            .register_type::<PatrolPath>()
            .register_type::<AIPerceptionSettings>()
            .register_type::<AiMovement>()
            .register_type::<VehicleAI>()
            .register_type::<WaypointPath>()
            .init_resource::<FactionSystem>()
            .init_resource::<FriendSystem>()
            .init_resource::<NoiseEventQueue>()
            .add_systems(Update, (
                update_ai_perception,
                update_ai_hearing,
                handle_friend_commands,
                update_ai_behavior,
                update_ai_suspicion,
                update_ai_movement,
                update_patrol,
                update_turrets,
                update_turret_firing,
                update_turret_lasers,
                update_ai_combat,
                update_ai_hiding,
                draw_ai_vision_cones,
                update_ai_state_visuals,
                update_faction_relations,
                alert_faction_members,
                update_vehicle_ai,
            ));
    }
}

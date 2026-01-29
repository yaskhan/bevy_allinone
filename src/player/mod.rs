use bevy::prelude::*;

pub mod extra_movements;
pub mod navmesh_override;
pub mod player_idle;
pub mod player_modes;
pub mod player_state;
pub mod player_state_icon;
pub mod ragdoll;
pub mod sprite_animator;
pub mod upper_body_rotation;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                extra_movements::ExtraMovementsPlugin,
                navmesh_override::NavMeshOverridePlugin,
                player_idle::PlayerIdlePlugin,
                player_modes::PlayerModesPlugin,
                player_state::PlayerStatePlugin,
                player_state_icon::PlayerStateIconPlugin,
                ragdoll::RagdollPlugin,
                sprite_animator::SpriteAnimatorPlugin,
                upper_body_rotation::UpperBodyRotationPlugin,
            ));
    }
}

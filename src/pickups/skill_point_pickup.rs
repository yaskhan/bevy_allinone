use bevy::prelude::*;

/// Skill point pickup data.
///
/// GKC reference: `skillPointPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SkillPointPickup {
    pub points: u32,
}

impl Default for SkillPointPickup {
    fn default() -> Self {
        Self { points: 0 }
    }
}

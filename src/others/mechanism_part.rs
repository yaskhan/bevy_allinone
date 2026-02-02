use bevy::prelude::*;

/// Mechanical part metadata.
///
/// GKC reference: `mechanismPart.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MechanismPart {
    pub part_id: String,
    pub active: bool,
}

impl Default for MechanismPart {
    fn default() -> Self {
        Self {
            part_id: String::new(),
            active: true,
        }
    }
}

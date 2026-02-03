use bevy::prelude::*;

/// Weapon attachment pickup data.
///
/// GKC reference: `weaponAttachmentPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponAttachmentPickup {
    pub attachment_id: String,
}

impl Default for WeaponAttachmentPickup {
    fn default() -> Self {
        Self {
            attachment_id: String::new(),
        }
    }
}

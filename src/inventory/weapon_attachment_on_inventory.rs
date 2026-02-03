use bevy::prelude::*;

/// Weapon attachment pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponAttachmentOnInventory {
    pub attachment_id: String,
}

impl Default for WeaponAttachmentOnInventory {
    fn default() -> Self {
        Self {
            attachment_id: String::new(),
        }
    }
}

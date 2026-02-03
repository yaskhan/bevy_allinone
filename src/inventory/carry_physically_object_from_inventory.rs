use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Spawns a physical item from inventory and attaches it to the carrier.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CarryPhysicallyObjectFromInventory {
    pub item: Option<InventoryItem>,
    pub carry: bool,
    pub attach_offset: Vec3,
}

impl Default for CarryPhysicallyObjectFromInventory {
    fn default() -> Self {
        Self {
            item: None,
            carry: false,
            attach_offset: Vec3::new(0.0, 1.0, 0.5),
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CarriedInventoryItem;

pub fn update_carry_physically_object_from_inventory(
    mut commands: Commands,
    carriers: Query<(Entity, &CarryPhysicallyObjectFromInventory, Option<&Children>)>,
    carried_items: Query<Entity, With<CarriedInventoryItem>>,
) {
    for (carrier, system, children) in carriers.iter() {
        let mut has_child = false;
        if let Some(children) = children {
            for child in children.iter() {
                if carried_items.get(*child).is_ok() {
                    has_child = true;
                    if !system.carry {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            }
        }

        if system.carry && !has_child {
            let Some(item) = system.item.clone() else { continue };
            commands.entity(carrier).with_children(|parent| {
                parent.spawn((
                    (
                        Transform::from_translation(system.attach_offset),
                        Visibility::default(),
                    ),
                    PhysicalItem { item },
                    CarriedInventoryItem,
                    Name::new("Carried Inventory Item"),
                ));
            });
        }
    }
}

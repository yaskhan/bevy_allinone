use bevy::prelude::*;

/// Sets the parent of an entity.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SetObjectParentSystem {
    pub target: Entity,
    pub parent: Entity,
}

impl Default for SetObjectParentSystem {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            parent: Entity::PLACEHOLDER,
        }
    }
}

pub fn update_set_object_parent_system(
    mut commands: Commands,
    query: Query<&SetObjectParentSystem>,
) {
    for set_parent in query.iter() {
        if set_parent.target == Entity::PLACEHOLDER || set_parent.parent == Entity::PLACEHOLDER {
            continue;
        }
        commands.entity(set_parent.target).set_parent(set_parent.parent);
    }
}

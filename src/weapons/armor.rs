use bevy::prelude::*;
use super::types::*;
use super::types::*;

#[derive(Debug, Clone, Copy, Event)]
pub struct ReturnProjectilesEvent {
    pub armor_entity: Entity,
    pub direction: Vec3,
}

#[derive(Resource, Default)]
pub struct ReturnProjectilesQueue(pub Vec<ReturnProjectilesEvent>);

/// System to handle returning "caught" projectiles from an ArmorSurface
pub fn handle_armor_projectile_return(
    mut events_queue: ResMut<ReturnProjectilesQueue>,
    mut armor_query: Query<&mut ArmorSurface>,
    mut projectile_query: Query<(&mut Projectile, &mut Transform, &CapturedProjectile, &mut Visibility)>,
    mut commands: Commands,
) {
    for event in events_queue.0.drain(..) {
        if let Ok(mut armor) = armor_query.get_mut(event.armor_entity) {
            let projectiles_to_return = std::mem::take(&mut armor.caught_projectiles);
            
            for proj_entity in projectiles_to_return {
                if let Ok((mut projectile, mut transform, _captured, mut visibility)) = projectile_query.get_mut(proj_entity) {
                    // Reset projectile state
                    projectile.owner = armor.owner.unwrap_or(proj_entity);
                    projectile.velocity = event.direction.normalize() * 15.0; // Standard return speed
                    
                    // Re-enable projectile
                    *visibility = Visibility::Visible;
                    
                    // Unparent and remove captured marker
                    commands.entity(proj_entity)
                        .remove::<ChildOf>()
                        .remove::<CapturedProjectile>();
                    
                    // Point toward direction
                    transform.look_to(projectile.velocity.normalize(), Vec3::Y);
                    
                    // Resume physics if using RigidBody (assuming it was set to kinematic)
                    // Note: Projectile system handles standard movement
                }
            }
        }
    }
}

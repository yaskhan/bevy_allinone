use bevy::prelude::*;
use super::types::VehicleIKTargets;

/// IK driving system for vehicle passengers.
///
/// GKC reference: `IKDrivingSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct IKDrivingSystem {
    pub enabled: bool,
    pub left_hand_target: Option<Entity>,
    pub right_hand_target: Option<Entity>,
    pub left_foot_target: Option<Entity>,
    pub right_foot_target: Option<Entity>,
}

impl Default for IKDrivingSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            left_hand_target: None,
            right_hand_target: None,
            left_foot_target: None,
            right_foot_target: None,
        }
    }
}

/// Apply IK target positions to target entities.
pub fn update_ik_driving(
    mut query: Query<(&IKDrivingSystem, &VehicleIKTargets)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (ik, targets) in query.iter_mut() {
        if !ik.enabled {
            continue;
        }

        if let (Some(entity), Some(pos)) = (ik.left_hand_target, targets.left_hand) {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation = pos;
            }
        }
        if let (Some(entity), Some(pos)) = (ik.right_hand_target, targets.right_hand) {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation = pos;
            }
        }
        if let (Some(entity), Some(pos)) = (ik.left_foot_target, targets.left_foot) {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation = pos;
            }
        }
        if let (Some(entity), Some(pos)) = (ik.right_foot_target, targets.right_foot) {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation = pos;
            }
        }
    }
}

use bevy::prelude::*;

/// Waypoints for hoverboard paths.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HoverBoardWaypoints {
    pub points: Vec<Vec3>,
    pub current_index: usize,
    pub loop_path: bool,
    pub reach_distance: f32,
}

impl Default for HoverBoardWaypoints {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            current_index: 0,
            loop_path: true,
            reach_distance: 1.0,
        }
    }
}

pub fn update_hoverboard_waypoints(
    mut query: Query<(&GlobalTransform, &mut HoverBoardWaypoints)>,
) {
    for (transform, mut waypoints) in query.iter_mut() {
        if waypoints.points.is_empty() {
            continue;
        }
        let current = waypoints.points[waypoints.current_index];
        if transform.translation().distance(current) <= waypoints.reach_distance {
            if waypoints.current_index + 1 < waypoints.points.len() {
                waypoints.current_index += 1;
            } else if waypoints.loop_path {
                waypoints.current_index = 0;
            }
        }
    }
}

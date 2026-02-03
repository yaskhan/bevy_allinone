use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use super::types::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiNavWaypoint {
    pub is_active: bool,
}

impl Default for AiNavWaypoint {
    fn default() -> Self {
        Self { is_active: true }
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AiPath {
    pub points: Vec<Vec3>,
    pub current_index: usize,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiNavigationSettings {
    pub use_pathfinding: bool,
    pub waypoint_connection_radius: f32,
    pub repath_interval: f32,
    pub last_repath_time: f32,
    pub accept_partial_path: bool,
}

impl Default for AiNavigationSettings {
    fn default() -> Self {
        Self {
            use_pathfinding: false,
            waypoint_connection_radius: 10.0,
            repath_interval: 1.0,
            last_repath_time: -999.0,
            accept_partial_path: true,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueNode {
    cost: u32,
    node: Entity,
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn update_ai_navigation(
    time: Res<Time>,
    waypoint_query: Query<(Entity, &GlobalTransform, &AiNavWaypoint)>,
    mut ai_query: Query<(Entity, &GlobalTransform, &mut AiMovement, &mut AiNavigationSettings, Option<&mut AiPath>)>,
    mut commands: Commands,
) {
    let now = time.elapsed_secs();
    let waypoints: Vec<(Entity, Vec3)> = waypoint_query
        .iter()
        .filter(|(_, _, w)| w.is_active)
        .map(|(e, gt, _)| (e, gt.translation()))
        .collect();

    if waypoints.is_empty() {
        return;
    }

    for (entity, transform, mut movement, mut nav_settings, path_opt) in ai_query.iter_mut() {
        if !nav_settings.use_pathfinding {
            continue;
        }

        let Some(destination) = movement.destination else { continue };
        if now - nav_settings.last_repath_time < nav_settings.repath_interval {
            continue;
        }

        let start = find_closest_waypoint(transform.translation(), &waypoints);
        let goal = find_closest_waypoint(destination, &waypoints);
        nav_settings.last_repath_time = now;

        let Some((start_ent, _)) = start else { continue };
        let Some((goal_ent, _)) = goal else { continue };

        let path = compute_path(start_ent, goal_ent, &waypoints, nav_settings.waypoint_connection_radius);
        if path.is_empty() {
            if nav_settings.accept_partial_path {
                continue;
            }
            movement.destination = None;
            continue;
        }

        let mut points = Vec::new();
        for node in path {
            if let Some((_e, pos)) = waypoints.iter().find(|(e, _)| *e == node) {
                points.push(*pos);
            }
        }
        points.push(destination);

        if let Some(mut path_component) = path_opt {
            path_component.points = points;
            path_component.current_index = 0;
        } else {
            movement.destination = points.first().copied();
            commands.entity(entity).insert(AiPath {
                points,
                current_index: 0,
            });
        }
    }
}

fn find_closest_waypoint(
    position: Vec3,
    waypoints: &[(Entity, Vec3)],
) -> Option<(Entity, Vec3)> {
    let mut best = None;
    let mut best_dist = f32::MAX;
    for (entity, pos) in waypoints.iter() {
        let dist = position.distance(*pos);
        if dist < best_dist {
            best_dist = dist;
            best = Some((*entity, *pos));
        }
    }
    best
}

fn compute_path(
    start: Entity,
    goal: Entity,
    waypoints: &[(Entity, Vec3)],
    radius: f32,
) -> Vec<Entity> {
    let mut frontier = BinaryHeap::new();
    let mut came_from: HashMap<Entity, Entity> = HashMap::new();
    let mut cost_so_far: HashMap<Entity, u32> = HashMap::new();

    frontier.push(QueueNode { cost: 0, node: start });
    cost_so_far.insert(start, 0);

    while let Some(QueueNode { node, .. }) = frontier.pop() {
        if node == goal {
            break;
        }

        let current_pos = match waypoints.iter().find(|(e, _)| *e == node) {
            Some((_, pos)) => *pos,
            None => continue,
        };

        for (neighbor, neighbor_pos) in waypoints.iter() {
            if *neighbor == node {
                continue;
            }
            if current_pos.distance(*neighbor_pos) > radius {
                continue;
            }
            let new_cost = cost_so_far.get(&node).copied().unwrap_or(0) + 1;
            if cost_so_far.get(neighbor).map_or(true, |&c| new_cost < c) {
                cost_so_far.insert(*neighbor, new_cost);
                let priority = new_cost + heuristic(*neighbor_pos, goal, waypoints);
                frontier.push(QueueNode { cost: priority, node: *neighbor });
                came_from.insert(*neighbor, node);
            }
        }
    }

    if !came_from.contains_key(&goal) {
        return Vec::new();
    }

    let mut current = goal;
    let mut path = vec![current];
    while current != start {
        if let Some(prev) = came_from.get(&current).copied() {
            current = prev;
            path.push(current);
        } else {
            break;
        }
    }
    path.reverse();
    path
}

fn heuristic(pos: Vec3, goal: Entity, waypoints: &[(Entity, Vec3)]) -> u32 {
    let goal_pos = waypoints
        .iter()
        .find(|(e, _)| *e == goal)
        .map(|(_, pos)| *pos)
        .unwrap_or(pos);
    pos.distance(goal_pos) as u32
}

//! AI system module
//!
//! NPC behavior, pathfinding, and AI controllers.

use bevy::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_ai_behavior,
            update_ai_patrol,
        ));
    }
}

/// AI controller component
/// TODO: Implement AI navigation system
#[derive(Component, Debug)]
pub struct AiController {
    pub target: Option<Entity>,
    pub patrol_path: Vec<Vec3>,
    pub current_waypoint: usize,
    pub detection_range: f32,
    pub attack_range: f32,
}

/// AI behavior state
/// TODO: Implement AI behavior system
#[derive(Debug, Clone, Copy)]
pub enum AiBehaviorState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
}

fn update_ai_behavior(/* TODO */) {}
fn update_ai_patrol(/* TODO */) {}

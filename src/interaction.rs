//! Interaction system module
//!
//! Object interaction, pickups, and usable devices.

use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            detect_interactables,
            process_interactions,
        ));
    }
}

/// Interactable component
/// TODO: Implement interaction system
#[derive(Component, Debug)]
pub struct Interactable {
    pub interaction_text: String,
    pub interaction_distance: f32,
    pub can_interact: bool,
    pub interaction_type: InteractionType,
}

/// Interaction type
#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    Pickup,
    Use,
    Talk,
    Open,
    Activate,
}

fn detect_interactables(/* TODO */) {}
fn process_interactions(/* TODO */) {}

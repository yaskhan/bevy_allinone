//! Vehicles system module
//!
//! Vehicle physics, controls, and passenger management.

use bevy::prelude::*;

pub struct VehiclesPlugin;

impl Plugin for VehiclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_vehicles);
    }
}

/// Vehicle component
/// TODO: Implement vehicle system
#[derive(Component, Debug)]
pub struct Vehicle {
    pub vehicle_name: String,
    pub max_speed: f32,
    pub acceleration: f32,
    pub steering_speed: f32,
    pub seats: Vec<VehicleSeat>,
}

/// Vehicle seat
#[derive(Debug, Clone)]
pub struct VehicleSeat {
    pub seat_index: usize,
    pub is_driver_seat: bool,
    pub occupied_by: Option<Entity>,
}

fn update_vehicles(/* TODO */) {}

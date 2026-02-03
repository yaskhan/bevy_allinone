use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct RequestSaveEvent {
    pub slot: usize,
}

#[derive(Debug, Clone, Event)]
pub struct RequestLoadEvent {
    pub slot: usize,
}


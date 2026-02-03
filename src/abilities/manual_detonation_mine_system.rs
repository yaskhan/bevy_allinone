use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct PlaceMineEvent {
    pub owner: Entity,
}

#[derive(Resource, Default)]
pub struct PlaceMineEventQueue(pub Vec<PlaceMineEvent>);

/// Mine object that can be activated manually.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ManualDetonationMineObject {
    pub active: bool,
}

impl Default for ManualDetonationMineObject {
    fn default() -> Self {
        Self { active: false }
    }
}

/// Manual detonation mine system.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ManualDetonationMineSystem {
    pub min_delay_to_activate_mine: f32,
    pub current_mine: Option<Entity>,
    pub last_time_mine_placed: f32,
}

impl Default for ManualDetonationMineSystem {
    fn default() -> Self {
        Self {
            min_delay_to_activate_mine: 0.5,
            current_mine: None,
            last_time_mine_placed: 0.0,
        }
    }
}

pub fn place_new_mine(
    time: Res<Time>,
    mut events: ResMut<PlaceMineEventQueue>,
    mut query: Query<(Entity, &mut ManualDetonationMineSystem)>,
) {
    for (entity, mut system) in query.iter_mut() {
        if system.current_mine.is_none() {
            events.0.push(PlaceMineEvent { owner: entity });
            system.last_time_mine_placed = time.elapsed_secs();
        }
    }
}

pub fn set_current_mine(
    mut query: Query<&mut ManualDetonationMineSystem>,
    mut mine_query: Query<&mut ManualDetonationMineObject>,
    mut events: ResMut<PlaceMineEventQueue>,
    time: Res<Time>,
) {
    for event in events.0.drain(..) {
        if let Ok(mut system) = query.get_mut(event.owner) {
            if let Some(mine_entity) = system.current_mine {
                if let Ok(mut mine) = mine_query.get_mut(mine_entity) {
                    mine.active = false;
                }
            }
            system.current_mine = None;
            system.last_time_mine_placed = time.elapsed_secs();
        }
    }
}

pub fn activate_current_mine(
    time: Res<Time>,
    mut query: Query<&mut ManualDetonationMineSystem>,
    mut mine_query: Query<&mut ManualDetonationMineObject>,
) {
    for mut system in query.iter_mut() {
        if let Some(mine_entity) = system.current_mine {
            if time.elapsed_secs() > system.min_delay_to_activate_mine + system.last_time_mine_placed {
                if let Ok(mut mine) = mine_query.get_mut(mine_entity) {
                    mine.active = true;
                }
                system.current_mine = None;
            }
        }
    }
}

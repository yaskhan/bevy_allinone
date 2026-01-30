use bevy::prelude::*;
use super::types::*;

pub fn update_play_time(
    time: Res<Time>,
    mut settings: ResMut<GameManagerSettings>,
    state: Res<State<GameState>>,
) {
    if *state == GameState::Playing {
        settings.play_time += time.delta_secs();
    }
}

pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn switch_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_manager: ResMut<PlayerManager>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if !player_manager.players.is_empty() {
            player_manager.current_player_index = 
                (player_manager.current_player_index + 1) % player_manager.players.len();
            
            info!("Switched to player index: {}", player_manager.current_player_index);
        }
    }
}

/// System to spawn a prefab from the registry
pub fn spawn_prefab(
    name: &str,
    position: Vec3,
    registry: &PrefabRegistry,
    commands: &mut Commands,
) {
    if let Some(scene_handle) = registry.prefabs.get(name) {
        commands.spawn((
            SceneRoot(scene_handle.clone()),
            Transform::from_translation(position),
        ));
    } else {
        warn!("Prefab '{}' not found in registry", name);
    }
}

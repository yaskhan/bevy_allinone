use bevy::prelude::*;
use super::types::*;
use crate::character::Player;
use crate::ai::{AiController, AiBehaviorState};
use crate::camera::types::CameraController;

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

pub fn switch_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_manager: ResMut<PlayerManager>,
    mut switch_queue: ResMut<SwitchPlayerQueue>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if !player_manager.players.is_empty() {
            let next_index = (player_manager.current_player_index + 1) % player_manager.players.len();
            switch_queue.0.push(SwitchPlayerEvent {
                target_index: Some(next_index),
                target_entity: None,
            });
        }
    }
}

pub fn handle_switch_player(
    mut commands: Commands,
    mut player_manager: ResMut<PlayerManager>,
    mut switch_queue: ResMut<SwitchPlayerQueue>,
    mut camera_query: Query<&mut CameraController>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in switch_queue.0.drain(..) {
        if player_manager.players.is_empty() {
            continue;
        }

        let target_entity = if let Some(entity) = event.target_entity {
            Some(entity)
        } else if let Some(index) = event.target_index {
            player_manager.players.get(index).copied()
        } else {
            None
        };

        let Some(new_player) = target_entity else { continue };

        let current_player = player_manager
            .get_current_player()
            .or_else(|| player_query.iter().next());

        if current_player == Some(new_player) {
            continue;
        }

        if let Some(old_player) = current_player {
            commands.entity(old_player).remove::<Player>();
            if player_manager.enable_ai_on_inactive {
                commands.entity(old_player).insert(AiController {
                    state: AiBehaviorState::Follow,
                    target: Some(new_player),
                    ..default()
                });
            }
        }

        commands.entity(new_player).insert(Player);
        commands.entity(new_player).remove::<AiController>();

        if let Some(index) = player_manager.players.iter().position(|p| *p == new_player) {
            player_manager.current_player_index = index;
        }

        for mut camera in camera_query.iter_mut() {
            camera.follow_target = Some(new_player);
        }

        info!("Switched active player to {:?}", new_player);
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

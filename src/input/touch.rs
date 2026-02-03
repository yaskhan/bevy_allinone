use bevy::prelude::*;
use super::components::InputState;
use super::types::InputAction;

#[derive(Component)]
pub struct TouchControlRoot;

#[derive(Component)]
pub struct TouchActionButton {
    pub action: InputAction,
    pub pressed: bool,
}

#[derive(Component)]
pub struct TouchJoystick {
    pub radius: f32,
    pub active_touch: Option<u64>,
}

#[derive(Component)]
pub struct TouchJoystickThumb;

#[derive(Resource, Debug, Clone)]
pub struct TouchControlsSettings {
    pub enabled: bool,
    pub hide_when_no_touch: bool,
}

impl Default for TouchControlsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            hide_when_no_touch: true,
        }
    }
}

pub fn update_touch_controls_visibility(
    touches: Res<Touches>,
    settings: Res<TouchControlsSettings>,
    mut query: Query<&mut Visibility, With<TouchControlRoot>>,
) {
    if !settings.enabled {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        return;
    }

    if settings.hide_when_no_touch && touches.iter().next().is_none() {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    } else {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

pub fn update_touch_buttons(
    mut input_state: ResMut<InputState>,
    mut query: Query<(&Interaction, &mut TouchActionButton), Changed<Interaction>>,
) {
    for (interaction, mut button) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                if !button.pressed {
                    apply_touch_action(&mut input_state, button.action, true, true);
                }
                button.pressed = true;
            }
            Interaction::Hovered => {
                // Keep pressed state if finger stays over button.
            }
            Interaction::None => {
                if button.pressed {
                    apply_touch_action(&mut input_state, button.action, false, false);
                }
                button.pressed = false;
            }
        }
    }
}

pub fn update_touch_joystick(
    mut input_state: ResMut<InputState>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut joystick_query: Query<(&GlobalTransform, &mut TouchJoystick, Option<&Children>)>,
    mut thumb_query: Query<&mut Style, With<TouchJoystickThumb>>,
) {
    let Ok(window) = windows.get_single() else { return };
    let window_size = Vec2::new(window.width(), window.height());

    for (transform, mut joystick, children) in joystick_query.iter_mut() {
        let size = Vec2::splat(joystick.radius * 2.0);
        let rect = Rect::from_center_size(transform.translation().truncate(), size);

        if joystick.active_touch.is_none() {
            for touch in touches.iter() {
                let pos = touch.position();
                let world_pos = Vec2::new(
                    pos.x - window_size.x * 0.5,
                    pos.y - window_size.y * 0.5,
                );
                if rect.contains(world_pos) {
                    joystick.active_touch = Some(touch.id());
                }
            }
        }

        if let Some(touch_id) = joystick.active_touch {
            if let Some(touch) = touches.get(touch_id) {
                let pos = touch.position();
                let world_pos = Vec2::new(
                    pos.x - window_size.x * 0.5,
                    pos.y - window_size.y * 0.5,
                );
                let center = rect.center();
                let mut delta = world_pos - center;
                let max = joystick.radius.max(1.0);
                delta = delta.clamp_length_max(max);
                input_state.movement = (delta / max).clamp_length_max(1.0);

                if let Some(children) = children {
                    for child in children.iter() {
                        if let Ok(mut style) = thumb_query.get_mut(*child) {
                            style.left = Val::Px(delta.x);
                            style.bottom = Val::Px(delta.y);
                        }
                    }
                }
            } else {
                joystick.active_touch = None;
                input_state.movement = Vec2::ZERO;
                if let Some(children) = children {
                    for child in children.iter() {
                        if let Ok(mut style) = thumb_query.get_mut(*child) {
                            style.left = Val::Px(0.0);
                            style.bottom = Val::Px(0.0);
                        }
                    }
                }
            }
        }
    }
}

fn apply_touch_action(
    input_state: &mut InputState,
    action: InputAction,
    pressed: bool,
    just_pressed: bool,
) {
    match action {
        InputAction::Jump => input_state.jump_pressed = just_pressed,
        InputAction::Interact => input_state.interact_pressed = just_pressed,
        InputAction::Attack => input_state.attack_pressed = just_pressed,
        InputAction::Block => input_state.block_pressed = pressed,
        InputAction::Aim => input_state.aim_pressed = pressed,
        InputAction::Fire => {
            input_state.fire_pressed = pressed;
            input_state.fire_just_pressed = just_pressed;
        }
        InputAction::Reload => input_state.reload_pressed = just_pressed,
        InputAction::Crouch => input_state.crouch_pressed = pressed,
        InputAction::Sprint => input_state.sprint_pressed = pressed,
        InputAction::AbilityUse => {
            input_state.ability_use_pressed = just_pressed;
            input_state.ability_use_held = pressed;
            input_state.ability_use_released = !pressed;
        }
        _ => {}
    }
}

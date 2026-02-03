use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::types::CameraTargetState;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LockOnReticleRoot;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LockOnReticleIcon;

pub fn setup_lock_on_reticle_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            Visibility::Hidden,
            LockOnReticleRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("X"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
                TextLayout::default(),
                LockOnReticleIcon,
            ));
        });
}

pub fn update_lock_on_reticle_ui(
    camera_query: Query<(&GlobalTransform, &Camera, &CameraTargetState)>,
    target_query: Query<&GlobalTransform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ui_query: Query<(&mut Node, &mut Visibility), With<LockOnReticleRoot>>,
) {
    let Ok((camera_gt, camera, target_state)) = camera_query.get_single() else {
        return;
    };
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((mut node, mut visibility)) = ui_query.get_single_mut() else {
        return;
    };

    let Some(target_entity) = target_state.locked_target else {
        *visibility = Visibility::Hidden;
        return;
    };

    let Ok(target_gt) = target_query.get(target_entity) else {
        *visibility = Visibility::Hidden;
        return;
    };

    if let Ok(viewport_pos) = camera.world_to_viewport(camera_gt, target_gt.translation()) {
        let size = 24.0;
        let left = viewport_pos.x - size * 0.5;
        let top = window.height() - viewport_pos.y - size * 0.5;

        node.left = Val::Px(left);
        node.top = Val::Px(top);
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}

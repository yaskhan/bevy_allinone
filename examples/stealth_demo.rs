//! Stealth System Demo
//!
//! This example demonstrates the stealth/hide system functionality.
//! Press H to toggle hide state, P to peek, C to corner lean.

use bevy::prelude::*;
use bevy_allinone::prelude::*;
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_plugins(PhysicsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_demo_input,
            update_demo_ui,
            update_camera,
        ))
        .run();
}

/// Setup the demo scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("DemoCamera"),
    ));

    // Light
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(5.0, 10.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("DemoLight"),
    ));

    // Ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new("Ground"),
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
    ));

    // Cover object (wall)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0, 1.5, 0.2).into()),
            material: materials.add(Color::rgb(0.6, 0.4, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.75, 2.0),
            ..default()
        },
        Name::new("CoverWall"),
        RigidBody::Static,
        Collider::cuboid(2.0, 1.5, 0.2),
        // Tag this as a cover object
        CoverObjectTag,
    ));

    // Another cover object (box)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            material: materials.add(Color::rgb(0.5, 0.3, 0.1).into()),
            transform: Transform::from_xyz(3.0, 0.5, 1.0),
            ..default()
        },
        Name::new("CoverBox"),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        CoverObjectTag,
    ));

    // Player character
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule::new(0.3, 0.5).into()),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 1.0, 5.0),
            ..default()
        },
        Name::new("Player"),
        Player,
        CharacterController {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            crouch_speed: 2.5,
            ..default()
        },
        CharacterMovementState::default(),
        CharacterAnimationState::default(),
        StealthController {
            character_need_to_crouch: true,
            character_cant_move: false,
            max_move_amount: 0.1,
            check_if_detected_while_hidden: true,
            ..default()
        },
        StealthState::default(),
        CoverDetection::default(),
        VisibilityMeter::default(),
        InputState::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.3, 0.5),
        GravityScale(1.0),
        GroundDetection::default(),
        GroundDetectionSettings::default(),
        CustomGravity::default(),
    ));

    // AI enemy (simple visual indicator)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Sphere::new(0.3).into()),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(5.0, 0.5, 0.0),
            ..default()
        },
        Name::new("Enemy"),
        AIDetection {
            detection_range: 8.0,
            ..default()
        },
    ));

    // UI Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Stealth System Demo\n",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Controls:\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "WASD - Move\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "H - Toggle Hide\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "P - Toggle Peek\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "C - Toggle Corner Lean\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "R - Reset Camera\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Status:\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "Hidden: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "No\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "Detected: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "No\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "Visibility: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0%\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "Detection: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0%\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "Sound: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0%\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "Cover: ",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "None\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        Name::new("DemoUI"),
    ));

    info!("Stealth Demo started!");
    info!("Press H to toggle hide, P to peek, C to corner lean, R to reset camera");
}

/// Tag component for cover objects
#[derive(Component)]
struct CoverObjectTag;

/// Handle demo input
fn handle_demo_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut CharacterMovementState,
        &mut StealthState,
        &StealthController,
        &mut Transform,
    )>,
) {
    for (mut movement, mut state, stealth, mut transform) in query.iter_mut() {
        // Handle movement
        let mut move_dir = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::W) {
            move_dir.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_dir.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_dir.x += 1.0;
        }
        
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
            
            // Apply movement based on hide state
            let speed = if state.is_hidden {
                stealth.max_move_amount * 10.0 // Slower when hiding
            } else {
                4.0 // Normal speed
            };
            
            transform.translation += move_dir * speed * 0.016; // Approximate delta time
            movement.raw_move_dir = move_dir;
        } else {
            movement.raw_move_dir = Vec3::ZERO;
        }
        
        // Handle hide toggle
        if keyboard_input.just_pressed(KeyCode::H) {
            api::toggle_hide(stealth, &mut state, &mut movement);
            info!("Hide toggled: {}", state.is_hidden);
        }
        
        // Handle peek toggle
        if keyboard_input.just_pressed(KeyCode::P) {
            api::toggle_peek(&mut state);
            info!("Peek toggled: {}", state.is_peeking);
        }
        
        // Handle corner lean toggle
        if keyboard_input.just_pressed(KeyCode::C) {
            api::toggle_corner_lean(&mut state);
            info!("Corner lean toggled: {}", state.is_corner_leaning);
        }
        
        // Handle camera reset
        if keyboard_input.just_pressed(KeyCode::R) {
            api::reset_camera_external(&mut state);
            info!("Camera reset");
        }
    }
}

/// Update demo UI
fn update_demo_ui(
    query: Query<(&StealthState, &VisibilityMeter, &CoverDetection), With<Player>>,
    mut ui_query: Query<&mut Text, Without<Player>>,
) {
    for (state, visibility, cover) in query.iter() {
        for mut text in ui_query.iter_mut() {
            let sections = &mut text.sections;
            
            if sections.len() >= 22 {
                // Update hidden status
                sections[10].value = if state.is_hidden { "Yes\n" } else { "No\n" };
                sections[10].style.color = if state.is_hidden { Color::GREEN } else { Color::RED };
                
                // Update detected status
                sections[12].value = if state.is_detected { "Yes\n" } else { "No\n" };
                sections[12].style.color = if state.is_detected { Color::RED } else { Color::GREEN };
                
                // Update visibility
                let vis_percent = (visibility.current_visibility * 100.0) as u32;
                sections[14].value = format!("{}%\n", vis_percent);
                sections[14].style.color = if visibility.current_visibility > 0.5 {
                    Color::RED
                } else if visibility.current_visibility > 0.2 {
                    Color::YELLOW
                } else {
                    Color::GREEN
                };
                
                // Update detection
                let det_percent = (visibility.detection_level * 100.0) as u32;
                sections[16].value = format!("{}%\n", det_percent);
                sections[16].style.color = if visibility.detection_level > 0.5 {
                    Color::RED
                } else if visibility.detection_level > 0.2 {
                    Color::YELLOW
                } else {
                    Color::GREEN
                };
                
                // Update sound
                let sound_percent = (visibility.sound_level * 100.0) as u32;
                sections[18].value = format!("{}%\n", sound_percent);
                sections[18].style.color = if visibility.sound_level > 0.5 {
                    Color::RED
                } else if visibility.sound_level > 0.2 {
                    Color::YELLOW
                } else {
                    Color::GREEN
                };
                
                // Update cover status
                let cover_text = if cover.is_in_cover {
                    match cover.cover_type {
                        CoverType::Low => "Low Cover",
                        CoverType::Medium => "Medium Cover",
                        CoverType::High => "High Cover",
                        CoverType::Corner => "Corner",
                        CoverType::Full => "Full Cover",
                    }
                } else {
                    "None"
                };
                sections[20].value = format!("{}\n", cover_text);
                sections[20].style.color = if cover.is_in_cover {
                    Color::GREEN
                } else {
                    Color::WHITE
                };
            }
        }
    }
}

/// Update camera based on stealth state
fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    player_query: Query<(&Transform, &StealthState), With<Player>>,
) {
    for (player_transform, stealth_state) in player_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            if stealth_state.is_hidden && stealth_state.camera_is_free {
                // Camera follows player with offset based on hide state
                let offset = match stealth_state.hide_state {
                    HideState::Peek => Vec3::new(0.0, 1.5, 2.0),
                    HideState::CornerLean => Vec3::new(1.0, 1.0, 2.0),
                    HideState::CrouchHide => Vec3::new(0.0, 0.8, 1.5),
                    HideState::ProneHide => Vec3::new(0.0, 0.3, 1.0),
                    _ => Vec3::new(0.0, 1.0, 2.0),
                };
                
                let target_position = player_transform.translation + offset;
                camera_transform.translation = camera_transform.translation.lerp(
                    target_position,
                    0.1,
                );
                
                // Look at player
                let look_target = player_transform.translation + Vec3::Y * 1.0;
                camera_transform.look_at(look_target, Vec3::Y);
            } else {
                // Normal third-person camera
                let target_position = player_transform.translation + Vec3::new(0.0, 5.0, 10.0);
                camera_transform.translation = camera_transform.translation.lerp(
                    target_position,
                    0.1,
                );
                
                let look_target = player_transform.translation;
                camera_transform.look_at(look_target, Vec3::Y);
            }
        }
    }
}

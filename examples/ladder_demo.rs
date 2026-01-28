//! # Ladder System Demo
//!
//! This example demonstrates the ladder system functionality.
//!
//! ## Controls
//!
//! - **W/A/S/D**: Move character
//! - **Space**: Jump
//! - **Shift**: Sprint
//! - **C**: Crouch
//! - **E**: Interact (mount ladder when near)
//! - **Mouse**: Look around
//!
//! ## Features Demonstrated
//!
//! - Ladder detection and mounting
//! - Climbing up/down ladders
//! - Horizontal movement on ladders
//! - Ladder exit detection
//! - Camera lock on ladder
//! - Footstep sounds while climbing
//! - Ladder types (wooden, metal, rope)

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            spawn_ladders,
            update_ladder_demo_ui,
            handle_ladder_demo_input,
        ))
        .run();
}

/// Setup the demo scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));

    // Spawn player with character controller and ladder system
    commands.spawn((
        Player,
        CharacterController {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            crouch_speed: 2.5,
            jump_power: 8.0,
            jump_hold_bonus: 0.5,
            max_jump_hold_time: 0.3,
            can_move: true,
            is_dead: false,
            is_strafing: false,
            acceleration: 10.0,
            deceleration: 15.0,
            fall_damage_enabled: true,
            min_velocity_for_damage: 15.0,
            falling_damage_multiplier: 1.0,
            crouch_sliding_enabled: true,
            crouch_sliding_speed: 12.0,
            crouch_sliding_duration: 0.5,
            obstacle_detection_distance: 0.5,
            fixed_axis: None,
            use_root_motion: false,
            zero_gravity_mode: false,
            free_floating_mode: false,
        },
        PlayerLadderSystem {
            ladder_found: false,
            ladder_movement_speed: 5.0,
            ladder_vertical_movement_amount: 0.3,
            ladder_horizontal_movement_amount: 0.1,
            min_angle_to_inverse_direction: 100.0,
            use_always_horizontal_movement_on_ladder: false,
            use_always_local_movement_direction: false,
            min_angle_vertical_direction: 60.0,
            max_angle_vertical_direction: 120.0,
            climb_ladder_foot_step_state_name: "Climb Ladders".to_string(),
            ladder_end_detected: false,
            ladder_start_detected: false,
            movement_direction: 1,
            ladder_vertical_input: 0.0,
            ladder_horizontal_input: 0.0,
            ladder_angle: 0.0,
            ladder_signed_angle: 0.0,
            current_vertical_input: 0.0,
            current_horizontal_input: 0.0,
            ladder_movement_direction: Vec3::ZERO,
            moving_on_ladder: false,
            moving_on_ladder_previously: false,
            ladder_direction_transform: None,
            ladder_raycast_direction_transform: None,
            use_events_on_third_person: false,
            use_ladder_horizontal_movement: false,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            current_ladder_system: None,
            previous_ladder_system: None,
        },
        LadderMovement {
            is_active: false,
            movement_direction: Vec3::ZERO,
            vertical_input: 0.0,
            horizontal_input: 0.0,
            movement_speed: 5.0,
            vertical_movement_amount: 0.3,
            horizontal_movement_amount: 0.1,
            move_in_ladder_center: false,
            use_horizontal_movement: false,
            use_local_direction: false,
            min_angle_vertical: 60.0,
            max_angle_vertical: 120.0,
            min_angle_to_inverse: 100.0,
        },
        LadderMovementTracker {
            current_state: LadderMovementState::None,
            previous_state: LadderMovementState::None,
            state_timer: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            climb_speed: 5.0,
            horizontal_climb_speed: 3.0,
        },
        LadderAnimation {
            is_mounting: false,
            is_dismounting: false,
            mount_progress: 0.0,
            dismount_progress: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            mount_start_position: Vec3::ZERO,
            mount_target_position: Vec3::ZERO,
            dismount_start_position: Vec3::ZERO,
            dismount_target_position: Vec3::ZERO,
        },
        LadderExitDetection {
            end_detected: false,
            start_detected: false,
            end_check_distance: 2.0,
            start_check_distance: 0.13,
            end_check_offset: Vec3::ZERO,
            start_check_offset: Vec3::new(0.0, 0.1, 0.0),
            layer_mask: 1,
        },
        LadderFootstep {
            climb_state_name: "Climb Ladders".to_string(),
            is_climbing: false,
            step_interval: 0.5,
            step_timer: 0.0,
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.8),
        Velocity::default(),
        ExternalForce::default(),
        GroundDetection::default(),
        CustomGravity::default(),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Spawn ground
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 0.5, 20.0),
        Transform::from_xyz(0.0, -0.25, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Spawn walls
    // Front wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 5.0, 0.5),
        Transform::from_xyz(0.0, 2.5, -5.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.5, 5.0, 10.0),
        Transform::from_xyz(-5.0, 2.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.5, 5.0, 10.0),
        Transform::from_xyz(5.0, 2.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Back wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 5.0, 0.5),
        Transform::from_xyz(0.0, 2.5, 5.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Spawn a wooden ladder
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.2, 4.0, 0.1),
        Transform::from_xyz(0.0, 2.0, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        LadderSystem {
            tag_to_check: "Player".to_string(),
            ladder_active: true,
            use_ladder_horizontal_movement: true,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            use_events_enter_exit_ladder: false,
            show_gizmo: true,
            gizmo_color: Color::BROWN,
            gizmo_length: 4.0,
            current_player: None,
        },
        LadderDirection {
            direction: Vec3::Y,
            raycast_direction: Vec3::Y,
        },
    ));

    // Spawn a metal ladder (wider, for horizontal movement)
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.3, 6.0, 0.1),
        Transform::from_xyz(3.0, 3.0, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        LadderSystem {
            tag_to_check: "Player".to_string(),
            ladder_active: true,
            use_ladder_horizontal_movement: true,
            move_in_ladder_center: true,
            use_local_movement_direction: true,
            use_events_enter_exit_ladder: false,
            show_gizmo: true,
            gizmo_color: Color::SILVER,
            gizmo_length: 4.0,
            current_player: None,
        },
        LadderDirection {
            direction: Vec3::Y,
            raycast_direction: Vec3::Y,
        },
    ));

    // Spawn a rope ladder (narrow, for vertical only)
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.1, 5.0, 0.1),
        Transform::from_xyz(-3.0, 2.5, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        LadderSystem {
            tag_to_check: "Player".to_string(),
            ladder_active: true,
            use_ladder_horizontal_movement: false,
            move_in_ladder_center: true,
            use_local_movement_direction: false,
            use_events_enter_exit_ladder: false,
            show_gizmo: true,
            gizmo_color: Color::YELLOW,
            gizmo_length: 4.0,
            current_player: None,
        },
        LadderDirection {
            direction: Vec3::Y,
            raycast_direction: Vec3::Y,
        },
    ));

    // Spawn platforms for ladder exits
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(4.0, 0.5, 4.0),
        Transform::from_xyz(0.0, 6.0, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(4.0, 0.5, 4.0),
        Transform::from_xyz(3.0, 7.0, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(4.0, 0.5, 4.0),
        Transform::from_xyz(-3.0, 6.5, -4.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // Spawn lights
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(5.0, 10.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Spawn UI for demo info
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Ladder System Demo\n\n",
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
                "W/A/S/D - Move\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Space - Jump\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Shift - Sprint\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "C - Crouch\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "E - Mount Ladder (when near)\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Mouse - Look around\n\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "State: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "None\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "\nOn Ladder: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "No\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "\nMoving: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "No\n",
                TextStyle {
                    font_size: 18.0,
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
        DemoUI,
    ));
}

/// Component to mark demo UI
#[derive(Component)]
struct DemoUI;

/// Spawn additional ladders for demo
fn spawn_ladders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<LadderSystem>>,
) {
    // Only spawn once
    if query.iter().count() > 0 {
        return;
    }

    // Spawn decorative elements
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.4, 0.2),
            ..default()
        }),
        transform: Transform::from_xyz(-3.0, 1.5, -2.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.8, 0.4),
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 1.5, -2.0),
        ..default()
    });
}

/// Update demo UI with ladder state
fn update_ladder_demo_ui(
    query: Query<(&PlayerLadderSystem, &LadderMovementTracker), With<Player>>,
    mut ui_query: Query<&mut Text, With<DemoUI>>,
) {
    for (ladder_system, ladder_tracker) in query.iter() {
        for mut text in ui_query.iter_mut() {
            // Update state display
            let state_str = match ladder_tracker.current_state {
                LadderMovementState::None => "None",
                LadderMovementState::Approaching => "Approaching",
                LadderMovementState::Mounting => "Mounting",
                LadderMovementState::ClimbingUp => "Climbing Up",
                LadderMovementState::ClimbingDown => "Climbing Down",
                LadderMovementState::ClimbingHorizontal => "Climbing Horizontal",
                LadderMovementState::Dismounting => "Dismounting",
            };

            // Update on ladder display
            let on_ladder_str = if ladder_system.ladder_found { "Yes" } else { "No" };
            let moving_str = if ladder_system.moving_on_ladder { "Yes" } else { "No" };

            // Update text sections
            if text.sections.len() >= 14 {
                text.sections[9].value = format!("{}\n", state_str);
                text.sections[11].value = format!("{}\n", on_ladder_str);
                text.sections[13].value = format!("{}\n", moving_str);
            }
        }
    }
}

/// Handle demo-specific input for ladder
fn handle_ladder_demo_input(
    input_state: Res<InputState>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        character,
        transform,
    ) in query.iter_mut() {
        // Handle mount ladder input (E key)
        if input_state.is_action_just_pressed(InputAction::Interact) {
            // Try to mount ladder
            if ladder_system.ladder_found &&
               !ladder_movement.is_active &&
               !character.is_dead &&
               !character.zero_gravity_mode &&
               !character.free_floating_mode {
                
                // Start mounting
                ladder_movement.is_active = true;
                ladder_tracker.current_state = LadderMovementState::Mounting;
                ladder_tracker.state_timer = 0.0;

                // Set up animation
                ladder_animation.is_mounting = true;
                ladder_animation.mount_progress = 0.0;
                ladder_animation.mount_start_position = transform.translation;
                ladder_animation.mount_target_position = transform.translation + Vec3::Y * 0.5;
            }
        }

        // Handle dismount (jump)
        if input_state.jump_pressed {
            if ladder_system.ladder_found && ladder_movement.is_active {
                // Start dismounting
                ladder_tracker.current_state = LadderMovementState::Dismounting;
                ladder_tracker.state_timer = 0.0;

                // Set up animation
                ladder_animation.is_dismounting = true;
                ladder_animation.dismount_progress = 0.0;
                ladder_animation.dismount_start_position = transform.translation;
                ladder_animation.dismount_target_position = transform.translation + Vec3::Y * -0.5;
            }
        }
    }
}

//! # Climb System Demo
//!
//! This example demonstrates the climb system functionality.
//!
//! ## Controls
//!
//! - **W/A/S/D**: Move character
//! - **Space**: Jump
//! - **Shift**: Sprint
//! - **C**: Crouch
//! - **E**: Interact (grab ledge when near climbable surface)
//! - **Mouse**: Look around
//!
//! ## Features Demonstrated
//!
//! - Wall detection and ledge grabbing
//! - Climbing up/down ledges
//! - Hanging from ledges
//! - Jumping from ledges
//! - Stamina system for climbing
//! - Auto-hang from ledges
//! - Grab surface on air
//! - Surface type detection
//! - Ledge zone configuration

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
            spawn_climbable_surfaces,
            update_climb_demo_ui,
            handle_climb_demo_input,
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

    // Spawn player with character controller and climb system
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
        ClimbLedgeSystem {
            climb_ledge_active: true,
            use_hang_from_ledge_icon: true,
            use_fixed_device_icon_position: false,
            keep_weapons_on_ledge_detected: false,
            draw_weapons_after_climb_ledge_if_previously_carried: false,
            climb_ledge_action_id: 1,
            hold_on_ledge_action_name: "Hold On Ledge".to_string(),
            action_active_animator_name: "Action Active".to_string(),
            action_id_animator_name: "Action ID".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,
            climb_ledge_ray_forward_distance: 1.0,
            climb_ledge_ray_down_distance: 1.0,
            layer_mask_to_check: 1,
            only_grab_ledge_if_moving_forward: false,
            adjust_to_hold_on_ledge_position_speed: 3.0,
            adjust_to_hold_on_ledge_rotation_speed: 10.0,
            hold_on_ledge_offset: Vec3::ZERO,
            climb_ledge_target_position_offset_third_person: Vec3::ZERO,
            climb_ledge_target_position_offset_first_person: Vec3::ZERO,
            hand_offset: 0.2,
            time_to_climb_ledge_third_person: 2.0,
            time_to_climb_ledge_first_person: 1.0,
            climb_ledge_speed_first_person: 1.0,
            climb_if_surface_found_below_player: false,
            check_for_ledge_zones_active: true,
            check_for_hang_from_ledge_on_ground: false,
            check_ledge_zone_detected_by_raycast: true,
            raycast_radius_to_check_surface_below_player: 0.5,
            check_for_hang_from_ledge_on_ground_raycast_distance: 2.0,
            only_hang_from_ledge_if_player_is_not_moving: true,
            time_to_cancel_hang_from_ledge_if_not_found: 3.0,
            can_cancel_hang_from_ledge: true,
            has_to_look_at_ledge_position_on_first_person: false,
            use_max_distance_to_camera_center: false,
            max_distance_to_camera_center: 100.0,
            auto_climb_in_third_person: false,
            auto_climb_in_first_person: false,
            can_jump_when_hold_ledge: true,
            jump_force_when_hold_ledge: 10.0,
            jump_force_mode: ForceMode::Impulse,
            can_grab_any_surface_on_air: true,
            use_grab_surface_amount_limit: true,
            grab_surface_amount_limit: 3,
            current_grab_surface_amount: 0,
            avoid_player_grab_ledge: false,
            ledge_zone_found: false,
            activate_climb_action: false,
            can_start_to_climb_ledge: false,
            climbing_ledge: false,
            can_use_climb_ledge: true,
            can_climb_current_ledge_zone: true,
            stop_grab_ledge: false,
            direction_angle: 0.0,
            surface_to_hang_on_ground_found: false,
            moving_toward_surface_to_hang: false,
            previously_moving_toward_surface_to_hang: false,
            on_air_while_searching_ledge_to_hang: false,
            ledge_zone_close_enough_to_screen_center: false,
            current_distance_to_target: 0.0,
            can_check_for_hang_from_ledge_on_ground: true,
            climb_ledge_action_activated: false,
            lose_ledge_action_activated: false,
            grabbing_surface: false,
            climb_ledge_action_paused: false,
        },
        ClimbStateTracker {
            current_state: ClimbState::None,
            previous_state: ClimbState::None,
            state_timer: 0.0,
            climb_speed: 3.0,
            stamina: 100.0,
            max_stamina: 100.0,
            stamina_drain_rate: 10.0,
            stamina_regen_rate: 5.0,
            is_stamina_depleted: false,
        },
        LedgeDetection {
            ledge_found: false,
            ledge_position: Vec3::ZERO,
            ledge_normal: Vec3::ZERO,
            ledge_distance: 0.0,
            ledge_height: 0.0,
            is_hangable: false,
            is_climbable: false,
            surface_type: SurfaceType::Default,
            raycast_hit_point: Vec3::ZERO,
            raycast_hit_normal: Vec3::ZERO,
        },
        AutoHang {
            active: false,
            moving_toward_ledge: false,
            target_ledge_position: Vec3::ZERO,
            target_ledge_normal: Vec3::ZERO,
            move_speed: 3.0,
            rotation_speed: 10.0,
            timeout: 3.0,
            timer: 0.0,
            only_when_not_moving: true,
            look_at_ledge_on_first_person: false,
            max_distance_to_camera_center: 100.0,
        },
        ClimbAnimation {
            is_active: false,
            action_id: 1,
            action_name: "Hold On Ledge".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,
            is_first_person: false,
            time_to_climb_third_person: 2.0,
            time_to_climb_first_person: 1.0,
        },
        ClimbMovement {
            is_active: false,
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            move_speed: 3.0,
            rotation_speed: 10.0,
            hand_offset: 0.2,
            is_first_person: false,
            climb_speed_first_person: 1.0,
            adjust_position_speed: 3.0,
            adjust_rotation_speed: 10.0,
        },
        LedgeJump {
            can_jump: true,
            jump_force: 10.0,
            jump_force_mode: ForceMode::Impulse,
            is_jumping: false,
            jump_timer: 0.0,
        },
        GrabSurfaceOnAir {
            can_grab: true,
            use_amount_limit: true,
            amount_limit: 3,
            current_amount: 0,
            is_grabbing: false,
            grab_timer: 0.0,
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

    // Spawn walls for climbing
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

    // Spawn a ledge for climbing
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(3.0, 0.5, 0.5),
        Transform::from_xyz(0.0, 3.0, -4.5),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        LedgeZone {
            tag_to_check: "Player".to_string(),
            ledge_zone_active: true,
            check_down_raycast_offset: Vec3::new(0.0, -0.5, 0.0),
            climb_ledge_forward_ray_distance: 1.0,
            climb_ledge_down_ray_distance: 1.0,
            ledge_zone_can_be_climbed: true,
            avoid_player_grab_ledge: false,
            can_check_for_hang_from_ledge_on_ground: true,
            only_hang_from_ledge_if_player_is_not_moving: true,
            can_grab_any_surface_on_air_active: true,
        },
    ));

    // Spawn a higher ledge for advanced climbing
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(3.0, 0.5, 0.5),
        Transform::from_xyz(0.0, 5.5, -4.5),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        LedgeZone {
            tag_to_check: "Player".to_string(),
            ledge_zone_active: true,
            check_down_raycast_offset: Vec3::new(0.0, -0.5, 0.0),
            climb_ledge_forward_ray_distance: 1.0,
            climb_ledge_down_ray_distance: 1.0,
            ledge_zone_can_be_climbed: true,
            avoid_player_grab_ledge: false,
            can_check_for_hang_from_ledge_on_ground: true,
            only_hang_from_ledge_if_player_is_not_moving: true,
            can_grab_any_surface_on_air_active: true,
        },
    ));

    // Spawn a platform for jumping to
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(4.0, 0.5, 4.0),
        Transform::from_xyz(0.0, 7.0, -2.0),
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
                "Climb System Demo\n\n",
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
                "E - Grab Ledge (when near ledge)\n",
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
                "Stamina: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            TextSection::new(
                "100%\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "\nState: ",
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

/// Spawn additional climbable surfaces for demo
fn spawn_climbable_surfaces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<LedgeZone>>,
) {
    // Only spawn once
    if query.iter().count() > 0 {
        return;
    }

    // Spawn decorative climbable surfaces
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.4, 0.2),
            ..default()
        }),
        transform: Transform::from_xyz(-3.0, 1.5, -4.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.8, 0.4),
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 1.5, -4.0),
        ..default()
    });
}

/// Update demo UI with climb state and stamina
fn update_climb_demo_ui(
    query: Query<(&ClimbStateTracker, &ClimbLedgeSystem), With<Player>>,
    mut ui_query: Query<&mut Text, With<DemoUI>>,
) {
    for (state_tracker, climb_system) in query.iter() {
        for mut text in ui_query.iter_mut() {
            // Update stamina display
            let stamina_percent = (state_tracker.stamina / state_tracker.max_stamina * 100.0) as i32;
            let stamina_color = if state_tracker.is_stamina_depleted {
                Color::RED
            } else if stamina_percent > 50 {
                Color::GREEN
            } else if stamina_percent > 25 {
                Color::YELLOW
            } else {
                Color::RED
            };

            // Update state display
            let state_str = match state_tracker.current_state {
                ClimbState::None => "None",
                ClimbState::Approaching => "Approaching",
                ClimbState::Hanging => "Hanging",
                ClimbState::ClimbingUp => "Climbing Up",
                ClimbState::ClimbingDown => "Climbing Down",
                ClimbState::ClimbingLeft => "Climbing Left",
                ClimbState::ClimbingRight => "Climbing Right",
                ClimbState::Vaulting => "Vaulting",
                ClimbState::Falling => "Falling",
            };

            // Update text sections
            if text.sections.len() >= 12 {
                text.sections[9].value = format!("{}\n", stamina_percent);
                text.sections[9].style.color = stamina_color;
                text.sections[11].value = format!("{}\n", state_str);
            }
        }
    }
}

/// Handle demo-specific input for climbing
fn handle_climb_demo_input(
    input_state: Res<InputState>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut GrabSurfaceOnAir,
        &mut AutoHang,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut grab_surface,
        mut auto_hang,
        character,
        transform,
    ) in query.iter_mut() {
        // Handle grab ledge input (E key)
        if input_state.is_action_just_pressed(InputAction::Interact) {
            // Try to grab surface on air
            if climb_system.can_grab_any_surface_on_air &&
               !character.is_dead &&
               !climb_system.climbing_ledge &&
               !climb_system.climb_ledge_action_paused &&
               !character.is_dead &&
               !character.zero_gravity_mode &&
               !character.free_floating_mode {
                
                // Set flag to attempt grab
                grab_surface.is_grabbing = true;
                grab_surface.grab_timer = 0.0;
            }
        }

        // Handle jump from ledge
        if input_state.is_action_just_pressed(InputAction::Jump) {
            if climb_system.can_jump_when_hold_ledge &&
               (climb_system.climbing_ledge || climb_system.grabbing_surface) &&
               !climb_system.activate_climb_action {
                // Trigger jump from ledge
                // This would be handled by the climb system
            }
        }
    }
}

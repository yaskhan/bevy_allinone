//! # Weapon Attachments System Demo
//!
//! This example demonstrates the weapon attachments system functionality.
//!
//! ## Controls
//!
//! - **1-5**: Select attachment place (1=Scope, 2=Muzzle, 3=Magazine, 4=Underbarrel)
//! - **Q/E**: Cycle through attachments for selected place
//! - **R**: Remove current attachment
//! - **T**: Toggle attachment editor
//! - **Space**: Fire weapon
//! - **Mouse**: Aim camera
//!
//! ## Attachments Available
//!
//! ### Scope
//! - **Iron Sights** (default)
//! - **Red Dot Sight** - Quick aiming, less spread
//! - **ACOG Scope** - Medium range magnification
//! - **Sniper Scope** - Long range magnification
//!
//! ### Muzzle
//! - **Standard Muzzle** (default)
//! - **Silencer** - Reduces noise and damage
//! - **Heavy Barrel** - Increases damage but adds recoil
//!
//! ### Magazine
//! - **Standard Magazine** (default)
//! - **Extended Magazine** - +50% capacity, slower reload
//!
//! ### Underbarrel
//! - **None** (default)
//! - **Laser Sight** - Improves accuracy when aiming

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            WeaponsPlugin,
            CharacterPlugin,
            CameraPlugin,
        ))
        .insert_resource(VisualEffectPool::default())
        .insert_resource(BallisticsEnvironment {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_density: 1.225,
            wind: Vec3::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_attachment_input,
            handle_firing,
            update_ui,
            update_weapon_stats_display,
        ))
        .run();
}

#[derive(Component)]
struct Crosshair;

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct AttachmentInfoText;

/// Current selected attachment place for input handling
#[derive(Resource, Default)]
struct SelectedAttachmentPlace {
    pub place_index: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Floor (Physical)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
    ));

    // Target wall (Physical)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 4.0, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 2.0, -10.0),
        RigidBody::Static,
        Collider::cuboid(2.0, 2.0, 0.1),
    ));

    // Player (Character entity)
    let player_id = commands.spawn((
        Player,
        Transform::from_xyz(0.0, 1.0, 5.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        CharacterController::default(),
    )).id();

    // Camera following the player
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.6, 0.0),
        CameraController {
            follow_target: Some(player_id),
            mode: bevy_allinone::camera::CameraMode::FirstPerson,
            rot_sensitivity_3p: 0.001,
            rot_sensitivity_1p: 0.001,
            ..default()
        },
    ));

    // Weapon with attachments
    let weapon = Weapon {
        weapon_name: "Tactical Rifle".to_string(),
        damage: 25.0,
        range: 100.0,
        fire_rate: 6.0,
        current_fire_timer: 0.0,
        ammo_capacity: 30,
        current_ammo: 30,
        reload_time: 2.0,
        current_reload_timer: 0.0,
        is_reloading: false,
        is_automatic: true,
        spread: 0.5,
        base_spread: 0.5,
        aim_spread_mult: 0.2,
        projectiles_per_shot: 1,
        projectile_speed: 100.0,
        weapon_type: bevy_allinone::weapons::WeaponType::Rifle,
        attachments: vec![],
        projectile_mass: 0.01,
        projectile_drag_coeff: 0.3,
        projectile_area: 0.00001,
        projectile_penetration: 1000.0,
        zeroing_distance: 10.0,
        pocket_id: None,
        key_number: 0,
        enabled: true,
        equipped: true,
        carrying: true,
        is_dual: false,
        linked_dual_weapon: None,
        using_right_hand: false,
        can_be_dropped: true,
        last_fired_time: 0.0,
        last_reloaded_time: 0.0,
        last_drawn_time: 0.0,
        // Base stats
        base_damage: 25.0,
        base_spread_value: 0.5,
        base_fire_rate: 6.0,
        base_reload_time: 2.0,
        base_ammo_capacity: 30,
        base_range: 100.0,
    };

    // Accuracy component
    let accuracy = Accuracy {
        current_bloom: 0.0,
        base_spread: 0.5,
        max_spread: 5.0,
        bloom_per_shot: 0.2,
        recovery_rate: 2.0,
        movement_penalty: 1.0,
        ads_modifier: 0.5,
        airborne_multiplier: 2.0,
    };

    // Weapon attachment system
    let attachment_system = bevy_allinone::weapons::attachments::create_weapon_with_attachments();

    // Add components to player
    commands.entity(player_id).insert((
        weapon,
        accuracy,
        attachment_system,
    ));

    // UI - Crosshair
    commands.spawn((
        Crosshair,
        Text::new("+"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
    ));

    // UI - Stats Display
    commands.spawn((
        StatsText,
        Text::new("Weapon Stats\n\nDamage: 25.0\nSpread: 0.5°\nFire Rate: 6.0/s\nReload: 2.0s\nMagazine: 30"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // UI - Attachment Info
    commands.spawn((
        AttachmentInfoText,
        Text::new("Attachments:\nScope: Iron Sights\nMuzzle: Standard\nMagazine: Standard\nUnderbarrel: None\n\nPress T to open editor"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));

    // Resource for tracking selected place
    commands.insert_resource(SelectedAttachmentPlace::default());
}

fn handle_attachment_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_place: ResMut<SelectedAttachmentPlace>,
    mut player_query: Query<(Entity, &mut Weapon, &mut WeaponAttachmentSystem), With<Player>>,
    mut commands: Commands,
    mut weapon_query: Query<&mut Weapon>,
    mut attachment_query: Query<&mut WeaponAttachmentSystem>,
    time: Res<Time>,
) {
    let Some((player_entity, mut weapon, mut attachment_system)) = player_query.iter_mut().next() else {
        return;
    };

    // Select attachment place (1-4)
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        selected_place.place_index = 0;
        info!("Selected place: Scope");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        selected_place.place_index = 1;
        info!("Selected place: Muzzle");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        selected_place.place_index = 2;
        info!("Selected place: Magazine");
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        selected_place.place_index = 3;
        info!("Selected place: Underbarrel");
    }

    // Cycle through attachments (Q/E)
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        cycle_attachment(
            player_entity,
            selected_place.place_index,
            1,
            &mut weapon_query,
            &mut attachment_query,
        );
    }
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        cycle_attachment(
            player_entity,
            selected_place.place_index,
            -1,
            &mut weapon_query,
            &mut attachment_query,
        );
    }

    // Remove attachment (R)
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if let Some(place_id) = get_place_id(selected_place.place_index) {
            let result = weapon_manager::WeaponManager::default().remove_attachment(
                player_entity,
                place_id,
                &mut commands,
                &mut weapon_query,
                &mut attachment_query,
            );
            
            if let Err(e) = result {
                info!("Error removing attachment: {}", e);
            }
        }
    }

    // Toggle attachment editor (T)
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        attachment_system.editing_attachments = !attachment_system.editing_attachments;
        
        if attachment_system.editing_attachments {
            info!("Attachment editor opened");
        } else {
            info!("Attachment editor closed");
        }
    }
}

fn cycle_attachment(
    weapon_entity: Entity,
    place_index: usize,
    direction: i32,
    weapon_query: &mut Query<&mut Weapon>,
    attachment_query: &mut Query<&mut WeaponAttachmentSystem>,
) {
    let Some(place_id) = get_place_id(place_index) else {
        return;
    };

    if let Ok(mut attachment_system) = attachment_query.get_mut(weapon_entity) {
        if let Some(place) = attachment_system
            .attachment_places
            .iter_mut()
            .find(|p| p.id == place_id)
        {
            let mut new_index = place.current_selection + direction;

            // Wrap around
            if new_index < 0 {
                new_index = place.available_attachments.len() as i32 - 1;
            } else if new_index >= place.available_attachments.len() as i32 {
                new_index = 0;
            }

            // Check if attachment is enabled
            if let Some(attachment) = place.available_attachments.get(new_index as usize) {
                if attachment.enabled {
                    // Deactivate previous
                    if place.current_selection >= 0 {
                        if let Some(prev) = place.available_attachments.get_mut(place.current_selection as usize) {
                            prev.active = false;
                        }
                    }

                    // Activate new
                    if let Some(new_attachment) = place.available_attachments.get_mut(new_index as usize) {
                        new_attachment.active = true;
                        place.current_selection = new_index;
                        attachment_system.selected_attachments.insert(place_id.to_string(), new_attachment.id.clone());

                        // Apply to weapon
                        if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
                            // Remove old modifiers
                            if place.current_selection >= 0 {
                                if let Some(prev) = place.available_attachments.get(place.current_selection as usize) {
                                    prev.stat_modifiers.remove_from_weapon(&mut weapon);
                                }
                            }
                            
                            // Apply new modifiers
                            new_attachment.stat_modifiers.apply_to_weapon(&mut weapon);
                            
                            info!(
                                "Applied '{}' to {}. New stats: damage={}, spread={}",
                                new_attachment.name, place.name, weapon.damage, weapon.spread
                            );
                        }
                    }
                }
            }
        }
    }
}

fn get_place_id(index: usize) -> Option<&'static str> {
    match index {
        0 => Some("scope"),
        1 => Some("muzzle"),
        2 => Some("magazine"),
        3 => Some("underbarrel"),
        _ => None,
    }
}

fn handle_firing(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut player_query: Query<(&Weapon, &mut Accuracy, &WeaponAttachmentSystem), With<Player>>,
    time: Res<Time>,
) {
    let Some((_camera, camera_transform)) = camera_query.iter().next() else { return; };
    let Some((weapon, mut accuracy, attachment_system)) = player_query.iter_mut().next() else { return; };

    if mouse_input.pressed(MouseButton::Left) && attachment_system.attachments_active {
        // Emulate fire_weapon logic for demo
        if weapon.current_fire_timer <= 0.0 {
            // Calculate view direction
            let ray_origin = camera_transform.translation();
            let _ray_direction = camera_transform.forward();

            // Apply spread (Bloom)
            accuracy.current_bloom += accuracy.bloom_per_shot;
            accuracy.current_bloom = accuracy.current_bloom.min(accuracy.max_spread);

            let total_spread_deg = weapon.spread + accuracy.current_bloom;
            let spread_angle = total_spread_deg.to_radians();

            // Simple random for demo
            let rand_x = (time.elapsed_secs().sin() * 10.0).fract() * 2.0 - 1.0;
            let rand_y = (time.elapsed_secs().cos() * 10.0).fract() * 2.0 - 1.0;

            let s_x = rand_x * rand_x * spread_angle * 0.5 * rand_x.signum();
            let s_y = rand_y * rand_y * spread_angle * 0.5 * rand_y.signum();

            let spread_rot = Quat::from_euler(EulerRot::XYZ, s_y, s_x, 0.0);

            // Zeroing (drop compensation)
            let zeroing_angle = if weapon.zeroing_distance > 0.0 && weapon.projectile_speed > 0.0 {
                let time_to_zero = weapon.zeroing_distance / weapon.projectile_speed;
                let drop = 0.5 * 9.81 * time_to_zero * time_to_zero;
                f32::atan2(drop, weapon.zeroing_distance)
            } else { 0.0 };
            let zeroing_rot = Quat::from_rotation_x(zeroing_angle);

            let final_dir = camera_transform.rotation() * zeroing_rot * spread_rot * Vec3::NEG_Z;

            let _owner_id = commands.spawn_empty().id();
            // Spawn projectile
            commands.spawn((
                Mesh3d(Handle::<Mesh>::default()), // Placeholder
                Transform::from_translation(ray_origin),
                GlobalTransform::default(),
                Projectile {
                    velocity: final_dir * weapon.projectile_speed,
                    damage: weapon.damage,
                    lifetime: 5.0,
                    owner: _owner_id,
                    mass: weapon.projectile_mass,
                    drag_coeff: weapon.projectile_drag_coeff,
                    reference_area: weapon.projectile_area,
                    penetration_power: weapon.projectile_penetration,
                },
                Name::new("DemoProjectile"),
            ));
        }
    }
}

fn update_ui(
    query: Query<&Accuracy, With<Player>>,
    mut text_query: Query<&mut TextColor, With<Crosshair>>,
) {
    if let Some(accuracy) = query.iter().next() {
        for mut color in text_query.iter_mut() {
            // Color changes depending on spread
            let intensity = 1.0 - (accuracy.current_bloom / accuracy.max_spread).clamp(0.0, 1.0);
            color.0 = Color::srgba(1.0, 1.0, 1.0, intensity);
        }
    }
}

fn update_weapon_stats_display(
    player_query: Query<(&Weapon, &WeaponAttachmentSystem), With<Player>>,
    mut stats_query: Query<&mut Text, With<StatsText>>,
    mut attachment_query: Query<&mut Text, With<AttachmentInfoText>>,
) {
    let Some((weapon, attachment_system)) = player_query.iter().next() else {
        return;
    };

    // Update stats text
    if let Some(mut stats_text) = stats_query.iter_mut().next() {
        stats_text.0 = format!(
            "Weapon Stats\n\nDamage: {:.1}\nSpread: {:.1}°\nFire Rate: {:.1}/s\nReload: {:.1}s\nMagazine: {}",
            weapon.damage,
            weapon.spread,
            weapon.fire_rate,
            weapon.reload_time,
            weapon.ammo_capacity
        );
    }

    // Update attachment info text
    if let Some(mut attachment_text) = attachment_query.iter_mut().next() {
        let mut info = String::from("Attachments:\n");
        
        for place in &attachment_system.attachment_places {
            let current = if place.current_selection >= 0 {
                place.available_attachments
                    .get(place.current_selection as usize)
                    .map(|a| a.name.as_str())
                    .unwrap_or("None")
            } else {
                "None"
            };
            info.push_str(&format!("{}: {}\n", place.name, current));
        }
        
        if attachment_system.editing_attachments {
            info.push_str("\nEDITOR ACTIVE");
        } else {
            info.push_str("\nPress T to open editor");
        }
        
        attachment_text.0 = info;
    }
}

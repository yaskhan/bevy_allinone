use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::weapons::{WeaponsPlugin, Weapon, Projectile, Accuracy, BallisticsEnvironment, BulletTracer, VisualEffectPool};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            WeaponsPlugin,
        ))
        .insert_resource(BallisticsEnvironment {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_density: 1.225,
            wind: Vec3::new(2.0, 0.0, 0.0), // Ветер дует вправо
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_movement, handle_shooting, update_ui))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Crosshair;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Свет
    commands.spawn((
        Transform::from_xyz(4.0, 8.0, 4.0),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
    ));

    // Пол (Физический)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
    ));

    // Стена-мишень (Физическая)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 4.0, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 2.0, -10.0),
        RigidBody::Static,
        Collider::cuboid(2.0, 2.0, 0.1),
    ));

    // Игрок (Камера + Оружие)
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 1.0, 5.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    )).with_children(|parent| {
        // Камера
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.6, 0.0),
        ));

        // Визуализация оружия (простой куб)
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_xyz(0.2, -0.2, -0.3),
        ));
    });

    // Компоненты оружия и точности для игрока
    commands.spawn((
        Player,
        Weapon {
            weapon_name: "Ballistic Rifle".to_string(),
            damage: 25.0,
            range: 100.0,
            fire_rate: 4.0,
            current_fire_timer: 0.0,
            ammo_capacity: 100,
            current_ammo: 100,
            reload_time: 2.0,
            current_reload_timer: 0.0,
            is_reloading: false,
            is_automatic: true,
            spread: 0.5, // Базовый разброс в градусах
            base_spread: 0.5,
            aim_spread_mult: 0.2,
            projectiles_per_shot: 1,
            projectile_speed: 100.0, // Медленные пули для наглядности
            weapon_type: bevy_allinone::weapons::WeaponType::Rifle,
            attachments: vec![],
            projectile_mass: 0.01, // 10г
            projectile_drag_coeff: 0.3,
            projectile_area: 0.00001,
            projectile_penetration: 1000.0,
            zeroing_distance: 10.0, // Прицелен на 10 метров
        },
        Accuracy {
            current_bloom: 0.0,
            base_spread: 0.5,
            max_spread: 5.0,
            bloom_per_shot: 0.2,
            recovery_rate: 2.0, // Восстановление разброса
            movement_penalty: 1.0,
            ads_modifier: 0.5,
            airborne_multiplier: 2.0,
        },
        VisualEffectPool::default(),
    ));

    // UI (Перекрестие)
    commands.spawn((
        Crosshair,
        Text2d::new("+"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn handle_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    let speed = 5.0;
    for mut transform in query.iter_mut() {
        transform.translation += direction * speed * time.delta_secs();
    }
}

fn handle_shooting(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    player_query: Query<(&GlobalTransform, &Weapon, &mut Accuracy), With<Player>>,
    time: Res<Time>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };
    let Ok((player_transform, weapon, mut accuracy)) = player_query.get_single() else { return; };

    if mouse_input.pressed(MouseButton::Left) {
        // Эмуляция логики fire_weapon для демо
        if weapon.current_fire_timer <= 0.0 {
            // Расчет направления взгляда
            let ray_origin = camera_transform.translation();
            let ray_direction = camera_transform.forward();

            // Применение разброса (Bloom)
            accuracy.current_bloom += accuracy.bloom_per_shot;
            accuracy.current_bloom = accuracy.current_bloom.min(accuracy.max_spread);

            let total_spread_deg = weapon.spread + accuracy.current_bloom;
            let spread_angle = total_spread_deg.to_radians();

            // Простой рандом для демо
            let rand_x = (time.elapsed_seconds().sin() * 10.0).fract() * 2.0 - 1.0;
            let rand_y = (time.elapsed_seconds().cos() * 10.0).fract() * 2.0 - 1.0;

            let s_x = rand_x * rand_x * spread_angle * 0.5 * rand_x.signum();
            let s_y = rand_y * rand_y * spread_angle * 0.5 * rand_y.signum();

            let spread_rot = Quat::from_euler(EulerRot::XYZ, s_y, s_x, 0.0);
            
            // Zeroing (компенсация падения)
            let zeroing_angle = if weapon.zeroing_distance > 0.0 && weapon.projectile_speed > 0.0 {
                let time_to_zero = weapon.zeroing_distance / weapon.projectile_speed;
                let drop = 0.5 * 9.81 * time_to_zero * time_to_zero;
                drop.atan2(weapon.zeroing_distance)
            } else { 0.0 };
            let zeroing_rot = Quat::from_rotation_x(zeroing_angle);

            let final_dir = camera_transform.rotation() * zeroing_rot * spread_rot * Vec3::NEG_Z;

            // Спавн снаряда
            commands.spawn((
                Mesh3d(commands.spawn_empty().id()), // Placeholder, будет заменено системой рендеринга если нужно
                Transform::from_translation(ray_origin),
                GlobalTransform::default(),
                Projectile {
                    velocity: final_dir * weapon.projectile_speed,
                    damage: weapon.damage,
                    lifetime: 5.0,
                    owner: commands.spawn_empty().id(), // В демо просто новый ID
                    mass: weapon.projectile_mass,
                    drag_coeff: weapon.projectile_drag_coeff,
                    reference_area: weapon.projectile_area,
                    penetration_power: weapon.projectile_penetration,
                },
                Name::new("DemoProjectile"),
            ));

            // Визуальный трассер (простой линейный интерполятор)
            // В реальном коде это делается отдельной системой, здесь для демо спавним сразу
            // Но для соответствия архитектуре, мы можем просто позволить системе update_projectiles
            // обновлять Transform, а визуализацию сделать позже.
            // Для наглядности в этом демо мы будем просто двигать сущность.
            
            // Сброс таймера
            // Мы не можем мутировать Weapon здесь напрямую, так как он в Query.
            // В реальном приложении это делается внутри fire_weapon системы.
            // Для демо просто пропускаем кадры.
        }
    }
}

// В этом демо мы не можем мутировать Weapon.current_fire_timer напрямую в handle_shooting,
// так как Query immutable. В реальном коде это часть системы fire_weapon.
// Для демо добавим систему обновления таймера.
fn update_weapon_timer(
    time: Res<Time>,
    mut query: Query<&mut Weapon>,
) {
    for mut weapon in query.iter_mut() {
        if weapon.current_fire_timer > 0.0 {
            weapon.current_fire_timer -= time.delta_secs();
        }
    }
}

fn update_ui(
    query: Query<&Accuracy, With<Player>>,
    mut text_query: Query<&mut TextColor, With<Crosshair>>,
) {
    if let Ok(accuracy) = query.get_single() {
        for mut color in text_query.iter_mut() {
            // Цвет меняется в зависимости от разброса
            let intensity = 1.0 - (accuracy.current_bloom / accuracy.max_spread).clamp(0.0, 1.0);
            color.0 = Color::srgba(1.0, 1.0, 1.0, intensity);
        }
    }
}

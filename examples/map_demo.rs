use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Player
    let player = spawn_character(&mut commands, Vec3::new(0.0, 1.0, 0.0));
    commands.entity(player).insert(Player);

    // Camera
    spawn_camera(&mut commands, player);

    // NPC 1 (Point of Interest)
    let shop = commands.spawn((
        Name::new("Shop"),
        MapMarker {
            name: "Magic Shop".to_string(),
            icon_type: MapIconType::PointOfInterest,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(10.0, 1.0, -10.0),
    )).id();

    // NPC 2 (Quest)
    let bob = commands.spawn((
        Name::new("Bob"),
        MapMarker {
            name: "Bob the Quest Giver".to_string(),
            icon_type: MapIconType::Quest,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(-5.0, 1.0, 5.0),
    )).id();

    // UI - Minimap Container
    let minimap = commands.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
        BorderColor::from(Color::WHITE),
    ))
    .with_children(|parent| {
        // Player Icon (Center of Minimap)
        parent.spawn((
            Node {
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
        ));

        // Shop Icon
        parent.spawn((
            Node {
                width: Val::Px(8.0),
                height: Val::Px(8.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
            MapMarkerIcon { marker_entity: shop },
        ));

        // Bob Icon
        parent.spawn((
            Node {
                width: Val::Px(8.0),
                height: Val::Px(8.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            MapMarkerIcon { marker_entity: bob },
        ));
        
        // Compass (Visual only for rotation)
        parent.spawn((
            Node {
                width: Val::Px(2.0),
                height: Val::Px(30.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            CompassUI,
        ));
    }).id();

    info!("Map Demo Started!");
    info!("Check the top-right corner for the minimap (v1).");
}

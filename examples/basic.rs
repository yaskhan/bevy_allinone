//! Basic example showing how to use the Bevy game controller plugin
//!
//! This example creates a simple scene with a character controller.

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
    // Spawn a light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a character controller
    let character = spawn_character(&mut commands, Vec3::new(0.0, 1.0, 0.0));
    
    // Add a visual mesh for the character (cube for now)
    commands.entity(character).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        ));
    });

    // Spawn a camera following the character
    spawn_camera(&mut commands, character);

    info!("Bevy Game Controller Example - Character controller spawned!");
    info!("Note: Most functionality is not yet implemented (TODO)");
}

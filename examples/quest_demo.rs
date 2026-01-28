use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, check_quest_log)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn Player
    let player = spawn_character(&mut commands, Vec3::new(0.0, 1.0, 0.0));
    commands.entity(player).insert(QuestLog::default());
    
    // Add visual for player
    commands.entity(player).with_children(|child| {
        child.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 1.8, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.8))),
        ));
    });

    // Spawn Quest NPC (Quest Station)
    let quest = Quest {
        id: 1,
        name: "First Steps".to_string(),
        description: "Talk to the NPC and learn the ropes.".to_string(),
        objectives: vec![
            Objective {
                name: "Talk to Bob".to_string(),
                description: "Find Bob and say hello.".to_string(),
                status: QuestStatus::Completed, // Pre-completing for demo
            }
        ],
        status: QuestStatus::NotStarted,
        rewards_description: "100 Gold".to_string(),
    };

    commands.spawn((
        Name::new("Bob"),
        QuestStation { quest },
        Interactable {
            interaction_text: "Bob (Quest)".to_string(),
            interaction_type: InteractionType::Talk,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.8, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.2))),
        Transform::from_xyz(3.0, 0.9, 0.0),
        GlobalTransform::default(),
        // Interaction system uses avian3d raycasting
        Collider::cuboid(0.5, 1.8, 0.5),
    ));

    // Spawn Camera
    spawn_camera(&mut commands, player);

    info!("Quest Demo Started!");
    info!("Interaction: Press E to talk to Bob and accept the quest.");
}

fn check_quest_log(query: Query<&QuestLog, Changed<QuestLog>>) {
    for log in query.iter() {
        info!("Quest Log Updated!");
        for quest in &log.active_quests {
            info!("Active Quest: {} - Status: {:?}", quest.name, quest.status);
        }
        for quest in &log.completed_quests {
            info!("Completed Quest: {}", quest.name);
        }
    }
}

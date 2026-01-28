use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_systems)
        .add_systems(Update, (update_skills, handle_input, display_skills))
        .run();
}

/// System for demonstrating skills system functionality
#[derive(Debug, Component)]
pub struct DemoSkillsSystem;

/// Initialize skills system
fn setup_systems(mut commands: Commands) {
    // Create skills system
    let mut skills_system = SkillsSystem::new();

    // Create skill categories
    let mut combat_category = SkillCategory::new("Combat");
    let mut magic_category = SkillCategory::new("Magic");
    let mut utility_category = SkillCategory::new("Utility");

    // Add skills to "Combat" category
    combat_category.add_skill(Skill {
        name: "Damage".to_string(),
        description: "Increases damage by 10% per level".to_string(),
        skill_type: SkillType::Numeric,
        enabled: true,
        unlocked: true,
        active: true,
        complete: false,
        current_level: 0,
        max_level: 5,
        required_points: 1,
        current_value: 0.0,
        value_to_configure: 10.0,
        current_bool_state: false,
        bool_state_to_configure: false,
        levels: vec![
            SkillLevel {
                description: "Basic damage".to_string(),
                required_points: 1,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Increased damage".to_string(),
                required_points: 2,
                value: 20.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(20.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Powerful damage".to_string(),
                required_points: 3,
                value: 30.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(30.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Expert damage".to_string(),
                required_points: 4,
                value: 40.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(40.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Master damage".to_string(),
                required_points: 5,
                value: 50.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(50.0),
                on_activate: SkillEvent::None,
            },
        ],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::None,
        on_activate_bool: SkillEvent::None,
        use_two_events: true,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    combat_category.add_skill(Skill {
        name: "Defense".to_string(),
        description: "Increases defense by 5% per level".to_string(),
        skill_type: SkillType::Numeric,
        enabled: true,
        unlocked: false,
        active: false,
        complete: false,
        current_level: 0,
        max_level: 3,
        required_points: 2,
        current_value: 0.0,
        value_to_configure: 5.0,
        current_bool_state: false,
        bool_state_to_configure: false,
        levels: vec![
            SkillLevel {
                description: "Basic defense".to_string(),
                required_points: 2,
                value: 5.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(5.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Improved defense".to_string(),
                required_points: 3,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Maximum defense".to_string(),
                required_points: 4,
                value: 15.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(15.0),
                on_activate: SkillEvent::None,
            },
        ],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::None,
        on_activate_bool: SkillEvent::None,
        use_two_events: true,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    combat_category.add_skill(Skill {
        name: "Critical Strike".to_string(),
        description: "Increases critical strike chance".to_string(),
        skill_type: SkillType::Boolean,
        enabled: true,
        unlocked: false,
        active: false,
        complete: false,
        current_level: 0,
        max_level: 1,
        required_points: 3,
        current_value: 0.0,
        value_to_configure: 15.0,
        current_bool_state: false,
        bool_state_to_configure: true,
        levels: vec![],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::WithBool(false),
        on_activate_bool: SkillEvent::WithBool(true),
        use_two_events: false,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    // Add skills to "Magic" category
    magic_category.add_skill(Skill {
        name: "Mana".to_string(),
        description: "Increases maximum mana".to_string(),
        skill_type: SkillType::Numeric,
        enabled: true,
        unlocked: true,
        active: true,
        complete: false,
        current_level: 0,
        max_level: 4,
        required_points: 1,
        current_value: 100.0,
        value_to_configure: 50.0,
        current_bool_state: false,
        bool_state_to_configure: false,
        levels: vec![
            SkillLevel {
                description: "Basic mana".to_string(),
                required_points: 1,
                value: 50.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(50.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Increased mana".to_string(),
                required_points: 2,
                value: 100.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(100.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Large mana".to_string(),
                required_points: 3,
                value: 150.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(150.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Maximum mana".to_string(),
                required_points: 4,
                value: 200.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(200.0),
                on_activate: SkillEvent::None,
            },
        ],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::None,
        on_activate_bool: SkillEvent::None,
        use_two_events: true,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    magic_category.add_skill(Skill {
        name: "Magic Shield".to_string(),
        description: "Activates magic shield".to_string(),
        skill_type: SkillType::Boolean,
        enabled: true,
        unlocked: false,
        active: false,
        complete: false,
        current_level: 0,
        max_level: 1,
        required_points: 2,
        current_value: 0.0,
        value_to_configure: 0.0,
        current_bool_state: false,
        bool_state_to_configure: true,
        levels: vec![],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::WithBool(false),
        on_activate_bool: SkillEvent::WithBool(true),
        use_two_events: false,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    // Add skills to "Utility" category
    utility_category.add_skill(Skill {
        name: "Speed".to_string(),
        description: "Increases movement speed".to_string(),
        skill_type: SkillType::Numeric,
        enabled: true,
        unlocked: true,
        active: true,
        complete: false,
        current_level: 0,
        max_level: 3,
        required_points: 1,
        current_value: 0.0,
        value_to_configure: 10.0,
        current_bool_state: false,
        bool_state_to_configure: false,
        levels: vec![
            SkillLevel {
                description: "Basic speed".to_string(),
                required_points: 1,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Increased speed".to_string(),
                required_points: 2,
                value: 20.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(20.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Maximum speed".to_string(),
                required_points: 3,
                value: 30.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(30.0),
                on_activate: SkillEvent::None,
            },
        ],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::None,
        on_activate_bool: SkillEvent::None,
        use_two_events: true,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    utility_category.add_skill(Skill {
        name: "Invisibility".to_string(),
        description: "Activates invisibility".to_string(),
        skill_type: SkillType::Boolean,
        enabled: true,
        unlocked: false,
        active: false,
        complete: false,
        current_level: 0,
        max_level: 1,
        required_points: 3,
        current_value: 0.0,
        value_to_configure: 0.0,
        current_bool_state: false,
        bool_state_to_configure: true,
        levels: vec![],
        on_initialize: SkillEvent::None,
        on_increase: SkillEvent::None,
        on_initialize_bool: SkillEvent::WithBool(false),
        on_activate_bool: SkillEvent::WithBool(true),
        use_two_events: false,
        on_initialize_active: SkillEvent::None,
        on_initialize_not_active: SkillEvent::None,
        template_id: None,
    });

    // Add categories to skill tree
    skills_system.skill_tree.add_category(combat_category);
    skills_system.skill_tree.add_category(magic_category);
    skills_system.skill_tree.add_category(utility_category);

    // Initialize skill values
    skills_system.initialize_values();

    // Create resource for storing skill points
    commands.insert_resource(SkillPoints(10));

    // Create entity with skills system
    commands.spawn((
        DemoSkillsSystem,
        skills_system,
    ));

    println!("=== Skills System Demo ===");
    println!("Controls:");
    println!("  1 - Level up 'Damage' (cost: 1 point)");
    println!("  2 - Level up 'Defense' (cost: 2 points)");
    println!("  3 - Activate 'Critical Strike' (cost: 3 points)");
    println!("  4 - Level up 'Mana' (cost: 1 point)");
    println!("  5 - Activate 'Magic Shield' (cost: 2 points)");
    println!("  6 - Level up 'Speed' (cost: 1 point)");
    println!("  7 - Activate 'Invisibility' (cost: 3 points)");
    println!("  S - Save settings to template");
    println!("  L - Load settings from template");
    println!("  R - Reset all skills");
    println!("  Q - Exit");
    println!("===========================");
}

/// Update skills
fn update_skills(
    mut query: Query<&mut SkillsSystem, With<DemoSkillsSystem>>,
    mut skill_points: ResMut<SkillPoints>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Add skill update logic here
        // For example, automatic mana recovery or processing long-term effects
    }
}

/// Handle input
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SkillsSystem, With<DemoSkillsSystem>>,
    mut skill_points: ResMut<SkillPoints>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Level up 'Damage'
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(0, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Damage leveled up! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to level up Damage");
            }
        }

        // Level up 'Defense'
        if keyboard_input.just_pressed(KeyCode::Digit2) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(0, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Defense leveled up! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to level up Defense");
            }
        }

        // Activate 'Critical Strike'
        if keyboard_input.just_pressed(KeyCode::Digit3) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(0, 2, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Critical Strike activated! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to activate Critical Strike");
            }
        }

        // Level up 'Mana'
        if keyboard_input.just_pressed(KeyCode::Digit4) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(1, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Mana leveled up! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to level up Mana");
            }
        }

        // Activate 'Magic Shield'
        if keyboard_input.just_pressed(KeyCode::Digit5) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(1, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Magic Shield activated! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to activate Magic Shield");
            }
        }

        // Level up 'Speed'
        if keyboard_input.just_pressed(KeyCode::Digit6) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(2, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Speed leveled up! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to level up Speed");
            }
        }

        // Activate 'Invisibility'
        if keyboard_input.just_pressed(KeyCode::Digit7) {
            if let Some(points_used) = skills_system.skill_tree.use_skill_points(2, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Invisibility activated! Points remaining: {}", skill_points.0);
            } else {
                println!("Not enough points to activate Invisibility");
            }
        }

        // Save to template
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            skills_system.skill_tree.save_to_template();
            println!("Settings saved to template");
        }

        // Load from template
        if keyboard_input.just_pressed(KeyCode::KeyL) {
            skills_system.skill_tree.load_from_template();
            println!("Settings loaded from template");
        }

        // Reset all skills
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            // Reset all skills
            for category in &mut skills_system.skill_tree.categories {
                for skill in &mut category.skills {
                    skill.current_level = 0;
                    skill.current_value = 0.0;
                    skill.current_bool_state = false;
                    skill.complete = false;
                    skill.active = false;
                    if skill.name != "Damage" && skill.name != "Mana" && skill.name != "Speed" {
                        skill.unlocked = false;
                    }
                }
            }
            skill_points.0 = 10;
            println!("All skills reset. Points: {}", skill_points.0);
        }

        // Exit
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            println!("Exiting demo...");
            std::process::exit(0);
        }
    }
}

/// Display skill information
fn display_skills(
    query: Query<&SkillsSystem, With<DemoSkillsSystem>>,
    skill_points: Res<SkillPoints>,
) {
    for skills_system in query.iter() {
        if !skills_system.active {
            continue;
        }

        println!("\n=== Skills State ===");
        println!("Skill points: {}", skill_points.0);

        for category in &skills_system.skill_tree.categories {
            println!("\nCategory: {}", category.name);
            for skill in &category.skills {
                if skill.enabled {
                    let status = if skill.unlocked {
                        if skill.complete {
                            "✓ Complete"
                        } else if skill.active {
                            "✓ Active"
                        } else {
                            "✓ Unlocked"
                        }
                    } else {
                        "🔒 Locked"
                    };

                    let level_info = if skill.levels.is_empty() {
                        format!("Level: {}", skill.current_level)
                    } else {
                        format!("Level: {}/{}", skill.current_level, skill.max_level)
                    };

                    let value_info = if skill.skill_type == SkillType::Boolean {
                        format!("State: {}", skill.current_bool_state)
                    } else {
                        format!("Value: {:.1}", skill.current_value)
                    };

                    println!(
                        "  {} - {} ({}) [{}]",
                        skill.name, status, level_info, value_info
                    );
                    println!("      {}", skill.description);
                }
            }
        }
        println!("=====================\n");
    }
}

/// Resource for storing skill points
#[derive(Debug, Resource)]
pub struct SkillPoints(pub u32);

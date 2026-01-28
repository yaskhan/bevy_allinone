use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_systems)
        .add_systems(Update, (update_skills, handle_input, display_skills))
        .run();
}

/// Система для демонстрации работы системы скиллов
#[derive(Debug, Component)]
pub struct DemoSkillsSystem;

/// Инициализация системы скиллов
fn setup_systems(mut commands: Commands) {
    // Создаем систему скиллов
    let mut skills_system = SkillsSystem::new();

    // Создаем категории скиллов
    let mut combat_category = SkillCategory::new("Боевые");
    let mut magic_category = SkillCategory::new("Магические");
    let mut utility_category = SkillCategory::new("Утилитарные");

    // Добавляем скиллы в категорию "Боевые"
    combat_category.add_skill(Skill {
        name: "Урон".to_string(),
        description: "Увеличивает урон на 10% за уровень".to_string(),
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
                description: "Базовый урон".to_string(),
                required_points: 1,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Увеличенный урон".to_string(),
                required_points: 2,
                value: 20.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(20.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Мощный урон".to_string(),
                required_points: 3,
                value: 30.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(30.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Экспертный урон".to_string(),
                required_points: 4,
                value: 40.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(40.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Мастерский урон".to_string(),
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
        name: "Защита".to_string(),
        description: "Увеличивает защиту на 5% за уровень".to_string(),
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
                description: "Базовая защита".to_string(),
                required_points: 2,
                value: 5.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(5.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Улучшенная защита".to_string(),
                required_points: 3,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Максимальная защита".to_string(),
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
        name: "Критический удар".to_string(),
        description: "Увеличивает шанс критического удара".to_string(),
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

    // Добавляем скиллы в категорию "Магические"
    magic_category.add_skill(Skill {
        name: "Мана".to_string(),
        description: "Увеличивает максимальную ману".to_string(),
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
                description: "Базовая мана".to_string(),
                required_points: 1,
                value: 50.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(50.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Увеличенная мана".to_string(),
                required_points: 2,
                value: 100.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(100.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Большая мана".to_string(),
                required_points: 3,
                value: 150.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(150.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Максимальная мана".to_string(),
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
        name: "Магический щит".to_string(),
        description: "Активирует магический щит".to_string(),
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

    // Добавляем скиллы в категорию "Утилитарные"
    utility_category.add_skill(Skill {
        name: "Скорость".to_string(),
        description: "Увеличивает скорость передвижения".to_string(),
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
                description: "Базовая скорость".to_string(),
                required_points: 1,
                value: 10.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(10.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Увеличенная скорость".to_string(),
                required_points: 2,
                value: 20.0,
                bool_value: false,
                on_initialize: SkillEvent::WithValue(20.0),
                on_activate: SkillEvent::None,
            },
            SkillLevel {
                description: "Максимальная скорость".to_string(),
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
        name: "Невидимость".to_string(),
        description: "Активирует невидимость".to_string(),
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

    // Добавляем категории в дерево скиллов
    skills_system.skill_tree.add_category(combat_category);
    skills_system.skill_tree.add_category(magic_category);
    skills_system.skill_tree.add_category(utility_category);

    // Инициализируем значения скиллов
    skills_system.initialize_values();

    // Создаем ресурс для хранения очков скиллов
    commands.insert_resource(SkillPoints(10));

    // Создаем сущность с системой скиллов
    commands.spawn((
        DemoSkillsSystem,
        skills_system,
    ));

    println!("=== Демонстрация системы скиллов ===");
    println!("Управление:");
    println!("  1 - Увеличить уровень 'Урон' (стоимость: 1 очко)");
    println!("  2 - Увеличить уровень 'Защита' (стоимость: 2 очка)");
    println!("  3 - Активировать 'Критический удар' (стоимость: 3 очка)");
    println!("  4 - Увеличить уровень 'Мана' (стоимость: 1 очко)");
    println!("  5 - Активировать 'Магический щит' (стоимость: 2 очка)");
    println!("  6 - Увеличить уровень 'Скорость' (стоимость: 1 очко)");
    println!("  7 - Активировать 'Невидимость' (стоимость: 3 очка)");
    println!("  S - Сохранить настройки в шаблон");
    println!("  L - Загрузить настройки из шаблона");
    println!("  R - Сбросить все скиллы");
    println!("  Q - Выход");
    println!("====================================");
}

/// Обновление скиллов
fn update_skills(
    mut query: Query<&mut SkillsSystem, With<DemoSkillsSystem>>,
    mut skill_points: ResMut<SkillPoints>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Здесь можно добавить логику обновления скиллов
        // Например, автоматическое восстановление маны или обработка длительных эффектов
    }
}

/// Обработка ввода
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SkillsSystem, With<DemoSkillsSystem>>,
    mut skill_points: ResMut<SkillPoints>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Увеличение уровня 'Урон'
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            if let Some(points_used) = skills_system.use_skill_points(0, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Уровень 'Урон' повышен! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для повышения уровня 'Урон'");
            }
        }

        // Увеличение уровня 'Защита'
        if keyboard_input.just_pressed(KeyCode::Digit2) {
            if let Some(points_used) = skills_system.use_skill_points(0, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Уровень 'Защита' повышен! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для повышения уровня 'Защита'");
            }
        }

        // Активация 'Критический удар'
        if keyboard_input.just_pressed(KeyCode::Digit3) {
            if let Some(points_used) = skills_system.use_skill_points(0, 2, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("'Критический удар' активирован! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для активации 'Критический удар'");
            }
        }

        // Увеличение уровня 'Мана'
        if keyboard_input.just_pressed(KeyCode::Digit4) {
            if let Some(points_used) = skills_system.use_skill_points(1, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Уровень 'Мана' повышен! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для повышения уровня 'Мана'");
            }
        }

        // Активация 'Магический щит'
        if keyboard_input.just_pressed(KeyCode::Digit5) {
            if let Some(points_used) = skills_system.use_skill_points(1, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("'Магический щит' активирован! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для активации 'Магический щит'");
            }
        }

        // Увеличение уровня 'Скорость'
        if keyboard_input.just_pressed(KeyCode::Digit6) {
            if let Some(points_used) = skills_system.use_skill_points(2, 0, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("Уровень 'Скорость' повышен! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для повышения уровня 'Скорость'");
            }
        }

        // Активация 'Невидимость'
        if keyboard_input.just_pressed(KeyCode::Digit7) {
            if let Some(points_used) = skills_system.use_skill_points(2, 1, skill_points.0, false) {
                skill_points.0 -= points_used;
                println!("'Невидимость' активирована! Осталось очков: {}", skill_points.0);
            } else {
                println!("Не хватает очков для активации 'Невидимость'");
            }
        }

        // Сохранить в шаблон
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            skills_system.save_to_template();
            println!("Настройки скиллов сохранены в шаблон");
        }

        // Загрузить из шаблона
        if keyboard_input.just_pressed(KeyCode::KeyL) {
            skills_system.load_from_template();
            println!("Настройки скиллов загружены из шаблона");
        }

        // Сбросить все скиллы
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            // Сбрасываем все скиллы
            for category in &mut skills_system.skill_tree.categories {
                for skill in &mut category.skills {
                    skill.current_level = 0;
                    skill.current_value = 0.0;
                    skill.current_bool_state = false;
                    skill.complete = false;
                    skill.active = false;
                    if skill.name != "Урон" && skill.name != "Мана" && skill.name != "Скорость" {
                        skill.unlocked = false;
                    }
                }
            }
            skill_points.0 = 10;
            println!("Все скиллы сброшены. Очистков: {}", skill_points.0);
        }

        // Выход
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            println!("Выход из демонстрации...");
            std::process::exit(0);
        }
    }
}

/// Отображение информации о скиллах
fn display_skills(
    query: Query<&SkillsSystem, With<DemoSkillsSystem>>,
    skill_points: Res<SkillPoints>,
) {
    for skills_system in query.iter() {
        if !skills_system.active {
            continue;
        }

        println!("\n=== Состояние скиллов ===");
        println!("Очков скиллов: {}", skill_points.0);

        for category in &skills_system.skill_tree.categories {
            println!("\nКатегория: {}", category.name);
            for skill in &category.skills {
                if skill.enabled {
                    let status = if skill.unlocked {
                        if skill.complete {
                            "✓ Завершен"
                        } else if skill.active {
                            "✓ Активен"
                        } else {
                            "✓ Разблокирован"
                        }
                    } else {
                        "🔒 Заблокирован"
                    };

                    let level_info = if skill.levels.is_empty() {
                        format!("Уровень: {}", skill.current_level)
                    } else {
                        format!("Уровень: {}/{}", skill.current_level, skill.max_level)
                    };

                    let value_info = if skill.skill_type == SkillType::Boolean {
                        format!("Состояние: {}", skill.current_bool_state)
                    } else {
                        format!("Значение: {:.1}", skill.current_value)
                    };

                    println!(
                        "  {} - {} ({}) [{}] {}",
                        skill.name, status, level_info, value_info, skill.description
                    );
                }
            }
        }
        println!("=========================\n");
    }
}

/// Ресурс для хранения очков скиллов
#[derive(Debug, Resource)]
pub struct SkillPoints(pub u32);

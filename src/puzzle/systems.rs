use bevy::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};
use crate::interaction::{Interactable, InteractionType, InteractionEventQueue, InteractionEvent};
use avian3d::prelude::*;
use super::types::*;

// ============================================================================
// Systems
// ============================================================================

/// System to setup puzzle UI
pub fn setup_puzzle_ui(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 20.0,
        ..default()
    };

    // Puzzle prompt UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(25.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            PuzzlePrompt,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
        });

    // Puzzle hint UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(10.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            PuzzleHintUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                TextLayout::default(),
            ));
        });

    // Puzzle timer UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(5.0),
                right: Val::Percent(5.0),
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            PuzzleTimerUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(1.0, 0.5, 0.0)),
                TextLayout::default(),
            ));
        });

    // Puzzle progress UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(5.0),
                left: Val::Percent(5.0),
                align_self: AlignSelf::FlexStart,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            PuzzleProgressUI,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                text_style.clone(),
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                TextLayout::default(),
            ));
        });
}

/// System to update puzzle buttons
pub fn update_puzzle_buttons(
    time: Res<Time>,
    input: Res<InputState>,
    input_buffer: ResMut<InputBuffer>,
    mut buttons: Query<(&mut PuzzleButton, &mut Transform, Option<&Interactable>)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut button, mut transform, interactable_opt) in buttons.iter_mut() {
        // Update cooldown
        if button.cooldown_timer > 0.0 {
            button.cooldown_timer -= time.delta_secs();
            if button.cooldown_timer <= 0.0 {
                button.can_press = true;
            }
        }

        // Update press timer
        if button.press_timer > 0.0 {
            button.press_timer -= time.delta_secs();
            if button.press_timer <= 0.0 && button.auto_reset {
                button.is_pressed = false;
                button.reset_timer = button.reset_delay;
            }
        }

        // Update reset timer
        if button.reset_timer > 0.0 {
            button.reset_timer -= time.delta_secs();
            if button.reset_timer <= 0.0 {
                button.is_pressed = false;
            }
        }

        // Check for interaction input
        let can_interact = if let Some(interactable) = interactable_opt {
            interactable.can_interact
        } else {
            true
        };

        if input.interact_pressed && can_interact && button.can_press && !button.is_pressed {
            // Press the button
            button.is_pressed = true;
            button.can_press = false;
            button.cooldown_timer = button.cooldown;
            button.press_timer = button.press_duration;

            // Play sound
            if let Some(sound) = &button.sound_effect {
                // Sound would be played here with audio system
                info!("Playing button sound: {:?}", sound);
            }

            // Visual feedback: press animation
            transform.translation.y -= button.press_amount;

            // Trigger event
            events.0.push(PuzzleEvent::ButtonPressed(PuzzleButtonPressedEvent {
                    button_entity: Entity::PLACEHOLDER, // Will be set by caller
                button_name: button.name.clone(),
            }));
        }

        // Visual feedback: release animation
        if !button.is_pressed && transform.translation.y < 0.0 {
            transform.translation.y += button.press_speed * time.delta_secs();
            if transform.translation.y > 0.0 {
                transform.translation.y = 0.0;
            }
        }
    }
}

/// System to update puzzle levers
pub fn update_puzzle_levers(
    time: Res<Time>,
    input: Res<InputState>,
    mut levers: Query<(&mut PuzzleLever, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut lever, mut transform) in levers.iter_mut() {
        if !lever.can_move {
            continue;
        }

        // Check for interaction input
        if input.interact_pressed {
            // Toggle lever state
            let new_state = match lever.state {
                LeverState::Neutral => lever.target_state,
                LeverState::Up => LeverState::Neutral,
                LeverState::Down => LeverState::Neutral,
                LeverState::Left => LeverState::Neutral,
                LeverState::Right => LeverState::Neutral,
            };

            if new_state != lever.state {
                lever.state = new_state;

                // Calculate target angle
                let target_angle = match lever.state {
                    LeverState::Up => lever.rotation_range.y,
                    LeverState::Down => lever.rotation_range.x,
                    LeverState::Left => lever.rotation_range.x,
                    LeverState::Right => lever.rotation_range.y,
                    LeverState::Neutral => 0.0,
                };

                // Smooth rotation
                let current_angle = lever.current_angle;
                let new_angle = current_angle.lerp(target_angle, time.delta_secs() * lever.rotation_speed);
                lever.current_angle = new_angle;

                // Apply rotation
                let rotation = Quat::from_axis_angle(lever.rotation_axis, new_angle.to_radians());
                transform.rotation = rotation;

                // Play sound
                if let Some(sound) = &lever.sound_effect {
                    info!("Playing lever sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::LeverMoved(PuzzleLeverMovedEvent {
                    lever_entity: Entity::PLACEHOLDER, // Will be set by caller
                    lever_name: lever.name.clone(),
                    new_state: lever.state,
                }));
            }
        }
    }
}

/// System to update puzzle pressure plates
pub fn update_puzzle_pressure_plates(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut plates: Query<(&mut PuzzlePressurePlate, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut plate, mut transform) in plates.iter_mut() {
        // Check for objects on plate using spatial query
        let plate_position = transform.translation;
        let plate_size = Vec3::new(1.0, 0.1, 1.0); // Default plate size

        // Simple AABB check for objects on plate
        let mut detected_weight = 0.0;
        // In a real implementation, you would query for rigid bodies in the plate area
        // For now, we'll use a simple distance check approach

        // Update current weight
        plate.current_weight = detected_weight;

        // Check if plate should be pressed
        let should_be_pressed = plate.current_weight >= plate.required_weight;

        if should_be_pressed && !plate.is_pressed {
            // Press the plate
            plate.is_pressed = true;
            plate.press_timer = plate.press_duration;

            // Visual feedback: press down
            transform.translation.y -= plate.press_amount;

            // Play sound
            if let Some(sound) = &plate.sound_effect {
                info!("Playing pressure plate sound: {:?}", sound);
            }

            // Trigger event
            events.0.push(PuzzleEvent::PressurePlatePressed(PuzzlePressurePlatePressedEvent {
                plate_entity: Entity::PLACEHOLDER, // Will be set by caller
                plate_name: plate.name.clone(),
                weight: plate.current_weight,
            }));
        } else if !should_be_pressed && plate.is_pressed {
            // Release the plate
            if !plate.stay_pressed {
                plate.is_pressed = false;
                plate.reset_timer = plate.reset_delay;

                // Play release sound
                if let Some(sound) = &plate.release_sound {
                    info!("Playing pressure plate release sound: {:?}", sound);
                }
            }
        }

        // Update press timer
        if plate.press_timer > 0.0 {
            plate.press_timer -= time.delta_secs();
        }

        // Update reset timer
        if plate.reset_timer > 0.0 {
            plate.reset_timer -= time.delta_secs();
            if plate.reset_timer <= 0.0 {
                // Visual feedback: release up
                transform.translation.y += plate.press_amount;
            }
        }
    }
}

/// System to update puzzle locks
pub fn update_puzzle_locks(
    mut locks: Query<(&mut PuzzleLock, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut lock, interactable) in locks.iter_mut() {
        // Check if lock is unlocked
        if lock.is_unlocked {
            lock.lock_state = LockState::Unlocked;
            continue;
        }

        // Check if enough keys are inserted
        if lock.multi_key {
            if lock.current_keys.len() as u32 >= lock.required_key_count {
                lock.is_unlocked = true;
                lock.lock_state = LockState::Unlocked;

                // Play unlock sound
                if let Some(sound) = &lock.unlock_sound {
                    info!("Playing lock unlock sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: lock.name.clone(),
                    time_spent: 0.0,
                    reset_count: 0,
                }));
            } else if lock.current_keys.len() > 0 {
                lock.lock_state = LockState::PartiallyUnlocked;
            }
        } else {
            // Single key lock
            if lock.current_keys.len() > 0 {
                lock.is_unlocked = true;
                lock.lock_state = LockState::Unlocked;

                // Play unlock sound
                if let Some(sound) = &lock.unlock_sound {
                    info!("Playing lock unlock sound: {:?}", sound);
                }

                // Trigger event
                events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: lock.name.clone(),
                    time_spent: 0.0,
                    reset_count: 0,
                }));
            }
        }
    }
}

/// System to update puzzle sequences
pub fn update_puzzle_sequences(
    mut sequences: Query<(&mut PuzzleSequence, &mut PuzzleProgress)>,
    mut sequence_items: Query<(&mut PuzzleSequenceItem, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    let total_items_count = sequence_items.iter().count();

    for (mut sequence, mut progress) in sequences.iter_mut() {
        if sequence.complete {
            continue;
        }

        // Check sequence items
        for (mut item, interactable) in sequence_items.iter_mut() {
            if item.pressed && interactable.can_interact {
                // Check if this is the correct item
                if item.order_index == sequence.correct_index {
                    // Correct item
                    item.pressed = false;

                    // Play correct sound
                    if let Some(sound) = &sequence.correct_sound {
                        info!("Playing sequence correct sound: {:?}", sound);
                    }

                    // Update progress
                    sequence.correct_index += 1;
                    progress.current_step += 1;
                    progress.progress = progress.current_step as f32 / progress.total_steps as f32;

                    // Trigger event
                    events.0.push(PuzzleEvent::SequenceItemPressed(PuzzleSequenceItemPressedEvent {
                        item_entity: Entity::PLACEHOLDER, // Will be set by caller
                        item_name: item.name.clone(),
                        order_index: item.order_index,
                        correct: true,
                    }));

                    // Check if sequence is complete
                    if sequence.correct_index >= total_items_count as u32 {
                        sequence.complete = true;
                        progress.state = PuzzleState::Solved;

                        // Trigger solved event
                        events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                            puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                            puzzle_name: sequence.name.clone(),
                            time_spent: progress.time_spent,
                            reset_count: progress.reset_count,
                        }));
                    }
                } else {
                    // Wrong item
                    if sequence.reset_on_wrong {
                        // Reset sequence
                        sequence.correct_index = 0;
                        progress.current_step = 0;
                        progress.progress = 0.0;
                        progress.reset_count += 1;

                        // Reset all items
                        // for (mut item, _) in sequence_items.iter_mut() {
                            // item.pressed = false;
                        // }

                        // Play incorrect sound
                        if let Some(sound) = &sequence.incorrect_sound {
                            info!("Playing sequence incorrect sound: {:?}", sound);
                        }

                        // Trigger event
                        events.0.push(PuzzleEvent::SequenceItemPressed(PuzzleSequenceItemPressedEvent {
                            item_entity: Entity::PLACEHOLDER, // Will be set by caller
                            item_name: item.name.clone(),
                            order_index: item.order_index,
                            correct: false,
                        }));
                    }
                }
            }
        }
    }
}

/// System to update puzzle pianos
pub fn update_puzzle_pianos(
    time: Res<Time>,
    mut pianos: Query<(&mut PuzzlePiano, &mut PuzzleProgress)>,
    mut piano_keys: Query<(&mut PuzzlePianoKey, &Interactable)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut piano, mut progress) in pianos.iter_mut() {
        if !piano.using_piano {
            continue;
        }

        // Check piano keys
        for (mut key, interactable) in piano_keys.iter_mut() {
            if interactable.can_interact {
                // Check if key should be pressed
                let should_press = key.is_pressed;

                if should_press {
                    // Play sound
                    if let Some(sound) = &key.sound {
                        info!("Playing piano key sound: {:?}", sound);
                    }

                    // Visual feedback: rotate key
                    let target_rotation = key.target_rotation.to_radians();
                    let current_rotation = key.current_rotation;
                    let new_rotation = current_rotation.lerp(target_rotation, time.delta_secs() * key.rotation_speed);
                    key.current_rotation = new_rotation;

                    // Trigger event
                    events.0.push(PuzzleEvent::PianoKeyPressed(PuzzlePianoKeyPressedEvent {
                        key_entity: Entity::PLACEHOLDER, // Will be set by caller
                        key_name: key.name.clone(),
                        note: key.name.clone(),
                    }));

                    // Reset key press
                    key.is_pressed = false;
                }

                // Return key to original position
                if key.current_rotation != 0.0 {
                    let new_rotation = key.current_rotation.lerp(0.0, time.delta_secs() * key.rotation_speed);
                    key.current_rotation = new_rotation;
                }
            }
        }
    }
}

/// System to update puzzle object placements
pub fn update_puzzle_object_placements(
    time: Res<Time>,
    mut placements: Query<(&mut PuzzleObjectPlacement, &mut PuzzleProgress, &Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut placement, mut progress, transform) in placements.iter_mut() {
        if placement.is_placed {
            continue;
        }

        // Check if object is inside trigger
        if placement.object_inside_trigger {
            // Check rotation limits
            if placement.use_rotation_limit {
                // In a real implementation, you would check the object's rotation
                // For now, we'll assume it's correct if inside trigger
                placement.object_in_correct_rotation = true;
            }

            // Check position limits
            if placement.use_position_limit {
                // In a real implementation, you would check the object's position
                // For now, we'll assume it's correct if inside trigger
                placement.object_in_correct_position = true;
            }

            // Check if object can be placed
            let can_place = if placement.use_rotation_limit && !placement.object_in_correct_rotation {
                false
            } else if placement.use_position_limit && !placement.object_in_correct_position {
                false
            } else if placement.needs_other_objects_before {
                placement.current_objects_placed >= placement.number_objects_before
            } else {
                true
            };

            if can_place {
                // Place the object
                placement.is_placed = true;
                progress.current_step += 1;
                progress.progress = progress.current_step as f32 / progress.total_steps as f32;

                // Trigger event
                events.0.push(PuzzleEvent::ObjectPlaced(PuzzleObjectPlacedEvent {
                    placement_entity: Entity::PLACEHOLDER, // Will be set by caller
                    placement_name: placement.name.clone(),
                    object_entity: placement.object_to_place.unwrap_or(Entity::PLACEHOLDER),
                }));

                // Check if puzzle is solved
                if progress.current_step >= progress.total_steps {
                    progress.state = PuzzleState::Solved;

                    // Trigger solved event
                    events.0.push(PuzzleEvent::Solved(PuzzleSolvedEvent {
                        puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                        puzzle_name: placement.name.clone(),
                        time_spent: progress.time_spent,
                        reset_count: progress.reset_count,
                    }));
                }
            }
        }
    }
}

/// System to update puzzle draggables
pub fn update_puzzle_draggables(
    time: Res<Time>,
    input: Res<InputState>,
    spatial_query: SpatialQuery,
    mut draggables: Query<(&mut PuzzleDraggable, &mut Transform)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut draggable, mut transform) in draggables.iter_mut() {
        if !draggable.can_grab {
            continue;
        }

        // Check for grab input
        if input.interact_pressed && !draggable.is_grabbed {
            // Raycast to check if object is hit
            // In a real implementation, you would cast a ray from the camera
            // For now, we'll assume the object is hit

            draggable.is_grabbed = true;
            draggable.is_rotating = false;

            // Store original position/rotation if not already stored
            if draggable.original_position == Vec3::ZERO {
                draggable.original_position = transform.translation;
                draggable.original_rotation = transform.rotation;
            }

            // Trigger event (would need to be specific to grabbing)
            info!("Grabbed draggable: {}", draggable.name);
        }

        // Check for release input
        if input.interact_pressed && draggable.is_grabbed {
            draggable.is_grabbed = false;
            info!("Released draggable: {}", draggable.name);
        }

        // Update grabbed object position
        if draggable.is_grabbed {
            // In a real implementation, you would update the position based on camera raycast
            // For now, we'll just log
            info!("Updating grabbed object position: {}", draggable.name);
        }
    }
}

/// System to update puzzle timers
pub fn update_puzzle_timers(
    time: Res<Time>,
    mut timers: Query<(&mut PuzzleTimer, &mut PuzzleProgress)>,
    mut events: ResMut<PuzzleEventQueue>,
) {
    for (mut timer, mut progress) in timers.iter_mut() {
        if !timer.active {
            continue;
        }

        // Update timer
        timer.current_time += time.delta_secs();

        // Check if timer is paused
        if progress.state == PuzzleState::Solved && timer.pause_on_solve {
            continue;
        }

        // Check if time limit is reached
        if timer.time_limit > 0.0 && timer.current_time >= timer.time_limit {
            timer.active = false;

            // Trigger timeout event
            if timer.use_event_on_timeout {
                events.0.push(PuzzleEvent::TimerTimeout(PuzzleTimerTimeoutEvent {
                    puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                    puzzle_name: String::new(), // Would be set by caller
                    time_spent: progress.time_spent,
                }));
            }

            // Mark puzzle as failed
            progress.state = PuzzleState::Failed;

            // Trigger failed event
            events.0.push(PuzzleEvent::Failed(PuzzleFailedEvent {
                puzzle_entity: Entity::PLACEHOLDER, // Will be set by caller
                puzzle_name: String::new(), // Would be set by caller
                reason: "Time limit reached".to_string(),
            }));
        }

        // Update progress time
        progress.time_spent += time.delta_secs();
    }
}

/// System to process puzzle events
pub fn process_puzzle_events(
    mut events: ResMut<PuzzleEventQueue>,
    mut solved_events: ResMut<PuzzleSolvedEventQueue>,
    mut failed_events: ResMut<PuzzleFailedEventQueue>,
    mut reset_events: ResMut<PuzzleResetEventQueue>,
) {
    for event in events.0.drain(..) {
        match event {
            PuzzleEvent::Solved(e) => {
                info!("Puzzle solved: {} (time: {:.1}s, resets: {})",
                    e.puzzle_name, e.time_spent, e.reset_count);
                solved_events.0.push(e);
            }
            PuzzleEvent::Failed(e) => {
                info!("Puzzle failed: {} - {}", e.puzzle_name, e.reason);
                failed_events.0.push(e);
            }
            PuzzleEvent::Reset(e) => {
                info!("Puzzle reset: {}", e.puzzle_name);
                reset_events.0.push(e);
            }
            PuzzleEvent::ButtonPressed(e) => {
                info!("Button pressed: {}", e.button_name);
            }
            PuzzleEvent::LeverMoved(e) => {
                info!("Lever moved: {} -> {:?}", e.lever_name, e.new_state);
            }
            PuzzleEvent::PressurePlatePressed(e) => {
                info!("Pressure plate pressed: {} (weight: {})", e.plate_name, e.weight);
            }
            PuzzleEvent::KeyUsed(e) => {
                info!("Key used: {} on lock: {} ({})",
                    e.key_name, e.lock_name, if e.success { "SUCCESS" } else { "FAILED" });
            }
            PuzzleEvent::SequenceItemPressed(e) => {
                info!("Sequence item pressed: {} (index: {}, correct: {})",
                    e.item_name, e.order_index, e.correct);
            }
            PuzzleEvent::PianoKeyPressed(e) => {
                info!("Piano key pressed: {} (note: {})", e.key_name, e.note);
            }
            PuzzleEvent::ObjectPlaced(e) => {
                info!("Object placed: {} (object: {:?})", e.placement_name, e.object_entity);
            }
            PuzzleEvent::TimerTimeout(e) => {
                info!("Puzzle timer timeout: {} (time: {:.1}s)", e.puzzle_name, e.time_spent);
            }
            PuzzleEvent::HintShown(e) => {
                info!("Puzzle hint shown: {} (level: {}, text: {})",
                    e.puzzle_name, e.hint_level, e.hint_text);
            }
        }
    }
}

/// System to update puzzle UI
pub fn update_puzzle_ui(
    ui_state: Res<PuzzleUIState>,
    mut prompt_query: Query<(&mut Visibility, &Children), With<PuzzlePrompt>>,
    mut hint_query: Query<(&mut Visibility, &Children), With<PuzzleHintUI>>,
    mut timer_query: Query<(&mut Visibility, &Children), With<PuzzleTimerUI>>,
    mut progress_query: Query<(&mut Visibility, &Children), With<PuzzleProgressUI>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    // Update prompt UI
    for (mut visibility, children) in prompt_query.iter_mut() {
        *visibility = if ui_state.is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.current_text.clone();
            }
        }
    }

    // Update hint UI
    for (mut visibility, children) in hint_query.iter_mut() {
        *visibility = if !ui_state.hint_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.hint_text.clone();
            }
        }
    }

    // Update timer UI
    for (mut visibility, children) in timer_query.iter_mut() {
        *visibility = if !ui_state.timer_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.timer_text.clone();
            }
        }
    }

    // Update progress UI
    for (mut visibility, children) in progress_query.iter_mut() {
        *visibility = if !ui_state.progress_text.is_empty() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for child in children.iter() {
            if let Ok((mut text, _)) = text_query.get_mut(child) {
                text.0 = ui_state.progress_text.clone();
            }
        }
    }
}

/// System to draw puzzle gizmos
pub fn debug_draw_puzzle_gizmos(
    debug_settings: Res<PuzzleDebugSettings>,
    gizmos_query: Query<(&Transform, &PuzzleGizmo)>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.enabled || !debug_settings.show_gizmos {
        return;
    }

    for (transform, gizmo) in gizmos_query.iter() {
        if !gizmo.show {
            continue;
        }

        // Draw sphere at puzzle position
        gizmos.sphere(
            transform.translation,
            gizmo.radius,
            gizmo.color,
        );

        // Draw arrow if needed
        if gizmo.arrow_length > 0.0 {
            let arrow_start = transform.translation;
            let arrow_end = arrow_start + transform.forward() * gizmo.arrow_line_length;
            gizmos.line(arrow_start, arrow_end, gizmo.arrow_color);
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to check if a puzzle is solved
pub fn is_puzzle_solved(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::Solved
}

/// Helper function to check if a puzzle is in progress
pub fn is_puzzle_in_progress(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::InProgress
}

/// Helper function to check if a puzzle is failed
pub fn is_puzzle_failed(puzzle_progress: &PuzzleProgress) -> bool {
    puzzle_progress.state == PuzzleState::Failed
}

/// Helper function to reset a puzzle
pub fn reset_puzzle(
    puzzle_progress: &mut PuzzleProgress,
    puzzle_buttons: &mut Query<&mut PuzzleButton>,
    puzzle_levers: &mut Query<&mut PuzzleLever>,
    puzzle_sequences: &mut Query<&mut PuzzleSequence>,
    puzzle_sequence_items: &mut Query<&mut PuzzleSequenceItem>,
) {
    // Reset progress
    puzzle_progress.state = PuzzleState::Unsolved;
    puzzle_progress.progress = 0.0;
    puzzle_progress.current_step = 0;
    puzzle_progress.reset_count += 1;

    // Reset buttons
    for mut button in puzzle_buttons.iter_mut() {
        button.is_pressed = false;
        button.can_press = true;
        button.cooldown_timer = 0.0;
        button.press_timer = 0.0;
        button.reset_timer = 0.0;
    }

    // Reset levers
    for mut lever in puzzle_levers.iter_mut() {
        lever.state = LeverState::Neutral;
        lever.current_angle = 0.0;
    }

    // Reset sequences
    for mut sequence in puzzle_sequences.iter_mut() {
        sequence.correct_index = 0;
        sequence.complete = false;
    }

    // Reset sequence items
    for mut item in puzzle_sequence_items.iter_mut() {
        item.pressed = false;
    }
}

/// Helper function to add a key to a lock
pub fn add_key_to_lock(
    lock: &mut PuzzleLock,
    key_id: &str,
) -> bool {
    if lock.is_unlocked {
        return false;
    }

    // Check if key is required
    if !lock.required_keys.contains(&key_id.to_string()) {
        return false;
    }

    // Add key
    lock.current_keys.push(key_id.to_string());

    // Check if lock is unlocked
    if lock.multi_key {
        if lock.current_keys.len() as u32 >= lock.required_key_count {
            lock.is_unlocked = true;
            return true;
        }
    } else {
        lock.is_unlocked = true;
        return true;
    }

    false
}

/// Helper function to press a sequence item
pub fn press_sequence_item(
    sequence: &mut PuzzleSequence,
    item: &mut PuzzleSequenceItem,
) -> bool {
    if sequence.complete {
        return false;
    }

    if item.order_index == sequence.correct_index {
        item.pressed = true;
        return true;
    }

    false
}

/// Helper function to press a piano key
pub fn press_piano_key(
    piano: &mut PuzzlePiano,
    key: &mut PuzzlePianoKey,
) -> bool {
    if !piano.using_piano {
        return false;
    }

    key.is_pressed = true;
    true
}

/// Helper function to place an object
pub fn place_object(
    placement: &mut PuzzleObjectPlacement,
    object_entity: Entity,
) -> bool {
    if placement.is_placed {
        return false;
    }

    placement.object_to_place = Some(object_entity);
    placement.object_inside_trigger = true;
    true
}

/// Helper function to grab a draggable object
pub fn grab_draggable(
    draggable: &mut PuzzleDraggable,
) -> bool {
    if !draggable.can_grab {
        return false;
    }

    draggable.is_grabbed = true;
    true
}

/// Helper function to release a draggable object
pub fn release_draggable(
    draggable: &mut PuzzleDraggable,
) {
    draggable.is_grabbed = false;
    draggable.is_rotating = false;
}

/// Helper function to update puzzle progress
pub fn update_progress(
    progress: &mut PuzzleProgress,
    total_steps: u32,
) {
    progress.total_steps = total_steps;
    progress.progress = progress.current_step as f32 / progress.total_steps as f32;
}

/// Helper function to show a hint
pub fn show_hint(
    hint: &mut PuzzleHint,
    ui_state: &mut PuzzleUIState,
) {
    if hint.level < hint.max_level {
        hint.level += 1;
        hint.visible = true;
        ui_state.hint_text = format!("Hint {}: {}", hint.level, hint.text);
    }
}

/// Helper function to update timer text
pub fn update_timer_text(
    timer: &PuzzleTimer,
    ui_state: &mut PuzzleUIState,
) {
    if timer.active && timer.time_limit > 0.0 {
        let remaining = timer.time_limit - timer.current_time;
        ui_state.timer_text = format!("Time: {:.1}s", remaining);
    } else {
        ui_state.timer_text = String::new();
    }
}

/// Helper function to update progress text
pub fn update_progress_text(
    progress: &PuzzleProgress,
    ui_state: &mut PuzzleUIState,
) {
    ui_state.progress_text = format!("Progress: {}/{} ({:.0}%)", 
        progress.current_step, 
        progress.total_steps, 
        progress.progress * 100.0
    );
}

/// System to handle puzzle interactions
/// This system integrates with the Interaction System's events
pub fn handle_puzzle_interactions(
    mut interaction_events: ResMut<InteractionEventQueue>,
    mut puzzle_events: ResMut<PuzzleEventQueue>,
    puzzle_interactables: Query<&PuzzleInteractable>,
    mut puzzle_buttons: Query<&mut PuzzleButton>,
    mut puzzle_levers: Query<&mut PuzzleLever>,
    mut puzzle_pressure_plates: Query<&mut PuzzlePressurePlate>,
    mut puzzle_locks: Query<&mut PuzzleLock>,
    mut puzzle_keys: Query<&mut PuzzleKey>,
    mut puzzle_sequences: Query<&mut PuzzleSequence>,
    mut puzzle_sequence_items: Query<&mut PuzzleSequenceItem>,
    mut puzzle_pianos: Query<&mut PuzzlePiano>,
    mut puzzle_piano_keys: Query<&mut PuzzlePianoKey>,
    mut puzzle_object_placements: Query<&mut PuzzleObjectPlacement>,
    mut puzzle_draggables: Query<&mut PuzzleDraggable>,
) {
    // Process interaction events
    for interaction_event in interaction_events.0.drain(..) {
        if let Ok(puzzle_interactable) = puzzle_interactables.get(interaction_event.target) {
            if !puzzle_interactable.active {
                continue;
            }

            match puzzle_interactable.interaction_type {
                PuzzleInteractionType::Button => {
                    if let Ok(mut button) = puzzle_buttons.get_mut(interaction_event.target) {
                        button.is_pressed = true;
                        button.can_press = false;
                        button.cooldown_timer = button.cooldown;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::ButtonPressed(PuzzleButtonPressedEvent {
                            button_entity: interaction_event.target,
                            button_name: button.name.clone(),
                        }));
                    }
                }
                PuzzleInteractionType::Lever => {
                    if let Ok(mut lever) = puzzle_levers.get_mut(interaction_event.target) {
                        // Toggle lever state
                        let new_state = match lever.state {
                            LeverState::Neutral => lever.target_state,
                            LeverState::Up => LeverState::Neutral,
                            LeverState::Down => LeverState::Neutral,
                            LeverState::Left => LeverState::Neutral,
                            LeverState::Right => LeverState::Neutral,
                        };

                        lever.state = new_state;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::LeverMoved(PuzzleLeverMovedEvent {
                            lever_entity: interaction_event.target,
                            lever_name: lever.name.clone(),
                            new_state: lever.state,
                        }));
                    }
                }
                PuzzleInteractionType::Lock => {
                    // Handle lock interaction
                    info!("Lock interaction for {:?}", interaction_event.target);
                }
                PuzzleInteractionType::PressurePlate => {
                    if let Ok(mut plate) = puzzle_pressure_plates.get_mut(interaction_event.target) {
                        plate.is_pressed = true;
                        plate.press_timer = plate.press_duration;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::PressurePlatePressed(PuzzlePressurePlatePressedEvent {
                            plate_entity: interaction_event.target,
                            plate_name: plate.name.clone(),
                            weight: plate.current_weight,
                        }));
                    }
                }
                PuzzleInteractionType::Key => {
                    if let Ok(mut key) = puzzle_keys.get_mut(interaction_event.target) {
                        // Find the lock this key should unlock
                        // In a real implementation, you would search for nearby locks
                        // For now, we'll just log
                        info!("Key used: {}", key.name);
                    }
                }
                PuzzleInteractionType::SequenceItem => {
                    if let Ok(mut item) = puzzle_sequence_items.get_mut(interaction_event.target) {
                        item.pressed = true;

                        // Find the sequence this item belongs to
                        // In a real implementation, you would search for the parent sequence
                        // For now, we'll just log
                        info!("Sequence item pressed: {}", item.name);
                    }
                }
                PuzzleInteractionType::PianoKey => {
                    if let Ok(mut key) = puzzle_piano_keys.get_mut(interaction_event.target) {
                        key.is_pressed = true;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::PianoKeyPressed(PuzzlePianoKeyPressedEvent {
                            key_entity: interaction_event.target,
                            key_name: key.name.clone(),
                            note: key.name.clone(),
                        }));
                    }
                }
                PuzzleInteractionType::ObjectPlacement => {
                    if let Ok(mut placement) = puzzle_object_placements.get_mut(interaction_event.target) {
                        placement.object_inside_trigger = true;

                        // Trigger event
                        puzzle_events.0.push(PuzzleEvent::ObjectPlaced(PuzzleObjectPlacedEvent {
                            placement_entity: interaction_event.target,
                            placement_name: placement.name.clone(),
                            object_entity: placement.object_to_place.unwrap_or(Entity::PLACEHOLDER),
                        }));
                    }
                }
                PuzzleInteractionType::Draggable => {
                    if let Ok(mut draggable) = puzzle_draggables.get_mut(interaction_event.target) {
                        draggable.is_grabbed = true;

                        // Trigger event (would need to be specific to grabbing)
                        info!("Draggable grabbed: {}", draggable.name);
                    }
                }
            }
        }
    }
}



use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::setup::GameStatsText;

// === UI AND FEEDBACK SYSTEMS ===

pub fn media_meter_system(
    mut media_meter: ResMut<MediaMeter>,
    mission_progress: Res<MissionProgress>,
    civilian_panic: Res<CivilianPanic>,
    time: Res<Time>,
) {
    // Media attention increases with:
    // - Time elapsed in operation
    // - Civilian casualties
    // - Government pressure
    // - Urban warfare intensity
    
    let time_factor = (mission_progress.time_elapsed / 1800.0) * 0.1; // Per 30 minutes
    let casualty_factor = civilian_panic.casualty_count as f32 * 0.05;
    let pressure_factor = mission_progress.government_pressure * 0.2;
    
    let attention_increase = (time_factor + casualty_factor + pressure_factor) * time.delta_seconds();
    media_meter.attention_level = (media_meter.attention_level + attention_increase).min(1.0);
    
    // Reputation changes based on actions
    if media_meter.attention_level > 0.5 {
        // High media attention affects both sides
        media_meter.reputation_cartel += 0.1 * time.delta_seconds(); // Cartel seen as powerful
        media_meter.reputation_government -= 0.15 * time.delta_seconds(); // Government seen as failing
    }
    
    // International observers arrive at high attention levels
    if media_meter.attention_level > 0.6 && !media_meter.international_observers {
        media_meter.international_observers = true;
        info!("BREAKING: International media and observers arrive in Culiacán");
    }
}

pub fn civilian_panic_system(
    mut civilian_panic: ResMut<CivilianPanic>,
    mut civilian_query: Query<(&Transform, &mut Civilian)>,
    combat_query: Query<&Transform, With<CombatTarget>>,
    roadblock_query: Query<&Transform, With<Roadblock>>,
    time: Res<Time>,
) {
    let mut total_panic = 0.0;
    let mut civilians_affected = 0;
    
    for (civilian_transform, mut civilian) in civilian_query.iter_mut() {
        let mut local_panic_factors = 0.0;
        
        // Panic increases near combat
        for combat_transform in combat_query.iter() {
            let distance = civilian_transform.translation.distance(combat_transform.translation);
            if distance < 150.0 {
                local_panic_factors += (150.0 - distance) / 150.0 * 0.5;
            }
        }
        
        // Panic increases near roadblocks (burning vehicles, chaos)
        for roadblock_transform in roadblock_query.iter() {
            let distance = civilian_transform.translation.distance(roadblock_transform.translation);
            if distance < 100.0 {
                local_panic_factors += (100.0 - distance) / 100.0 * 0.3;
            }
        }
        
        // Update individual civilian panic
        civilian.panic_level += local_panic_factors * time.delta_seconds();
        civilian.panic_level = civilian.panic_level.min(1.0);
        
        // Evacuation behavior
        if civilian.panic_level > 0.7 && civilian.evacuation_status == EvacuationStatus::Normal {
            civilian.evacuation_status = EvacuationStatus::Panicking;
        }
        
        total_panic += civilian.panic_level;
        civilians_affected += 1;
    }
    
    // Update city-wide panic level
    if civilians_affected > 0 {
        civilian_panic.city_wide_panic = total_panic / civilians_affected as f32;
    }
    
    // Evacuation rate increases with panic
    civilian_panic.evacuation_rate = civilian_panic.city_wide_panic * 0.1;
    
    // International concern rises with civilian impact
    if civilian_panic.casualty_count > 0 {
        civilian_panic.international_concern += 0.05 * time.delta_seconds();
    }
}

pub fn update_ui_system(
    mut text_query: Query<&mut Text, With<GameStatsText>>,
    media_meter: Res<MediaMeter>,
    mission_progress: Res<MissionProgress>,
    civilian_panic: Res<CivilianPanic>,
) {
    for mut text in text_query.iter_mut() {
        let minutes = (mission_progress.time_elapsed / 60.0) as u32;
        let seconds = (mission_progress.time_elapsed % 60.0) as u32;
        
        let media_level = match media_meter.attention_level {
            x if x < 0.2 => "Low",
            x if x < 0.5 => "Moderate", 
            x if x < 0.8 => "High",
            _ => "Critical",
        };
        
        let pressure_level = match mission_progress.government_pressure {
            x if x < 0.2 => "Low",
            x if x < 0.5 => "Moderate",
            x if x < 0.8 => "High", 
            _ => "Critical",
        };
        
        text.sections[0].value = format!(
            "Media Attention: {} | Government Pressure: {} | Civilian Panic: {:.0}% | Time: {:02}:{:02}",
            media_level,
            pressure_level,
            civilian_panic.city_wide_panic * 100.0,
            minutes,
            seconds
        );
    }
}

// === INPUT HANDLING ===

pub fn handle_player_input(
    input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut selected_units: ResMut<SelectedUnits>,
    mut unit_query: Query<(Entity, &Transform, &mut MovementTarget, &Unit)>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();
    
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            // Convert screen coordinates to world coordinates
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Clear previous selection
                for entity in &selected_units.units {
                    commands.entity(*entity).remove::<Selected>();
                }
                selected_units.units.clear();
                
                // Select units near click position
                for (entity, transform, _, unit) in unit_query.iter() {
                    if unit.faction == Faction::SinaloaCartel {
                        let distance = transform.translation.truncate().distance(world_pos);
                        if distance < 30.0 {
                            selected_units.units.push(entity);
                            commands.entity(entity).insert(Selected);
                        }
                    }
                }
            }
        }
    }
    
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Move selected units to right-click position
                for &unit_entity in &selected_units.units {
                    if let Ok((_, _, mut movement, _)) = unit_query.get_mut(unit_entity) {
                        movement.destination = world_pos;
                        movement.is_moving = true;
                    }
                }
            }
        }
    }
    
    // Special abilities hotkeys
    if input.just_pressed(KeyCode::KeyR) {
        info!("Roadblock deployment requested (handled by roadblock_system)");
    }
    
    if input.just_pressed(KeyCode::KeyH) {
        info!("Hostage operation initiated");
        // This would trigger hostage-taking behavior in selected units
    }
    
    if input.just_pressed(KeyCode::KeyP) {
        info!("Propaganda push activated");
        // This would boost cartel coordination and media attention
    }
}

pub fn camera_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let camera_speed = 300.0;
    
    let mut movement = Vec2::ZERO;
    
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    
    if movement != Vec2::ZERO {
        movement = movement.normalize();
        let movement_delta = movement * camera_speed * time.delta_seconds();
        camera_transform.translation += movement_delta.extend(0.0);
    }
}

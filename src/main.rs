use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

// ==================== COMPONENTS ====================

#[derive(Component, Clone)]
struct Unit {
    health: f32,
    max_health: f32,
    faction: Faction,
    unit_type: UnitType,
    damage: f32,
    range: f32,
    movement_speed: f32,
    target: Option<Entity>,
    attack_cooldown: Timer,
}

#[derive(Component)]
struct Movement {
    target_position: Option<Vec3>,
    speed: f32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct WaveSpawner {
    next_wave_timer: Timer,
    wave_number: u32,
    units_in_wave: u32,
}

#[derive(Component)]
struct Objective {
    objective_type: ObjectiveType,
    position: Vec3,
    radius: f32,
    health: f32,
}

// ==================== ENUMS & TYPES ====================

#[derive(Clone, PartialEq, Debug)]
enum Faction {
    Cartel,
    Military,
    Civilian,
}

#[derive(Clone, PartialEq, Debug)]
enum UnitType {
    // Cartel units
    Sicario,
    Enforcer,
    Roadblock,
    // Military units  
    Soldier,
    SpecialForces,
    Vehicle,
    // Special
    Ovidio, // High value target
}

#[derive(Clone, PartialEq, Debug)]
enum ObjectiveType {
    Safehouse,      // Cartel must defend
    ExtractionPoint, // Military tries to reach
    Checkpoint,     // Control points
}

// ==================== RESOURCES ====================

#[derive(Resource)]
struct GameState {
    mission_timer: f32,
    current_wave: u32,
    cartel_score: u32,
    military_score: u32,
    game_phase: GamePhase,
    ovidio_captured: bool,
}

#[derive(PartialEq, Debug, Clone)]
enum GamePhase {
    Preparation,    // Initial setup
    InitialRaid,   // Mission 1: Defend safehouse
    BlockConvoy,   // Mission 2: Block extraction
    ApplyPressure, // Mission 3: Escalate pressure
    HoldTheLine,   // Mission 4: Final showdown
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mission_timer: 0.0,
            current_wave: 0,
            cartel_score: 0,
            military_score: 0,
            game_phase: GamePhase::Preparation,
            ovidio_captured: false,
        }
    }
}

// ==================== MAIN FUNCTION ====================

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Battle of Culiacán - El Culiacanazo RTS".into(),
                resolution: (1400.0, 900.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            wave_spawner_system,
            unit_ai_system,
            movement_system,
            combat_system,
            health_bar_system,
            game_phase_system,
            handle_input,
            ui_system,
        ))
        .run();
}

// ==================== SETUP SYSTEM ====================

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());
    
    info!("🎮 Battle of Culiacán - October 17, 2019");
    info!("🏛️  Government forces attempt to capture Ovidio Guzmán López");
    info!("⚔️  Sinaloa Cartel prepares defensive operations");
    
    // Spawn Ovidio (High Value Target) in safehouse
    spawn_ovidio(&mut commands, Vec3::new(-300.0, 200.0, 0.0));
    
    // Spawn initial cartel defenders
    for i in 0..3 {
        spawn_unit(&mut commands, UnitType::Sicario, Faction::Cartel, 
                   Vec3::new(-250.0 + i as f32 * 50.0, 150.0, 0.0));
    }
    
    // Spawn safehouse objective
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.4, 0.3, 0.2),
                custom_size: Some(Vec2::new(80.0, 60.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 200.0, -1.0)),
            ..default()
        },
        Objective {
            objective_type: ObjectiveType::Safehouse,
            position: Vec3::new(-300.0, 200.0, 0.0),
            radius: 100.0,
            health: 200.0,
        },
    ));
    
    // Spawn wave spawner
    commands.spawn(WaveSpawner {
        next_wave_timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
        wave_number: 0,
        units_in_wave: 2,
    });
    
    info!("🎯 Mission: Defend Ovidio and prevent extraction!");
    info!("📱 Controls: SPACE=Roadblock, R=Reinforcements, ESC=Exit");
}

fn spawn_ovidio(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.8, 0.0), // Golden color for HVT
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Unit {
            health: 50.0,
            max_health: 50.0,
            faction: Faction::Cartel,
            unit_type: UnitType::Ovidio,
            damage: 0.0,
            range: 0.0,
            movement_speed: 60.0,
            target: None,
            attack_cooldown: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        },
        Movement {
            target_position: None,
            speed: 60.0,
        },
    ));
}

fn spawn_unit(commands: &mut Commands, unit_type: UnitType, faction: Faction, position: Vec3) {
    let (color, size, health, damage, range, speed) = match (&unit_type, &faction) {
        (UnitType::Sicario, Faction::Cartel) => 
            (Color::rgb(0.8, 0.1, 0.1), Vec2::new(15.0, 15.0), 80.0, 25.0, 120.0, 100.0),
        (UnitType::Enforcer, Faction::Cartel) => 
            (Color::rgb(0.6, 0.1, 0.1), Vec2::new(20.0, 20.0), 120.0, 40.0, 150.0, 80.0),
        (UnitType::Soldier, Faction::Military) => 
            (Color::rgb(0.1, 0.4, 0.1), Vec2::new(15.0, 15.0), 100.0, 30.0, 140.0, 90.0),
        (UnitType::SpecialForces, Faction::Military) => 
            (Color::rgb(0.1, 0.6, 0.1), Vec2::new(18.0, 18.0), 140.0, 50.0, 180.0, 110.0),
        (UnitType::Vehicle, Faction::Military) => 
            (Color::rgb(0.2, 0.5, 0.2), Vec2::new(30.0, 20.0), 200.0, 60.0, 200.0, 70.0),
        _ => (Color::GRAY, Vec2::new(15.0, 15.0), 100.0, 20.0, 100.0, 80.0),
    };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Unit {
            health,
            max_health: health,
            faction: faction.clone(),
            unit_type,
            damage,
            range,
            movement_speed: speed,
            target: None,
            attack_cooldown: Timer::new(Duration::from_millis(800), TimerMode::Repeating),
        },
        Movement {
            target_position: None,
            speed,
        },
    ));
}

// ==================== GAME SYSTEMS ====================

fn wave_spawner_system(
    time: Res<Time>,
    mut commands: Commands,
    mut wave_query: Query<&mut WaveSpawner>,
    mut game_state: ResMut<GameState>,
) {
    for mut spawner in wave_query.iter_mut() {
        spawner.next_wave_timer.tick(time.delta());
        
        if spawner.next_wave_timer.just_finished() {
            spawner.wave_number += 1;
            game_state.current_wave = spawner.wave_number;
            
            let wave_difficulty = (spawner.wave_number as f32 * 0.5 + 1.0).min(4.0);
            let units_to_spawn = (spawner.units_in_wave as f32 * wave_difficulty) as u32;
            
            info!("🌊 WAVE {} INCOMING! {} military units deployed", spawner.wave_number, units_to_spawn);
            
            // Spawn military units from different entry points
            let entry_points = vec![
                Vec3::new(600.0, 300.0, 0.0),   // North entry
                Vec3::new(600.0, 0.0, 0.0),     // East entry  
                Vec3::new(600.0, -300.0, 0.0),  // South entry
            ];
            
            for i in 0..units_to_spawn {
                let entry_point = entry_points[i as usize % entry_points.len()];
                let offset = Vec3::new(
                    thread_rng().gen_range(-50.0..50.0),
                    thread_rng().gen_range(-50.0..50.0), 
                    0.0
                );
                
                let unit_type = match spawner.wave_number {
                    1..=2 => UnitType::Soldier,
                    3..=4 => if thread_rng().gen_bool(0.7) { UnitType::Soldier } else { UnitType::SpecialForces },
                    _ => if thread_rng().gen_bool(0.4) { UnitType::Vehicle } else { UnitType::SpecialForces },
                };
                
                spawn_unit(&mut commands, unit_type, Faction::Military, entry_point + offset);
            }
            
            // Increase difficulty for next wave
            spawner.units_in_wave = (spawner.units_in_wave as f32 * 1.2) as u32;
        }
    }
}

fn unit_ai_system(
    mut unit_query: Query<(&mut Unit, &Transform, &mut Movement), Without<Objective>>,
    objective_query: Query<(&Objective, &Transform), Without<Unit>>,
    other_units: Query<(Entity, &Unit, &Transform)>,
) {
    for (mut unit, transform, mut movement) in unit_query.iter_mut() {
        match unit.faction {
            Faction::Military => {
                // Military AI: Find and attack cartel units or move to objectives
                if unit.target.is_none() {
                    // Find nearest enemy or objective
                    let mut nearest_target = None;
                    let mut nearest_distance = f32::MAX;
                    
                    // Check for enemy units in range
                    for (entity, other_unit, other_transform) in other_units.iter() {
                        if other_unit.faction == Faction::Cartel {
                            let distance = transform.translation.distance(other_transform.translation);
                            if distance < nearest_distance && distance <= unit.range {
                                nearest_distance = distance;
                                nearest_target = Some(entity);
                            }
                        }
                    }
                    
                    unit.target = nearest_target;
                    
                    // If no target, move toward safehouse
                    if unit.target.is_none() {
                        for (objective, obj_transform) in objective_query.iter() {
                            if objective.objective_type == ObjectiveType::Safehouse {
                                movement.target_position = Some(obj_transform.translation);
                                break;
                            }
                        }
                    }
                }
            },
            Faction::Cartel => {
                // Cartel AI: Defend position and attack nearby military units
                if unit.target.is_none() {
                    let mut nearest_enemy = None;
                    let mut nearest_distance = f32::MAX;
                    
                    for (entity, other_unit, other_transform) in other_units.iter() {
                        if other_unit.faction == Faction::Military {
                            let distance = transform.translation.distance(other_transform.translation);
                            if distance < nearest_distance && distance <= unit.range {
                                nearest_distance = distance;
                                nearest_enemy = Some(entity);
                            }
                        }
                    }
                    
                    unit.target = nearest_enemy;
                }
            },
            _ => {}
        }
    }
}

fn movement_system(
    time: Res<Time>,
    mut unit_query: Query<(&mut Transform, &Movement, &Unit)>,
    target_query: Query<&Transform, (With<Unit>, Without<Movement>)>,
) {
    for (mut transform, movement, unit) in unit_query.iter_mut() {
        if let Some(target_pos) = movement.target_position {
            let direction = (target_pos - transform.translation).normalize();
            let movement_delta = direction * movement.speed * time.delta_seconds();
            
            // Stop when close enough to target
            if transform.translation.distance(target_pos) > 20.0 {
                transform.translation += movement_delta;
            }
        }
        
        // Move toward target unit if attacking
        if let Some(target_entity) = unit.target {
            if let Ok(target_transform) = target_query.get(target_entity) {
                let direction = (target_transform.translation - transform.translation).normalize();
                let distance = transform.translation.distance(target_transform.translation);
                
                // Move closer if out of range
                if distance > unit.range * 0.8 {
                    let movement_delta = direction * unit.movement_speed * time.delta_seconds();
                    transform.translation += movement_delta;
                }
            }
        }
    }
}

fn combat_system(
    time: Res<Time>,
    mut commands: Commands,
    mut unit_query: Query<(Entity, &mut Unit, &Transform)>,
    mut game_state: ResMut<GameState>,
) {
    let mut combat_events = Vec::new();
    
    // Collect combat events first
    let units: Vec<(Entity, Unit, Transform)> = unit_query.iter()
        .map(|(e, u, t)| (e, u.clone(), *t))
        .collect();
    
    for (entity, mut unit, _transform) in unit_query.iter_mut() {
        unit.attack_cooldown.tick(time.delta());
        
        if let Some(target_entity) = unit.target {
            if unit.attack_cooldown.just_finished() {
                // Check if target still exists in our collected units
                if let Some((_, _, target_transform)) = units.iter()
                    .find(|(e, _, _)| *e == target_entity) {
                    let distance = _transform.translation.distance(target_transform.translation);
                    if distance <= unit.range {
                        combat_events.push((target_entity, unit.damage, unit.faction.clone()));
                    } else {
                        unit.target = None; // Target out of range
                    }
                } else {
                    unit.target = None; // Target no longer exists
                }
            }
        }
    }
    
    // Apply damage
    for (target_entity, damage, attacker_faction) in combat_events {
        if let Ok((entity, mut unit, transform)) = unit_query.get_mut(target_entity) {
            unit.health -= damage;
            
            // Visual feedback - spawn damage indicator
            let indicator_color = match attacker_faction {
                Faction::Military => Color::rgb(0.2, 0.8, 0.2),
                Faction::Cartel => Color::rgb(0.8, 0.2, 0.2),
                _ => Color::WHITE,
            };
            
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: indicator_color,
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation + Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            });
            
            if unit.health <= 0.0 {
                match unit.faction {
                    Faction::Cartel => {
                        game_state.military_score += 10;
                        if unit.unit_type == UnitType::Ovidio {
                            game_state.ovidio_captured = true;
                            info!("🎯 CRITICAL: Ovidio Guzmán López captured!");
                        }
                    },
                    Faction::Military => {
                        game_state.cartel_score += 15;
                    },
                    _ => {}
                }
                
                commands.entity(entity).despawn();
            }
        }
    }
}

fn health_bar_system(
    _unit_query: Query<(Entity, &Unit, &Transform), Changed<Unit>>,
) {
    // Simple health bar system - could be enhanced with actual health bar sprites
    // Currently just a placeholder
}

fn game_phase_system(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    unit_query: Query<&Unit>,
) {
    game_state.mission_timer += time.delta_seconds();
    
    let cartel_alive = unit_query.iter().any(|u| u.faction == Faction::Cartel && u.unit_type != UnitType::Roadblock);
    let military_alive = unit_query.iter().any(|u| u.faction == Faction::Military);
    let ovidio_alive = unit_query.iter().any(|u| u.unit_type == UnitType::Ovidio && u.health > 0.0);
    
    // Phase transitions based on time and events
    let new_phase = match game_state.game_phase {
        GamePhase::Preparation if game_state.mission_timer > 5.0 => {
            info!("🚁 Phase 1: INITIAL RAID - Government forces storm the safehouse!");
            GamePhase::InitialRaid
        },
        GamePhase::InitialRaid if game_state.mission_timer > 120.0 => {
            info!("🛑 Phase 2: BLOCK CONVOY - Cut off all escape routes!");
            GamePhase::BlockConvoy  
        },
        GamePhase::BlockConvoy if game_state.mission_timer > 300.0 => {
            info!("👨‍👩‍👧‍👦 Phase 3: APPLY PRESSURE - Target military families!");
            GamePhase::ApplyPressure
        },
        GamePhase::ApplyPressure if game_state.mission_timer > 480.0 => {
            info!("⏰ Phase 4: HOLD THE LINE - Final showdown!");
            GamePhase::HoldTheLine
        },
        _ => game_state.game_phase.clone(),
    };
    
    if new_phase != game_state.game_phase {
        game_state.game_phase = new_phase;
    }
    
    // Victory/Defeat conditions
    if !ovidio_alive && !game_state.ovidio_captured {
        info!("💀 DEFEAT: Ovidio Guzmán López was killed in the operation");
        game_state.game_phase = GamePhase::GameOver;
    } else if game_state.ovidio_captured && !cartel_alive {
        info!("🎖️ MILITARY VICTORY: Target captured, cartel eliminated");  
        game_state.game_phase = GamePhase::GameOver;
    } else if game_state.mission_timer > 600.0 && cartel_alive {
        info!("🏆 HISTORICAL OUTCOME: Government releases Ovidio to prevent casualties");
        info!("📰 'El Culiacanazo' - Cartel demonstrates power over the state");
        game_state.game_phase = GamePhase::GameOver;
    }
}

fn ui_system(
    game_state: Res<GameState>,
    unit_query: Query<&Unit>,
) {
    // This is a placeholder for UI rendering
    // In a full implementation, we'd use bevy_ui for proper UI elements
    
    // Count units for status display
    let cartel_count = unit_query.iter().filter(|u| u.faction == Faction::Cartel && u.unit_type != UnitType::Roadblock).count();
    let military_count = unit_query.iter().filter(|u| u.faction == Faction::Military).count();
    let ovidio_alive = unit_query.iter().any(|u| u.unit_type == UnitType::Ovidio);
    
    // Log status every 30 seconds
    if (game_state.mission_timer % 30.0) < 0.1 {
        info!("📊 STATUS | Wave: {} | Cartel: {} | Military: {} | Ovidio: {} | Time: {:.0}s", 
              game_state.current_wave, cartel_count, military_count, 
              if ovidio_alive { "ALIVE" } else { "CAPTURED/KIA" }, 
              game_state.mission_timer);
    }
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if input.just_pressed(KeyCode::Space) {
        // Deploy roadblock
        let position = Vec3::new(
            thread_rng().gen_range(-400.0..400.0),
            thread_rng().gen_range(-300.0..300.0),
            0.0
        );
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.6, 0.3, 0.0),
                    custom_size: Some(Vec2::new(60.0, 25.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Unit {
                health: 50.0,
                max_health: 50.0,
                faction: Faction::Cartel,
                unit_type: UnitType::Roadblock,
                damage: 0.0,
                range: 0.0,
                movement_speed: 0.0,
                target: None,
                attack_cooldown: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            },
        ));
        
        info!("🛑 ROADBLOCK deployed! Military convoy movement disrupted");
        game_state.cartel_score += 5;
    }
    
    if input.just_pressed(KeyCode::R) {
        // Call reinforcements
        let spawn_positions = vec![
            Vec3::new(-400.0, 200.0, 0.0),
            Vec3::new(-350.0, 150.0, 0.0),
            Vec3::new(-400.0, 100.0, 0.0),
        ];
        
        for (i, position) in spawn_positions.iter().enumerate() {
            let unit_type = if i == 0 { UnitType::Enforcer } else { UnitType::Sicario };
            spawn_unit(&mut commands, unit_type, Faction::Cartel, *position);
        }
        
        info!("📱 REINFORCEMENTS arriving! Cartel sends backup to the safehouse");
        game_state.cartel_score += 10;
    }
    
    if input.just_pressed(KeyCode::Escape) {
        info!("🏛️ SIMULATION ENDED");
        info!("📚 Historical Note: The real Battle of Culiacán ended with the government releasing Ovidio Guzmán López");
        info!("⚖️ This demonstrated the complex balance of power between organized crime and the state in Mexico");
        std::process::exit(0);
    }
    
    // Debug keys
    if input.just_pressed(KeyCode::F1) {
        info!("🎮 CONTROLS:");
        info!("SPACE - Deploy roadblock (block military movement)");
        info!("R - Call reinforcements (spawn cartel backup)");  
        info!("ESC - End simulation");
        info!("F1 - Show this help");
    }
}

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::game::GameState;

// === MISSION SYSTEMS ===

pub fn mission_trigger_system(
    mut mission_progress: ResMut<MissionProgress>,
    mut next_state: ResMut<NextState<GameState>>,
    ovidio_query: Query<&OvidioGuzmán>,
    time: Res<Time>,
) {
    mission_progress.time_elapsed += time.delta_seconds();
    
    if let Ok(ovidio) = ovidio_query.get_single() {
        mission_progress.ovidio_status = ovidio.capture_status.clone();
        
        // Mission progression logic
        match mission_progress.current_mission {
            0 => {
                // Start Mission 1 when Ovidio is located by military
                if ovidio.location_known {
                    mission_progress.current_mission = 1;
                    next_state.set(GameState::Mission1);
                    info!("Mission 1 Started: Initial Raid - Military has located Ovidio!");
                }
            }
            1 => {
                // Progress to Mission 2 if Ovidio is captured
                if ovidio.capture_status == CaptureStatus::Captured {
                    mission_progress.current_mission = 2;
                    next_state.set(GameState::Mission2);
                    info!("Mission 2 Started: Cut Off the Convoy - Ovidio captured, prevent extraction!");
                }
            }
            2 => {
                // Progress to Mission 3 if convoy is blocked for sufficient time
                if mission_progress.time_elapsed > 600.0 { // 10 minutes
                    mission_progress.current_mission = 3;
                    next_state.set(GameState::Mission3);
                    info!("Mission 3 Started: Apply Pressure - Escalate to force government retreat!");
                }
            }
            3 => {
                // Progress to Mission 4 if pressure tactics are successful
                if mission_progress.government_pressure > 0.7 {
                    mission_progress.current_mission = 4;
                    next_state.set(GameState::Mission4);
                    info!("Mission 4 Started: Hold the Line - Government under maximum pressure!");
                }
            }
            4 => {
                // Victory condition: Government releases Ovidio
                if ovidio.capture_status == CaptureStatus::Released {
                    next_state.set(GameState::Victory);
                    info!("VICTORY: Government releases Ovidio Guzmán - Cartel achieves objectives!");
                }
                
                // Failure condition: Ovidio successfully extracted
                if ovidio.extraction_progress >= 1.0 {
                    next_state.set(GameState::GameOver);
                    info!("DEFEAT: Ovidio Guzmán successfully extracted by military forces");
                }
            }
            _ => {}
        }
    }
}

pub fn ovidio_capture_system(
    mut ovidio_query: Query<(&Transform, &mut OvidioGuzmán)>,
    military_query: Query<&Transform, (With<MilitaryInfantry>, Without<OvidioGuzmán>)>,
    mut mission_progress: ResMut<MissionProgress>,
) {
    if let Ok((ovidio_transform, mut ovidio)) = ovidio_query.get_single_mut() {
        let mut closest_military_distance = f32::INFINITY;
        let mut military_nearby = false;
        
        // Check if military units are near Ovidio
        for military_transform in military_query.iter() {
            let distance = ovidio_transform.translation.distance(military_transform.translation);
            if distance < closest_military_distance {
                closest_military_distance = distance;
            }
            
            if distance < 30.0 { // Capture range
                military_nearby = true;
            }
        }
        
        // Update Ovidio's status based on military proximity
        if military_nearby && ovidio.capture_status == CaptureStatus::Free {
            ovidio.location_known = true;
            
            // Start capture sequence
            if closest_military_distance < 15.0 {
                ovidio.capture_status = CaptureStatus::Captured;
                info!("Ovidio Guzmán has been captured by military forces!");
            }
        }
        
        // Government pressure affects release decision
        if ovidio.capture_status == CaptureStatus::Captured && 
           mission_progress.government_pressure > 0.8 {
            ovidio.capture_status = CaptureStatus::Released;
            info!("BREAKING: Government orders release of Ovidio Guzmán!");
        }
    }
}

pub fn government_pressure_system(
    mut mission_progress: ResMut<MissionProgress>,
    media_meter: Res<MediaMeter>,
    civilian_panic: Res<CivilianPanic>,
    asymmetric_balance: Res<AsymmetricBalance>,
    roadblock_query: Query<&Roadblock>,
    hostage_query: Query<&HostageSquad>,
    time: Res<Time>,
) {
    let mut pressure_factors = 0.0;
    
    // Media attention increases pressure
    pressure_factors += media_meter.attention_level * 0.3;
    
    // International observers multiply pressure
    if media_meter.international_observers {
        pressure_factors *= 1.5;
    }
    
    // Civilian casualties create political pressure
    pressure_factors += civilian_panic.casualty_count as f32 * 0.05;
    
    // Roadblocks show cartel control of city
    let roadblock_count = roadblock_query.iter().count();
    pressure_factors += roadblock_count as f32 * 0.1;
    
    // Hostage situations create immediate pressure
    let total_hostages: u8 = hostage_query.iter().map(|h| h.hostages_held).sum();
    pressure_factors += total_hostages as f32 * 0.15;
    
    // Military morale affects government confidence
    pressure_factors += (1.0 - asymmetric_balance.military_morale) * 0.2;
    
    // Time pressure - longer operations are politically costly
    let time_pressure = (mission_progress.time_elapsed / 3600.0) * 0.1; // Per hour
    pressure_factors += time_pressure;
    
    mission_progress.government_pressure += pressure_factors * time.delta_seconds();
    mission_progress.government_pressure = mission_progress.government_pressure.min(1.0);
    
    // Log significant pressure events
    if mission_progress.government_pressure > 0.5 && mission_progress.government_pressure - pressure_factors * time.delta_seconds() <= 0.5 {
        info!("Government under significant pressure - questioning operation viability");
    }
    
    if mission_progress.government_pressure > 0.8 {
        info!("CRITICAL: Government considering retreat from operation");
    }
}

// === MISSION SETUP SYSTEMS ===

pub fn setup_mission_1(
    mut commands: Commands,
    mut ovidio_query: Query<&mut OvidioGuzmán>,
) {
    info!("Setting up Mission 1: Initial Raid");
    
    if let Ok(mut ovidio) = ovidio_query.get_single_mut() {
        ovidio.location_known = true;
    }
    
    // Spawn additional military units for the raid
    let raid_positions = vec![
        Vec2::new(170.0, 280.0),
        Vec2::new(230.0, 320.0),
        Vec2::new(190.0, 270.0),
    ];
    
    for pos in raid_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.6, 0.1), // Dark green - special forces
                    custom_size: Some(Vec2::new(12.0, 12.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            Unit {
                unit_type: UnitType::ExtractionTeam,
                faction: Faction::MexicanMilitary,
                health: 120.0,
                max_health: 120.0,
                damage: 30.0,
                range: 110.0,
                movement_speed: 110.0,
            },
            MilitaryInfantry {
                unit_cohesion: 0.9,
                morale: 1.0,
            },
            MovementTarget {
                destination: Vec2::new(200.0, 300.0), // Move towards Ovidio
                is_moving: true,
            },
            CombatTarget {
                target_entity: None,
                last_attack_time: 0.0,
                attack_cooldown: 1.5,
            },
        ));
    }
}

pub fn setup_mission_2(mut commands: Commands) {
    info!("Setting up Mission 2: Cut Off the Convoy");
    
    // Spawn convoy vehicles
    let convoy_positions = vec![
        Vec2::new(210.0, 310.0), // Near Ovidio's location
        Vec2::new(220.0, 300.0),
    ];
    
    for pos in convoy_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.7, 0.2), // Medium green - armored vehicles
                    custom_size: Some(Vec2::new(20.0, 12.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            Unit {
                unit_type: UnitType::TacticalConvoy,
                faction: Faction::MexicanMilitary,
                health: 200.0,
                max_health: 200.0,
                damage: 15.0,
                range: 80.0,
                movement_speed: 90.0,
            },
            TacticalConvoy {
                armor_rating: 0.7,
                convoy_formation: true,
            },
            MovementTarget {
                destination: Vec2::new(600.0, 200.0), // Head to military base
                is_moving: true,
            },
            CombatTarget {
                target_entity: None,
                last_attack_time: 0.0,
                attack_cooldown: 3.0,
            },
        ));
    }
}

pub fn setup_mission_3(mut commands: Commands) {
    info!("Setting up Mission 3: Apply Pressure");
    
    // Spawn civilians in military housing areas
    let civilian_positions = vec![
        Vec2::new(150.0, 400.0),
        Vec2::new(165.0, 410.0),
        Vec2::new(180.0, 420.0),
        Vec2::new(160.0, 390.0),
        Vec2::new(175.0, 405.0),
    ];
    
    for pos in civilian_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 0.8), // Light gray - civilians
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.5)),
                ..default()
            },
            Unit {
                unit_type: UnitType::Civilian,
                faction: Faction::Civilian,
                health: 50.0,
                max_health: 50.0,
                damage: 0.0,
                range: 0.0,
                movement_speed: 60.0,
            },
            Civilian {
                panic_level: 0.3,
                evacuation_status: EvacuationStatus::Normal,
            },
            MovementTarget {
                destination: pos,
                is_moving: false,
            },
        ));
    }
}

pub fn setup_mission_4(mut commands: Commands) {
    info!("Setting up Mission 4: Hold the Line");
    
    // Spawn final wave of military reinforcements
    let reinforcement_positions = vec![
        Vec2::new(650.0, 150.0),
        Vec2::new(670.0, 170.0),
        Vec2::new(630.0, 140.0),
        Vec2::new(660.0, 190.0),
    ];
    
    for pos in reinforcement_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.5, 0.0), // Dark green - elite forces
                    custom_size: Some(Vec2::new(14.0, 14.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            Unit {
                unit_type: UnitType::Infantry,
                faction: Faction::MexicanMilitary,
                health: 150.0,
                max_health: 150.0,
                damage: 35.0,
                range: 140.0,
                movement_speed: 95.0,
            },
            MilitaryInfantry {
                unit_cohesion: 1.0,
                morale: 0.8, // Lower due to prolonged operation
            },
            MovementTarget {
                destination: Vec2::new(400.0, 400.0), // Move to city center
                is_moving: true,
            },
            CombatTarget {
                target_entity: None,
                last_attack_time: 0.0,
                attack_cooldown: 1.2,
            },
        ));
    }
}

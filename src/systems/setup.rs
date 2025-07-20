use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::game::GameState;

// === STARTUP SYSTEMS ===

pub fn setup_camera(mut commands: Commands) {
    // Spawn 2D camera for RTS top-down view
    commands.spawn(Camera2dBundle::default());
}

pub fn load_culiacan_map(mut commands: Commands, mut game_map: ResMut<GameMap>) {
    info!("Loading Culiacán map...");
    
    // Initialize map zones based on real Culiacán geography
    game_map.width = 1200.0;
    game_map.height = 800.0;
    
    // Key zones from the Battle of Culiacán
    let zones = vec![
        MapZoneData {
            zone_type: ZoneType::TresRíos,
            position: Vec2::new(200.0, 300.0),
            size: Vec2::new(150.0, 100.0),
            initial_control: Faction::SinaloaCartel,
            strategic_value: 0.9, // High - where Ovidio was located
            civilian_density: 0.8,
            infrastructure_level: 0.7,
        },
        MapZoneData {
            zone_type: ZoneType::MilitaryBase,
            position: Vec2::new(600.0, 200.0),
            size: Vec2::new(100.0, 80.0),
            initial_control: Faction::MexicanMilitary,
            strategic_value: 0.8,
            civilian_density: 0.1,
            infrastructure_level: 0.9,
        },
        MapZoneData {
            zone_type: ZoneType::CityCenter,
            position: Vec2::new(400.0, 400.0),
            size: Vec2::new(200.0, 150.0),
            initial_control: Faction::Civilian,
            strategic_value: 0.6,
            civilian_density: 1.0,
            infrastructure_level: 0.8,
        },
        MapZoneData {
            zone_type: ZoneType::Airport,
            position: Vec2::new(100.0, 600.0),
            size: Vec2::new(120.0, 90.0),
            initial_control: Faction::MexicanMilitary,
            strategic_value: 0.7, // Potential escape route
            civilian_density: 0.3,
            infrastructure_level: 0.9,
        },
    ];
    
    // Spawn map zone entities
    for (i, zone_data) in zones.iter().enumerate() {
        let zone_id = format!("zone_{}", i);
        game_map.zones.insert(zone_id, zone_data.clone());
        
        // Visual representation of zones
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match zone_data.zone_type {
                        ZoneType::TresRíos => Color::rgb(0.8, 0.3, 0.3), // Red - cartel controlled
                        ZoneType::MilitaryBase => Color::rgb(0.3, 0.8, 0.3), // Green - military
                        ZoneType::CityCenter => Color::rgb(0.8, 0.8, 0.8), // Gray - civilian
                        ZoneType::Airport => Color::rgb(0.3, 0.3, 0.8), // Blue - strategic
                        _ => Color::rgb(0.5, 0.5, 0.5),
                    },
                    custom_size: Some(zone_data.size),
                    ..default()
                },
                transform: Transform::from_translation(zone_data.position.extend(0.0)),
                ..default()
            },
            MapZone {
                zone_type: zone_data.zone_type.clone(),
                control_level: 1.0,
                cartel_influence: if zone_data.initial_control == Faction::SinaloaCartel { 1.0 } else { 0.0 },
                military_presence: if zone_data.initial_control == Faction::MexicanMilitary { 1.0 } else { 0.0 },
            },
        ));
    }
    
    // Add strategic points
    let strategic_points = vec![
        StrategicPointData {
            point_type: StrategicPointType::Intersection,
            position: Vec2::new(300.0, 280.0),
            importance: 0.8,
            requires_holding_time: 30.0,
            defensive_bonus: 0.2,
        },
        StrategicPointData {
            point_type: StrategicPointType::Bridge,
            position: Vec2::new(450.0, 350.0),
            importance: 0.7,
            requires_holding_time: 45.0,
            defensive_bonus: 0.3,
        },
        StrategicPointData {
            point_type: StrategicPointType::Communications_Tower,
            position: Vec2::new(350.0, 500.0),
            importance: 0.6,
            requires_holding_time: 60.0,
            defensive_bonus: 0.1,
        },
    ];
    
    for (_i, point_data) in strategic_points.iter().enumerate() {
        game_map.strategic_points.push(point_data.clone());
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0), // Yellow for strategic points
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_translation(point_data.position.extend(1.0)),
                ..default()
            },
            StrategicPoint {
                point_type: point_data.point_type.clone(),
                importance: point_data.importance,
                contested: false,
            },
        ));
    }
}

pub fn spawn_initial_units(mut commands: Commands) {
    info!("Spawning initial units...");
    
    // Spawn Ovidio Guzmán at Tres Ríos location
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0), // Red - high value target
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, 300.0, 2.0)),
            ..default()
        },
        Unit {
            unit_type: UnitType::OvidioGuzmán,
            faction: Faction::SinaloaCartel,
            health: 100.0,
            max_health: 100.0,
            damage: 0.0, // Non-combatant
            range: 0.0,
            movement_speed: 80.0,
        },
        OvidioGuzmán {
            capture_status: CaptureStatus::Free,
            location_known: false,
            extraction_progress: 0.0,
        },
        MovementTarget {
            destination: Vec2::new(200.0, 300.0),
            is_moving: false,
        },
    ));
    
    // Spawn initial cartel sicarios around Tres Ríos
    let cartel_positions = vec![
        Vec2::new(180.0, 320.0),
        Vec2::new(220.0, 280.0),
        Vec2::new(190.0, 290.0),
        Vec2::new(210.0, 310.0),
    ];
    
    for pos in cartel_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2), // Dark red - cartel
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            Unit {
                unit_type: UnitType::Sicario,
                faction: Faction::SinaloaCartel,
                health: 80.0,
                max_health: 80.0,
                damage: 25.0,
                range: 100.0,
                movement_speed: 120.0,
            },
            Sicario {
                experience_level: 2,
                coordination_bonus: 0.2,
            },
            MovementTarget {
                destination: pos,
                is_moving: false,
            },
            CombatTarget {
                target_entity: None,
                last_attack_time: 0.0,
                attack_cooldown: 2.0,
            },
        ));
    }
    
    // Spawn initial military units at base
    let military_positions = vec![
        Vec2::new(580.0, 190.0),
        Vec2::new(600.0, 210.0),
        Vec2::new(620.0, 180.0),
    ];
    
    for pos in military_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.8, 0.2), // Green - military
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            Unit {
                unit_type: UnitType::Infantry,
                faction: Faction::MexicanMilitary,
                health: 100.0,
                max_health: 100.0,
                damage: 20.0,
                range: 120.0,
                movement_speed: 100.0,
            },
            MilitaryInfantry {
                unit_cohesion: 0.8,
                morale: 0.9,
            },
            MovementTarget {
                destination: pos,
                is_moving: false,
            },
            CombatTarget {
                target_entity: None,
                last_attack_time: 0.0,
                attack_cooldown: 1.8,
            },
        ));
    }
}

pub fn setup_ui(mut commands: Commands) {
    // Create UI root
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top panel - Mission info
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Battle of Culiacán - October 17, 2019",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            
            // Bottom panel - Game stats
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        left: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Media Attention: Low | Government Pressure: Low | Time: 00:00",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        GameStatsText,
                    ));
                });
        });
}

// UI marker component
#[derive(Component)]
pub struct GameStatsText;

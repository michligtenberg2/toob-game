use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::*;
use std::collections::HashMap;

// === GAME STATE RESOURCES ===

#[derive(Resource, Default)]
pub struct GameMap {
    pub zones: HashMap<String, MapZoneData>,
    pub strategic_points: Vec<StrategicPointData>,
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Default)]
pub struct MissionProgress {
    pub current_mission: u8,
    pub objectives_completed: Vec<bool>,
    pub time_elapsed: f32,
    pub ovidio_status: CaptureStatus,
    pub government_pressure: f32,
}

#[derive(Resource, Default)]
pub struct MediaMeter {
    pub attention_level: f32,        // 0.0 to 1.0
    pub international_observers: bool,
    pub headline_timer: f32,
    pub reputation_cartel: f32,      // Affects recruitment and coordination
    pub reputation_government: f32,   // Affects military morale
}

#[derive(Resource, Default)]
pub struct CivilianPanic {
    pub city_wide_panic: f32,       // 0.0 to 1.0
    pub evacuation_rate: f32,
    pub casualty_count: u32,
    pub international_concern: f32,
}

#[derive(Resource, Default)]
pub struct AsymmetricBalance {
    pub cartel_coordination: f32,    // Improves with successful operations
    pub military_morale: f32,        // Decreases with failures
    pub urban_control_bonus: f32,    // Cartel advantage in city
    pub air_superiority_penalty: f32, // Military advantage
}

#[derive(Resource, Default)]
pub struct SelectedUnits {
    pub units: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct GameTimer {
    pub mission_start_time: f32,
    pub current_time: f32,
    pub escalation_thresholds: Vec<f32>,
}

// === MAP DATA STRUCTURES ===

#[derive(Clone, Serialize, Deserialize)]
pub struct MapZoneData {
    pub zone_type: ZoneType,
    pub position: Vec2,
    pub size: Vec2,
    pub initial_control: Faction,
    pub strategic_value: f32,
    pub civilian_density: f32,
    pub infrastructure_level: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StrategicPointData {
    pub point_type: StrategicPointType,
    pub position: Vec2,
    pub importance: f32,
    pub requires_holding_time: f32,
    pub defensive_bonus: f32,
}

// === MISSION CONFIGURATION ===

#[derive(Resource)]
pub struct MissionConfig {
    pub mission_1: Mission1Config,
    pub mission_2: Mission2Config, 
    pub mission_3: Mission3Config,
    pub mission_4: Mission4Config,
}

impl Default for MissionConfig {
    fn default() -> Self {
        Self {
            mission_1: Mission1Config {
                ovidio_location: Vec2::new(200.0, 300.0), // Tres Ríos area
                military_spawn_points: vec![
                    Vec2::new(100.0, 100.0),
                    Vec2::new(150.0, 120.0),
                ],
                cartel_response_time: 60.0, // seconds
                objectives: vec![
                    "Delay Ovidio's capture".to_string(),
                    "Minimize cartel casualties".to_string(),
                    "Establish urban control".to_string(),
                ],
            },
            mission_2: Mission2Config {
                convoy_route: vec![
                    Vec2::new(200.0, 300.0), // Start at Tres Ríos
                    Vec2::new(400.0, 250.0), // Highway intersection
                    Vec2::new(600.0, 200.0), // Military base
                ],
                roadblock_positions: vec![
                    Vec2::new(300.0, 280.0),
                    Vec2::new(350.0, 260.0),
                    Vec2::new(500.0, 230.0),
                ],
                extraction_time_limit: 900.0, // 15 minutes
            },
            mission_3: Mission3Config {
                military_housing_locations: vec![
                    Vec2::new(150.0, 400.0),
                    Vec2::new(180.0, 420.0),
                    Vec2::new(220.0, 380.0),
                ],
                pressure_threshold: 0.7,
                escalation_penalties: vec![0.1, 0.3, 0.5, 0.8], // Per civilian casualty
            },
            mission_4: Mission4Config {
                government_retreat_threshold: 0.8,
                international_pressure_multiplier: 1.5,
                final_wave_intensity: 2.0,
            },
        }
    }
}

pub struct Mission1Config {
    pub ovidio_location: Vec2,
    pub military_spawn_points: Vec<Vec2>,
    pub cartel_response_time: f32,
    pub objectives: Vec<String>,
}

pub struct Mission2Config {
    pub convoy_route: Vec<Vec2>,
    pub roadblock_positions: Vec<Vec2>,
    pub extraction_time_limit: f32,
}

pub struct Mission3Config {
    pub military_housing_locations: Vec<Vec2>,
    pub pressure_threshold: f32,
    pub escalation_penalties: Vec<f32>,
}

pub struct Mission4Config {
    pub government_retreat_threshold: f32,
    pub international_pressure_multiplier: f32,
    pub final_wave_intensity: f32,
}

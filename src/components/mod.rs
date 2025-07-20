use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// === UNIT COMPONENTS ===

#[derive(Component)]
pub struct Unit {
    pub unit_type: UnitType,
    pub faction: Faction,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub range: f32,
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct MovementTarget {
    pub destination: Vec2,
    pub is_moving: bool,
}

#[derive(Component)]
pub struct CombatTarget {
    pub target_entity: Option<Entity>,
    pub last_attack_time: f32,
    pub attack_cooldown: f32,
}

// === CARTEL-SPECIFIC COMPONENTS ===

#[derive(Component)]
pub struct Sicario {
    pub experience_level: u8,
    pub coordination_bonus: f32,
}

#[derive(Component)]
pub struct NarcoTechTruck {
    pub drone_jammer_active: bool,
    pub roadblock_supplies: u8,
}

#[derive(Component)]
pub struct HostageSquad {
    pub hostages_held: u8,
    pub pressure_value: f32,
}

// === MILITARY COMPONENTS ===

#[derive(Component)]
pub struct MilitaryInfantry {
    pub unit_cohesion: f32,
    pub morale: f32,
}

#[derive(Component)]
pub struct TacticalConvoy {
    pub armor_rating: f32,
    pub convoy_formation: bool,
}

#[derive(Component)]
pub struct AirSupport {
    pub fuel_remaining: f32,
    pub altitude: f32,
}

// === SPECIAL COMPONENTS ===

#[derive(Component)]
pub struct OvidioGuzmán {
    pub capture_status: CaptureStatus,
    pub location_known: bool,
    pub extraction_progress: f32,
}

#[derive(Component)]
pub struct Roadblock {
    pub blocking_strength: f32,
    pub burn_timer: f32,
}

#[derive(Component)]
pub struct Civilian {
    pub panic_level: f32,
    pub evacuation_status: EvacuationStatus,
}

// === MAP COMPONENTS ===

#[derive(Component)]
pub struct MapZone {
    pub zone_type: ZoneType,
    pub control_level: f32,
    pub cartel_influence: f32,
    pub military_presence: f32,
}

#[derive(Component)]
pub struct StrategicPoint {
    pub point_type: StrategicPointType,
    pub importance: f32,
    pub contested: bool,
}

// === ENUMS ===

#[derive(Clone, PartialEq)]
pub enum UnitType {
    // Cartel units
    Sicario,
    NarcoTechTruck,
    DroneBlocker,
    HostageSquad,
    
    // Military units
    Infantry,
    TacticalConvoy,
    Helicopter,
    ExtractionTeam,
    
    // Special
    OvidioGuzmán,
    Civilian,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Faction {
    SinaloaCartel,
    MexicanMilitary,
    Civilian,
}

#[derive(Clone, PartialEq, Default)]
pub enum CaptureStatus {
    #[default]
    Free,
    Pursued,
    Captured,
    Released,
}

#[derive(Clone, PartialEq)]
pub enum EvacuationStatus {
    Normal,
    Panicking,
    Evacuating,
    Evacuated,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ZoneType {
    TresRíos,        // Residential area where Ovidio was captured
    MilitaryBase,    // Government staging area
    CityCenter,      // Commercial district
    Airport,         // Potential escape route
    Highway,         // Main arteries for convoy movement
    Residential,     // General housing areas
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum StrategicPointType {
    Intersection,
    Bridge,
    GovernmentBuilding,
    CommunicationsTower,
    FuelDepot,
    Hospital,
    School,
}

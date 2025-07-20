use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add game states
            .add_state::<GameState>()
            
            // Initialize resources
            .init_resource::<GameMap>()
            .init_resource::<MissionProgress>()
            .init_resource::<MediaMeter>()
            .init_resource::<CivilianPanic>()
            .init_resource::<AsymmetricBalance>()
            .init_resource::<SelectedUnits>()
            .init_resource::<GameTimer>()
            .init_resource::<MissionConfig>()
            
            // Startup systems
            .add_systems(Startup, (
                setup_camera,
                load_culiacan_map,
                spawn_initial_units,
                setup_ui,
            ))
            
            // Game loop systems
            .add_systems(Update, (
                // Core RTS systems
                unit_movement_system,
                unit_combat_system,
                ai_behavior_system,
                
                // Cartel-specific systems
                roadblock_system,
                hostage_pressure_system,
                propaganda_system,
                
                // Mission systems
                mission_trigger_system,
                ovidio_capture_system,
                government_pressure_system,
                
                // UI and feedback
                media_meter_system,
                civilian_panic_system,
                update_ui_system,
                
                // Input handling
                handle_player_input,
                camera_controls,
            ).run_if(in_state(GameState::Playing)))
            
            // Mission-specific systems
            .add_systems(OnEnter(GameState::Mission1), setup_mission_1)
            .add_systems(OnEnter(GameState::Mission2), setup_mission_2)
            .add_systems(OnEnter(GameState::Mission3), setup_mission_3)
            .add_systems(OnEnter(GameState::Mission4), setup_mission_4);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Mission1,
    Mission2, 
    Mission3,
    Mission4,
    GameOver,
    Victory,
}

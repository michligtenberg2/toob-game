use bevy::prelude::*;

#[derive(Component)]
struct Unit {
    health: f32,
    faction: Faction,
}

#[derive(Clone, PartialEq, Debug)]
enum Faction {
    Cartel,
    Military,
    Civilian,
}

#[derive(Resource, Default)]
struct GameState {
    mission_timer: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Battle of Culiacán - Historical RTS".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            game_loop,
            handle_input,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());
    
    // Spawn initial units
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Unit {
            health: 100.0,
            faction: Faction::Cartel,
        },
    ));
    
    info!("Battle of Culiacán initialized - October 17, 2019");
    info!("Historical RTS simulation ready");
}

fn game_loop(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    unit_query: Query<&Unit>,
) {
    game_state.mission_timer += time.delta_seconds();
    
    if game_state.mission_timer > 60.0 && (game_state.mission_timer % 60.0) < 1.0 {
        let cartel_count = unit_query.iter()
            .filter(|u| u.faction == Faction::Cartel)
            .count();
        let military_count = unit_query.iter()
            .filter(|u| u.faction == Faction::Military)
            .count();
            
        info!("Mission Status - Cartel: {}, Military: {}, Time: {:.0}s", 
              cartel_count, military_count, game_state.mission_timer);
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if input.just_pressed(KeyCode::Space) {
        info!("BREAKING: Roadblock deployed in Culiacán streets!");
        
        // Spawn a roadblock
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.4, 0.0),
                custom_size: Some(Vec2::new(40.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                rand::random::<f32>() * 400.0 - 200.0,
                rand::random::<f32>() * 400.0 - 200.0,
                0.0
            )),
            ..default()
        });
    }
    
    if input.just_pressed(KeyCode::KeyR) {
        info!("Government considering retreat due to escalating violence");
        game_state.mission_timer = 0.0; // Reset for demo
    }
    
    if input.just_pressed(KeyCode::Escape) {
        info!("Simulation ended - Historical outcome: Government releases Ovidio");
        std::process::exit(0);
    }
}

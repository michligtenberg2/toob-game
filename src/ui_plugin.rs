use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct StatusText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Main UI root
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            // Top status bar
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Mission title
                    parent.spawn((
                        TextBundle::from_section(
                            "Battle of Culiacán - October 17, 2019",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                    ));
                    
                    // Status text
                    parent.spawn((
                        TextBundle::from_section(
                            "Preparing...",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::YELLOW,
                                ..default()
                            },
                        ),
                        StatusText,
                    ));
                });
            
            // Bottom control hints
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "SPACE: Deploy Roadblock | R: Call Reinforcements | ESC: Exit | F1: Help",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

use crate::GameState;
use crate::Unit;

fn update_ui(
    game_state: Res<GameState>,
    unit_query: Query<&Unit>,
    mut status_text_query: Query<&mut Text, With<StatusText>>,
) {
    if let Ok(mut text) = status_text_query.get_single_mut() {
        let cartel_count = unit_query.iter()
            .filter(|u| u.faction == crate::Faction::Cartel && u.unit_type != crate::UnitType::Roadblock)
            .count();
        let military_count = unit_query.iter()
            .filter(|u| u.faction == crate::Faction::Military)
            .count();
        let ovidio_alive = unit_query.iter()
            .any(|u| u.unit_type == crate::UnitType::Ovidio);
        
        text.sections[0].value = format!(
            "Wave: {} | Cartel: {} | Military: {} | Ovidio: {} | Time: {:.0}s",
            game_state.current_wave,
            cartel_count,
            military_count,
            if ovidio_alive { "SAFE" } else { "CAPTURED" },
            game_state.mission_timer
        );
    }
}

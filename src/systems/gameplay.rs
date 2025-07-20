use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

// === CORE RTS SYSTEMS ===

pub fn unit_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovementTarget, &Unit)>,
) {
    for (mut transform, mut movement, unit) in query.iter_mut() {
        if !movement.is_moving {
            continue;
        }
        
        let current_pos = transform.translation.truncate();
        let direction = (movement.destination - current_pos).normalize_or_zero();
        let distance_to_target = current_pos.distance(movement.destination);
        
        if distance_to_target < 5.0 {
            // Reached destination
            movement.is_moving = false;
            transform.translation = movement.destination.extend(transform.translation.z);
        } else {
            // Move towards destination
            let movement_delta = direction * unit.movement_speed * time.delta_seconds();
            transform.translation += movement_delta.extend(0.0);
        }
    }
}

pub fn unit_combat_system(
    time: Res<Time>,
    mut combat_query: Query<(Entity, &Transform, &mut CombatTarget, &Unit)>,
    mut health_query: Query<&mut Unit>,
    mut commands: Commands,
) {
    let current_time = time.elapsed_seconds();
    
    for (entity, transform, mut combat, unit) in combat_query.iter_mut() {
        // Find nearest enemy
        if combat.target_entity.is_none() || current_time - combat.last_attack_time > combat.attack_cooldown {
            let mut nearest_enemy: Option<(Entity, f32)> = None;
            
            for (other_entity, other_transform, _, other_unit) in combat_query.iter() {
                if entity == other_entity || unit.faction == other_unit.faction {
                    continue;
                }
                
                let distance = transform.translation.distance(other_transform.translation);
                if distance <= unit.range {
                    if let Some((_, nearest_distance)) = nearest_enemy {
                        if distance < nearest_distance {
                            nearest_enemy = Some((other_entity, distance));
                        }
                    } else {
                        nearest_enemy = Some((other_entity, distance));
                    }
                }
            }
            
            combat.target_entity = nearest_enemy.map(|(e, _)| e);
        }
        
        // Attack target if in range and cooldown is ready
        if let Some(target_entity) = combat.target_entity {
            if current_time - combat.last_attack_time >= combat.attack_cooldown {
                if let Ok(mut target_unit) = health_query.get_mut(target_entity) {
                    target_unit.health -= unit.damage;
                    combat.last_attack_time = current_time;
                    
                    info!("{:?} attacks {:?} for {} damage", unit.faction, target_unit.faction, unit.damage);
                    
                    // Remove dead units
                    if target_unit.health <= 0.0 {
                        commands.entity(target_entity).despawn();
                        combat.target_entity = None;
                    }
                }
            }
        }
    }
}

pub fn ai_behavior_system(
    mut military_query: Query<(Entity, &Transform, &mut MovementTarget, &Unit, &MilitaryInfantry)>,
    ovidio_query: Query<&Transform, (With<OvidioGuzmán>, Without<MilitaryInfantry>)>,
    mission_progress: Res<MissionProgress>,
) {
    if let Ok(ovidio_transform) = ovidio_query.get_single() {
        for (_entity, transform, mut movement, _unit, _military) in military_query.iter_mut() {
            match mission_progress.current_mission {
                1 => {
                    // Mission 1: Military moves to capture Ovidio
                    if !movement.is_moving {
                        let distance_to_ovidio = transform.translation.distance(ovidio_transform.translation);
                        if distance_to_ovidio > 50.0 {
                            movement.destination = ovidio_transform.translation.truncate();
                            movement.is_moving = true;
                        }
                    }
                }
                2 => {
                    // Mission 2: Form convoy and extract Ovidio
                    // AI will attempt to move in formation towards military base
                    movement.destination = Vec2::new(600.0, 200.0); // Military base
                    movement.is_moving = true;
                }
                _ => {
                    // Default: Defensive behavior
                    if !movement.is_moving {
                        // Stay near military base for reinforcements
                        movement.destination = Vec2::new(600.0, 200.0);
                        movement.is_moving = true;
                    }
                }
            }
        }
    }
}

// === CARTEL-SPECIFIC SYSTEMS ===

pub fn roadblock_system(
    mut commands: Commands,
    mut roadblock_query: Query<(Entity, &mut Roadblock, &Transform)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    selected_units: Res<SelectedUnits>,
    unit_query: Query<(&Transform, &Unit)>,
) {
    // Deploy roadblocks with 'R' key
    if input.just_pressed(KeyCode::KeyR) {
        for &unit_entity in &selected_units.units {
            if let Ok((transform, unit)) = unit_query.get(unit_entity) {
                if unit.unit_type == UnitType::NarcoTechTruck {
                    // Spawn roadblock at unit location
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.6, 0.3, 0.0), // Brown - burning vehicle
                                custom_size: Some(Vec2::new(25.0, 15.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(transform.translation),
                            ..default()
                        },
                        Roadblock {
                            blocking_strength: 0.8,
                            burn_timer: 300.0, // 5 minutes
                        },
                    ));
                    
                    info!("Roadblock deployed at {:?}", transform.translation);
                }
            }
        }
    }
    
    // Update existing roadblocks
    for (entity, mut roadblock, _transform) in roadblock_query.iter_mut() {
        roadblock.burn_timer -= time.delta_seconds();
        
        if roadblock.burn_timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn hostage_pressure_system(
    mut hostage_query: Query<(&Transform, &mut HostageSquad)>,
    civilian_query: Query<&Transform, (With<Civilian>, Without<HostageSquad>)>,
    mut media_meter: ResMut<MediaMeter>,
    mut mission_progress: ResMut<MissionProgress>,
    time: Res<Time>,
) {
    let mut total_pressure = 0.0;
    
    for (hostage_transform, mut hostage_squad) in hostage_query.iter_mut() {
        // Count nearby civilians as potential hostages
        let mut nearby_civilians = 0;
        
        for civilian_transform in civilian_query.iter() {
            let distance = hostage_transform.translation.distance(civilian_transform.translation);
            if distance < 100.0 {
                nearby_civilians += 1;
            }
        }
        
        hostage_squad.hostages_held = nearby_civilians as u8;
        hostage_squad.pressure_value = nearby_civilians as f32 * 0.1;
        total_pressure += hostage_squad.pressure_value;
    }
    
    // Apply pressure to government
    mission_progress.government_pressure += total_pressure * time.delta_seconds();
    
    // Increase media attention based on hostage situation
    if total_pressure > 0.5 {
        media_meter.attention_level += 0.1 * time.delta_seconds();
        media_meter.attention_level = media_meter.attention_level.min(1.0);
    }
}

pub fn propaganda_system(
    mut media_meter: ResMut<MediaMeter>,
    mut asymmetric_balance: ResMut<AsymmetricBalance>,
    mission_progress: Res<MissionProgress>,
    time: Res<Time>,
) {
    // Cartel gains coordination bonus as media attention increases
    asymmetric_balance.cartel_coordination = 0.5 + (media_meter.attention_level * 0.5);
    
    // Military morale decreases with prolonged operations and media scrutiny
    let time_penalty = (mission_progress.time_elapsed / 3600.0) * 0.1; // Penalty per hour
    let media_penalty = media_meter.attention_level * 0.2;
    
    asymmetric_balance.military_morale = (1.0 - time_penalty - media_penalty).max(0.1);
    
    // Generate headline events
    media_meter.headline_timer -= time.delta_seconds();
    if media_meter.headline_timer <= 0.0 {
        media_meter.headline_timer = 120.0; // New headline every 2 minutes
        
        if media_meter.attention_level > 0.3 {
            info!("BREAKING: Intense gunfight erupts in Culiacán as military operation continues");
            media_meter.international_observers = true;
        }
        
        if media_meter.attention_level > 0.7 {
            info!("INTERNATIONAL: U.S. monitoring situation in Culiacán closely");
            // Note: mission_progress should be ResMut to modify
        }
    }
}

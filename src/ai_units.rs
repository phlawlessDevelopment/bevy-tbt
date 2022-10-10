use crate::grid::{BlockedTiles, GridConfig, GridPosition, SelectedPath, SelectedTile, Tile};
use crate::pathfinding::{calculate_a_star_path, AllUnitsActed};
use crate::player_units::Player;
use crate::states::TurnPhase;
use crate::units::{ActiveUnit, Attack, Health, Movement, Unit};
use bevy::prelude::*;

pub struct AiUnitsPlugin;

#[derive(Component, Debug)]
pub struct Ai;

fn setup_active(mut commands: Commands) {
    commands.insert_resource(ActiveUnit { ..default() });
}

fn move_active_unit(
    time: Res<Time>,
    active: ResMut<ActiveUnit>,
    grid_config: Res<GridConfig>,
    mut selected_path: ResMut<SelectedPath>,
    mut ai_units: Query<(Entity, &mut Transform, &mut GridPosition, &mut Unit), With<Ai>>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let active = active.as_ref();
    if let Some((_e, mut transform, mut grid, mut ai)) = ai_units
        .iter_mut()
        .find(|(e, _t, _g, _ai)| e.id() == active.value)
    {
        let mut should_pop = false;
        if let Some(next_tile) = selected_path.tiles.last() {
            let direction = Vec3::new(
                next_tile.0 as f32 * grid_config.tile_size - grid_config.offset(),
                next_tile.1 as f32 * grid_config.tile_size - grid_config.offset(),
                0.0,
            ) - transform.translation;

            if direction.length() > 1.0 {
                transform.translation += direction.normalize() * time.delta_seconds() * 100.0;
            } else {
                transform.translation = Vec3::new(
                    next_tile.0 as f32 * grid_config.tile_size - grid_config.offset(),
                    next_tile.1 as f32 * grid_config.tile_size - grid_config.offset(),
                    0.0,
                );
                grid.x = next_tile.0;
                grid.y = next_tile.1;
                should_pop = true;
            }
        } else {
            ai.has_acted = true;
            ai.set_changed();
            phase.set(TurnPhase::AISelectUnit).unwrap();
        }
        if should_pop {
            selected_path.tiles.pop();
        }
    }
}

fn spawn_unit(
    x: f32,
    y: f32,
    i: i32,
    grid: (i32, i32),
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_config: &Res<GridConfig>,
    sprite_path: &str,
    movement: i32,
    health: i32,
    dmg: i32,
    range: i32,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(sprite_path),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite: Sprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Unit { has_acted: false })
        .insert(Ai)
        .insert(Name::new(format!("Ai Unit {}", i)))
        .insert(Attack {
            dmg: dmg,
            range: range,
        })
        .insert(Movement { distance: movement })
        .insert(Health {
            max: health,
            value: health,
        })
        .insert(GridPosition {
            x: grid.0,
            y: grid.1,
        })
        .id()
}

fn make_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let mut units = Vec::new();
    let sprites = ["sprites/sword.png", "sprites/fire.png", "sprites/skull.png"];
    let movements = [4, 3, 1];
    let healths = [20, 15, 10];
    let dmgs = [3, 2, 5];
    let ranges = [1, 5, 2];
    let positions = [(8, 7), (7, 8), (8, 8)];
    for i in 0..sprites.len() as i32 {
        let x = (positions[i as usize].0 as f32 * grid_config.tile_size) - grid_config.offset();
        let y = (positions[i as usize].1 as f32 * grid_config.tile_size) - grid_config.offset();
        let unit = spawn_unit(
            x,
            y,
            i,
            positions[i as usize],
            &mut commands,
            &asset_server,
            &grid_config,
            sprites[i as usize],
            movements[i as usize],
            healths[i as usize],
            dmgs[i as usize],
            ranges[i as usize],
        );
        units.push(unit);
    }
    commands
        .spawn()
        .insert(Name::new("Ai Units"))
        .insert_bundle(SpatialBundle::default())
        .push_children(&units);
}

fn select_move(
    active: Res<ActiveUnit>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement, &Attack, &Transform), With<Ai>>,
    mut selected_tile: ResMut<SelectedTile>,
    mut phase: ResMut<State<TurnPhase>>,
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    player_grids_q: Query<(&GridPosition, &Transform), With<Player>>,
    blocked: Res<BlockedTiles>,
    grid_config: Res<GridConfig>,
) {
    let active = active.as_ref();

    if let Some((_e, active_grid)) = unit_grids
        .into_iter()
        .find(|(e, _g)| e.id() == active.value)
    {
        if let Some((_e, active_movement, active_attack, active_transform)) = movements
            .into_iter()
            .find(|(e, _m, _a, _t)| e.id() == active.value)
        {
            let mut reachable: Vec<(&Tile, &GridPosition, &Sprite)> = tiles
                .iter()
                .filter(|(tile, grid, _s)| {
                    calculate_a_star_path(
                        (active_grid.x, active_grid.y),
                        (grid.x, grid.y),
                        &blocked,
                    )
                    .len() as i32
                        <= active_movement.distance
                        && !tile.blocked
                })
                .collect();
            let mut player_grids: Vec<(&GridPosition, &Transform)> =
                player_grids_q.iter().collect();
            player_grids.sort_by(|(g_a, t_a), (g_b, t_b)| {
                calculate_a_star_path((g_a.x, g_a.y), (active_grid.x, active_grid.y), &blocked)
                    .len()
                    .cmp(
                        &calculate_a_star_path(
                            (g_b.x, g_b.y),
                            (active_grid.x, active_grid.y),
                            &blocked,
                        )
                        .len(),
                    )
            });

            let (closest_player_grid, closest_player_transform) = player_grids[0];
            let dist = closest_player_transform
                .translation
                .distance(active_transform.translation);
            if dist <= active_attack.range as f32 * grid_config.tile_size {
                selected_tile.x = active_grid.x;
                selected_tile.y = active_grid.y;
            } else {
                reachable.sort_by(|a, b| {
                    calculate_a_star_path(
                        (closest_player_grid.x, closest_player_grid.y),
                        (a.1.x, a.1.y),
                        &blocked,
                    )
                    .len()
                    .cmp(
                        &calculate_a_star_path(
                            (closest_player_grid.x, closest_player_grid.y),
                            (b.1.x, b.1.y),
                            &blocked,
                        )
                        .len(),
                    )
                });

                selected_tile.x = reachable[0].1.x;
                selected_tile.y = reachable[0].1.y;
            }

            selected_tile.set_changed();
            phase.set(TurnPhase::AIDoMove).unwrap();
        }
    }
}

fn select_unit(
    entities: Query<(Entity, &Unit), With<Ai>>,
    mut active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
    mut all_acted: ResMut<AllUnitsActed>,
) {
    if !all_acted.value {
        for (entity, unit) in entities.into_iter() {
            if unit.has_acted == false {
                active.value = entity.id();
                active.set_changed();
                phase.set(TurnPhase::AISelectMove).unwrap();
                break;
            }
        }
    } else {
        all_acted.value = false;
    }
}

fn select_attacker(
    entities: Query<(Entity, &Unit), With<Ai>>,
    mut active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
    mut all_acted: ResMut<AllUnitsActed>,
) {
    if !all_acted.value {
        for (entity, unit) in entities.into_iter() {
            if unit.has_acted == false {
                active.value = entity.id();
                active.set_changed();
                phase.set(TurnPhase::AISelectTarget).unwrap();
                break;
            }
        }
    } else {
        all_acted.value = false;
    }
}

fn check_enemy_has_moved(
    mut ai_units: Query<&mut Unit, With<Ai>>,
    mut phase: ResMut<State<TurnPhase>>,
    mut all_acted: ResMut<AllUnitsActed>,
) {
    let mut still_to_act = false;
    for unit in ai_units.iter() {
        if !unit.has_acted {
            still_to_act = true;
        }
    }
    if !still_to_act {
        for mut unit in ai_units.iter_mut() {
            unit.has_acted = false;
        }
        all_acted.value = true;
        phase.set(TurnPhase::AISelectAttacker).unwrap();
    }
}
fn check_enemy_has_attacked(
    mut ai_units: Query<&mut Unit, With<Ai>>,
    mut phase: ResMut<State<TurnPhase>>,
    mut all_acted: ResMut<AllUnitsActed>,
) {
    let mut still_to_act = false;
    for unit in ai_units.iter() {
        if !unit.has_acted {
            still_to_act = true;
        }
    }
    if !still_to_act {
        for mut unit in ai_units.iter_mut() {
            unit.has_acted = false;
        }
        all_acted.value = true;
        phase.set(TurnPhase::SelectUnit).unwrap();
    }
}
fn select_target(
    mut ai_units: Query<(Entity, &mut Unit, &GridPosition, &Attack), With<Ai>>,
    mut player_units: Query<(Entity, &GridPosition, &Transform, &mut Health), With<Player>>,
    active: Res<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
    mut commands: Commands,
) {
    if let Some((active, mut active_ai, active_grid, active_attack)) = ai_units
        .iter_mut()
        .find(|(entity, unit, _grid, _attack)| entity.id() == active.value)
    {
        let selection = player_units
            .iter_mut()
            .find(|(e, grid, transform, health)| {
                let dist = std::cmp::max(
                    i32::abs(grid.x - active_grid.x),
                    i32::abs(grid.y - active_grid.y),
                );
                dist > 0 && dist <= active_attack.range
            });
        match selection {
            Some((e, _g, _t, mut target_health)) => {
                target_health.value -= active_attack.dmg;
                if target_health.value <= 0 {
                    commands.entity(e).despawn_recursive();
                }
                phase.set(TurnPhase::AISelectAttacker).unwrap();
                active_ai.has_acted = true;
            }
            None => {
                phase.set(TurnPhase::AISelectAttacker).unwrap();
                active_ai.has_acted = true;
            }
        }
    }
}
fn clear_active_unit(mut active: ResMut<ActiveUnit>) {
    active.value = 0;
}

impl Plugin for AiUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units)
            .add_startup_system(setup_active)
            .add_system_set(SystemSet::on_update(TurnPhase::AIDoMove).with_system(move_active_unit))
            .add_system_set(SystemSet::on_update(TurnPhase::AISelectMove).with_system(select_move))
            .add_system_set(
                SystemSet::on_update(TurnPhase::AISelectUnit)
                    .with_system(check_enemy_has_moved)
                    .with_system(select_unit.after(check_enemy_has_moved)),
            )
            .add_system_set(
                SystemSet::on_update(TurnPhase::AISelectAttacker)
                    .with_system(check_enemy_has_attacked)
                    .with_system(select_attacker.after(check_enemy_has_attacked)),
            )
            .add_system_set(
                SystemSet::on_update(TurnPhase::AISelectTarget).with_system(select_target),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AISelectUnit).with_system(clear_active_unit),
            );
    }
}

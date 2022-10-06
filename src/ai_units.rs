use bevy::prelude::*;

use crate::grid::{
    calculate_manhattan_distance, GridConfig, GridPosition, SelectedPath, SelectedTile, Tile,
};
use crate::pathfinding::AllUnitsActed;
use crate::player_units::Player;
use crate::states::TurnPhase;
use crate::turns::ActiveUnit;
use crate::units::{Health, Movement, Unit};

pub struct AiUnitsPlugin;

#[derive(Component, Debug)]
pub struct Ai {
    pub has_acted: bool,
}

fn setup_active(mut commands: Commands) {
    commands.insert_resource(ActiveUnit { ..default() });
}

fn move_active_unit(
    time: Res<Time>,
    active: ResMut<ActiveUnit>,
    grid_config: Res<GridConfig>,
    mut selected_path: ResMut<SelectedPath>,
    mut ai_units: Query<(Entity, &mut Transform, &mut GridPosition, &mut Ai)>,
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
                transform.translation +=
                    direction.normalize() * time.delta_seconds() * grid_config.tile_size;
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
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_config: &Res<GridConfig>,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/chess_pawn.png"),
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
        .insert(Unit)
        .insert(Ai { has_acted: false })
        .insert(Name::new(format!("Ai Unit {}", i)))
        .insert(Movement { distance: 2 })
        .insert(Health { max: 5, value: 5 })
        .insert(GridPosition {
            x: i / grid_config.rows_cols + grid_config.rows_cols - 1,
            y: i % grid_config.rows_cols,
        })
        .id()
}

fn make_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let mut units = Vec::new();
    for i in 0..4 {
        let x = (((i / grid_config.rows_cols) + grid_config.rows_cols - 1) as f32
            * grid_config.tile_size)
            - grid_config.offset();
        let y = ((i % grid_config.rows_cols) as f32 * grid_config.tile_size) - grid_config.offset();
        let unit = spawn_unit(x, y, i, &mut commands, &asset_server, &grid_config);
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
    movements: Query<(Entity, &Movement), With<Ai>>,
    mut selected_tile: ResMut<SelectedTile>,
    mut phase: ResMut<State<TurnPhase>>,
    tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    player_grids_q: Query<&GridPosition, With<Player>>,
) {
    let active = active.as_ref();
    if let Some((_e, active_grid)) = unit_grids
        .into_iter()
        .find(|(e, _g)| e.id() == active.value)
    {
        if let Some((_e, active_movement)) =
            movements.into_iter().find(|(e, _m)| e.id() == active.value)
        {
            let mut reachable: Vec<(&Tile, &GridPosition, &Sprite)> = tiles
                .iter()
                .filter(|(tile, grid, _s)| {
                    calculate_manhattan_distance(&active_grid, grid) <= active_movement.distance
                        && !tile.blocked
                })
                .collect();
            let mut player_grids: Vec<&GridPosition> = player_grids_q.iter().collect();
            player_grids.sort_by(|a, b| {
                calculate_manhattan_distance(a, active_grid)
                    .cmp(&calculate_manhattan_distance(b, active_grid))
            });
            let closest_player_grid = player_grids[0];
            reachable.sort_by(|a, b| {
                calculate_manhattan_distance(a.1, closest_player_grid)
                    .cmp(&calculate_manhattan_distance(b.1, closest_player_grid))
            });
            selected_tile.x = reachable[0].1.x;
            selected_tile.y = reachable[0].1.y;
            selected_tile.set_changed();
            phase.set(TurnPhase::AIDoMove).unwrap();
        }
    }
}

fn select_unit(
    entities: Query<(Entity, &Ai)>,
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

fn check_enemy_has_acted(
    mut ai_units: Query<&mut Ai>,
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
                    .with_system(check_enemy_has_acted)
                    .with_system(select_unit.after(check_enemy_has_acted)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AISelectUnit).with_system(clear_active_unit),
            );
    }
}

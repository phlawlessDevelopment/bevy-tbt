use crate::{
    pathfinding::calculate_a_star_path,
    player_units::Player,
    states::TurnPhase,
    units::{ActiveUnit, Attack, Health, Movement, SelectedUnit, Spawners, Unit},
};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub blocked: bool,
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Default, Debug)]
pub struct SelectedPath {
    pub tiles: Vec<(i32, i32)>,
}

#[derive(Default)]
pub struct GridConfig {
    pub tile_size: f32,
    pub rows_cols: i32,
}

#[derive(Default)]
pub struct BlockedTiles(pub HashMap<(i32, i32), bool>);

impl GridConfig {
    pub fn offset(&self) -> f32 {
        self.tile_size * (self.rows_cols as f32 * 0.5)
    }
}

#[derive(Default, Debug)]
pub struct SelectedTile {
    pub x: i32,
    pub y: i32,
}

fn highlight_selected_unit(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    selected_res: Res<SelectedUnit>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
) {
    match selected_res.value {
        Some(selected) => {
            for (_t, _grid, mut sprite) in tiles.iter_mut() {
                if sprite.color.a() == 0.1 {
                    sprite.color.set_r(1.0);
                    sprite.color.set_g(1.0);
                    sprite.color.set_b(1.0);
                    sprite.color.set_a(1.0);
                }
            }
            match unit_grids.get(selected) {
                Ok((_e, grid)) => {
                    if let Some((_tile, _grid, mut sprite)) = tiles
                        .iter_mut()
                        .find(|(_t, g, _s)| g.x == grid.x && g.y == grid.y)
                    {
                        sprite.color.set_r(0.0);
                        sprite.color.set_g(1.0);
                        sprite.color.set_b(0.0);
                        sprite.color.set_a(1.0);
                    }
                }
                Err(_) => {}
            }
        }
        None => {}
    }
}
fn highlight_attackable_tiles(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    ai_units: Query<(Entity, &GridPosition), (With<Health>, Without<Player>)>,
    player_units: Query<(Entity, &Attack, &Player, &GridPosition)>,
    active_res: Res<ActiveUnit>,
) {
    match active_res.value {
        Some(active) => match player_units.get(active) {
            Ok((_e, attack, _player, active_grid)) => {
                for (_e, grid) in ai_units.into_iter() {
                    let dist = std::cmp::max(
                        i32::abs(grid.x - active_grid.x),
                        i32::abs(grid.y - active_grid.y),
                    );
                    if dist > 0 && dist <= attack.range {
                        if let Some((_tile, _grid, mut sprite)) = tiles
                            .iter_mut()
                            .find(|(_t, g, _s)| g.x == grid.x && g.y == grid.y)
                        {
                            sprite.color.set_b(0.0);
                            sprite.color.set_g(0.0);
                            sprite.color.set_a(1.0);
                        }
                    }
                }
            }
            Err(_) => {}
        },
        None => {}
    }
}
fn highlight_reachable_tiles(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    active_res: Res<ActiveUnit>,
    blocked_res: Res<BlockedTiles>,
) {
    match active_res.value {
        Some(active) => match unit_grids.get(active) {
            Ok((_e, active_grid)) => match movements.get(active) {
                Ok((_e, active_movement)) => {
                    for (_tile, _grid, mut sprite) in tiles.iter_mut().filter(|(tile, grid, _s)| {
                        let dist = calculate_a_star_path(
                            (active_grid.x, active_grid.y),
                            (grid.x, grid.y),
                            &blocked_res,
                        )
                        .len() as i32;
                        dist > 0 && dist <= active_movement.distance && !tile.blocked
                    }) {
                        sprite.color.set_r(0.0);
                        sprite.color.set_b(0.0);
                        sprite.color.set_a(1.0);
                    }
                }
                Err(_) => {}
            },
            Err(_) => {}
        },
        None => {}
    }
}

pub fn clear_highlighted_tiles_func(tiles: &mut Query<&mut Sprite, With<Tile>>) {
    for mut sprite in tiles.iter_mut() {
        sprite.color.set_r(1.0);
        sprite.color.set_g(1.0);
        sprite.color.set_b(1.0);
        sprite.color.set_a(1.0);
    }
}
fn clear_highlighted_tiles(mut tiles: Query<&mut Sprite, With<Tile>>) {
    for mut sprite in tiles.iter_mut() {
        sprite.color.set_r(1.0);
        sprite.color.set_g(1.0);
        sprite.color.set_b(1.0);
        sprite.color.set_a(1.0);
    }
}

fn spawn_tile(
    x: f32,
    y: f32,
    i: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_config: &Res<GridConfig>,
    blocked: bool,
) -> Entity {
    let mut tile = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(if blocked {
            "sprites/blocked.png"
        } else {
            "sprites/tile.png"
        }),
        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
        ..default()
    });
    tile.insert(Name::new(format!(
        "Tile ({},{})",
        i / grid_config.rows_cols,
        i % grid_config.rows_cols
    )))
    .insert(Tile { blocked })
    .insert(GridPosition {
        x: i / grid_config.rows_cols,
        y: i % grid_config.rows_cols,
    });
    if blocked {
        tile.insert(Obstacle);
    }

    return tile.id();
}

fn create_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
    mut spawners: ResMut<Spawners>,
) {
    let mut tiles = Vec::new();
    let mut rng = rand::thread_rng();
    let positions = [
        (
            grid_config.rows_cols / 2 as i32,
            grid_config.rows_cols / 2 as i32,
        ),
        (
            grid_config.rows_cols / 2 as i32 - 1,
            grid_config.rows_cols / 2 as i32,
        ),
        (
            grid_config.rows_cols / 2 as i32 + 1,
            grid_config.rows_cols / 2 as i32,
        ),
    ];
    for i in 0..81 {
        let x_ = i / grid_config.rows_cols;
        let y_ = i % grid_config.rows_cols;
        let x = (x_ as f32 * grid_config.tile_size) - grid_config.offset();
        let y = (y_ as f32 * grid_config.tile_size) - grid_config.offset();
        let chance = 0.25;
        let roll = rng.gen_range(0.0..1.0);
        let edge = x_ == 0
            || y_ == 0
            || x_ == grid_config.rows_cols - 1
            || y_ == grid_config.rows_cols - 1;
        let tile = spawn_tile(
            x,
            y,
            i,
            &mut commands,
            &asset_server,
            &grid_config,
            !edge && !positions.contains(&(x_, y_)) && roll <= chance,
        );
        tiles.push(tile);
        if edge {
            spawners.ai_locations.push((x, y));
        }
    }
    commands
        .spawn()
        .insert(Name::new("MapTiles"))
        .insert_bundle(SpatialBundle::default())
        .push_children(&tiles);
}

fn set_blocked_tiles(
    units: Query<&GridPosition, With<Unit>>,
    obstacles: Query<&GridPosition, With<Obstacle>>,
    mut tiles: Query<(&GridPosition, &mut Tile)>,
    mut blocked: ResMut<BlockedTiles>,
) {
    for (tile_pos, mut tile) in tiles.iter_mut() {
        if let Some(_unit_pos) = units
            .into_iter()
            .find(|u| u.x == tile_pos.x && u.y == tile_pos.y)
        {
            blocked.0.insert((tile_pos.x, tile_pos.y), true);
            tile.blocked = true;
        } else if let Some(_obs_pos) = obstacles
            .into_iter()
            .find(|o| o.x == tile_pos.x && o.y == tile_pos.y)
        {
            blocked.0.insert((tile_pos.x, tile_pos.y), true);
            tile.blocked = true;
        } else {
            blocked.0.insert((tile_pos.x, tile_pos.y), false);
            tile.blocked = false;
        }
    }
}
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPath>()
            .init_resource::<SelectedTile>()
            .init_resource::<BlockedTiles>()
            .insert_resource(GridConfig {
                tile_size: 64.0,
                rows_cols: 9,
            })
            .add_startup_system(create_level.before(crate::ai_units::spawn_wave))
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectAttacker)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles))
                    .with_system(highlight_reachable_tiles.after(set_blocked_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectTarget)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles))
                    .with_system(highlight_attackable_tiles.after(set_blocked_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::DoMove)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AISelectMove)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AISelectUnit)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_exit(TurnPhase::AIDoMove)
                    .with_system(clear_highlighted_tiles)
                    .with_system(set_blocked_tiles.after(clear_highlighted_tiles)),
            )
            .add_system(highlight_selected_unit);
    }
}

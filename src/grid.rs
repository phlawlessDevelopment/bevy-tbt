use crate::{
    pathfinding::calculate_a_star_path,
    player_units::Player,
    states::TurnPhase,
    units::{Attack, Health, Movement, Unit,ActiveUnit},
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::collections::HashMap;

pub struct GridPlugin;

#[derive(Component, Debug, Inspectable)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub blocked: bool,
}

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

fn highlight_attackable_tiles(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    ai_units: Query<(Entity, &GridPosition), (With<Health>, Without<Player>)>,
    player_units: Query<(Entity, &Attack, &Player, &GridPosition)>,
    active: Res<ActiveUnit>,
) {
    let active = active.as_ref();

    if let Some((_e, attack, player, active_grid)) = player_units
        .into_iter()
        .find(|(e, _a, _p, _g)| e.id() == active.value)
    {
        for (_e, grid) in ai_units.into_iter() {
            let dist = std::cmp::max(
                i32::abs(grid.x - active_grid.x),
                i32::abs(grid.y - active_grid.y),
            );
            if dist > 0 && dist <= attack.range {
                if let Some((_tile, grid, mut sprite)) = tiles
                    .iter_mut()
                    .find(|(_t, g, _s)| g.x == grid.x && g.y == grid.y)
                {
                    sprite.color.set_b(0.0);
                    sprite.color.set_g(0.0);
                    sprite.color.set_a(0.2);
                }
            }
        }
    }
}
fn highlight_reachable_tiles(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    active: Res<ActiveUnit>,
    blocked: Res<BlockedTiles>,
) {
    let active = active.as_ref();

    if let Some((_e, active_grid)) = unit_grids
        .into_iter()
        .find(|(e, _g)| e.id() == active.value)
    {
        if let Some((_e, active_movement)) =
            movements.into_iter().find(|(e, _m)| e.id() == active.value)
        {
            for (_tile, _grid, mut sprite) in tiles.iter_mut().filter(|(tile, grid, _s)| {
                let dist = calculate_a_star_path(
                    (active_grid.x, active_grid.y),
                    (grid.x, grid.y),
                    &blocked,
                )
                .len() as i32;
                dist > 0 && dist <= active_movement.distance && !tile.blocked
            }) {
                sprite.color.set_r(0.0);
                sprite.color.set_b(0.0);
                sprite.color.set_a(0.05);
            }
        }
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
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/dice_empty.png"),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        })
        .insert(Name::new(format!(
            "Tile ({},{})",
            i / grid_config.rows_cols,
            i % grid_config.rows_cols
        )))
        .insert(Tile { blocked: false })
        .insert(GridPosition {
            x: i / grid_config.rows_cols,
            y: i % grid_config.rows_cols,
        })
        .id()
}

fn make_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let mut tiles = Vec::new();
    for i in 0..81 {
        let x = ((i / grid_config.rows_cols) as f32 * grid_config.tile_size) - grid_config.offset();
        let y = ((i % grid_config.rows_cols) as f32 * grid_config.tile_size) - grid_config.offset();
        let tile = spawn_tile(x, y, i, &mut commands, &asset_server, &grid_config);
        tiles.push(tile);
    }
    commands
        .spawn()
        .insert(Name::new("MapTiles"))
        .insert_bundle(SpatialBundle::default())
        .push_children(&tiles);
}

fn set_blocked_tiles(
    units: Query<&GridPosition, With<Unit>>,
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
            .add_startup_system(make_tiles)
            // .add_system(set_blocked_tiles)
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove).with_system(set_blocked_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AISelectMove).with_system(set_blocked_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove)
                    .with_system(clear_highlighted_tiles.after(set_blocked_tiles))
                    .with_system(highlight_reachable_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectTarget)
                    .with_system(clear_highlighted_tiles.after(set_blocked_tiles))
                    .with_system(highlight_attackable_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_exit(TurnPhase::SelectTarget)
                    .with_system(clear_highlighted_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::DoMove).with_system(clear_highlighted_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit).with_system(clear_highlighted_tiles),
            );
    }
}

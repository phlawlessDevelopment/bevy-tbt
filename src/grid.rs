use crate::{
    common::{Label, Selectable},
    states::TurnPhase,
    turns::ActiveUnit,
    units::{Movement, Unit},
};
use bevy::prelude::*;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Tile {
    pub blocked: bool,
}

#[derive(Default)]
pub struct SelectedPath {
    pub tiles: Vec<(i32, i32)>,
}

#[derive(Default)]
pub struct GridConfig {
    pub tile_size: f32,
    pub rows_cols: i32,
}

impl GridConfig {
    fn offset(&self) -> f32 {
        self.tile_size * (self.rows_cols as f32 * 0.5)
    }
}

#[derive(Default, Debug)]
pub struct SelectedTile {
    pub x: i32,
    pub y: i32,
}
pub fn calculate_manhattan_distance(a: &GridPosition, b: &GridPosition) -> i32 {
    i32::abs(b.x - a.x) + i32::abs(b.y - a.y)
}
fn highlight_reachable_tiles(
    mut tiles: Query<(&mut Tile, &GridPosition, &mut Sprite), With<Tile>>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    active: Res<ActiveUnit>,
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
                calculate_manhattan_distance(&active_grid, grid) <= active_movement.distance
                    && !tile.blocked
            }) {
                sprite.color.set_r(1.0);
                sprite.color.set_a(0.3);
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

fn make_tiles(mut commands: Commands,
     asset_server: Res<AssetServer>,
     grid_config: Res<GridConfig>,
    ) {
    for i in 0..81 {
        let x = ((i / grid_config.rows_cols) as f32 * grid_config.tile_size) - grid_config.offset();
        let y = (i % grid_config.rows_cols) as f32 * grid_config.tile_size - grid_config.offset();
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("sprites/dice_empty.png"),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            })
            .insert(Selectable)
            .insert(Label {
                text: String::from(format!("tile {:?}", [i / grid_config.rows_cols, i % grid_config.rows_cols])),
            })
            .insert(Tile { blocked: false })
            .insert(GridPosition { x: i / grid_config.rows_cols, y: i % grid_config.rows_cols });
    }
}

fn set_blocked_tiles(
    units: Query<&GridPosition, With<Unit>>,
    mut tiles: Query<(&GridPosition, &mut Tile)>,
) {
    for (tile_pos, mut tile) in tiles.iter_mut() {
        if let Some(_unit_pos) = units
            .into_iter()
            .find(|u| u.x == tile_pos.x && u.y == tile_pos.y)
        {
            tile.blocked = true;
        } else {
            tile.blocked = false;
        }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPath>()
            .init_resource::<SelectedTile>()
            .insert_resource(GridConfig {
                tile_size: 64.0,
                rows_cols: 9,
            })
            .add_startup_system(make_tiles)
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove).with_system(set_blocked_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove)
                    .with_system(clear_highlighted_tiles)
                    .with_system(highlight_reachable_tiles.after(clear_highlighted_tiles)),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::DoMove).with_system(clear_highlighted_tiles),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit).with_system(clear_highlighted_tiles),
            );
    }
}
